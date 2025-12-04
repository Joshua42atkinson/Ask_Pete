use crate::services::pete::PeteResponse;
use infra_ai::socratic_engine::SocraticEngine;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use uuid::Uuid;

// 1. Define the States
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "status", content = "data")]
pub enum JobStatus {
    Queued,
    Processing,
    Completed(PeteResponse),
    Failed(String),
}

struct ChatJob {
    id: Uuid,
    message: String,
    // Add user_id or context here if needed
}

// 2. The Service Struct
#[derive(Clone)]
pub struct ChatQueueService {
    sender: mpsc::Sender<ChatJob>,
    results: Arc<RwLock<HashMap<Uuid, JobStatus>>>,
}

impl ChatQueueService {
    pub fn new(engine: Arc<tokio::sync::RwLock<SocraticEngine>>) -> Self {
        let (sender, mut receiver) = mpsc::channel::<ChatJob>(100); // Buffer of 100 jobs
        let results = Arc::new(RwLock::new(HashMap::new()));
        let results_clone = results.clone();

        // 3. Spawn the Background Worker
        tokio::spawn(async move {
            println!("🤖 Chat Queue Worker Started...");
            while let Some(job) = receiver.recv().await {
                // A. Mark as Processing
                {
                    let mut map = results_clone.write().unwrap();
                    map.insert(job.id, JobStatus::Processing);
                }

                // B. Perform the Heavy Lifting (AI Inference)
                // We lock the engine only for the duration of this specific generation
                let response = {
                    let mut engine_guard = engine.write().await;
                    // Mock context for now
                    let context = infra_ai::socratic_engine::SessionContext {
                        session_id: job.id,
                        user_id: 1,
                        archetype: None,
                        focus_area: Some("chat".to_string()),
                    };
                    engine_guard.respond(&job.message, &context).await
                };

                // C. Save Result
                let mut map = results_clone.write().unwrap();
                match response {
                    Ok(data) => {
                        // Convert SocraticResponse to PeteResponse
                        let pete_response = PeteResponse {
                            answer: data.text,
                            citations: vec![], // TODO: Extract citations
                            confidence: 1.0,   // TODO: Get confidence
                            suggestions: vec![],
                        };
                        map.insert(job.id, JobStatus::Completed(pete_response));
                    }
                    Err(e) => {
                        map.insert(job.id, JobStatus::Failed(e.to_string()));
                    }
                }
            }
        });

        Self { sender, results }
    }

    pub async fn enqueue(&self, message: String) -> Uuid {
        let id = Uuid::new_v4();
        let job = ChatJob { id, message };

        // Initialize status
        {
            let mut map = self.results.write().unwrap();
            map.insert(id, JobStatus::Queued);
        }

        // Send to worker (fire and forget)
        if let Err(e) = self.sender.send(job).await {
            let mut map = self.results.write().unwrap();
            map.insert(
                id,
                JobStatus::Failed(format!("Queue full or closed: {}", e)),
            );
        }

        id
    }

    pub fn get_status(&self, id: &Uuid) -> Option<JobStatus> {
        self.results.read().unwrap().get(id).cloned()
    }
}
