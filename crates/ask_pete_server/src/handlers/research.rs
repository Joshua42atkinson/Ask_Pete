use crate::error::Result;
use crate::AppState;
use axum::{extract::State, Json};
use domain_physics::components::{ResearchLog, VirtueTopology};

pub async fn get_research_log(State(state): State<AppState>) -> Result<Json<ResearchLog>> {
    let log = state.shared_research_log.read().unwrap().clone();
    Ok(Json(log))
}

pub async fn get_virtue_topology(State(state): State<AppState>) -> Result<Json<VirtueTopology>> {
    let virtues = state.shared_virtues.read().unwrap().clone();
    Ok(Json(virtues))
}

pub async fn log_research_event(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<String>> {
    // For now, just log to console or append to shared log
    let mut log = state.shared_research_log.write().unwrap();

    let event = domain_physics::components::ResearchEvent {
        timestamp: 0.0, // TODO: Get actual time or simulation time
        event_type: "LOG".to_string(),
        data: payload.to_string(),
    };

    log.events.push(event);
    Ok(Json("Logged".to_string()))
}
