use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Enum representing the detected sentiment of a journal entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SentimentEnum {
    Despair,
    Joy,
    Frustration,
    Neutral,
    Curiosity,
}

/// THE "TOXIC" ASSET (High Liability)
/// Strictly Private. Never Syncs to Server without specific "Granular Consent".
/// This struct represents the raw data on the client device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateJournalEntry {
    pub id: Uuid,      // Local UUID
    pub user_id: Uuid, // Link to User
    pub timestamp: DateTime<Utc>,

    // The "Psychogenic" Payload (PPRA Trigger)
    pub raw_content: String,         // "I hate my mom, I want to quit..."
    pub audio_blob: Option<Vec<u8>>, // Voice recording (Biometric Data)

    // AI Analysis (Local 1B Model)
    pub detected_sentiment: SentimentEnum,
    pub ai_response_text: String, // The Sage's reply
}

/// THE RESEARCH ASSET (Safe for Server)
/// Anonymized and sanitized data packet for the Research Nexus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchPacket {
    // Hashed ID: Impossible to reverse-engineer to student name
    pub research_id: String,

    // Pedagogical Metadata (Safe for Research)
    pub vocab_complexity_score: f64,
    pub word_count: usize,

    // The AI Intervention Type (Not the text)
    pub intervention_type: String,

    // Latency Metrics (Technical Performance)
    pub time_on_task_ms: u64,

    // Boolean Flags (No narrative context)
    pub sentiment_flag: String,
}

impl PrivateJournalEntry {
    /// The "Airlock" Transformation
    /// Converts a private entry into a safe research packet.
    /// This function runs strictly on the Client. It destroys the narrative and keeps the metrics.
    pub fn sanitize_for_research(&self, salt: &str) -> ResearchPacket {
        // 1. Hash the User ID with a salt to create a persistent but anonymous Research ID
        let mut hasher = Sha256::new();
        hasher.update(self.user_id.as_bytes());
        hasher.update(salt.as_bytes());
        let result = hasher.finalize();
        let research_id = format!("{:x}", result);

        // 2. Calculate Pedagogical Metadata
        let word_count = self.raw_content.split_whitespace().count();
        let vocab_complexity_score = calculate_flesch_kincaid(&self.raw_content);

        // 3. Determine Sentiment Flag (Coarse-grained)
        let sentiment_flag = match self.detected_sentiment {
            SentimentEnum::Despair => "NEGATIVE_HIGH",
            SentimentEnum::Frustration => "NEGATIVE_MODERATE",
            SentimentEnum::Joy => "POSITIVE",
            SentimentEnum::Curiosity => "POSITIVE_ACTIVE",
            SentimentEnum::Neutral => "NEUTRAL",
        }
        .to_string();

        // 4. Construct the Research Packet
        ResearchPacket {
            research_id,
            vocab_complexity_score,
            word_count,
            intervention_type: "Scaffolding_Vocabulary_Hint".to_string(), // Placeholder for now
            time_on_task_ms: 4500, // Placeholder, would come from actual session tracking
            sentiment_flag,
        }
    }
}

/// Helper function to calculate a rough Flesch-Kincaid grade level
/// This is a simplified implementation for the prototype.
fn calculate_flesch_kincaid(text: &str) -> f64 {
    let sentences = text
        .split(|c| c == '.' || c == '!' || c == '?')
        .filter(|s| !s.is_empty())
        .count()
        .max(1);
    let words = text.split_whitespace().count().max(1);
    let syllables = count_syllables(text);

    let score =
        0.39 * (words as f64 / sentences as f64) + 11.8 * (syllables as f64 / words as f64) - 15.59;
    score.max(0.0) // Ensure non-negative
}

/// Very basic syllable counter (heuristic)
fn count_syllables(text: &str) -> usize {
    let mut count = 0;
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
        let mut word_syllables = 0;
        let mut prev_is_vowel = false;

        for c in word.chars() {
            let is_vowel = vowels.contains(&c);
            if is_vowel && !prev_is_vowel {
                word_syllables += 1;
            }
            prev_is_vowel = is_vowel;
        }

        // Adjust for silent 'e' at the end
        if word.ends_with('e') && word_syllables > 1 {
            word_syllables -= 1;
        }

        count += word_syllables.max(1);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitization_removes_pii() {
        let entry = PrivateJournalEntry {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            raw_content: "I hate my mom, I want to quit school.".to_string(),
            audio_blob: None,
            detected_sentiment: SentimentEnum::Despair,
            ai_response_text: "It sounds like you are in pain.".to_string(),
        };

        let salt = "test_salt";
        let packet = entry.sanitize_for_research(salt);

        // Verify PII is gone
        assert_ne!(packet.research_id, entry.user_id.to_string());

        // Verify metrics are preserved
        assert_eq!(packet.word_count, 9);
        assert_eq!(packet.sentiment_flag, "NEGATIVE_HIGH");

        // Verify content is NOT present (implicit by struct definition, but good to check logic)
        // ResearchPacket doesn't even have a content field.
    }

    #[test]
    fn test_flesch_kincaid() {
        let text = "The quick brown fox jumps over the lazy dog.";
        let score = calculate_flesch_kincaid(text);
        assert!(score > 0.0, "Score should be positive, got {}", score);
    }
}
