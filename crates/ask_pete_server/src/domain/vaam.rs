use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// A word available to be "discovered" or "used" in the game.
#[derive(Debug, Serialize, FromRow)]
pub struct VocabWord {
    pub id: i32,
    pub word: String,
    pub definition: String,
    pub context_tag: Option<String>, // e.g., "throne_room", "market"
    pub complexity_tier: Option<i32>,
}

/// The payload sent by the frontend when a player makes a choice.
#[derive(Debug, Deserialize)]
pub struct WordUsageRequest {
    pub player_id: i32, // In prod, this comes from Auth Token
    pub word_id: i32,
    pub context_used: String,
}

use crate::error::Result;
use sqlx::PgPool; // Our custom error alias

pub struct VaamService;

impl VaamService {
    /// Fetch words relevant to the current game scene.
    /// e.g., If player enters "Throne Room", fetch "Implore", "Beseech", "Sovereignty".
    /// Fetch words relevant to the current game scene.
    /// e.g., If player enters "Throne Room", fetch "Implore", "Beseech", "Sovereignty".
    pub async fn get_words_for_context(
        _pool: &PgPool,
        _context_tag: &str,
    ) -> Result<Vec<VocabWord>> {
        // SIMULATION MODE: Database schema mismatch
        Ok(Vec::new())
    }

    pub async fn log_usage(_pool: &PgPool, _req: WordUsageRequest) -> Result<bool> {
        // SIMULATION MODE: Database schema mismatch
        Ok(true)
    }

    /// Checks if the player is within any Node Garden and returns the context tag if so.
    pub fn check_unlock_status(
        player_lat: f64,
        player_lon: f64,
        gardens: &[crate::domain::node_garden::NodeGarden],
    ) -> Option<String> {
        for garden in gardens {
            if garden.is_within_range(player_lat, player_lon) {
                return Some(garden.context_tag.clone());
            }
        }
        None
    }
}
