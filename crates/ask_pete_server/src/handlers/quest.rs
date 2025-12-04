use crate::error::{AppError, Result};
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StartQuestResponse {
    pub success: bool,
    pub message: String,
}

pub async fn start_quest(
    State(state): State<AppState>,
    Path(quest_id): Path<i32>,
) -> Result<Json<StartQuestResponse>> {
    // 1. Load Data (Via Repository)
    let graph = state.quest_repo.get_story_graph(quest_id).await?;

    // 2. Business Logic: Calculate Intrinsic Load (Via Socratic Engine)
    let intrinsic_load = {
        let engine = state.socratic_engine.read().await;
        match engine.analyze_load(&graph.title).await {
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

    // 3. Game State: Update ECS Inbox
    if let Ok(mut inbox) = state.quest_command_inbox.0.write() {
        inbox.push(domain_physics::components::StartQuestEvent {
            quest_id: quest_id.to_string(),
        });
    }

    // 4. Update Shared Progress (UI Sync)
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
) -> Result<Json<CompleteQuestResponse>> {
    let user_id = 1; // Hardcoded for MVP

    // 1. Calculate Steam (Physics State)
    let steam_earned = {
        let physics = state
            .shared_physics
            .0
            .read()
            .map_err(|_| AppError::InternalServerError)?;
        // Temporary calculation: Steam = velocity * miles traveled * 0.1
        (physics.velocity * physics.miles * 0.1) as f64
    };

    // 2. Persist Data (Via Repository)
    let new_balance = state
        .quest_repo
        .complete_quest_transaction(user_id, quest_id, steam_earned)
        .await?;

    Ok(Json(CompleteQuestResponse {
        success: true,
        steam_earned,
        new_balance,
    }))
}
