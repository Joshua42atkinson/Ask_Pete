use crate::ai::llm::gemini_client::GeminiClient;
use anyhow::Result;
use common::expert::StoryGraph;
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
    gemini: GeminiClient,
}

impl CurriculumArchitect {
    pub fn new(gemini: GeminiClient) -> Self {
        Self { gemini }
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
               - Example: A node that requires 'Strength' > 5 to enter, or a node that grants a 'Key' upon visit.
            "#,
            lore_context = crate::ai::lore::get_lore_context(),
            subject = req.subject,
            focus = req.focus,
            device = req.literary_device,
            device_prompt = crate::ai::lore::get_device_prompt(&req.literary_device),
            vocab = if req.vocabulary.is_empty() {
                // Auto-inject physics vocabulary if none provided
                crate::ai::vocabulary::get_physics_vocabulary()
                    .into_iter()
                    .map(|t| format!("{}: {}", t.word, t.definition))
                    .collect::<Vec<String>>()
            } else {
                req.vocabulary
            }
        );

        // 2. Call Gemini
        let response_text = self.gemini.generate(&prompt).await?;

        // 3. Parse JSON (Naive parsing for now, assuming Gemini returns pure JSON or wrapped in markdown)
        // In a real impl, we'd need robust JSON extraction from markdown code blocks.
        let clean_json = extract_json(&response_text).unwrap_or(response_text.to_string());

        let response: BlueprintResponse = serde_json::from_str(&clean_json)?;

        Ok(response)
    }
}

pub fn extract_json(text: &str) -> Option<String> {
    // Simple helper to strip markdown code blocks if present
    if let Some(start) = text.find("```json") {
        if let Some(end) = text[start..].find("```") {
            // This logic is flawed for finding the *closing* brace, need better extraction.
            // For now, let's assume Gemini behaves or we use a library later.
            // Just returning the whole text if simple stripping fails.
            return Some(
                text.replace("```json", "")
                    .replace("```", "")
                    .trim()
                    .to_string(),
            );
        }
    }
    Some(text.to_string())
}
