use common::expert::{StoryGraph, StoryNode};

#[derive(Clone, Debug, PartialEq)]
pub struct GameState {
    pub current_node_id: Option<String>,
    pub history: Vec<String>, // Log of visited node titles
    pub is_finished: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            current_node_id: None,
            history: Vec::new(),
            is_finished: false,
        }
    }
}

#[derive(Clone)]
pub struct GameEngine {
    graph: StoryGraph,
    state: GameState,
}

impl GameEngine {
    pub fn new(graph: StoryGraph) -> Self {
        // Find the first node as the start node
        let start_node = graph.nodes.first().map(|n| n.id.clone());

        let mut engine = Self {
            graph,
            state: GameState::new(),
        };

        if let Some(start) = start_node {
            engine.state.current_node_id = Some(start);
        }

        engine
    }

    pub fn get_current_node(&self) -> Option<&StoryNode> {
        self.state
            .current_node_id
            .as_ref()
            .and_then(|id| self.graph.nodes.iter().find(|n| &n.id == id))
    }

    pub fn get_options(&self) -> Vec<(String, String)> {
        // Find all connections originating from the current node
        if let Some(current_id) = &self.state.current_node_id {
            self.graph
                .connections
                .iter()
                .filter(|conn| &conn.from_node == current_id)
                .map(|conn| {
                    // Find the title of the target node to display as the button text
                    let target_title = self
                        .graph
                        .nodes
                        .iter()
                        .find(|n| n.id == conn.to_node)
                        .map(|n| n.title.clone())
                        .unwrap_or_else(|| "Unknown Path".to_string());
                    (conn.to_node.clone(), target_title)
                })
                .collect()
        } else {
            vec![]
        }
    }

    pub fn make_choice(&mut self, target_id: String) {
        // Validate the move exists (optional security check)
        self.state.current_node_id = Some(target_id);

        if let Some(node) = self.get_current_node() {
            self.state.history.push(node.title.clone());
        }

        // Check if leaf node (no options) -> Game Over
        if self.get_options().is_empty() {
            self.state.is_finished = true;
        }
    }
}
