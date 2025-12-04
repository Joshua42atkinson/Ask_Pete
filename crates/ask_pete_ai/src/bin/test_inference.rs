use anyhow::{Error, Result};
use candle_core::{Device, Tensor};
use candle_transformers::models::quantized_llama::ModelWeights as QLlama;
use std::path::PathBuf;
use tokenizers::Tokenizer;

fn main() -> Result<()> {
    println!("🚂 Ask Pete: Local Inference Test Harness 🚂");
    println!("============================================");

    // 1. Configuration
    let model_path = "C:/Users/Trinity/.lmstudio/models/itlwas/Mistral-7B-Instruct-v0.1-Q4_K_M-GGUF/mistral-7b-instruct-v0.1-q4_k_m.gguf";
    // We'll try to find a tokenizer. usually it's alongside or we download it.
    // For now, let's assume we might need to download it or use a standard one.
    // Ideally, we'd point to a local tokenizer.json if available.
    // Let's try to use the HF Hub to get the tokenizer for Mistral-7B-Instruct-v0.1 if local isn't found.
    let _tokenizer_repo = "mistralai/Mistral-7B-Instruct-v0.1";

    println!("Target Model: {}", model_path);

    // 2. Load Device
    let device = Device::Cpu; // Start with CPU for safety/compatibility
    println!("Device: CPU (Safe Mode)");

    // 3. Load Model
    println!("Loading Model... (This may take a moment)");
    let path = PathBuf::from(model_path);
    if !path.exists() {
        eprintln!("❌ Error: Model file not found at {}", model_path);
        return Ok(());
    }

    let mut file = std::fs::File::open(&path)?;
    // Fix: Read content first as required by newer candle versions
    let content = candle_core::quantized::gguf_file::Content::read(&mut file)?;
    let mut model = QLlama::from_gguf(content, &mut file, &device)?;
    println!("✅ Model Loaded Successfully!");

    // 4. Load Tokenizer
    // Simplified: We will assume a local tokenizer.json exists for the test,
    // or we can use a fallback. For this test harness, let's try to load from a known path
    // or just fail gracefully if not found, instructing the user to download it.
    let tokenizer_path = PathBuf::from("tokenizer.json");
    if !tokenizer_path.exists() {
        eprintln!("⚠️  Warning: 'tokenizer.json' not found in current directory.");
        eprintln!(
            "Please download a generic Llama/Mistral tokenizer.json and place it here: {}",
            std::env::current_dir()?.display()
        );
        return Ok(());
    }
    let tokenizer = Tokenizer::from_file(tokenizer_path).map_err(Error::msg)?;
    println!("✅ Tokenizer Loaded!");

    // 5. Run Inference
    let prompt = "<s>[INST] You are a helpful AI. What is the capital of France? [/INST]";
    println!("\n📝 Prompt: {}", prompt);
    println!("generating...");

    let tokens = tokenizer.encode(prompt, true).map_err(Error::msg)?;
    let prompt_tokens = tokens.get_ids();
    let mut all_tokens = prompt_tokens.to_vec();
    let mut generated_tokens = Vec::new();

    let _next_token = *prompt_tokens.last().unwrap(); // Placeholder, logic needs to be loop

    // Simple generation loop (very basic)
    let max_tokens = 50;

    for _index in 0..max_tokens {
        let input = Tensor::new(
            &all_tokens[all_tokens.len().saturating_sub(1024)..],
            &device,
        )?
        .unsqueeze(0)?;
        let logits = model.forward(&input, all_tokens.len())?;
        let logits = logits.squeeze(0)?.squeeze(0)?;

        // Greedy sampling
        let next_token_id = logits.argmax(0)?.to_scalar::<u32>()?;

        all_tokens.push(next_token_id);
        generated_tokens.push(next_token_id);

        let token_str = tokenizer
            .decode(&[next_token_id], true)
            .map_err(Error::msg)?;
        print!("{}", token_str);
        use std::io::Write;
        std::io::stdout().flush()?;

        if next_token_id == 2 {
            // EOS for Llama/Mistral often 2
            break;
        }
    }

    println!("\n\n✅ Inference Test Complete.");
    Ok(())
}
