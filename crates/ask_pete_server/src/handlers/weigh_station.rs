use crate::services::weigh_station::WordPhysics;
use crate::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};

#[derive(serde::Deserialize)]
pub struct WeighRequest {
    pub word: String,
}

pub async fn weigh_word(
    State(state): State<AppState>,
    Json(payload): Json<WeighRequest>,
) -> impl IntoResponse {
    // Calls the hybrid service
    if let Some(service) = &state.weigh_station {
        match service.weigh_word(&payload.word).await {
            Ok(physics) => Json(physics).into_response(),
            Err(e) => {
                tracing::error!("Weigh Station Error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Weigh Station Malfunction",
                )
                    .into_response()
            }
        }
    } else {
        tracing::warn!("Weigh Station Service not available");
        (StatusCode::SERVICE_UNAVAILABLE, "Weigh Station Offline").into_response()
    }
}
