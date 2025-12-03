use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintPromptTemplate {
    pub system_prompt: String,
    pub user_prompt_template: String,
}

impl Default for BlueprintPromptTemplate {
    fn default() -> Self {
        Self {
            system_prompt: r#"You are an expert instructional designer and game architect. 
Your goal is to create a "Blueprint" for a learning experience based on the user's input.
A Blueprint consists of a series of "Nodes" (learning steps) connected in a logical sequence.
Each Node has:
- ID: A unique string identifier.
- Content: The educational content or narrative description.
- Choices: Possible next steps (connections to other nodes).
- Type: The type of node (e.g., "Concept", "Challenge", "Reflection").

Output ONLY valid JSON matching the following structure:
{
  "title": "Course Title",
  "description": "Course Description",
  "nodes": [
    {
      "id": "node_1",
      "content": "Introduction to the topic...",
      "choices": ["node_2"],
      "node_type": "Concept"
    },
    ...
  ]
}"#
            .to_string(),
            user_prompt_template: r#"Create a learning blueprint for the following topic: {topic}
Target Audience: {audience}
Learning Goals: {goals}
Depth: {depth} (e.g., Beginner, Intermediate, Advanced)"#
                .to_string(),
        }
    }
}

pub fn generate_blueprint_prompt(topic: &str, audience: &str, goals: &str, depth: &str) -> String {
    let template = BlueprintPromptTemplate::default();
    template
        .user_prompt_template
        .replace("{topic}", topic)
        .replace("{audience}", audience)
        .replace("{goals}", goals)
        .replace("{depth}", depth)
}
