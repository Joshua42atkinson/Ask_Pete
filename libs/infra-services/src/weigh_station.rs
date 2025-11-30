use pete_core::railway::{TrainCar, VocabularyTier, WordDefinition};

pub struct WeighStation;

impl WeighStation {
    /// Calculates the "Intrinsic Load" (Weight) of a word based on heuristics.
    /// In a full implementation, this would use an LLM or frequency dictionary.
    pub fn weigh_cargo(word: &str) -> WordDefinition {
        let length = word.len();
        let (weight, tier) = match length {
            0..=4 => (5.0, VocabularyTier::Tier1), // Short words (e.g., "Cat")
            5..=8 => (20.0, VocabularyTier::Tier2), // Medium words (e.g., "Planet")
            _ => (50.0, VocabularyTier::Tier3),    // Long words (e.g., "Photosynthesis")
        };

        WordDefinition {
            word: word.to_string(),
            definition: format!("Definition of {}", word), // Placeholder
            weight,
            tier,
            embedding: vec![], // Placeholder for actual embeddings
        }
    }

    /// Validates if a TrainCar is safe to depart (not overloaded).
    pub fn validate_car(car: &TrainCar) -> Result<bool, String> {
        if car.is_overloaded() {
            return Err(format!(
                "SAFETY LOCKOUT: Cognitive Overload Detected! Current: {} / Max: {}",
                car.current_load, car.max_cognitive_capacity
            ));
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weigh_cargo() {
        let w1 = WeighStation::weigh_cargo("Cat");
        assert_eq!(w1.weight, 5.0);
        assert_eq!(w1.tier, VocabularyTier::Tier1);

        let w2 = WeighStation::weigh_cargo("Planet");
        assert_eq!(w2.weight, 20.0);
        assert_eq!(w2.tier, VocabularyTier::Tier2);

        let w3 = WeighStation::weigh_cargo("Photosynthesis");
        assert_eq!(w3.weight, 50.0);
        assert_eq!(w3.tier, VocabularyTier::Tier3);
    }
}
