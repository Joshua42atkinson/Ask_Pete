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
    db: Option<PgPool>,
    llm: Option<LocalModel>, // Optional to handle missing AI
}

impl WeighStationService {
    pub fn new(db: Option<PgPool>, llm: Option<LocalModel>) -> Self {
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
        let pool = match &self.db {
            Some(p) => p,
            None => return Ok(None),
        };
        // Use query_as function to avoid compile-time schema check
        let row = sqlx::query_as::<_, WordPhysics>(
            r#"
            SELECT word, definition, grade_level, tier, weight, tags
            FROM vocabulary_words WHERE word = $1
            "#,
        )
        .bind(word)
        .fetch_optional(pool)
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
        let pool = match &self.db {
            Some(p) => p,
            None => return Ok(()),
        };
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
        .execute(pool)
        .await?;
        Ok(())
    }

    // [NEW] Weigh Node Logic
    pub async fn weigh_node(&self, content: &str) -> Result<NodePhysics> {
        // 1. Heuristic Check (Fast Path)
        // If content is very short, don't waste AI cycles
        if content.len() < 50 {
            return Ok(NodePhysics {
                complexity_score: 1,
                concept_count: 1,
                reasoning: "Short text, minimal load.".to_string(),
            });
        }

        // 2. AI Inference
        if let Some(llm) = &self.llm {
            let prompt = pete_core::prompts::weigh_station::generate_weigh_prompt(content);
            let system_prompt = pete_core::prompts::weigh_station::WEIGH_STATION_SYSTEM_PROMPT;

            let config = GenerationConfig {
                max_tokens: 300,
                temperature: 0.1, // Low temp for consistent scoring
                ..Default::default()
            };

            // We need a way to pass system prompt.
            // Assuming LocalModel supports it or we prepend it.
            // For now, prepending to user prompt if API doesn't support separate system prompt.
            // But LocalModel::generate takes (prompt, config).
            // Let's prepend.
            let full_prompt = format!("{}\n\n{}", system_prompt, prompt);

            let json_response = llm.generate(full_prompt, config).await?;

            let clean_json = infra_ai::json_utils::extract_json_from_text(&json_response)
                .unwrap_or_else(|| json_response.to_string());

            let physics: NodePhysics = serde_json::from_str(&clean_json).unwrap_or(NodePhysics {
                complexity_score: 5,
                concept_count: 3,
                reasoning: "Failed to parse AI response, defaulting to medium.".to_string(),
            });

            Ok(physics)
        } else {
            // Fallback
            Ok(NodePhysics {
                complexity_score: 3,
                concept_count: 1,
                reasoning: "AI offline, heuristic default.".to_string(),
            })
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodePhysics {
    pub complexity_score: i32,
    pub concept_count: i32,
    pub reasoning: String,
}
