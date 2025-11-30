use crate::handlers::architect::generate_blueprint;
use crate::AppState;
use axum::{routing::post, Router};

pub fn architect_routes(_state: &AppState) -> Router<AppState> {
    Router::new().route("/api/architect/blueprint", post(generate_blueprint))
}
