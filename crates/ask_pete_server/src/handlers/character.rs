use crate::state::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateCharacterRequest {
    pub name: String,
    pub role: String,
    pub archetype: String,
}

pub async fn create_character(
    State(state): State<AppState>,
    Json(payload): Json<CreateCharacterRequest>,
) -> impl IntoResponse {
    // TODO: Get user_id from auth context (mocking for now)
    let user_id = 1;

    let result = sqlx::query!(
        r#"
        INSERT INTO characters (user_id, name, role, archetype)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        user_id,
        payload.name,
        payload.role,
        payload.archetype
    )
    .fetch_one(state.pool.as_ref().unwrap())
    .await;

    match result {
        Ok(record) => (StatusCode::CREATED, Json(record.id)).into_response(),
        Err(e) => {
            eprintln!("Failed to create character: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create character",
            )
                .into_response()
        }
    }
}
