use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_transformers::models::quantized_llama::ModelWeights as QLlama;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokenizers::Tokenizer;

/// Server-side engine for the Gemma 27B model (The "Orchestrator")
pub struct Gemma27BServer {
    model: Arc<Mutex<Option<QLlama>>>, // Mutex for thread safety, Option for lazy loading
    tokenizer: Arc<Mutex<Option<Tokenizer>>>,
    device: Device,
    model_path: PathBuf,
}

impl Gemma27BServer {
    pub fn new(model_path: PathBuf) -> Self {
        Self {
            model: Arc::new(Mutex::new(None)),
            tokenizer: Arc::new(Mutex::new(None)),
            device: Device::Cpu, // Default to CPU, can upgrade to Cuda if available
            model_path,
        }
    }

    /// Loads the model if not already loaded.
    /// This is "lazy" to avoid blocking startup if the model is missing or huge.
    pub fn load_model(&self) -> Result<()> {
        let mut model_guard = self.model.lock().unwrap();
        if model_guard.is_some() {
            return Ok(());
        }

        println!("Loading Gemma 27B from: {:?}", self.model_path);

        // 1. Load Tokenizer (Assumes tokenizer.json is in the same dir or standard location)
        // For simplicity, we'll look for tokenizer.json next to the model
        let tokenizer_path = self.model_path.parent().unwrap().join("tokenizer.json");
        let tokenizer = Tokenizer::from_file(&tokenizer_path).map_err(|e| {
            anyhow::anyhow!("Failed to load tokenizer from {:?}: {}", tokenizer_path, e)
        })?;

        // 2. Load Model
        let mut file = std::fs::File::open(&self.model_path).map_err(|e| {
            anyhow::anyhow!("Failed to open model file at {:?}: {}", self.model_path, e)
        })?;

        let content = candle_core::quantized::gguf_file::Content::read(&mut file)?;
        let model = QLlama::from_gguf(content, &mut file, &self.device)?;

        // Store
        *self.tokenizer.lock().unwrap() = Some(tokenizer);
        *model_guard = Some(model);

        println!("Gemma 27B loaded successfully.");
        Ok(())
    }

    /// Generates text based on a prompt.
    /// This is a blocking operation for the model lock.
    pub fn generate(&self, prompt: &str, max_tokens: usize) -> Result<String> {
        // Ensure loaded
        if self.model.lock().unwrap().is_none() {
            self.load_model()?;
        }

        let mut model_guard = self.model.lock().unwrap();
        let model = model_guard.as_mut().unwrap();

        let tokenizer_guard = self.tokenizer.lock().unwrap();
        let tokenizer = tokenizer_guard.as_ref().unwrap();

        // Encoding
        let tokens = tokenizer
            .encode(prompt, true)
            .map_err(|e| anyhow::anyhow!("Encoding failed: {}", e))?
            .get_ids()
            .to_vec();

        let mut all_tokens = tokens.clone();
        let mut generated_tokens = Vec::new();
        let mut index_pos = 0;

        // Generation Loop
        for _ in 0..max_tokens {
            let (context_tokens, context_index) = if index_pos == 0 {
                (all_tokens.clone(), 0)
            } else {
                (all_tokens[all_tokens.len() - 1..].to_vec(), index_pos)
            };

            let input = Tensor::new(context_tokens.as_slice(), &self.device)?.unsqueeze(0)?;
            let logits = model.forward(&input, context_index)?;
            let logits = logits.squeeze(0)?.squeeze(0)?;

            // Greedy sampling
            let next_token = logits.argmax(0)?.to_scalar::<u32>()?;

            all_tokens.push(next_token);
            generated_tokens.push(next_token);
            index_pos += context_tokens.len();

            if next_token == 1 || next_token == 107 {
                // EOS
                break;
            }
        }

        // Decoding
        let output = tokenizer
            .decode(&generated_tokens, true)
            .map_err(|e| anyhow::anyhow!("Decoding failed: {}", e))?;

        Ok(output)
    }
}
