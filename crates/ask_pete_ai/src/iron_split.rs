use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::quantized_llama::ModelWeights as QLlama;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokenizers::Tokenizer;

pub struct IronSplitSystem {
    // Shared resources wrapped in Arc/Mutex for thread safety if needed
    model: Arc<Mutex<QLlama>>,
    tokenizer: Arc<Tokenizer>,
    device: Device,
}

impl IronSplitSystem {
    pub fn new() -> Result<Self> {
        println!("⚙️  HEAVILON PROTOCOL: Initializing Single-Model System (Mistral 7B)...");

        let device = Device::Cpu;
        let repo_path = PathBuf::from("assets/models");

        // 1. Load Mistral Model
        let filename = "mistral-7b-instruct-v0.1.Q4_K_M.gguf";
        let path = repo_path.join(filename);
        println!("🏗️  Loading Brain: {:?}", filename);

        if !path.exists() {
            return Err(E::msg(format!("CRITICAL: Model missing at {:?}", path)));
        }

        let mut file = std::fs::File::open(&path)?;
        let model_content = candle_core::quantized::gguf_file::Content::read(&mut file)?;
        let model = QLlama::from_gguf(model_content, &mut file, &device)?;

        // 2. Load Tokenizer
        let tokenizer_path = repo_path.join("tokenizer.json");
        println!("📖  Loading Tokenizer: {:?}", tokenizer_path);
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| E::msg(format!("Tokenizer error: {}", e)))?;

        Ok(Self {
            model: Arc::new(Mutex::new(model)),
            tokenizer: Arc::new(tokenizer),
            device,
        })
    }

    // The Architect: Careful, creative, longer context
    pub fn ask_architect(&mut self, prompt: &str) -> Result<String> {
        let formatted = format!(
            "[INST] You are an expert Curriculum Architect. {}\nOutput JSON only. [/INST]",
            prompt
        );
        println!("🏗️  Architect Generating...");
        // 1000 tokens for blueprints
        self.generate(&formatted, 1000)
    }

    // The Navigator: Fast, helpful, shorter context
    pub fn ask_navigator(&mut self, prompt: &str) -> Result<String> {
        let formatted = format!(
            "[INST] You are Pete, a helpful train conductor. Keep it brief. {} [/INST]",
            prompt
        );
        println!("🧭  Navigator Speaking...");
        // 200 tokens for chat
        self.generate(&formatted, 200)
    }

    fn generate(&self, prompt: &str, max_tokens: usize) -> Result<String> {
        let tokenizer = self.tokenizer.clone();
        let mut model = self.model.lock().unwrap(); // Lock the model for inference

        let mut tokens = tokenizer
            .encode(prompt, true)
            .map_err(E::msg)?
            .get_ids()
            .to_vec();
        let mut output = String::new();
        let mut logits_processor = LogitsProcessor::new(299792458, Some(0.7), Some(0.9));

        // 1. Prefill (Process the prompt)
        let input = Tensor::new(tokens.as_slice(), &self.device)?.unsqueeze(0)?;
        let logits = model.forward(&input, 0)?; // Pos 0 for prompt
        let logits = logits.squeeze(0)?;
        let mut next_token = logits_processor.sample(&logits)?;

        tokens.push(next_token);
        if let Some(text) = tokenizer.id_to_token(next_token) {
            let text = text.replace(' ', " ").replace("<0x0A>", "\n");
            output.push_str(&text);
        }

        // 2. Generation Loop (Incremental)
        for i in 0..max_tokens {
            let input = Tensor::new(&[next_token], &self.device)?.unsqueeze(0)?;
            let start_pos = tokens.len() - 1; // Position of the new token
            let logits = model.forward(&input, start_pos)?;
            let logits = logits.squeeze(0)?;
            next_token = logits_processor.sample(&logits)?;

            tokens.push(next_token);

            if let Some(text) = tokenizer.id_to_token(next_token) {
                let text = text.replace(' ', " ").replace("<0x0A>", "\n");
                output.push_str(&text);
                print!("{}", text); // Stream to console
                use std::io::Write;
                std::io::stdout().flush().ok();

                if text.contains("</s>") {
                    break;
                }
            }
        }
        println!("\n");
        Ok(output.trim().to_string())
    }
}
