use crate::error::{AppError, Result};
use async_trait::async_trait;
use pete_core::expert::StoryGraph;
use sqlx::{PgPool, Row};

// The Interface
#[async_trait]
pub trait QuestRepository: Send + Sync {
    async fn get_story_graph(&self, quest_id: i32) -> Result<StoryGraph>;
    async fn complete_quest_transaction(
        &self,
        user_id: i64,
        quest_id: i32,
        steam_earned: f64,
    ) -> Result<f64>;
}

// The Implementation
pub struct PostgresQuestRepository {
    pool: PgPool,
}

impl PostgresQuestRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl QuestRepository for PostgresQuestRepository {
    async fn get_story_graph(&self, quest_id: i32) -> Result<StoryGraph> {
        let row = sqlx::query("SELECT graph_data FROM story_graphs WHERE id = $1")
            .bind(quest_id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(AppError::NotFound)?;

        let graph_data: serde_json::Value = row
            .try_get("graph_data")
            .map_err(|e| anyhow::anyhow!("Failed to get graph_data: {}", e))?;

        let graph: StoryGraph = serde_json::from_value(graph_data)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize graph: {}", e))?;

        Ok(graph)
    }

    async fn complete_quest_transaction(
        &self,
        user_id: i64,
        quest_id: i32,
        steam_earned: f64,
    ) -> Result<f64> {
        let mut tx = self.pool.begin().await?;

        // 1. Update User Balance
        let row = sqlx::query(
            "UPDATE users SET steam_balance = steam_balance + $1 WHERE id = $2 RETURNING steam_balance",
        )
        .bind(steam_earned)
        .bind(user_id)
        .fetch_one(&mut *tx)
        .await?;

        let new_balance: f64 = row.try_get("steam_balance")?;

        // 2. Log Completion
        sqlx::query(
            "INSERT INTO quest_completions (user_id, quest_id, steam_earned) VALUES ($1, $2, $3)",
        )
        .bind(user_id)
        .bind(quest_id)
        .bind(steam_earned)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(new_balance)
    }
}

// Mock Repository for Simulation Mode
pub struct MockQuestRepository;

#[async_trait]
impl QuestRepository for MockQuestRepository {
    async fn get_story_graph(&self, _quest_id: i32) -> Result<StoryGraph> {
        // Return a dummy graph or error
        Err(AppError::NotFound)
    }

    async fn complete_quest_transaction(
        &self,
        _user_id: i64,
        _quest_id: i32,
        steam_earned: f64,
    ) -> Result<f64> {
        // Just return the earned steam as the new balance (simulated)
        Ok(steam_earned)
    }
}
