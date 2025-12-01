use crate::error::{AppError, Result};
use crate::AppState;
use axum::{extract::State, Json};
use pete_core::expert::StoryGraph;
use sqlx::Row;
use std::fs;
use std::path::Path;

const DATA_FILE: &str = "data/story_graph.json";

pub async fn get_graph(State(app_state): State<AppState>) -> Result<Json<StoryGraph>> {
    let pool = match app_state.pool {
        Some(pool) => pool,
        None => {
            // Try to load from file
            if Path::new(DATA_FILE).exists() {
                match fs::read_to_string(DATA_FILE) {
                    Ok(content) => match serde_json::from_str::<StoryGraph>(&content) {
                        Ok(graph) => return Ok(Json(graph)),
                        Err(e) => tracing::error!("Failed to parse story graph file: {}", e),
                    },
                    Err(e) => tracing::error!("Failed to read story graph file: {}", e),
                }
            }

            // Return a default graph if file doesn't exist or fails to load
            let default_graph = StoryGraph {
                id: "demo_graph".to_string(),
                title: "New Story".to_string(),
                nodes: vec![pete_core::expert::StoryNode {
                    id: "start".to_string(),
                    title: "The Beginning".to_string(),
                    content: "Welcome! Click 'Blueprint' to start designing.".to_string(),
                    x: 0.0,
                    y: 0.0,
                    passenger_count: 0,
                    complexity_level: 1,
                    learner_profiles: vec![],
                    gardens_active: vec![],
                    required_stats: std::collections::HashMap::new(),
                    logic: Default::default(),
                    style: Default::default(),
                }],
                connections: vec![],
            };
            return Ok(Json(default_graph));
        }
    };

    // Fetch the graph (hardcoded ID for now, similar to mock)
    let row = sqlx::query("SELECT nodes, connections, title FROM story_graphs WHERE id = $1")
        .bind("demo_graph")
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {:?}", e);
            AppError::InternalServerError
        })?;

    if let Some(row) = row {
        let nodes: serde_json::Value = row.get("nodes");
        let connections: serde_json::Value = row.get("connections");
        let title: String = row.get("title");

        let graph = StoryGraph {
            id: "demo_graph".to_string(),
            title,
            nodes: serde_json::from_value(nodes).unwrap_or_default(),
            connections: serde_json::from_value(connections).unwrap_or_default(),
        };
        Ok(Json(graph))
    } else {
        // Return a default empty graph if not found (auto-create logic could go here)
        let default_graph = StoryGraph {
            id: "demo_graph".to_string(),
            title: "New Story".to_string(),
            nodes: vec![],
            connections: vec![],
        };
        Ok(Json(default_graph))
    }
}

pub async fn save_graph(
    State(app_state): State<AppState>,
    Json(payload): Json<StoryGraph>,
) -> Result<Json<StoryGraph>> {
    let pool = match app_state.pool {
        Some(pool) => pool,
        None => {
            // Save to file in simulation mode
            // Ensure data directory exists
            if let Some(parent) = Path::new(DATA_FILE).parent() {
                let _ = fs::create_dir_all(parent);
            }

            match serde_json::to_string_pretty(&payload) {
                Ok(json) => {
                    if let Err(e) = fs::write(DATA_FILE, json) {
                        tracing::error!("Failed to write story graph file: {}", e);
                        return Err(AppError::InternalServerError);
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to serialize story graph: {}", e);
                    return Err(AppError::InternalServerError);
                }
            }

            return Ok(Json(payload));
        }
    };

    let nodes_json = serde_json::to_value(&payload.nodes).unwrap();
    let connections_json = serde_json::to_value(&payload.connections).unwrap();

    sqlx::query(
        r#"
        INSERT INTO story_graphs (id, title, nodes, connections, updated_at)
        VALUES ($1, $2, $3, $4, NOW())
        ON CONFLICT (id) DO UPDATE
        SET title = EXCLUDED.title,
        nodes = EXCLUDED.nodes,
        connections = EXCLUDED.connections,
        updated_at = NOW()
        "#,
    )
    .bind(&payload.id)
    .bind(&payload.title)
    .bind(nodes_json)
    .bind(connections_json)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {:?}", e);
        AppError::InternalServerError
    })?;

    Ok(Json(payload))
}
