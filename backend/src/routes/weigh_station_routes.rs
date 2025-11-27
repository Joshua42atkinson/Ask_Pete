use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::handlers::weigh_station::WeighStation;
use crate::AppState;

#[derive(Deserialize)]
pub struct WeighRequest {
    pub word: String,
}

pub fn weigh_station_routes() -> Router<AppState> {
    Router::new().route("/weigh", post(weigh_word))
}

async fn weigh_word(
    State(state): State<AppState>,
    Json(payload): Json<WeighRequest>,
) -> impl IntoResponse {
    // In a real app, WeighStation would be in AppState.
    // For now, we'll instantiate it here if we can access the LLM,
    // or better, put it in AppState in main.rs.

    // Assuming we add weigh_station to AppState in main.rs
    // For this step, I'll just return a placeholder or try to use the one in state if added.

    // Let's assume we added it to AppState.
    // But wait, I haven't updated AppState yet.

    // To make this work without changing AppState definition too much right now (or maybe I should):
    // I will update AppState in main.rs to include WeighStation.

    let mut station = state.weigh_station.lock().await;
    match station.process_word(&payload.word).await {
        Ok(physics) => Json(physics).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
