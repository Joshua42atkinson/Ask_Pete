use crate::expert::{StoryGraph, StoryNode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "ssr")]
use bevy_ecs::prelude::Resource;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ssr", derive(Resource))]
pub struct GraphManager {
    pub nodes: HashMap<String, StoryNode>,
    pub current_node_id: Option<String>,
    pub graph_title: String,
}

impl GraphManager {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            current_node_id: None,
            graph_title: "Empty Graph".to_string(),
        }
    }

    pub fn load_graph(&mut self, graph: StoryGraph) {
        self.graph_title = graph.title;
        self.nodes.clear();
        for node in graph.nodes {
            self.nodes.insert(node.id.clone(), node);
        }
        // Naive start: find node with id "1" or first available
        if self.current_node_id.is_none() {
            if self.nodes.contains_key("1") {
                self.current_node_id = Some("1".to_string());
            } else if let Some(key) = self.nodes.keys().next() {
                self.current_node_id = Some(key.clone());
            }
        }
    }

    pub fn get_current_node(&self) -> Option<&StoryNode> {
        self.current_node_id
            .as_ref()
            .and_then(|id| self.nodes.get(id))
    }

    pub fn advance_to(&mut self, node_id: String) -> Option<&StoryNode> {
        if self.nodes.contains_key(&node_id) {
            self.current_node_id = Some(node_id.clone());
            self.nodes.get(&node_id)
        } else {
            None
        }
    }
}
