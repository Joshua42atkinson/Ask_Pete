use anyhow::{Context, Result};
use infra_ai::local_inference::GenerationConfig;
use infra_ai::LocalModel;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

/// Standardized output for the Weigh Station
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]

pub struct WordPhysics {
    pub word: String,
    pub definition: String,
    pub grade_level: i32,
    pub tier: i32,
    pub weight: i32, // 1-100
    pub tags: Vec<String>,
}

impl WordPhysics {
    pub fn simple(word: &str) -> Self {
        Self {
            word: word.to_string(),
            definition: "Basic vocabulary word.".to_string(),
            grade_level: 1,
            tier: 1,
            weight: 5,
            tags: vec!["basic".to_string()],
        }
    }
}

pub struct WeighStationService {
    db: PgPool,
    llm: Option<LocalModel>, // Optional to handle missing AI
}

impl WeighStationService {
    pub fn new(db: PgPool, llm: Option<LocalModel>) -> Self {
        Self { db, llm }
    }

    pub fn calculate_intrinsic_load(text: &str) -> f64 {
        // Simple heuristic: (Density * 0.1) + (AvgLength * 0.2)
        let words: Vec<&str> = text.split_whitespace().collect();
        let word_count = words.len() as f64;
        if word_count == 0.0 {
            return 0.0;
        }
        let total_chars: usize = words.iter().map(|w| w.len()).sum();
        let avg_len = total_chars as f64 / word_count;

        // Density proxy (just 1.0 for now as we don't have full density logic)
        let density = 1.0;

        (density * 0.1) + (avg_len * 0.2)
    }

    /// The "Hybrid" Weighing Logic
    pub async fn weigh_word(&self, raw_word: &str) -> Result<WordPhysics> {
        let word = raw_word.trim();

        // 1. CACHE LAYER (Database Check)
        if let Some(cached) = self.check_depot(word).await? {
            return Ok(cached);
        }

        // 2. FAST PATH (Heuristics)
        if word.len() <= 5 {
            let physics = WordPhysics::simple(word);
            // Save to DB so we have it for metrics later
            self.store_in_depot(&physics).await?;
            return Ok(physics);
        }

        // 3. SLOW PATH (AI Inference)
        if let Some(llm) = &self.llm {
            self.ask_pete_to_weigh(llm, word).await
        } else {
            // Fallback if AI is missing
            let physics = WordPhysics::simple(word);
            self.store_in_depot(&physics).await?;
            Ok(physics)
        }
    }

    async fn check_depot(&self, word: &str) -> Result<Option<WordPhysics>> {
        // Use query_as function to avoid compile-time schema check
        let row = sqlx::query_as::<_, WordPhysics>(
            r#"
            SELECT word, definition, grade_level, tier, weight, tags
            FROM vocabulary_words WHERE word = $1
            "#,
        )
        .bind(word)
        .fetch_optional(&self.db)
        .await?;

        Ok(row)
    }

    async fn ask_pete_to_weigh(&self, llm: &LocalModel, word: &str) -> Result<WordPhysics> {
        let prompt = format!(
            r#"Analyze the word: "{}". Return JSON: {{
                "word": "{}",
                "definition": "Simple definition",
                "grade_level": <int 0-12>,
                "tier": <int 1-3>,
                "weight": <int 1-100 representing cognitive load>,
                "tags": ["tag1", "tag2"]
            }}"#,
            word, word
        );

        let config = GenerationConfig {
            max_tokens: 300,
            temperature: 0.2,
            ..Default::default()
        };

        let json_response = llm.generate(prompt, config).await?;

        // Clean and parse
        let clean_json = infra_ai::json_utils::extract_json_from_text(&json_response)
            .unwrap_or_else(|| json_response.to_string());

        let physics: WordPhysics =
            serde_json::from_str(&clean_json).context("Failed to parse Pete's weighing ticket")?;

        self.store_in_depot(&physics).await?;
        Ok(physics)
    }

    async fn store_in_depot(&self, p: &WordPhysics) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO vocabulary_words (word, definition, grade_level, tier, weight, tags)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (word) DO UPDATE 
            SET weight = $5, definition = $2, grade_level = $3, tier = $4, tags = $6
            "#,
        )
        .bind(&p.word)
        .bind(&p.definition)
        .bind(p.grade_level)
        .bind(p.tier)
        .bind(p.weight)
        .bind(&p.tags)
        .execute(&self.db)
        .await?;
        Ok(())
    }
}
