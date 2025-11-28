use crate::state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

// Request DTO for casting a vote
#[derive(Debug, Deserialize)]
pub struct VoteRequest {
    pub campaign_id: String,
    pub option_index: usize,
}

pub async fn get_campaign_state(State(state): State<AppState>) -> impl IntoResponse {
    // Read from the Shared Resource
    if let Ok(guard) = state.shared_campaign_state.0.read() {
        let campaign_state = &*guard;
        return Json(campaign_state.clone()).into_response();
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Failed to read campaign state",
    )
        .into_response()
}

pub async fn cast_vote(
    State(state): State<AppState>,
    Json(payload): Json<VoteRequest>,
) -> impl IntoResponse {
    // Push vote to the Inbox for Bevy to process
    if let Ok(mut inbox) = state.vote_inbox.0.write() {
        inbox.push(crate::game::components::VoteEvent {
            campaign_id: payload.campaign_id,
            option_index: payload.option_index,
        });
        return (StatusCode::OK, "Vote received").into_response();
    }

    (StatusCode::INTERNAL_SERVER_ERROR, "Failed to process vote").into_response()
}
