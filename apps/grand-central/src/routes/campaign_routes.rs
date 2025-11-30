use crate::handlers::campaign::{cast_vote, get_campaign_state};
use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn campaign_routes() -> Router<AppState> {
    Router::new()
        .route("/api/campaign/state", get(get_campaign_state))
        .route("/api/campaign/vote", post(cast_vote))
}
