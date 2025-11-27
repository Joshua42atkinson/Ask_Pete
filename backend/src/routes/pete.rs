use crate::services::model_manager::ModelDefinition;
use crate::AppState;
use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

pub fn pete_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/api/pete/models", get(list_models))
        .route("/api/pete/models/download", post(download_model))
        .route("/api/pete/chat", post(chat_with_pete))
        .with_state(state.clone())
}

async fn list_models(State(state): State<AppState>) -> impl IntoResponse {
    let manager = state.model_manager.lock().await;
    let models = crate::services::model_manager::ModelManager::list_available_models();

    // Enrich with status (downloaded or not)
    let enriched_models: Vec<EnrichedModelDefinition> = models
        .into_iter()
        .map(|m| {
            let downloaded = manager.has_model(&m.alias);
            EnrichedModelDefinition {
                definition: m,
                downloaded,
            }
        })
        .collect();

    Json(enriched_models)
}

#[derive(Deserialize)]
struct DownloadRequest {
    alias: String,
}

async fn download_model(
    State(state): State<AppState>,
    Json(payload): Json<DownloadRequest>,
) -> impl IntoResponse {
    let mut manager = state.model_manager.lock().await;
    let models = crate::services::model_manager::ModelManager::list_available_models();

    if let Some(model_def) = models.iter().find(|m| m.alias == payload.alias) {
        match manager.download_model(model_def).await {
            Ok(_) => {
                Json(serde_json::json!({ "status": "success", "message": "Model downloaded" }))
            }
            Err(e) => Json(serde_json::json!({ "status": "error", "message": e.to_string() })),
        }
    } else {
        Json(serde_json::json!({ "status": "error", "message": "Model not found" }))
    }
}

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
}

async fn chat_with_pete(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> impl IntoResponse {
    match state.pete_assistant.answer_question(&payload.message).await {
        Ok(response) => Json(serde_json::json!({ "status": "success", "data": response })),
        Err(e) => Json(serde_json::json!({ "status": "error", "message": e.to_string() })),
    }
}

#[derive(Serialize)]
struct EnrichedModelDefinition {
    #[serde(flatten)]
    definition: ModelDefinition,
    downloaded: bool,
}
