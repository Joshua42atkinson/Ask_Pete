use crate::models::triggers::LogicBlock;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StoryNode {
    pub id: String,
    pub title: String,
    pub content: String,
    pub x: f64,
    pub y: f64,
    // Train Yard Metaphor Fields
    #[serde(default)]
    pub passenger_count: u8, // # of concepts introduced (Cognitive Load)
    #[serde(default)]
    pub complexity_level: u8, // 1-3 difficulty
    #[serde(default)]
    pub learner_profiles: Vec<String>, // Which "trains" can use this
    #[serde(default)]
    pub gardens_active: Vec<String>, // Which activities (Knowledge, Skills, Community)

    // Game State Logic (Triggers)
    #[serde(default)]
    pub required_stats: std::collections::HashMap<String, u32>, // e.g. "Strength" -> 5

    #[serde(default)]
    pub logic: LogicBlock, // [NEW] Condition & Effect

    #[serde(default)]
    pub style: NodeStyle, // [NEW] CRAP Styling
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NodeStyle {
    #[serde(default)]
    pub contrast: bool, // Highlighting
    #[serde(default)]
    pub alignment: String, // "left", "center", "right"
    #[serde(default)]
    pub proximity: f32, // Padding scale (1.0 default)
}

impl Default for NodeStyle {
    fn default() -> Self {
        Self {
            contrast: false,
            alignment: "left".to_string(),
            proximity: 1.0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Connection {
    pub id: String,
    pub from_node: String,
    pub to_node: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StoryGraph {
    pub id: String,
    pub title: String,
    pub nodes: Vec<StoryNode>,
    pub connections: Vec<Connection>,
}
