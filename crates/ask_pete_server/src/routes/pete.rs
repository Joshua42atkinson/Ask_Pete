use crate::services::model_manager::ModelDefinition;
use crate::AppState;
use axum::{
    extract::{Json, Path, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn pete_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/api/pete/models", get(list_models))
        .route("/api/pete/models/download", post(download_model))
        .route("/api/pete/chat", post(submit_chat)) // [MODIFIED] Async Submit
        .route("/api/pete/chat/:job_id", get(check_chat)) // [NEW] Poll Status
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

// 1. Submit (Fast)
async fn submit_chat(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> impl IntoResponse {
    // Immediately enqueue and return the Ticket ID
    let job_id = state.chat_queue.enqueue(payload.message).await;

    // Return 202 Accepted
    (
        axum::http::StatusCode::ACCEPTED,
        Json(serde_json::json!({
            "job_id": job_id,
            "status": "Queued",
            "message": "Pete is thinking..."
        })),
    )
}

// 2. Poll (Fast)
async fn check_chat(State(state): State<AppState>, Path(job_id): Path<Uuid>) -> impl IntoResponse {
    match state.chat_queue.get_status(&job_id) {
        Some(status) => Json::<crate::services::chat_queue::JobStatus>(status).into_response(),
        None => (
            axum::http::StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Job not found"})),
        )
            .into_response(),
    }
}

#[derive(Serialize)]
struct EnrichedModelDefinition {
    #[serde(flatten)]
    definition: ModelDefinition,
    downloaded: bool,
}
