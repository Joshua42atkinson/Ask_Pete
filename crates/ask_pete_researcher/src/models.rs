use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub link: Option<String>, // URL to the document/video
    pub link_text: String,    // "Read Report", "Watch Video"
    pub icon: String,         // SVG string
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerCommand {
    pub command_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameTurn {
    pub player_command: String,
    pub ai_narrative: String,
    pub system_message: Option<String>,
    // pub updated_character: PlayerCharacter,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub description: String,
    pub filename: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DownloadProgress {
    pub model_id: String,
    pub progress: f32,  // 0.0 to 1.0
    pub status: String, // "downloading", "completed", "error"
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
}
