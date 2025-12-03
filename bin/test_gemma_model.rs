use infra_ai::llm::{GemmaModel, GemmaConfigWrapper, GenerationConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    println!("🚀 Testing Gemma 3B Local Model Integration");
    println!("=".repeat(60));
    
    // Load model with default configuration
    println!("\n📦 Loading model...");
    let config = GemmaConfigWrapper::default();
    println!("   Model path: {:?}", config.model_path);
    println!("   Tokenizer path: {:?}", config.tokenizer_path);
    
    let model = match GemmaModel::load(config) {
        Ok(m) => {
            println!("   ✅ Model loaded successfully!");
            m
        }
        Err(e) => {
            eprintln!("   ❌ Failed to load model: {}", e);
            return Err(e);
        }
    };
    
    // Test simple inference
    println!("\n🤖 Testing inference...");
    let test_prompts = vec![
        "What is physics?",
        "Explain Newton's First Law",
        "How does velocity work?",
    ];
    
    for (i, prompt) in test_prompts.iter().enumerate() {
        println!("\n--- Test {} ---", i + 1);
        println!("Prompt: {}", prompt);
        
        let gen_config = GenerationConfig {
            max_tokens: 100,
            temperature: 0.7,
            top_p: 0.9,
            repeat_penalty: 1.1,
            seed: 42,
        };
        
        match model.generate(prompt.to_string(), gen_config).await {
            Ok(response) => {
                println!("Response:\n{}", response);
            }
            Err(e) => {
                eprintln!("❌ Generation failed: {}", e);
            }
        }
    }
    
    println!("\n{}", "=".repeat(60));
    println!("✅ All tests completed!");
    
    Ok(())
}
