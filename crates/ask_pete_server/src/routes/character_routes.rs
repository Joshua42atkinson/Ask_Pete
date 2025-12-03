// use crate::handlers::character::create_character;
use crate::state::AppState;
use axum::{routing::post, Router};

pub fn character_routes(state: &AppState) -> Router<AppState> {
    Router::new() //.route("/api/character/create", post(create_character))
}
