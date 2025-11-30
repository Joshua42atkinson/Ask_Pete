use domain_physics::components::{
    PeteCommandInbox, PeteResponseOutbox, ResearchLog, SharedCampaignStateResource,
    SharedPhysicsResource, StoryProgress, VirtueTopology, VoteInbox,
};
use infra_ai::socratic_engine::SocraticEngine;
use infra_db::ConversationMemory;
use leptos::prelude::*;
use leptos::LeptosOptions;
use sqlx::PgPool;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: Option<PgPool>,
    pub shared_research_log: Arc<RwLock<ResearchLog>>,
    pub shared_virtues: Arc<RwLock<VirtueTopology>>,
    // pub gemma_server: Arc<crate::ai::llm::gemma_server::Gemma27BServer>,
    pub conversation_memory: Arc<ConversationMemory>,
    pub socratic_engine: Arc<tokio::sync::RwLock<SocraticEngine>>,
    pub model_manager: Arc<tokio::sync::Mutex<crate::services::model_manager::ModelManager>>,
    pub pete_assistant: Arc<crate::services::pete::PeteAssistant>,
    pub pete_command_inbox: PeteCommandInbox,
    pub pete_response_outbox: PeteResponseOutbox,
    pub shared_physics: SharedPhysicsResource,
    pub weigh_station:
        Option<Arc<tokio::sync::Mutex<crate::handlers::weigh_station::WeighStation>>>,
    pub shared_campaign_state: SharedCampaignStateResource, // [NEW]
    pub vote_inbox: VoteInbox,                              // [NEW]
    pub shared_story_progress: Arc<RwLock<StoryProgress>>,  // [NEW]
                                                            // pub memory_store: Option<Arc<crate::ai::memory::LanceDbConnection>>, // [NEW] - Local Vector DB
}

impl axum::extract::FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone().expect(
            "Database pool not available. This handler should not be reachable in simulation mode.",
        )
    }
}
