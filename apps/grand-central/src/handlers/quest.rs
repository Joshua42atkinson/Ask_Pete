use crate::error::{AppError, Result};
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use pete_core::expert::StoryGraph;
use serde::{Deserialize, Serialize};
use sqlx::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct StartQuestResponse {
    pub success: bool,
    pub message: String,
}

pub async fn start_quest(
    State(state): State<AppState>,
    Path(quest_id): Path<i32>,
) -> Result<Json<StartQuestResponse>> {
    // 1. Load StoryGraph from DB
    let pool = state.pool.as_ref().ok_or(AppError::InternalServerError)?;

    let row = sqlx::query("SELECT graph_data FROM story_graphs WHERE id = $1")
        .bind(quest_id)
        .fetch_optional(pool)
        .await?
        .ok_or(AppError::NotFound)?;

    let graph_data: serde_json::Value = row
        .try_get("graph_data")
        .map_err(|e| anyhow::anyhow!("Failed to get graph_data: {}", e))?;

    let graph: StoryGraph = serde_json::from_value(graph_data)
        .map_err(|e| anyhow::anyhow!("Failed to deserialize graph: {}", e))?;

    // 2. Calculate Initial Load via WeighStation (Cognitive Logistics)
    let intrinsic_load = {
        let engine = state.socratic_engine.read();
        // Use a read lock to access the engine
        match engine.await.analyze_load(&graph.title).await {
            Ok(profile) => {
                log::info!(
                    "Cognitive Load Analysis for '{}': {:?}",
                    graph.title,
                    profile
                );
                profile.intrinsic
            }
            Err(e) => {
                log::warn!(
                    "Failed to analyze load for '{}': {}. Using default.",
                    graph.title,
                    e
                );
                5.0 // Default safe load
            }
        }
    };

    // 3. Update ECS State via Inbox
    if let Ok(mut inbox) = state.quest_command_inbox.0.write() {
        inbox.push(domain_physics::components::StartQuestEvent {
            quest_id: quest_id.to_string(),
            start_node_id: graph
                .nodes
                .first()
                .map(|n| n.id.clone())
                .unwrap_or_default(),
            initial_load: intrinsic_load,
        });
    }

    // Also update SharedStoryProgress immediately for UI responsiveness (optional but good for UX)
    if let Ok(mut progress) = state.shared_story_progress.write() {
        progress.current_quest_id = Some(quest_id.to_string());
        if let Some(start_node) = graph.nodes.first() {
            progress.current_step_id = Some(start_node.id.clone());
            progress.current_step_description = start_node.content.clone();
        }
        progress.history.clear();
        progress.inventory.clear();
    }

    Ok(Json(StartQuestResponse {
        success: true,
        message: format!("Started quest: {}", graph.title),
    }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteQuestResponse {
    pub success: bool,
    pub steam_earned: f64,
    pub new_balance: f64,
}

pub async fn complete_quest(
    State(state): State<AppState>,
    Path(quest_id): Path<i32>,
    // In a real app, we'd extract UserId from the token. For MVP, we'll use a hardcoded ID or header.
    // headers: HeaderMap,
) -> Result<Json<CompleteQuestResponse>> {
    let user_id = 1; // Hardcoded for MVP (The "Student" user)

    // 1. Calculate Steam Earned (from Physics State)
    let steam_earned = {
        let physics = state
            .shared_physics
            .0
            .read()
            .map_err(|_| AppError::InternalServerError)?;
        physics.steam as f64
    };

    // 2. Update Database
    let pool = state.pool.as_ref().ok_or(AppError::InternalServerError)?;

    let mut tx = pool.begin().await?;

    // Update User Balance
    let row = sqlx::query(
        "UPDATE users SET steam_balance = steam_balance + $1 WHERE id = $2 RETURNING steam_balance",
    )
    .bind(steam_earned)
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    let new_balance: f64 = row.try_get("steam_balance")?;

    // Log Completion
    sqlx::query(
        "INSERT INTO quest_completions (user_id, quest_id, steam_earned) VALUES ($1, $2, $3)",
    )
    .bind(user_id)
    .bind(quest_id)
    .bind(steam_earned)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    // 3. Reset Physics State (Optional, or let it persist until next quest)
    // For now, we leave it as "history" of the run.

    Ok(Json(CompleteQuestResponse {
        success: true,
        steam_earned,
        new_balance,
    }))
}
