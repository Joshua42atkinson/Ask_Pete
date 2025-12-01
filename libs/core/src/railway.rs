use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VocabularyTier {
    Tier1, // Basic
    Tier2, // High-utility academic
    Tier3, // Domain-specific
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordDefinition {
    pub word: String,
    pub definition: String,
    pub tier: VocabularyTier,
    pub weight: f32,         // Cognitive load
    pub embedding: Vec<f32>, // Vector embedding
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainCar {
    pub id: String,
    pub capacity: u32,
    pub max_cognitive_capacity: f32,
    pub current_load: f32,
    pub cargo: Vec<WordDefinition>,
}

impl TrainCar {
    pub fn new(id: String, capacity: u32, max_cognitive_capacity: f32) -> Self {
        Self {
            id,
            capacity,
            max_cognitive_capacity,
            current_load: 0.0,
            cargo: Vec::new(),
        }
    }

    pub fn add_cargo(&mut self, item: WordDefinition) {
        self.current_load += item.weight;
        self.cargo.push(item);
    }

    pub fn is_overloaded(&self) -> bool {
        self.current_load > self.max_cognitive_capacity
    }
}
