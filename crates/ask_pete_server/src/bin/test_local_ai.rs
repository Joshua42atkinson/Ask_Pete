#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("=== Local Candle Mistral Test ===\n");

    // Load model
    use infra_ai::{LocalConfigWrapper, LocalModel};

    println!("📦 Loading Mistral model...");
    let config = LocalConfigWrapper::default();
    println!("   Model path: {:?}", config.model_path);
    println!("   Tokenizer: {:?}\n", config.tokenizer_path);

    let model = match LocalModel::load(config) {
        Ok(m) => {
            println!("✅ Model loaded successfully!\n");
            m
        }
        Err(e) => {
            eprintln!("❌ Failed to load model: {}", e);
            eprintln!("\nMake sure model files exist:");
            eprintln!("  - assets/models/mistral-7b-instruct-v0.1.Q4_K_M.gguf");
            eprintln!("  - assets/models/tokenizer.json");
            return;
        }
    };

    // Test generation
    println!("🧪 Testing text generation...");
    let prompt = "What is the capital of France?";
    println!("   Prompt: {}\n", prompt);

    use infra_ai::local_inference::GenerationConfig;
    let gen_config = GenerationConfig {
        max_tokens: 50,
        ..Default::default()
    };

    println!("⚙️  Generating (this may take 10-30 seconds on CPU)...\n");

    match model.generate(prompt.to_string(), gen_config).await {
        Ok(response) => {
            println!("✅ SUCCESS!\n");
            println!("📝 Response:\n{}\n", response);
        }
        Err(e) => {
            eprintln!("❌ Generation failed: {}", e);
        }
    }
}
