//! # Grand Central Station (Backend Server)
//!
//! This is the main entry point for the Ask Pete backend.
//! It handles:
//! - HTTP requests via Axum
//! - Real-time game state via Bevy (ECS)
//! - Database interactions via SQLx
//! - AI integration via Socratic Engine
//!
//! ## Architecture
//! The server runs a hybrid architecture:
//! 1. **Axum**: Handles standard web routes (REST/RPC).
//! 2. **Bevy**: Runs a simulation loop for the game world (physics, state).
//! 3. **Tokio**: Manages async tasks and concurrency.

#![allow(dead_code, unused_imports, unused_variables, unused_mut)]
use leptos::prelude::*;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::timeout::TimeoutLayer;

mod ai;
mod antigravity;
mod core; // Plugin system traits
mod data;
mod domain;
mod error;
mod game;
mod handlers;
mod middleware;
mod plugins; // Plugin registry and implementations
mod routes;
mod services; // Model Manager and Pete AI
mod state;
pub use state::AppState;
mod repositories; // [NEW]
mod static_assets;
use crate::domain::player::get_simulated_character;
use crate::routes::ai_mirror::ai_mirror_routes;
use crate::routes::campaign_routes::campaign_routes;
use crate::routes::expert::expert_routes;
use crate::routes::knowledge::knowledge_routes;
use crate::routes::model_routes::model_routes;
use crate::routes::persona::persona_routes;
use crate::routes::pete::pete_routes;
use crate::routes::player::player_routes;
use crate::routes::research::research_routes;
use crate::routes::weigh_station_routes::weigh_station_routes;
use crate::static_assets::Assets as StaticAssets;
use axum::response::IntoResponse;
use axum::Router;
use bevy::core::Name;
use bevy::prelude::*;
// use bevy_yarnspinner::events::{DialogueCompleteEvent, ExecuteCommandEvent};
// use bevy_yarnspinner::prelude::*;
use domain_physics::components::{
    Archetype, AskPeteEvent, Coal, CognitiveLoad, DownloadCommandInbox, DownloadProgressEvent,
    EnginePower, Experience, Level, Location, Mass, Persona, PeteCommandInbox, PeteResponseEvent,
    PeteResponseOutbox, PhysicsState, PlayWhistleEvent, QuestCommandInbox, ResearchLog,
    SharedCampaignStateResource, SharedDownloadStateResource, SharedPhysicsResource,
    SharedResearchLogResource, SharedStoryProgressResource, SharedVirtuesResource, StallEvent,
    StartDownloadEvent, StartQuestEvent, Steam, StoryProgress, StudentBundle, StudentMiles,
    TrainVelocity, VirtueTopology, VoteInbox,
};
use domain_physics::multiplayer_client::{CampaignState, MultiplayerPlugin};
use domain_physics::systems::*;
use infra_ai::socratic_engine::SocraticEngine;
use infra_ai::{LocalConfigWrapper, LocalModel};
use infra_db::ConversationMemory;
use leptos::config::get_configuration; // Try this path
use leptos::prelude::*;

fn run_bevy_app(
    shared_log: SharedResearchLogResource,
    shared_virtues: SharedVirtuesResource,
    shared_physics: SharedPhysicsResource,
    download_inbox: DownloadCommandInbox,
    download_state: SharedDownloadStateResource,
    pete_command_inbox: PeteCommandInbox,
    pete_response_outbox: PeteResponseOutbox,
    shared_campaign_state: SharedCampaignStateResource,
    vote_inbox: VoteInbox,
    shared_story_progress: SharedStoryProgressResource,
    quest_command_inbox: QuestCommandInbox,
) {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::asset::AssetPlugin::default());
    app.add_plugins(bevy::audio::AudioPlugin::default());
    // app.add_plugins(YarnSpinnerPlugin::new()); // Disabled due to version mismatch
    // app.add_plugins(bevy_defer::AsyncPlugin::default_settings());
    app.add_plugins(MultiplayerPlugin);

    // Register Events
    app.add_event::<PlayWhistleEvent>();
    app.add_event::<StartDownloadEvent>();
    app.add_event::<DownloadProgressEvent>();
    app.add_event::<AskPeteEvent>();
    app.add_event::<PeteResponseEvent>();
    app.add_event::<StartQuestEvent>();
    app.add_event::<StallEvent>();

    // Insert Shared Resources
    app.insert_resource(shared_log);
    app.insert_resource(shared_virtues);
    app.insert_resource(shared_physics);
    app.insert_resource(download_inbox);
    app.insert_resource(download_state);
    app.insert_resource(pete_command_inbox.clone());
    app.insert_resource(pete_response_outbox.clone());
    app.insert_resource(shared_campaign_state);
    app.insert_resource(vote_inbox);
    app.insert_resource(shared_story_progress);
    app.insert_resource(quest_command_inbox);

    // Register Systems
    app.add_systems(
        Update,
        (
            update_virtue_topology,
            monitor_cognitive_load,
            log_research_events,
            // sync_yarn_to_story_progress, // Disabled due to version mismatch
            sync_ecs_to_shared,
            whistle_system,
            download_manager_system,
            progress_update_system,
            sync_inbox_to_events,
            calculate_train_velocity,
            sync_pete_bridge,
            track_student_miles,
            sync_physics_to_shared,
            sync_campaign_to_shared,
            sync_vote_inbox,
            sync_story_progress_to_shared,
            generate_steam,
            sync_quest_inbox,
            handle_start_quest,
        ),
    );

    let simulated_player = get_simulated_character();

    // Spawn LocomotiveBundle (Replaces StudentBundle)
    // Spawn StudentBundle (Replaces LocomotiveBundle)
    app.world.spawn(StudentBundle {
        name: bevy::core::Name::new(simulated_player.name),
        persona: Persona {
            archetype: Archetype::Innocent,
            shadow_trait: "None".to_string(),
            projective_dissonance: 0.0,
        },
        virtue_topology: VirtueTopology::default(),
        cognitive_load: CognitiveLoad::default(),
        story_progress: StoryProgress {
            current_quest_id: simulated_player.current_quest_id,
            current_step_id: simulated_player.current_step_id,
            current_step_description: simulated_player.current_step_description,
            history: Vec::new(),
            inventory: simulated_player.inventory,
            quest_flags: simulated_player.quest_flags,
            learned_vocab: simulated_player.learned_vocab,
        },
        research_log: ResearchLog::default(),
        mass: Mass(10.0),
        engine_power: EnginePower(10.0),
        velocity: TrainVelocity(0.0),
        miles: StudentMiles::default(),
        coal: Coal(100.0),
        steam: Steam(0.0),
        location: Location {
            latitude: 40.4282,
            longitude: -86.9144,
        },
        level: Level(1),
        xp: Experience(0),
    });

    app.run();
}

// [NEW] Handler for static assets
async fn static_handler(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.is_empty() {
        path = "index.html".to_string();
    }

    match StaticAssets::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            (
                [(axum::http::header::CONTENT_TYPE, mime.as_ref())],
                content.data,
            )
                .into_response()
        }
        None => {
            if path.contains('.') {
                return axum::http::StatusCode::NOT_FOUND.into_response();
            }
            // Fallback to index.html for SPA routing
            match StaticAssets::get("index.html") {
                Some(content) => {
                    let mime = mime_guess::from_path("index.html").first_or_octet_stream();
                    (
                        [(axum::http::header::CONTENT_TYPE, mime.as_ref())],
                        content.data,
                    )
                        .into_response()
                }
                None => axum::http::StatusCode::NOT_FOUND.into_response(),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    println!("Starting Ask Pete Backend Server...");

    // Initialize Shared Resources
    let shared_research_log =
        SharedResearchLogResource(Arc::new(RwLock::new(ResearchLog::default())));
    let shared_virtues = SharedVirtuesResource(Arc::new(RwLock::new(VirtueTopology::default())));
    let shared_physics = SharedPhysicsResource(Arc::new(RwLock::new(PhysicsState::default())));
    let download_inbox = DownloadCommandInbox(Arc::new(RwLock::new(Vec::new())));
    let download_state = SharedDownloadStateResource(Arc::new(RwLock::new(None)));
    let pete_command_inbox = PeteCommandInbox(Arc::new(RwLock::new(Vec::new())));
    let pete_response_outbox = PeteResponseOutbox(Arc::new(RwLock::new(Vec::new())));
    let shared_campaign_state =
        SharedCampaignStateResource(Arc::new(RwLock::new(CampaignState::default())));
    let vote_inbox = VoteInbox(Arc::new(RwLock::new(Vec::new())));
    let shared_story_progress =
        SharedStoryProgressResource(Arc::new(RwLock::new(StoryProgress::default())));
    let quest_command_inbox = QuestCommandInbox(Arc::new(RwLock::new(Vec::new())));

    let state_clone = download_state.clone();
    let pete_inbox_clone = pete_command_inbox.clone();
    let pete_outbox_clone = pete_response_outbox.clone();
    let physics_clone = shared_physics.clone();
    let campaign_clone = shared_campaign_state.clone();
    let vote_inbox_clone = vote_inbox.clone();
    let story_progress_clone = shared_story_progress.clone();
    let quest_inbox_clone = quest_command_inbox.clone();

    let log_clone = shared_research_log.clone();
    let virtues_clone = shared_virtues.clone();
    let inbox_clone = download_inbox.clone();

    thread::spawn(move || {
        run_bevy_app(
            log_clone,
            virtues_clone,
            physics_clone,
            inbox_clone,
            state_clone,
            pete_inbox_clone,
            pete_outbox_clone,
            campaign_clone,
            vote_inbox_clone,
            story_progress_clone,
            quest_inbox_clone,
        )
    });

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    // Cloud Run Support: Override site_addr if PORT env var is set
    let addr = if let Ok(port) = env::var("PORT") {
        format!("0.0.0.0:{}", port).parse().unwrap()
    } else {
        leptos_options.site_addr.clone()
    };

    let pool = match env::var("DATABASE_URL") {
        Ok(database_url) => {
            println!("DATABASE_URL found, connecting to the database...");
            Some(
                PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&database_url)
                    .await
                    .expect("Failed to create database pool"),
            )
        }
        Err(_) => {
            println!("WARN: DATABASE_URL not found. Running in SIMULATION MODE - No Database.");
            None
        }
    };

    // Initialize Gemini 3 Ultra Client
    let gemini_config = infra_ai::llm::gemini_client::GeminiConfig::default();
    let gemini_client = infra_ai::llm::gemini_client::GeminiClient::new(gemini_config);

    // Initialize Shared Local Model (Llama/Mistral)
    let model_path = std::env::var("LOCAL_MODEL_PATH")
        .unwrap_or_else(|_| "assets/models/mistral-7b-instruct-v0.1.Q4_K_M.gguf".to_string());

    let shared_local_model: Option<LocalModel> = match LocalModel::load(LocalConfigWrapper {
        model_path: std::path::PathBuf::from(&model_path),
        ..Default::default()
    }) {
        Ok(model) => {
            println!("✅ Local AI Model Loaded Successfully from {}", model_path);
            Some(model)
        }
        Err(e) => {
            println!(
                "⚠️ Failed to load Local AI Model from {}: {}",
                model_path, e
            );
            None
        }
    };

    // Initialize Iron Split System (Mistral 7B)
    let iron_split = match infra_ai::iron_split::IronSplitSystem::new() {
        Ok(system) => {
            println!("✅ Iron Split System (Mistral 7B) Loaded Successfully");
            Some(Arc::new(std::sync::Mutex::new(system)))
        }
        Err(e) => {
            println!("⚠️ Failed to load Iron Split System: {}", e);
            None
        }
    };

    // Initialize AI Mirror components
    let conversation_memory = Arc::new(match pool.as_ref() {
        Some(p) => ConversationMemory::new(p.clone(), 100),
        None => {
            println!("Using in-memory conversation storage for AI Mirror");
            ConversationMemory::new_in_memory(100)
        }
    });

    let mut socratic_engine_instance = SocraticEngine::new(conversation_memory.clone());
    socratic_engine_instance.set_gemini_client(gemini_client);

    // Initialize Antigravity Client (Enterprise Bridge)
    let antigravity_client = infra_ai::antigravity::AntigravityClient::new();
    socratic_engine_instance.set_antigravity_client(antigravity_client);

    // Pass shared Local model to Socratic Engine
    if let Some(ref model) = shared_local_model {
        socratic_engine_instance.set_local_model(model.clone());
    }

    // Pass Iron Split System to Socratic Engine
    if let Some(ref system) = iron_split {
        socratic_engine_instance.set_iron_split(system.clone());
    }

    // Pass database pool to Socratic Engine for RAG
    if let Some(ref db_pool) = pool {
        socratic_engine_instance.set_db_pool(db_pool.clone());
    }

    let socratic_engine = Arc::new(tokio::sync::RwLock::new(socratic_engine_instance));

    println!("AI Mirror Socratic Engine initialized and connected to Gemini 3 Ultra");

    let model_manager = Arc::new(tokio::sync::Mutex::new(
        crate::services::model_manager::ModelManager::new()
            .expect("Failed to initialize ModelManager"),
    ));

    // Auto-download "pete" model if missing
    {
        let mut manager = model_manager.lock().await;
        if !manager.has_model("pete") {
            println!("'pete' model not found. Starting automatic download...");
            let models = crate::services::model_manager::ModelManager::list_available_models();
            if let Some(pete_def) = models.iter().find(|m| m.alias == "pete") {
                match manager.download_model(pete_def).await {
                    Ok(path) => println!("Successfully downloaded 'pete' model to {:?}", path),
                    Err(e) => eprintln!("Failed to download 'pete' model: {}", e),
                }
            }
        } else {
            println!("'pete' model found. Ready for inference.");
        }
    }

    let pete_assistant = Arc::new(
        crate::services::pete::PeteAssistant::new().expect("Failed to initialize PeteAssistant"),
    );

    // Initialize Local Vector DB (LanceDB)
    let lancedb_path =
        std::env::var("LANCEDB_PATH").unwrap_or_else(|_| "data/brain_vectors".to_string());
    let memory_store = match infra_db::LanceDbConnection::new(&lancedb_path).await {
        Ok(store) => {
            println!(
                "🧠 [Memory] Local Vector DB initialized at '{}'",
                lancedb_path
            );
            Some(Arc::new(store))
        }
        Err(e) => {
            eprintln!("⚠️ [Memory] Failed to initialize LanceDB: {}", e);
            None
        }
    };

    // Initialize Weigh Station & Shared Local Model
    let weigh_station = if let Some(db_pool) = pool.clone() {
        // We pass the optional local model. If it's None, the service will use heuristics only.
        Some(Arc::new(
            crate::services::weigh_station::WeighStationService::new(
                db_pool,
                shared_local_model.clone(),
            ),
        ))
    } else {
        println!("⚠️ Database not available, Weigh Station disabled.");
        None
    };

    // Initialize Quest Repository
    let quest_repo: Arc<dyn crate::repositories::quest_repo::QuestRepository> = match pool.clone() {
        Some(p) => Arc::new(crate::repositories::quest_repo::PostgresQuestRepository::new(p)),
        None => {
            println!("⚠️ Database not available, using Mock Quest Repository.");
            Arc::new(crate::repositories::quest_repo::MockQuestRepository)
        }
    };

    // Initialize Chat Queue Service
    let chat_queue = crate::services::chat_queue::ChatQueueService::new(socratic_engine.clone());

    // Create the application state
    let app_state = AppState {
        leptos_options,
        pool,
        shared_research_log: shared_research_log.0.clone(),
        shared_virtues: shared_virtues.0.clone(),
        // gemma_server,
        conversation_memory,
        socratic_engine,
        model_manager,
        pete_assistant,
        pete_command_inbox,
        pete_response_outbox,
        shared_physics,
        weigh_station, // Enabled
        chat_queue,    // [NEW] Job Queue
        shared_campaign_state,
        vote_inbox,
        shared_story_progress: shared_story_progress.0,
        quest_command_inbox,
        quest_repo, // [NEW]
                    // memory_store,
    };

    // Create Model App State
    let model_app_state = crate::routes::model_routes::ModelAppState {
        download_inbox,
        download_state,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route(
            "/auth/login",
            axum::routing::get(crate::handlers::auth::google_login_url),
        )
        .route(
            "/auth/callback",
            axum::routing::get(crate::handlers::auth::google_callback),
        )
        // Grand Central Station Routes
        .route("/cab", axum::routing::get(static_handler))
        .route("/yard", axum::routing::get(static_handler))
        .route("/tower", axum::routing::get(static_handler))
        .merge(player_routes(&app_state))
        .merge(persona_routes(&app_state))
        .merge(expert_routes(&app_state))
        .merge(research_routes(&app_state))
        .merge(crate::routes::pete::pete_routes(&app_state))
        .merge(crate::routes::recharge::recharge_routes(&app_state))
        .merge(crate::routes::simulation::simulation_routes())
        .merge(ai_mirror_routes())
        .nest(
            "/api/models",
            crate::routes::model_routes::model_routes().with_state(model_app_state),
        )
        // .merge(crate::routes::debug::debug_routes())
        .merge(crate::routes::knowledge::knowledge_routes())
        .nest("/api/weigh_station", weigh_station_routes())
        .merge(crate::routes::scenarios::scenarios_routes(&app_state))
        .merge(crate::routes::story_graphs::story_graph_routes(&app_state)) // [NEW] Story graph persistence
        .merge(crate::routes::campaign_routes::campaign_routes())
        .merge(crate::routes::character_routes::character_routes(
            &app_state,
        ))
        .route(
            "/api/architect/blueprint",
            axum::routing::post(crate::handlers::architect::generate_blueprint),
        )
        .route(
            "/api/quest/start/:id",
            axum::routing::post(crate::handlers::quest::start_quest),
        )
        .route(
            "/api/quest/complete/:id",
            axum::routing::post(crate::handlers::quest::complete_quest),
        )
        .layer(axum::middleware::from_fn(
            crate::middleware::auth::auth_middleware,
        ))
        .layer(cors)
        .layer(TimeoutLayer::new(Duration::from_secs(60))) // 60s timeout for inference
        .layer(CatchPanicLayer::new())
        .with_state(app_state) // Apply state BEFORE fallback
        .fallback(static_handler); // Fallback LAST so it doesn't catch API routes

    println!("Backend listening on http://{}", &addr);
    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}
