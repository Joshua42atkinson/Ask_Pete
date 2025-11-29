use crate::ai::architect::{BlueprintRequest, BlueprintResponse, CurriculumArchitect};
use crate::error::Result;
use crate::state::AppState;
use axum::{extract::State, Json};

pub async fn generate_blueprint(
    State(state): State<AppState>,
    Json(payload): Json<BlueprintRequest>,
) -> Result<Json<BlueprintResponse>> {
    // Use the shared Socratic Engine (The AI Mirror)
    let mut engine = state.socratic_engine.write().await;
    
    // Generate blueprint using the engine's available model (Gemma or Gemini)
    let response = engine.generate_blueprint(payload).await?;

    Ok(Json(response))
}
