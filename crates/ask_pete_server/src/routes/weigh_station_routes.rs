use crate::services::weigh_station::{WeighStationService, WordPhysics};
use crate::AppState;
use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WeighRequest {
    pub word: String,
}

#[derive(Deserialize)]
pub struct AnalyzeRequest {
    pub text: String,
}

pub fn weigh_station_routes() -> Router<AppState> {
    Router::new()
        .route("/weigh", post(weigh_word))
        .route("/analyze", post(analyze_text))
}

async fn weigh_word(
    State(state): State<AppState>,
    Json(payload): Json<WeighRequest>,
) -> impl IntoResponse {
    if let Some(station) = &state.weigh_station {
        // No lock needed for Arc<WeighStationService> as it uses internal mutability (PgPool) or immutable state
        match station.weigh_word(&payload.word).await {
            Ok(physics) => Json::<WordPhysics>(physics).into_response(),
            Err(e) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    } else {
        (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "Weigh Station is offline. Llama 3.2 model not found.",
        )
            .into_response()
    }
}

async fn analyze_text(
    State(_state): State<AppState>,
    Json(payload): Json<AnalyzeRequest>,
) -> impl IntoResponse {
    // Static method
    let result = WeighStationService::calculate_intrinsic_load(&payload.text);
    Json(result).into_response()
}
