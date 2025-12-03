use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub description: String,
    pub hf_repo: String,
    pub filename: String,
    pub size_mb: f32,
}

pub fn get_pete_brains() -> Vec<ModelConfig> {
    vec![
        ModelConfig {
            name: "Speedy Pete".to_string(),
            description: "Fast responses, low memory usage. Good for older devices.".to_string(),
            hf_repo: "bartowski/Llama-3.2-1B-Instruct-GGUF".to_string(),
            filename: "Llama-3.2-1B-Instruct-Q4_K_M.gguf".to_string(),
            size_mb: 800.0,
        },
        ModelConfig {
            name: "Deep Thinker Pete".to_string(),
            description: "Better nuance, handles complex scenarios. Recommended for desktops."
                .to_string(),
            hf_repo: "bartowski/Llama-3.2-3B-Instruct-GGUF".to_string(),
            filename: "Llama-3.2-3B-Instruct-Q4_K_M.gguf".to_string(),
            size_mb: 2200.0,
        },
    ]
}
