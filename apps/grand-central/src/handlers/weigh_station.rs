use anyhow::{Context, Result};
use infra_ai::local_inference::{GemmaModel, GenerationConfig};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

/// The "Ticket" Pete gives back after weighing a word
#[derive(Debug, Serialize, Deserialize)]
pub struct WordPhysics {
    pub word: String,
    pub definition: String,
    pub grade_level: i32,
    pub tier: i32,
    pub weight: i32, // 1-100
    pub tags: Vec<String>,
}

use std::sync::Arc;
use tokio::sync::Mutex;

// use crate::ai::memory::{Document, VectorStore}; // [NEW]

pub struct WeighStation {
    db: PgPool,
    llm: GemmaModel,
    // memory: Option<Arc<crate::ai::memory::LanceDbConnection>>, // [NEW]
}

impl WeighStation {
    pub fn new(
        db: PgPool,
        llm: infra_ai::GemmaModel,
        // memory: Option<Arc<crate::ai::memory::LanceDbConnection>>, // [NEW]
    ) -> Self {
        Self { db, llm } //, memory }
    }

    /// The core loop: Takes a raw word, weighs it, stores it.
    pub async fn process_word(&mut self, raw_word: &str) -> Result<WordPhysics> {
        println!("⚖️  Weighing: '{}'...", raw_word);

        // 1. Construct the Prompt (The "Scale")
        let prompt = format!(
            r#"You are an Expert Instructional Designer and Linguist. 
            Analyze the English word: "{}" for a K-12 Curriculum.
            
            Return ONLY a JSON object with this exact structure:
            {{
                "word": "{}",
                "definition": "A simple, clear definition for a student.",
                "grade_level": <integer 0-12>,
                "tier": <integer 1, 2, or 3>,
                "weight": <integer 1-100 representing cognitive load difficulty>,
                "tags": ["<tag1>", "<tag2>"]
            }}
            
            RUBRIC:
            - Tier 1: Basic conversation (Weight 1-10)
            - Tier 2: Academic cross-curriculum (Weight 11-50)
            - Tier 3: Domain specific/Technical (Weight 51-100)
            "#,
            raw_word, raw_word
        );

        // 2. Ask Pete (The Inference)
        let config = GenerationConfig {
            max_tokens: 300,
            temperature: 0.2, // Low temp for precision/consistency
            ..Default::default()
        };

        let json_response = self.llm.generate(prompt, config).await?;

        // 3. Parse the Physics
        // Clean markdown code blocks if present
        let clean_json = json_response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        let physics: WordPhysics =
            serde_json::from_str(clean_json).context("Failed to parse Pete's weighing ticket")?;

        // 4. Store in the Depot
        self.store_in_depot(&physics).await?;

        println!("✅ Stored: {} (Weight: {})", physics.word, physics.weight);
        Ok(physics)
    }

    async fn store_in_depot(&self, p: &WordPhysics) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO vocabulary_words 
            (word, definition, grade_level, tier, weight, tags)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (word) DO UPDATE 
            SET weight = $5, -- Update weight if it changed
                definition = $2,
                grade_level = $3,
                tier = $4,
                tags = $6
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

    /// Calculates the "Intrinsic Load" (Cognitive Weight) of a text.
    pub fn calculate_intrinsic_load(text: &str) -> TextAnalysisResult {
        let words: Vec<&str> = text.split_whitespace().collect();
        let word_count = words.len();

        // Simple sentence splitting by punctuation
        let sentence_count = text
            .split(|c| c == '.' || c == '!' || c == '?')
            .filter(|s| !s.trim().is_empty())
            .count();

        // Avoid division by zero
        let safe_word_count = word_count.max(1);
        let safe_sentence_count = sentence_count.max(1);

        // Calculate Lexical Density (Simplified: assume words > 6 chars are "content" words for now)
        // In a real implementation, we'd use a POS tagger to find Nouns/Verbs vs Prepositions.
        let complex_words = words.iter().filter(|w| w.len() > 6).count();
        let lexical_density = (complex_words as f32 / safe_word_count as f32) * 100.0;

        // Calculate Average Sentence Length
        let avg_sentence_length = safe_word_count as f32 / safe_sentence_count as f32;

        // Formula for Intrinsic Load (Heuristic)
        // Load = (Density * 0.1) + (AvgSentenceLength * 0.2)
        let raw_load = (lexical_density * 0.1) + (avg_sentence_length * 0.2);

        // Normalize to 0-10 scale (approximate)
        let intrinsic_load = raw_load.clamp(0.0, 10.0);

        TextAnalysisResult {
            word_count,
            sentence_count,
            lexical_density,
            intrinsic_load,
            is_overloaded: intrinsic_load > 7.0, // Threshold for "Red Zone"
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextAnalysisResult {
    pub word_count: usize,
    pub sentence_count: usize,
    pub lexical_density: f32,
    pub intrinsic_load: f32, // 0.0 to 10.0
    pub is_overloaded: bool,
}
