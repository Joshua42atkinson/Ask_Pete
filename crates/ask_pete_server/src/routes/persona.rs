use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::persona::{get_archetypes, get_dilemmas, submit_quiz};

pub fn persona_routes(app_state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/api/dilemmas", get(get_dilemmas))
        .route("/api/archetypes", get(get_archetypes))
        .route("/api/submit_quiz", post(submit_quiz))
        .with_state(app_state.clone())
}
