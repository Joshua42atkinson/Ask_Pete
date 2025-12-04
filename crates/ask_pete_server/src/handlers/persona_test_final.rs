#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AppState;
    use axum::extract::State;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::{Arc, RwLock};

    #[tokio::test]
    async fn test_get_archetypes_returns_locomotive_stats() {
        // 1. Setup DB Connection
        dotenvy::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .connect(&database_url)
            .await
            .expect("Failed to connect to DB");

        // 2. Mock AppState (Minimal)
        let leptos_options = leptos::config::LeptosOptions::builder()
            .output_name("app")
            .site_root("target/site")
            .build();

        let app_state = AppState {
            pool: Some(pool),
            leptos_options,
            shared_research_log: Arc::new(RwLock::new(Default::default())),
            shared_virtues: Arc::new(RwLock::new(Default::default())),
            conversation_memory: Arc::new(
                crate::ai::conversation_memory::ConversationMemory::new_in_memory(10),
            ),
            socratic_engine: Arc::new(tokio::sync::RwLock::new(
                crate::ai::socratic_engine::SocraticEngine::new(Arc::new(
                    crate::ai::conversation_memory::ConversationMemory::new_in_memory(10),
                )),
            )),
            model_manager: Arc::new(tokio::sync::Mutex::new(
                crate::services::model_manager::ModelManager::new().unwrap(),
            )),
            pete_assistant: Arc::new(crate::services::pete::PeteAssistant::new().unwrap()),
            pete_command_inbox: crate::game::components::PeteCommandInbox(Arc::new(RwLock::new(
                Vec::new(),
            ))),
            pete_response_outbox: crate::game::components::PeteResponseOutbox(Arc::new(
                RwLock::new(Vec::new()),
            )),
            shared_physics: crate::game::components::SharedPhysicsResource(Arc::new(RwLock::new(
                Default::default(),
            ))),
            weigh_station: None,
            shared_campaign_state: crate::game::components::SharedCampaignStateResource(Arc::new(
                RwLock::new(Default::default()),
            )),
            vote_inbox: crate::game::components::VoteInbox(Arc::new(RwLock::new(Vec::new()))),
            memory_store: None,
        };

        // 3. Call Handler
        let result = get_archetypes(State(app_state)).await;

        // 4. Assertions
        let Json(archetypes) = result.expect("Failed to get archetypes");
        assert!(!archetypes.is_empty());

        // Check for "The Innocent" and its stats
        let innocent = archetypes
            .iter()
            .find(|a| a.name == "The Innocent")
            .expect("The Innocent not found");
        assert_eq!(innocent.locomotive_type, "Light Commuter Rail");
        assert_eq!(innocent.fuel_efficiency, 1.5);
        assert_eq!(innocent.cargo_capacity, 5.0);
    }
}
