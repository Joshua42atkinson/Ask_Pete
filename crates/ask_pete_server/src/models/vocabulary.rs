use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Vocabulary {
    pub id: Uuid,
    pub word: String,
    pub definition: String,
    pub grade_level: i32,
    pub tier_level: i32,
    pub cognitive_weight: i32,
    pub domain_tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TrainCar {
    pub id: Uuid,
    pub node_id: Option<Uuid>,
    pub learning_objective: String,
    pub max_capacity: i32,
    pub current_load: i32,
    pub is_locked: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CarCargo {
    pub car_id: Uuid,
    pub vocab_id: Uuid,
    pub is_mastered: bool,
}
