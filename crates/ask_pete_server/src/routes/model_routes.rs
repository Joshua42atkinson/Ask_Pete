use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;

use crate::services::model_registry::get_pete_brains;
use domain_physics::components::{
    DownloadCommandInbox, SharedDownloadStateResource, StartDownloadEvent,
};

// AppState to hold the shared resources
#[derive(Clone)]
pub struct ModelAppState {
    pub download_inbox: DownloadCommandInbox,
    pub download_state: SharedDownloadStateResource,
}

pub fn model_routes() -> Router<ModelAppState> {
    Router::new()
        .route("/", get(list_models))
        .route("/download", post(download_model))
        .route("/progress", get(get_progress))
}

async fn list_models() -> impl IntoResponse {
    Json(get_pete_brains())
}

#[derive(Deserialize)]
struct DownloadRequest {
    model_alias: String,
}

async fn download_model(
    State(state): State<ModelAppState>,
    Json(payload): Json<DownloadRequest>,
) -> impl IntoResponse {
    let models = get_pete_brains();
    // Simple matching by name or filename for now, assuming alias maps to name roughly
    // In a real app, we'd have a proper ID map.
    // Let's assume payload.model_alias matches ModelConfig.name for now.

    if let Some(model) = models.into_iter().find(|m| m.name == payload.model_alias) {
        let event = StartDownloadEvent {
            model_config: model,
        };

        // Push to inbox
        if let Ok(mut inbox) = state.download_inbox.0.write() {
            inbox.push(event);
            return Json(serde_json::json!({ "status": "queued", "message": "Download started" }));
        } else {
            return Json(
                serde_json::json!({ "status": "error", "message": "Failed to acquire lock" }),
            );
        }
    }

    Json(serde_json::json!({ "status": "error", "message": "Model not found" }))
}

async fn get_progress(State(state): State<ModelAppState>) -> impl IntoResponse {
    if let Ok(guard) = state.download_state.0.read() {
        if let Some(progress) = &*guard {
            return Json(serde_json::json!({
                "status": "downloading",
                "percent": progress.percent,
                "downloaded_bytes": progress.downloaded_bytes,
                "total_bytes": progress.total_bytes
            }));
        }
    }

    Json(serde_json::json!({ "status": "idle", "percent": 0.0 }))
}
