use std::env;

fn main() {
    dotenv::dotenv().ok();

    println!("Environment Variable Test:");
    println!("==========================");

    match env::var("GEMINI_API_KEY") {
        Ok(key) => {
            let masked = if key.len() > 8 {
                format!("{}...{}", &key[..4], &key[key.len() - 4..])
            } else {
                "****".to_string()
            };
            println!("✅ GEMINI_API_KEY found: {}", masked);
        }
        Err(_) => {
            println!("❌ GEMINI_API_KEY not found");
        }
    }

    match env::var("DATABASE_URL") {
        Ok(url) => println!("✅ DATABASE_URL found: {}", url),
        Err(_) => println!("❌ DATABASE_URL not found"),
    }
}
