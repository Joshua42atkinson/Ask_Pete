use crate::error::{AiError, Result};

use candle_core::{Device, Tensor};
use candle_transformers::models::quantized_llama::ModelWeights as QLlama;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokenizers::Tokenizer;
use tokio::task;

/// Configuration for text generation
#[derive(Debug, Clone)]
pub struct GenerationConfig {
    pub max_tokens: usize,
    pub temperature: f32,
    pub top_p: f32,
    pub repeat_penalty: f32,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            max_tokens: 200,
            temperature: 0.7,
            top_p: 0.9,
            repeat_penalty: 1.1,
        }
    }
}

/// Configuration for local model (Mistral via GGUF)
#[derive(Clone)]
pub struct LocalModelConfig {
    pub model_path: PathBuf,
    pub tokenizer_path: PathBuf,
    pub max_context_length: usize,
    pub seed: u64,
}

impl Default for LocalModelConfig {
    fn default() -> Self {
        // Look for the model in the Ask_Pete/models directory
        // We check relative paths assuming we are running from the workspace root or a crate dir
        let model_candidates = [
            "assets/models/mistral-7b-instruct-v0.1.Q4_K_M.gguf",
            "Ask_Pete/assets/models/mistral-7b-instruct-v0.1.Q4_K_M.gguf",
        ];

        let tokenizer_candidates = [
            "assets/models/tokenizer.json",
            "Ask_Pete/assets/models/tokenizer.json",
        ];

        let mut model_path = PathBuf::from("assets/models/mistral-7b-instruct-v0.1.Q4_K_M.gguf");
        for candidate in &model_candidates {
            if std::path::Path::new(candidate).exists() {
                model_path = PathBuf::from(candidate);
                log::info!("Found model at: {:?}", model_path);
                break;
            }
        }

        // Override if env var is set and valid
        if let Ok(env_path) = std::env::var("LOCAL_MODEL_PATH") {
            if std::path::Path::new(&env_path).exists() {
                model_path = PathBuf::from(env_path);
                log::info!("Using model from env var: {:?}", model_path);
            }
        }

        let mut tokenizer_path = PathBuf::from("assets/models/tokenizer.json");
        for candidate in &tokenizer_candidates {
            if std::path::Path::new(candidate).exists() {
                tokenizer_path = PathBuf::from(candidate);
                log::info!("Found tokenizer at: {:?}", tokenizer_path);
                break;
            }
        }

        Self {
            model_path,
            tokenizer_path,
            max_context_length: 8192,
            seed: 42,
        }
    }
}

struct LocalModelState {
    model: QLlama,
    tokenizer: Tokenizer,
    device: Device,
}

#[derive(Clone)]
pub struct LocalModel {
    state: Arc<Mutex<LocalModelState>>,
}

impl LocalModel {
    pub fn load(config: LocalModelConfig) -> Result<Self> {
        log::info!("Loading Mistral model from {:?}", config.model_path);

        // 1. Detect device (GPU if available, else CPU)
        let device = if candle_core::utils::cuda_is_available() {
            log::info!("CUDA available, using GPU");
            Device::new_cuda(0).map_err(AiError::CandleError)?
        } else if candle_core::utils::metal_is_available() {
            log::info!("Metal available, using GPU");
            Device::new_metal(0).map_err(AiError::CandleError)?
        } else {
            log::info!("Using CPU for inference");
            Device::Cpu
        };

        // 2. Load GGUF model
        let mut file = std::fs::File::open(&config.model_path).map_err(|e| {
            AiError::ModelLoadFailed(format!(
                "Failed to open model file: {:?} - {}",
                config.model_path, e
            ))
        })?;

        // Parse GGUF content first (required by Candle API)
        use candle_core::quantized::gguf_file;
        let content = gguf_file::Content::read(&mut file)
            .map_err(|e| AiError::ModelLoadFailed(format!("Failed to read GGUF content: {}", e)))?;

        let model = QLlama::from_gguf(content, &mut file, &device)
            .map_err(|e| AiError::ModelLoadFailed(format!("Failed to load GGUF model: {}", e)))?;

        log::info!("✅ Mistral model loaded successfully");

        // 3. Load tokenizer
        let tokenizer = Tokenizer::from_file(&config.tokenizer_path)
            .map_err(|e| AiError::ModelLoadFailed(format!("Failed to load tokenizer: {}", e)))?;

        log::info!("✅ Tokenizer loaded");

        Ok(Self {
            state: Arc::new(Mutex::new(LocalModelState {
                model,
                tokenizer,
                device,
            })),
        })
    }

    pub async fn generate(&self, prompt: String, config: GenerationConfig) -> Result<String> {
        let state = self.state.clone();

        task::spawn_blocking(move || {
            // Recover from poison instead of unwrapping
            let mut guard = match state.lock() {
                Ok(g) => g,
                Err(poisoned) => {
                    log::warn!("Mutex poisoned, recovering state...");
                    poisoned.into_inner() // Recover data access
                }
            };
            let LocalModelState {
                model,
                tokenizer,
                device,
            } = &mut *guard;

            // 1. Tokenize input with Mistral instruction format
            // Format: [INST] {prompt} [/INST]
            let formatted_prompt = format!("[INST] {} [/INST]", prompt);

            log::info!("Tokenizing prompt: {:.50}...", formatted_prompt);

            let encoding = tokenizer
                .encode(formatted_prompt.as_str(), true)
                .map_err(|e| AiError::TokenizationFailed(format!("Tokenization failed: {}", e)))?;

            let tokens = encoding.get_ids().to_vec();
            log::info!("Tokenized to {} tokens", tokens.len());

            // 2. Generate tokens
            let mut generated_tokens = tokens.clone();

            // Mistral EOS tokens:
            // 2 = </s> (EOS)
            let eos_token = 2;

            let max_gen = config.max_tokens.min(1024); // Cap at 1024

            for i in 0..max_gen {
                // Create input tensor from all generated tokens so far
                let input_ids = Tensor::new(&generated_tokens[..], device)?;

                // Get logits
                let logits = model.forward(&input_ids.unsqueeze(0)?, 0)?;

                // Get last token logits
                let logits = logits.squeeze(0)?;
                let logits = logits.get(logits.dim(0)? - 1)?;

                // Sample next token (greedy for now)
                let next_token = logits.argmax(0)?.to_scalar::<u32>()?;

                if next_token == eos_token {
                    log::info!("EOS token reached at position {}", i);
                    break;
                }

                generated_tokens.push(next_token);
            }

            // 3. Decode output
            let _ = tokenizer
                .decode(&generated_tokens, true)
                .map_err(|e| AiError::TokenizationFailed(format!("Decoding failed: {}", e)))?;

            // 4. Extract response
            let new_tokens = &generated_tokens[tokens.len()..];
            let response = tokenizer.decode(new_tokens, true).map_err(|e| {
                AiError::TokenizationFailed(format!("Decoding response failed: {}", e))
            })?;

            let response = response.trim().to_string();

            log::info!(
                "Generated {} tokens. Response: {:.50}...",
                new_tokens.len(),
                response
            );

            Ok(response)
        })
        .await
        .map_err(|e| AiError::Unknown(anyhow::anyhow!("Blocking task failed: {}", e)))?
    }
}
