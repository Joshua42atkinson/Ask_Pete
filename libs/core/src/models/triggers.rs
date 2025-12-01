use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a condition that must be met for a transition to be valid.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TriggerCondition {
    /// Checks if a variable (e.g., "strength") is greater than a value.
    GreaterThan { variable: String, value: f32 },
    /// Checks if a variable is less than a value.
    LessThan { variable: String, value: f32 },
    /// Checks if a variable equals a value.
    Equals { variable: String, value: f32 },
    /// Checks if the player has a specific item (by ID).
    HasItem { item_id: String },
    /// Always true (default).
    None,
}

/// Represents an effect that happens when a node is visited.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TriggerEffect {
    /// Adds (or subtracts) to a variable.
    ModifyVariable { variable: String, delta: f32 },
    /// Adds an item to inventory.
    GrantItem { item_id: String },
    /// Removes an item from inventory.
    ConsumeItem { item_id: String },
    /// No effect.
    None,
}

/// A container for logic attached to a Node or Connection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogicBlock {
    pub condition: TriggerCondition,
    pub effect: TriggerEffect,
}

impl Default for LogicBlock {
    fn default() -> Self {
        Self {
            condition: TriggerCondition::None,
            effect: TriggerEffect::None,
        }
    }
}

/// Represents the dynamic state of a playthrough.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameState {
    pub variables: HashMap<String, f32>,
    pub inventory: Vec<String>,
    pub visited_nodes: Vec<String>,
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_var(&self, key: &str) -> f32 {
        *self.variables.get(key).unwrap_or(&0.0)
    }

    pub fn set_var(&mut self, key: String, value: f32) {
        self.variables.insert(key, value);
    }

    pub fn has_item(&self, item_id: &str) -> bool {
        self.inventory.contains(&item_id.to_string())
    }
}
