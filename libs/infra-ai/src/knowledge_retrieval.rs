use sqlx::PgPool;
use std::collections::HashMap;

/// A knowledge chunk retrieved from the database
#[derive(Debug, Clone)]
pub struct KnowledgeChunk {
    pub id: String,
    pub title: String,
    pub content: String,
    pub source_type: String,
    pub relevance_score: f32,
}

/// Retrieve relevant knowledge chunks from the database based on a user query
///
/// This uses PostgreSQL ILIKE for keyword-based search. It's simple but effective
/// for Mistral 7B's 4K context window limitation.
///
/// # Arguments
/// * `query` - The user's question/message
/// * `pool` - Database connection pool
/// * `max_chunks` - Maximum number of chunks to return (default: 3)
///
/// # Returns
/// A vector of relevant knowledge chunks, ranked by relevance score
pub async fn retrieve_knowledge(
    query: &str,
    pool: &PgPool,
    max_chunks: Option<usize>,
) -> Result<Vec<KnowledgeChunk>, Box<dyn std::error::Error>> {
    let limit = max_chunks.unwrap_or(3);

    // Extract keywords from query (simple approach - split on whitespace and filter stop words)
    let keywords = extract_keywords(query);

    if keywords.is_empty() {
        return Ok(Vec::new());
    }

    // Build ILIKE query for each keyword
    // We'll search in both title and content
    let mut chunks_map: HashMap<String, (KnowledgeChunk, usize)> = HashMap::new();

    for keyword in &keywords {
        let search_pattern = format!("%{}%", keyword);

        #[derive(sqlx::FromRow)]
        struct KnowledgeRow {
            id: uuid::Uuid,
            title: String,
            content: String,
            source_type: String,
        }

        let rows: Vec<KnowledgeRow> = sqlx::query_as(
            r#"
            SELECT id, title, content, source_type
            FROM knowledge_sources
            WHERE title ILIKE $1 OR content ILIKE $1
            LIMIT 10
            "#,
        )
        .bind(&search_pattern)
        .fetch_all(pool)
        .await?;

        for row in rows {
            let id = row.id.to_string();

            // Count how many keywords match this chunk
            chunks_map
                .entry(id.clone())
                .and_modify(|(_, count)| *count += 1)
                .or_insert_with(|| {
                    (
                        KnowledgeChunk {
                            id: id.clone(),
                            title: row.title,
                            content: row.content,
                            source_type: row.source_type,
                            relevance_score: 0.0,
                        },
                        1,
                    )
                });
        }
    }

    // Convert to vector and calculate relevance scores
    let mut chunks: Vec<KnowledgeChunk> = chunks_map
        .into_iter()
        .map(|(_, (mut chunk, keyword_count))| {
            // Simple relevance score: percentage of query keywords that matched
            chunk.relevance_score = keyword_count as f32 / keywords.len() as f32;
            chunk
        })
        .collect();

    // Sort by relevance score (descending)
    chunks.sort_by(|a, b| {
        b.relevance_score
            .partial_cmp(&a.relevance_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Return top N chunks
    chunks.truncate(limit);
    Ok(chunks)
}

/// Extract keywords from a query by removing common stop words
///
/// This is a simple implementation - just splits on whitespace and filters
/// common English stop words. Could be improved with NLP libraries.
fn extract_keywords(query: &str) -> Vec<String> {
    let stop_words = [
        "a", "an", "and", "are", "as", "at", "be", "by", "for", "from", "has", "he", "in", "is",
        "it", "its", "of", "on", "that", "the", "to", "was", "what", "when", "where", "who",
        "will", "with", "the", "i", "you", "can", "do", "does", "how", "tell", "me", "about",
    ];

    query
        .to_lowercase()
        .split_whitespace()
        .filter(|word| {
            // Filter out stop words and very short words
            !stop_words.contains(word) && word.len() > 2
        })
        .map(|s| s.to_string())
        .collect()
}

/// Format knowledge chunks for injection into prompts
///
/// Returns a formatted string that can be inserted into the AI prompt
pub fn format_chunks_for_prompt(chunks: &[KnowledgeChunk]) -> String {
    if chunks.is_empty() {
        return String::new();
    }

    let mut formatted = String::from("## Relevant Knowledge\n\n");
    formatted.push_str(
        "I have access to the following information that may help answer your question:\n\n",
    );

    for (idx, chunk) in chunks.iter().enumerate() {
        formatted.push_str(&format!(
            "**Source {}: {}** ({})\n{}\n\n",
            idx + 1,
            chunk.title,
            chunk.source_type,
            chunk.content.trim()
        ));
    }

    formatted.push_str("---\n\n");
    formatted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_keywords() {
        let query = "What is the Purdue Bell Tower called?";
        let keywords = extract_keywords(query);

        assert!(keywords.contains(&"purdue".to_string()));
        assert!(keywords.contains(&"bell".to_string()));
        assert!(keywords.contains(&"tower".to_string()));
        assert!(keywords.contains(&"called".to_string()));

        // Stop words should be filtered
        assert!(!keywords.contains(&"what".to_string()));
        assert!(!keywords.contains(&"is".to_string()));
        assert!(!keywords.contains(&"the".to_string()));
    }

    #[test]
    fn test_format_chunks_for_prompt() {
        let chunks = vec![KnowledgeChunk {
            id: "1".to_string(),
            title: "Purdue Bell Tower".to_string(),
            content: "The Purdue Bell Tower is called the Campanile.".to_string(),
            source_type: "txt".to_string(),
            relevance_score: 0.8,
        }];

        let formatted = format_chunks_for_prompt(&chunks);

        assert!(formatted.contains("Relevant Knowledge"));
        assert!(formatted.contains("Purdue Bell Tower"));
        assert!(formatted.contains("Campanile"));
    }
}
