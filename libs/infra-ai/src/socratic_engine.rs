use crate::architect::{BlueprintRequest, BlueprintResponse};
use crate::knowledge_retrieval::{format_chunks_for_prompt, retrieve_knowledge};
use crate::prompts::PromptStrategy;
use anyhow::Result;
use chrono::Utc;
use infra_db::conversation_memory::{ConversationMemory, Speaker, Turn};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

/// Response from the Socratic engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocraticResponse {
    pub text: String,
    pub strategy_used: PromptStrategy,
}

/// Context for the current session
pub struct SessionContext {
    pub session_id: Uuid,
    pub user_id: i64,
    pub archetype: Option<String>,
    pub focus_area: Option<String>,
}

/// Main Socratic dialogue engine
pub struct SocraticEngine {
    gemini_client: Option<crate::llm::gemini_client::GeminiClient>,
    local_model: Option<crate::local_inference::LocalModel>,
    antigravity_client: Option<crate::antigravity::AntigravityClient>,
    weigh_station: Option<crate::weigh_station::WeighStation>,
    memory: Arc<ConversationMemory>,
    db_pool: Option<PgPool>,
}

impl SocraticEngine {
    /// Create a new Socratic engine
    pub fn new(memory: Arc<ConversationMemory>) -> Self {
        Self {
            gemini_client: None,
            local_model: None,
            antigravity_client: None,
            weigh_station: None,
            memory,
            db_pool: None,
        }
    }

    /// Set the database pool for RAG knowledge retrieval
    pub fn set_db_pool(&mut self, pool: PgPool) {
        self.db_pool = Some(pool);
        log::info!("Database pool connected to Socratic engine for RAG");
    }

    /// Set the Gemini client for LLM inference
    pub fn set_gemini_client(&mut self, client: crate::llm::gemini_client::GeminiClient) {
        self.gemini_client = Some(client);
        log::info!("Gemini client connected to Socratic engine");
    }

    /// Set the Antigravity client for Steam sync
    pub fn set_antigravity_client(&mut self, client: crate::antigravity::AntigravityClient) {
        self.antigravity_client = Some(client);
        log::info!("Antigravity client connected to Socratic engine");
    }

    /// Set the local model
    pub fn set_local_model(&mut self, model: crate::local_inference::LocalModel) {
        // Initialize WeighStation with the model
        let weigh_station = crate::weigh_station::WeighStation::new(model.clone());
        self.weigh_station = Some(weigh_station);

        self.local_model = Some(model);
        log::info!("Local model connected to Socratic engine");
    }

    /// Generate a Socratic response to user input
    pub async fn respond(
        &mut self,
        user_input: &str,
        context: &SessionContext,
    ) -> Result<SocraticResponse> {
        log::debug!(
            "Generating Socratic response for session {}",
            context.session_id
        );

        // 1. Save user's turn to memory
        let user_turn = Turn {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            speaker: Speaker::User,
            content: user_input.to_string(),
            metadata: Default::default(),
        };
        self.memory.add_turn(context.session_id, user_turn).await?;

        // 2. Retrieve recent conversation history
        let history = self.memory.get_recent(context.session_id, 10).await?;
        log::debug!(
            "Retrieved {} turns from conversation history",
            history.len()
        );

        // 3. Retrieve relevant knowledge from RAG database
        let knowledge_context = if let Some(ref pool) = self.db_pool {
            match retrieve_knowledge(user_input, pool, Some(3)).await {
                Ok(chunks) => {
                    log::info!("Retrieved {} knowledge chunks for RAG", chunks.len());
                    format_chunks_for_prompt(&chunks)
                }
                Err(e) => {
                    log::warn!("Failed to retrieve knowledge: {}", e);
                    String::new()
                }
            }
        } else {
            log::debug!("No database pool available for RAG retrieval");
            String::new()
        };

        // 4. Select prompting strategy based on user input
        let strategy = PromptStrategy::select_strategy(user_input, &history);
        log::debug!("Selected strategy: {:?}", strategy);

        // 5. Build prompt with template (including RAG knowledge)
        let mut prompt = strategy.build_prompt(user_input, &history, context);

        // Inject knowledge context before user question if available
        if !knowledge_context.is_empty() {
            prompt = format!("{}{}", knowledge_context, prompt);
            log::debug!(
                "Injected {} chars of knowledge into prompt",
                knowledge_context.len()
            );
        }
        log::debug!("Built prompt: {} chars", prompt.len());

        // 6. Generate response using LLM
        let response_text = if let Some(ref model) = self.local_model {
            // Use local model
            let config = crate::local_inference::GenerationConfig {
                max_tokens: 1024,
                temperature: 0.7,
                top_p: 0.9,
                repeat_penalty: 1.1,
                seed: 42,
            };
            match model.generate(prompt.clone(), config).await {
                Ok(text) => text,
                Err(e) => {
                    log::error!("Local model generation failed: {}", e);
                    "I'm having trouble accessing my local memory banks.".to_string()
                }
            }
        } else if let Some(ref mut gemini_client) = self.gemini_client {
            // Actual inference using Gemini
            match gemini_client.generate(&prompt).await {
                Ok(text) => text,
                Err(e) => {
                    log::error!("Gemini generation failed: {}", e);
                    "I'm having trouble connecting to my thoughts (Gemini API Error).".to_string()
                }
            }
        } else {
            // Fallback if Gemini client not connected
            log::warn!("Gemini client not connected, using fallback response");
            "I'm listening. Can you tell me more about that?".to_string()
        };

        // 7. Post-process response
        let processed_response = Self::post_process_response(&response_text);

        // 8. Save AI's turn to memory
        let ai_turn = Turn {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            speaker: Speaker::AI,
            content: processed_response.clone(),
            metadata: Default::default(),
        };
        self.memory.add_turn(context.session_id, ai_turn).await?;

        // 9. Generate Steam (Mastery) & Sync to Antigravity
        // Simple heuristic: 1 Steam per successful turn
        let steam_earned = pete_core::economy::Steam(1.0);
        if let Some(ref client) = self.antigravity_client {
            let user_id_str = context.user_id.to_string();
            // Fire and forget sync (don't block response)
            let _ = client
                .sync_steam(&user_id_str, steam_earned, "socratic_dialogue")
                .await;
            log::info!(
                "Generated Steam: {:.2} and syncing to Antigravity",
                steam_earned.0
            );
        }

        Ok(SocraticResponse {
            text: processed_response,
            strategy_used: strategy,
        })
    }

    /// Analyze the cognitive load of content
    pub async fn analyze_load(
        &self,
        content: &str,
    ) -> Result<pete_core::physics::CognitiveLoadProfile> {
        if let Some(ref ws) = self.weigh_station {
            ws.analyze_load(content).await
        } else {
            // Fallback if no WeighStation (no local model)
            // Return a default safe profile
            Ok(pete_core::physics::CognitiveLoadProfile {
                intrinsic: 5.0,
                extraneous: 5.0,
                germane: 5.0,
            })
        }
    }

    /// Generate a curriculum blueprint (StoryGraph)
    pub async fn generate_blueprint(&mut self, req: BlueprintRequest) -> Result<BlueprintResponse> {
        log::info!(
            "Socratic Engine: Generating blueprint for '{}'",
            req.subject
        );

        let weigh_station = self
            .weigh_station
            .clone()
            .ok_or_else(|| anyhow::anyhow!("WeighStation not available (requires local model)"))?;

        let mut architect = crate::architect::CurriculumArchitect::new(
            self.gemini_client.clone(),
            self.local_model.clone(),
            weigh_station,
        );

        architect.generate_blueprint(req).await
    }

    /// Post-process AI response to ensure it's Socratic
    fn post_process_response(response: &str) -> String {
        let mut processed = response.trim().to_string();

        // Ensure response doesn't give direct answers (basic heuristic)
        // TODO: More sophisticated filtering

        // Ensure response ends with a question mark
        if !processed.ends_with('?') {
            if !processed.is_empty() {
                processed.push_str("?");
            }
        }

        processed
    }
}
