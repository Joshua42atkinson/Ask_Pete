use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("=== Direct Gemini API Test ===\n");

    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
    println!(
        "✅ API Key loaded: {}...{}",
        &api_key[..4],
        &api_key[api_key.len() - 4..]
    );

    let model = "gemini-2.0-flash-exp";
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );

    println!("📡 Testing model: {}", model);
    println!("🔗 URL: {}\n", url.replace(&api_key, "***"));

    let payload = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": "Say hello in exactly 5 words."
            }]
        }],
        "generationConfig": {
            "maxOutputTokens": 100,
            "temperature": 0.7
        }
    });

    println!("📤 Sending test request...");

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    println!("📥 Status: {}\n", response.status());

    let text = response.text().await.expect("Failed to read response");

    if text.contains("\"error\"") {
        println!("❌ ERROR Response:\n{}\n", text);
    } else {
        println!("✅ SUCCESS Response:\n{}\n", text);

        // Try to parse and extract the actual text
        let json: serde_json::Value = serde_json::from_str(&text).expect("Failed to parse JSON");
        if let Some(candidates) = json.get("candidates") {
            if let Some(first) = candidates.get(0) {
                if let Some(content) = first.get("content") {
                    if let Some(parts) = content.get("parts") {
                        if let Some(part) = parts.get(0) {
                            if let Some(text) = part.get("text") {
                                println!("🎯 Generated Text: {}", text);
                            }
                        }
                    }
                }
            }
        }
    }
}
