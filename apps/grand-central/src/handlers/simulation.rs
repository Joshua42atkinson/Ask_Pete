use crate::state::AppState;
use axum::{extract::State, Json};
use domain_physics::components::PhysicsState;

pub async fn get_simulation_state(State(state): State<AppState>) -> Json<PhysicsState> {
    let physics = state.shared_physics.0.read().unwrap();
    Json(physics.clone())
}
