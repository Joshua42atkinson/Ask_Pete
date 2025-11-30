use crate::error::{AppError, Result};
use crate::AppState;
use axum::{
    extract::{Path, State},
    routing::{get, post, put},
    Json, Router,
};
use pete_core::expert::StoryGraph;
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveStoryGraphRequest {
    pub title: String,
    pub subject: Option<String>,
    pub literary_device: Option<String>,
    pub focus: Option<f32>,
    pub vocabulary: Vec<String>,
    pub graph_data: StoryGraph,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoryGraphResponse {
    pub id: i32,
    pub title: String,
    pub subject: Option<String>,
    pub literary_device: Option<String>,
    pub focus: Option<f32>,
    pub vocabulary: Vec<String>,
    pub graph_data: StoryGraph,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct StoryGraphRow {
    id: i32,
    title: String,
    subject: Option<String>,
    literary_device: Option<String>,
    focus: Option<f32>,
    vocabulary: Vec<String>,
    graph_data: JsonValue,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

pub fn story_graph_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/api/story_graphs",
            post(save_story_graph).get(list_story_graphs),
        )
        .route(
            "/api/story_graphs/:id",
            get(get_story_graph).put(update_story_graph),
        )
        .with_state(state.clone())
}

/// POST /api/story_graphs - Save a new story graph
async fn save_story_graph(
    State(state): State<AppState>,
    Json(payload): Json<SaveStoryGraphRequest>,
) -> Result<Json<StoryGraphResponse>> {
    let pool = state.pool.as_ref().ok_or(AppError::InternalServerError)?;

    let graph_json = serde_json::to_value(&payload.graph_data)
        .map_err(|e| anyhow::anyhow!("Failed to serialize graph: {}", e))?;

    let row = sqlx::query_as::<_, StoryGraphRow>(
        r#"
        INSERT INTO story_graphs (title, subject, literary_device, focus, vocabulary,graph_data)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, title, subject, literary_device, focus, vocabulary, graph_data, created_at, updated_at
        "#,
    )
    .bind(&payload.title)
    .bind(&payload.subject)
    .bind(&payload.literary_device)
    .bind(payload.focus)
    .bind(&payload.vocabulary)
    .bind(&graph_json)
    .fetch_one(pool)
    .await?;

    let graph_data: StoryGraph = serde_json::from_value(row.graph_data)
        .map_err(|e| anyhow::anyhow!("Failed to deserialize graph: {}", e))?;

    Ok(Json(StoryGraphResponse {
        id: row.id,
        title: row.title,
        subject: row.subject,
        literary_device: row.literary_device,
        focus: row.focus,
        vocabulary: row.vocabulary,
        graph_data,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }))
}

/// GET /api/story_graphs - List all story graphs
async fn list_story_graphs(State(state): State<AppState>) -> Result<Json<Vec<StoryGraphResponse>>> {
    let pool = state.pool.as_ref().ok_or(AppError::InternalServerError)?;

    let rows = sqlx::query_as::<_, StoryGraphRow>(
        r#"
        SELECT id, title, subject, literary_device, focus, vocabulary, graph_data, created_at, updated_at
        FROM story_graphs
        ORDER BY updated_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    let responses: Result<Vec<StoryGraphResponse>> = rows
        .into_iter()
        .map(|row| {
            let graph_data: StoryGraph = serde_json::from_value(row.graph_data)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize graph: {}", e))?;

            Ok(StoryGraphResponse {
                id: row.id,
                title: row.title,
                subject: row.subject,
                literary_device: row.literary_device,
                focus: row.focus,
                vocabulary: row.vocabulary,
                graph_data,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
        })
        .collect();

    Ok(Json(responses?))
}

/// GET /api/story_graphs/:id - Get a specific story graph
async fn get_story_graph(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<StoryGraphResponse>> {
    let pool = state.pool.as_ref().ok_or(AppError::InternalServerError)?;

    let row = sqlx::query_as::<_, StoryGraphRow>(
        r#"
        SELECT id, title, subject, literary_device, focus, vocabulary, graph_data, created_at, updated_at
        FROM story_graphs
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    let graph_data: StoryGraph = serde_json::from_value(row.graph_data)
        .map_err(|e| anyhow::anyhow!("Failed to deserialize graph: {}", e))?;

    Ok(Json(StoryGraphResponse {
        id: row.id,
        title: row.title,
        subject: row.subject,
        literary_device: row.literary_device,
        focus: row.focus,
        vocabulary: row.vocabulary,
        graph_data,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }))
}

/// PUT /api/story_graphs/:id - Update an existing story graph
async fn update_story_graph(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<SaveStoryGraphRequest>,
) -> Result<Json<StoryGraphResponse>> {
    let pool = state.pool.as_ref().ok_or(AppError::InternalServerError)?;

    let graph_json = serde_json::to_value(&payload.graph_data)
        .map_err(|e| anyhow::anyhow!("Failed to serialize graph: {}", e))?;

    let row = sqlx::query_as::<_, StoryGraphRow>(
        r#"
        UPDATE story_graphs
        SET title = $1, subject = $2, literary_device = $3, focus = $4, vocabulary = $5, graph_data = $6
        WHERE id = $7
        RETURNING id, title, subject, literary_device, focus, vocabulary, graph_data, created_at, updated_at
        "#,
    )
    .bind(&payload.title)
    .bind(&payload.subject)
    .bind(&payload.literary_device)
    .bind(payload.focus)
    .bind(&payload.vocabulary)
    .bind(&graph_json)
    .bind(id)
    .fetch_one(pool)
    .await?;

    let graph_data: StoryGraph = serde_json::from_value(row.graph_data)
        .map_err(|e| anyhow::anyhow!("Failed to deserialize graph: {}", e))?;

    Ok(Json(StoryGraphResponse {
        id: row.id,
        title: row.title,
        subject: row.subject,
        literary_device: row.literary_device,
        focus: row.focus,
        vocabulary: row.vocabulary,
        graph_data,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }))
}
