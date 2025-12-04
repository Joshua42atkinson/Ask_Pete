use crate::llm::gemini_client::GeminiClient;
use crate::LocalModel;
use anyhow::Result;
use pete_core::expert::StoryGraph;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlueprintRequest {
    pub subject: String,
    pub focus: f32,              // 0.0 (Pure Subject) to 1.0 (Pure Story)
    pub literary_device: String, // e.g., "Hero's Journey"
    pub vocabulary: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlueprintResponse {
    pub graph: StoryGraph,
    pub reasoning: String,
}

/// The Architect: Generates a curriculum map (StoryGraph) from constraints.
pub struct CurriculumArchitect {
    gemini: Option<GeminiClient>,
    local_model: Option<LocalModel>,
}

impl CurriculumArchitect {
    pub fn new(gemini: Option<GeminiClient>, local_model: Option<LocalModel>) -> Self {
        Self {
            gemini,
            local_model,
        }
    }

    pub async fn generate_blueprint(&mut self, req: BlueprintRequest) -> Result<BlueprintResponse> {
        // 1. Construct the Prompt
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

            INSTRUCTIONS (CHAIN OF THOUGHT):
            Step 1: Outline 5 key concepts related to the subject.
            Step 2: Connect them logically to form a narrative progression.
            Step 3: Output the result as valid JSON matching the schema below.
            
            OUTPUT FORMAT:
            Return a JSON object matching this structure. Ensure the JSON is the LAST part of your response.
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
                            }},
                            "quest": {{
                                "title": "Quest Title",
                                "chapter_theme": "Theme (e.g. 'The Call')",
                                "description": "Quest Description",
                                "starting_step": "step_1",
                                "completion_reward": {{
                                    "type": "xp",
                                    "value": 100
                                }},
                                "steps": {{
                                    "step_1": {{
                                        "description": "Step 1 Description",
                                        "trigger_condition": "None",
                                        "next_step": "None",
                                        "is_major_plot_point": true
                                    }}
                                }}
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
            7. **CRITICAL**: The 'logic' field MUST use proper JSON enum format (NOT Rust syntax strings):
            
               CONDITION OPTIONS (choose one):
               - No condition: "None"
               - Check variable greater than value: {{"GreaterThan": {{"variable": "Strength", "value": 10.0}}}}
               - Check variable less than value: {{"LessThan": {{"variable": "Speed", "value": 5.0}}}}
               - Check variable equals value: {{"Equals": {{"variable": "Level", "value": 1.0}}}}
               - Check if player has item: {{"HasItem": {{"item_id": "ancient_key"}}}}
               
               EFFECT OPTIONS (choose one):
               - No effect: "None"
               - Modify a variable: {{"ModifyVariable": {{"variable": "Strength", "delta": 5.0}}}}
               - Grant an item: {{"GrantItem": {{"item_id": "rusty_wrench"}}}}
               - Consume an item: {{"ConsumeItem": {{"item_id": "coal_chunk"}}}}
            
               EXAMPLE LOGIC BLOCKS:
               - Simple node (no logic): {{"condition": "None", "effect": "None"}}
               - Locked node requiring strength: {{"condition": {{"GreaterThan": {{"variable": "Strength", "value": 5.0}}}}, "effect": "None"}}
               - Node that grants item: {{"condition": "None", "effect": {{"GrantItem": {{"item_id": "station_key"}}}}}}
               - Complex: requires item AND grants stat boost: {{"condition": {{"HasItem": {{"item_id": "wrench"}}}}, "effect": {{"ModifyVariable": {{"variable": "Strength", "delta": 10.0}}}}}}
            "#,
            lore_context = crate::lore::get_lore_context(),
            subject = req.subject,
            focus = req.focus,
            device = req.literary_device,
            device_prompt = crate::lore::get_device_prompt(&req.literary_device),
            vocab = if req.vocabulary.is_empty() {
                // Auto-inject physics vocabulary if none provided
                crate::vocabulary::get_physics_vocabulary()
                    .into_iter()
                    .map(|t| format!("{}: {}", t.word, t.definition))
                    .collect::<Vec<String>>()
            } else {
                req.vocabulary
            }
        );

        // 2. Call LLM (Local or Gemini)
        let response_text = if let Some(ref model) = self.local_model {
            log::info!("Architect using Local Model (Gemma 2)");
            let config = crate::local_inference::GenerationConfig {
                max_tokens: 2048, // Need more tokens for JSON
                temperature: 0.7,
                top_p: 0.9,
                repeat_penalty: 1.1,
            };
            model.generate(prompt.clone(), config).await?
        } else if let Some(ref mut gemini) = self.gemini {
            log::info!("Architect using Gemini Cloud");
            gemini.generate(&prompt).await?
        } else {
            return Err(anyhow::anyhow!("No AI model available for Architect"));
        };

        // 3. Parse JSON using robust shared utility
        let clean_json = crate::json_utils::extract_json_from_text(&response_text)
            .unwrap_or_else(|| response_text.to_string());

        let response: BlueprintResponse = serde_json::from_str(&clean_json)?;

        Ok(response)
    }
}
