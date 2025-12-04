use crate::AppState;
use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Scenario {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub difficulty: String,
    pub status: String,
    pub tags: Vec<String>,
}

pub fn scenarios_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/api/scenarios", get(list_scenarios))
        .with_state(state.clone())
}

async fn list_scenarios(State(state): State<AppState>) -> impl IntoResponse {
    let pool = match &state.pool {
        Some(p) => p,
        None => return Json(vec![]), // Return empty if no DB
    };

    let scenarios = sqlx::query_as::<_, Scenario>(
        "SELECT id, title, description, difficulty, status, tags FROM scenarios ORDER BY id",
    )
    .fetch_all(pool)
    .await
    .unwrap_or_else(|e| {
        eprintln!("Failed to fetch scenarios: {}", e);
        vec![]
    });

    Json(scenarios)
}
