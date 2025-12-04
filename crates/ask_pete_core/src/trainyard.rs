use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The Unified Story Graph
/// Replaces:
/// - ask_pete_core::expert::StoryGraph
/// - ask_pete_server::train_yard::TrainYard
/// - ask_pete_server::models::narrative::NarrativeGraph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryGraph {
    pub id: String,
    pub title: String,
    pub nodes: Vec<StoryNode>,
    pub connections: Vec<Connection>,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// A Node in the Story Graph (Station)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryNode {
    pub id: String,
    pub title: String,
    pub content: String,
    pub x: f64,
    pub y: f64,

    // --- Trainyard Metaphor ---
    #[serde(default)]
    pub station_type: StationType,
    #[serde(default)]
    pub passenger_count: u8, // Cognitive Load
    #[serde(default)]
    pub complexity_level: u8, // 1-3

    // --- AI & Logic ---
    #[serde(default)]
    pub context_prompt: String, // "Stage Directions" for AI
    #[serde(default)]
    pub completion_criteria: String, // "Grading Rubric"

    #[serde(default)]
    pub required_stats: HashMap<String, u32>,

    #[serde(default)]
    pub logic: crate::models::triggers::LogicBlock,

    #[serde(default)]
    pub style: NodeStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum StationType {
    #[default]
    Story,
    Choice,
    Condition,
    Effect,
    Lesson,
    Quiz,
    Project,
    Hub,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Connection {
    pub id: String,
    pub from_node: String,
    pub to_node: String,
    #[serde(default)]
    pub connection_type: ConnectionType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ConnectionType {
    #[default]
    Standard,
    Choice(String),    // Choice ID
    Condition(String), // Condition Logic
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodeStyle {
    #[serde(default)]
    pub contrast: bool,
    #[serde(default)]
    pub alignment: String,
    #[serde(default)]
    pub proximity: f32,
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
