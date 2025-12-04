use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NarrativeGraph {
    pub nodes: HashMap<String, NarrativeNode>,
    pub start_node_id: String,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NarrativeNode {
    pub id: String,
    pub speaker: String,
    pub text: String,
    #[serde(default)]
    pub choices: Vec<NarrativeChoice>,
    #[serde(default)]
    pub events: Vec<NarrativeEvent>,
    #[serde(default)]
    pub position: Option<NodePosition>, // For React Flow visualization
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NarrativeChoice {
    pub text: String,
    pub next_node_id: Option<String>,
    #[serde(default)]
    pub conditions: Vec<NarrativeCondition>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NarrativeEvent {
    pub event_type: String,
    pub payload: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NarrativeCondition {
    pub condition_type: String,
    pub parameters: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NodePosition {
    pub x: f32,
    pub y: f32,
}
