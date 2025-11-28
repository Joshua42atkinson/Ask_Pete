use crate::ai::architect::{BlueprintRequest, BlueprintResponse, CurriculumArchitect};
use crate::error::Result;
use crate::state::AppState;
use axum::{extract::State, Json};

pub async fn generate_blueprint(
    State(state): State<AppState>,
    Json(payload): Json<BlueprintRequest>,
) -> Result<Json<BlueprintResponse>> {
    // 1. Get Gemini Client (Clone from state or create new if not in state - currently in SocraticEngine)
    // For now, we'll create a new one using the default config which reads env vars
    // In a production app, we should share the client instance
    let gemini_config = crate::ai::llm::gemini_client::GeminiConfig::default();
    let gemini_client = crate::ai::llm::gemini_client::GeminiClient::new(gemini_config);

    // 2. Initialize Architect
    let mut architect = CurriculumArchitect::new(gemini_client);

    // 3. Generate
    let response = architect.generate_blueprint(payload).await?;

    Ok(Json(response))
}
