use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[cfg(feature = "bevy_ecs")]
use bevy_ecs::prelude::Component;

// --- Locomotive Components ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy_ecs", derive(Component))]
pub struct LocomotiveStats {
    pub archetype: String, // Hero, Sage, etc.
    // Core Attributes (The Engine Block)
    pub traction: f32,   // STR: Pulling power
    pub velocity: f32,   // DEX: Agility/Speed
    pub efficiency: f32, // CON: Fuel economy
    pub analysis: f32,   // INT: Logic/Diagnostics
    pub signaling: f32,  // WIS: Intuition
    pub coupling: f32,   // CHA: Social connection
}

impl Default for LocomotiveStats {
    fn default() -> Self {
        Self {
            archetype: "Novice".to_string(),
            traction: 10.0,
            velocity: 10.0,
            efficiency: 1.0,
            analysis: 10.0,
            signaling: 10.0,
            coupling: 10.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy_ecs", derive(Component))]
pub struct FuelTank {
    pub coal: f32,  // Potential Energy (Attention)
    pub steam: f32, // Kinetic Energy (Mastery/Action Points)
    pub max_coal: f32,
    pub max_steam: f32,
}

impl Default for FuelTank {
    fn default() -> Self {
        Self {
            coal: 100.0,
            steam: 0.0,
            max_coal: 100.0,
            max_steam: 50.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy_ecs", derive(Component))]
pub struct CargoHold {
    pub items: Vec<VaaMItem>,
    pub capacity: f32, // Max Tonnage
}

impl Default for CargoHold {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            capacity: 50.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy_ecs", derive(Component))]
pub struct MentalState {
    pub scale_level: f32, // Trauma/Stress (0.0 - 1.0)
    pub is_stalled: bool,
    pub is_derailed: bool,
}

impl Default for MentalState {
    fn default() -> Self {
        Self {
            scale_level: 0.0,
            is_stalled: false,
            is_derailed: false,
        }
    }
}

// --- VaaM Item Definition ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VaaMItem {
    pub id: String,
    pub name: String,
    pub intrinsic_weight: f32, // Cognitive Load cost
    pub mastery_state: MasteryState,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MasteryState {
    Familiar,  // Bronze
    Practiced, // Silver
    Mastered,  // Gold (Weight = 0)
}
