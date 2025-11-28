use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a Node in the learning graph (a "Station").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub station_type: StationType,
    pub coordinates: (f32, f32), // For the visual editor
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StationType {
    Lesson,
    Quiz,
    Project,
    Hub,
}

/// Represents a Connection between nodes (a "Track").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: Uuid,
    pub source_station_id: Uuid,
    pub target_station_id: Uuid,
    pub logic_gate: Option<LogicGate>,
    pub friction: f32, // Represents difficulty/resistance
}

/// Logic Gate for conditional traversal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicGate {
    RequiresMastery(Vec<String>), // List of concepts required
    RequiresItem(String),         // Inventory item required
    MinFuel(f32),                 // Minimum motivation required
}

/// The Graph Manager
pub struct TrainYard {
    pub stations: HashMap<Uuid, Station>,
    pub tracks: Vec<Track>,
}

impl TrainYard {
    pub fn new() -> Self {
        Self {
            stations: HashMap::new(),
            tracks: Vec::new(),
        }
    }

    pub fn add_station(
        &mut self,
        title: String,
        content: String,
        station_type: StationType,
        x: f32,
        y: f32,
    ) -> Uuid {
        let id = Uuid::new_v4();
        let station = Station {
            id,
            title,
            content,
            station_type,
            coordinates: (x, y),
        };
        self.stations.insert(id, station);
        id
    }

    pub fn connect_stations(&mut self, source: Uuid, target: Uuid, logic: Option<LogicGate>) {
        let track = Track {
            id: Uuid::new_v4(),
            source_station_id: source,
            target_station_id: target,
            logic_gate: logic,
            friction: 1.0, // Default friction
        };
        self.tracks.push(track);
    }
}
