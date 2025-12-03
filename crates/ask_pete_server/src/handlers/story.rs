use crate::error::Result;
use crate::state::AppState;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GenerateStoryRequest {
    pub vaam_words: Vec<String>,
    pub theme: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateStoryResponse {
    pub story: String,
}

/// Generate a short story incorporating the provided VaaM words
pub async fn generate_story(
    State(state): State<AppState>,
    Json(payload): Json<GenerateStoryRequest>,
) -> Result<Json<GenerateStoryResponse>> {
    println!(
        "Story Handler: Generating story for theme: '{}' with words: {:?}",
        payload.theme, payload.vaam_words
    );

    let words_list = payload.vaam_words.join(", ");
    let prompt = format!(
        r#"You are a Storyteller for the Iron Network.

THEME: "{}"
VOCABULARY WORDS: {}

Write a short, engaging story (approx. 200-300 words) that naturally incorporates ALL of the provided vocabulary words.
The story should be fun, slightly gamified (referencing "Operators", "Trains", or "The Static" if appropriate for the theme, otherwise stick to the requested theme), and educational.

Highlight the vocabulary words in the story by wrapping them in **bold**.

Generate the story now:"#,
        payload.theme, words_list
    );

    // Use Socratic Engine's LLM client
    let engine = state.socratic_engine.read().await;

    // Generate using LLM
    let response_text = match engine.llm_client() {
        Some(client) => client
            .generate_text(&prompt)
            .await
            .map_err(|e| anyhow::anyhow!("LLM generation failed: {}", e))?,
        None => {
            return Err(anyhow::anyhow!("LLM client not available").into());
        }
    };

    println!(
        "Story Handler: Generated story of length {}",
        response_text.len()
    );

    Ok(Json(GenerateStoryResponse {
        story: response_text,
    }))
}
