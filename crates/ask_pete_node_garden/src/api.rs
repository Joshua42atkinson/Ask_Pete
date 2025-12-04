use leptos::prelude::ServerFnError;
use pete_core::expert::StoryGraph;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PartyStyle {
    #[default]
    IronHorse, // Speed/Efficiency
    ScenicRoute, // Exploration/Lore
    NightTrain,  // Hardcore/Challenge
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MemberRole {
    #[default]
    Engineer, // DPS / Warrior
    Conductor, // Tank / Leader
    Stoker,    // Healer / Support
    Signalman, // CC / Mage
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CharacterProfile {
    pub name: String,
    pub role: MemberRole,
    pub archetype: String,
    pub level: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReflectionData {
    pub focus_area: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendMessageRequest {
    pub session_id: Uuid,
    pub user_id: i64,
    pub message: String,
    pub archetype: Option<String>,
    pub focus_area: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendMessageResponse {
    pub ai_response: String,
    pub session_id: Uuid,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateSessionResponse {
    pub session_id: Uuid,
}

pub async fn create_session() -> Result<Uuid, String> {
    let client = Client::new();
    let res = client
        .post("/api/ai-mirror/create-session")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let body: CreateSessionResponse = res.json().await.map_err(|e| e.to_string())?;
        Ok(body.session_id)
    } else {
        Err(format!("Failed to create session: {}", res.status()))
    }
}

pub async fn send_message(req: SendMessageRequest) -> Result<SendMessageResponse, String> {
    let client = Client::new();
    let res = client
        .post("/api/ai-mirror/send-message")
        .json(&req)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let body: SendMessageResponse = res.json().await.map_err(|e| e.to_string())?;
        Ok(body)
    } else {
        Err(format!("Failed to send message: {}", res.status()))
    }
}

// --- Expert Module API ---

pub async fn get_graph(id: Option<String>) -> Result<StoryGraph, String> {
    let client = Client::new();
    let url = if let Some(graph_id) = id {
        format!("/api/story_graphs/{}", graph_id)
    } else {
        "/api/expert/graph".to_string() // Fallback to "current" or "latest"
    };

    let res = client.get(&url).send().await.map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let graph: StoryGraph = res.json().await.map_err(|e| e.to_string())?;
        Ok(graph)
    } else {
        Err(format!("Failed to fetch graph: {}", res.status()))
    }
}

pub async fn save_graph(graph: StoryGraph) -> Result<StoryGraph, String> {
    let client = Client::new();
    let res = client
        .post("/api/expert/graph")
        .json(&graph)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let saved_graph: StoryGraph = res.json().await.map_err(|e| e.to_string())?;
        Ok(saved_graph)
    } else {
        Err(format!("Failed to save graph: {}", res.status()))
    }
}

// --- Model Management API ---

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub size: String,
    pub downloaded: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DownloadProgress {
    pub model_id: Option<String>,
    pub status: String,  // "idle", "downloading", "completed", "error"
    pub percentage: f32, // 0.0 to 100.0
    pub speed: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadRequest {
    pub model_id: String,
}

pub async fn get_models() -> Result<Vec<ModelInfo>, String> {
    let client = Client::new();
    let res = client
        .get("/api/models")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let models: Vec<ModelInfo> = res.json().await.map_err(|e| e.to_string())?;
        Ok(models)
    } else {
        Err(format!("Failed to fetch models: {}", res.status()))
    }
}

pub async fn download_model(model_id: String) -> Result<(), String> {
    let req = DownloadRequest { model_id };
    let client = Client::new();
    let res = client
        .post("/api/models/download")
        .json(&req)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(format!("Failed to start download: {}", res.status()))
    }
}

pub async fn get_download_progress() -> Result<DownloadProgress, String> {
    let client = Client::new();
    let res = client
        .get("/api/models/progress")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let progress: DownloadProgress = res.json().await.map_err(|e| e.to_string())?;
        Ok(progress)
    } else {
        Err(format!("Failed to fetch progress: {}", res.status()))
    }
}

// --- Multiplayer Campaign API ---

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignState {
    pub campaign_id: String,
    pub style: PartyStyle, // [NEW]
    pub current_station_id: String,
    pub party_members: HashMap<String, CharacterProfile>, // [MODIFIED]
    pub collective_coal: u32,
    pub collective_steam: u32,
    pub active_vote: Option<VoteParams>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoteParams {
    pub question: String,
    pub options: Vec<String>,
    pub deadline: u64,
    pub current_tally: Vec<u32>,
}

#[derive(Debug, Serialize)]
pub struct VoteRequest {
    pub campaign_id: String,
    pub option_index: usize,
}

pub async fn fetch_campaign_state() -> Result<CampaignState, String> {
    let client = Client::new();
    let res = client
        .get("/api/campaign/state")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let state: CampaignState = res.json().await.map_err(|e| e.to_string())?;
        Ok(state)
    } else {
        Err(format!("Failed to fetch campaign state: {}", res.status()))
    }
}

pub async fn submit_vote(campaign_id: String, option_index: usize) -> Result<(), String> {
    let req = VoteRequest {
        campaign_id,
        option_index,
    };
    let client = Client::new();
    let res = client
        .post("/api/campaign/vote")
        .json(&req)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(format!("Failed to submit vote: {}", res.status()))
    }
}

// --- Character Creation API ---

#[derive(Debug, Serialize)]
pub struct CreateCharacterRequest {
    pub name: String,
    pub role: MemberRole,
    pub archetype: String,
}

pub async fn create_character(req: CreateCharacterRequest) -> Result<(), String> {
    // In a real app, this would POST to /api/character/create
    // For now, we'll just log it to console as a mock
    // PII SAFE: Logging removed for privacy compliance
    // gloo_console::info!(format!("Creating character: {:?}", req));
    Ok(())
}
