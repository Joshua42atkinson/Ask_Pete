use crate::AppState;
use axum::{routing::post, Router};

use crate::handlers::story::generate_story;

pub fn story_routes(app_state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/api/story/generate", post(generate_story))
        .with_state(app_state.clone())
}
