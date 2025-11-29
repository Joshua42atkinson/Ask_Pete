use super::conversation_memory::{ConversationMemory, Speaker, Turn};
use super::prompts::PromptStrategy;
use crate::ai::architect::{BlueprintRequest, BlueprintResponse}; // [NEW]
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
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

use tokio::sync::Mutex;

/// Main Socratic dialogue engine
pub struct SocraticEngine {
    gemini_client: Option<crate::ai::llm::gemini_client::GeminiClient>,
    gemma_model: Option<crate::ai::local_inference::GemmaModel>,
    antigravity_client: Option<crate::antigravity::AntigravityClient>,
    memory: Arc<ConversationMemory>,
}

impl SocraticEngine {
    /// Create a new Socratic engine
    pub fn new(memory: Arc<ConversationMemory>) -> Self {
        Self {
            gemini_client: None,
            gemma_model: None,
            antigravity_client: None,
            memory,
        }
    }

    /// Set the Gemini client for LLM inference
    pub fn set_gemini_client(&mut self, client: crate::ai::llm::gemini_client::GeminiClient) {
        self.gemini_client = Some(client);
        log::info!("Gemini client connected to Socratic engine");
    }

    /// Set the Antigravity client for Steam sync
    pub fn set_antigravity_client(&mut self, client: crate::antigravity::AntigravityClient) {
        self.antigravity_client = Some(client);
        log::info!("Antigravity client connected to Socratic engine");
    }

    /// Set the local Gemma model
    pub fn set_gemma_model(&mut self, model: crate::ai::local_inference::GemmaModel) {
        self.gemma_model = Some(model);
        log::info!("Local Gemma model connected to Socratic engine");
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

        // 3. Select prompting strategy based on user input
        let strategy = PromptStrategy::select_strategy(user_input, &history);
        log::debug!("Selected strategy: {:?}", strategy);

        // 4. Build prompt with template
        let prompt = strategy.build_prompt(user_input, &history, context);
        log::debug!("Built prompt: {} chars", prompt.len());

        // 5. Generate response using LLM
        let response_text = if let Some(ref model) = self.gemma_model {
            // Use local Gemma model
            // TODO: Pass proper config
            let config = crate::ai::local_inference::GenerationConfig {
                max_tokens: 1024,
                temperature: 0.7,
                top_p: 0.9,
                repeat_penalty: 1.1,
            };
            match model.generate(prompt.clone(), config).await {
                Ok(text) => text,
                Err(e) => {
                    log::error!("Gemma generation failed: {}", e);
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

        // 6. Post-process response
        let processed_response = Self::post_process_response(&response_text);

        // 7. Save AI's turn to memory
        let ai_turn = Turn {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            speaker: Speaker::AI,
            content: processed_response.clone(),
            metadata: Default::default(),
        };
        self.memory.add_turn(context.session_id, ai_turn).await?;

        // 8. Generate Steam (Mastery) & Sync to Antigravity
        // Simple heuristic: 1 Steam per successful turn
        let steam_earned = common::economy::Steam(1.0);
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

    /// Generate a curriculum blueprint (StoryGraph)
    pub async fn generate_blueprint(&mut self, req: BlueprintRequest) -> Result<BlueprintResponse> {
        log::info!("Socratic Engine: Generating blueprint for '{}'", req.subject);

        // 1. Construct the Prompt (Reusing logic from CurriculumArchitect for now)
        // TODO: Centralize prompt construction
        let prompt = format!(
            r#"
            You are the "Curriculum Architect", an expert instructional designer and storyteller.
            
            CONTEXT & LORE:
            {lore_context}

            GOAL: Create a non-linear learning path (StoryGraph) for the subject: "{subject}".
            
            CONSTRAINTS:
            - Focus Balance: {focus} (0.0 = Pure Academic, 1.0 = Pure Narrative).
            - Literary Device: "{device}".
            - Required Vocabulary: {vocab:?}.
            
            NARRATIVE DEVICE INSTRUCTIONS:
            {device_prompt}

            OUTPUT FORMAT:
            Return a JSON object matching this structure:
            {{
                "graph": {{
                    "id": "generated_uuid",
                    "title": "Campaign Title",
                    "nodes": [
                        {{
                            "id": "node_1",
                            "title": "Node Title",
                            "content": "Narrative or Instructional Content",
                            "x": 0.0,
                            "y": 0.0,
                            "passenger_count": 1,
                            "complexity_level": 1,
                            "learner_profiles": [],
                            "gardens_active": [],
                            "required_stats": {{}},
                            "logic": {{
                                "condition": "None", 
                                "effect": "None"
                            }}
                        }}
                    ],
                    "connections": [
                        {{
                            "id": "conn_1",
                            "from_node": "node_1",
                            "to_node": "node_2"
                        }}
                    ]
                }},
                "reasoning": "Brief explanation of design choices."
            }}
            
            RULES:
            1. Create at least 5 nodes.
            2. Ensure the graph branches (non-linear).
            3. Integrate the vocabulary words into the node content.
            4. Adjust 'complexity_level' based on the progression.
            5. Use the terminology from the LORE (Sectors, Chassis, etc.) in the node titles and content where appropriate.
            6. TREAT WORDS AS SYMBOLS OF POWER. In the Iron Network, knowing the definition of a word (like 'Velocity') is not just academic—it grants control over the environment (e.g., opening doors, powering engines).
            7. **IMPORTANT**: Use the 'logic' field to create interactive elements.
               - 'condition': "None", "GreaterThan {{ variable: 'Strength', value: 10.0 }}", "HasItem {{ item_id: 'Key' }}"
               - 'effect': "None", "ModifyVariable {{ variable: 'Strength', delta: 5.0 }}", "GrantItem {{ item_id: 'Key' }}"
            "#,
            lore_context = crate::ai::lore::get_lore_context(),
            subject = req.subject,
            focus = req.focus,
            device = req.literary_device,
            device_prompt = crate::ai::lore::get_device_prompt(&req.literary_device),
            vocab = if req.vocabulary.is_empty() {
                crate::ai::vocabulary::get_physics_vocabulary()
                    .into_iter()
                    .map(|t| format!("{}: {}", t.word, t.definition))
                    .collect::<Vec<String>>()
            } else {
                req.vocabulary
            }
        );

        // 2. Generate using available model
        let response_text = if let Some(ref model) = self.gemma_model {
             // Use local Gemma model
             let config = crate::ai::local_inference::GenerationConfig {
                max_tokens: 2048, // Higher limit for JSON
                temperature: 0.7,
                top_p: 0.9,
                repeat_penalty: 1.1,
            };
            model.generate(prompt, config).await?
        } else if let Some(ref mut gemini_client) = self.gemini_client {
            gemini_client.generate(&prompt).await?
        } else {
            return Err(anyhow::anyhow!("No AI model available for blueprint generation"));
        };

        // 3. Parse JSON
        let clean_json = crate::ai::architect::extract_json(&response_text).unwrap_or(response_text);
        let response: BlueprintResponse = serde_json::from_str(&clean_json)?;

        Ok(response)
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
