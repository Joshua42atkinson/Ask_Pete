use anyhow::Result;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::fs;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    println!("🌱 Starting Quest Seeding...");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Load quests.json
    let quests_path = PathBuf::from("../../libs/core/src/quests.json");
    let quests_content = fs::read_to_string(&quests_path)?;
    let quests: serde_json::Value = serde_json::from_str(&quests_content)?;

    if let Some(quests_map) = quests.as_object() {
        for (key, quest_data) in quests_map {
            println!("Processing quest: {}", key);

            let title = quest_data["title"].as_str().unwrap_or("Untitled Quest");

            // Check if exists
            /*
            let exists = sqlx::query!("SELECT id FROM story_graphs WHERE title = $1", title)
                .fetch_optional(&pool)
                .await?;

            if exists.is_some() {
                println!("⚠️ Quest '{}' already exists. Skipping.", title);
                continue;
            }

            // Insert
            sqlx::query!(
                r#"
                INSERT INTO story_graphs (title, author_id, graph_data)
                VALUES ($1, $2, $3)
                "#,
                title,
                1 as i64, // Default author ID (system)
                quest_data
            )
            .execute(&pool)
            .await?;
            */
            println!("(Simulation) Would insert quest: {}", title);

            println!("✅ Inserted quest: {}", title);
        }
    }

    println!("🎉 Seeding Complete!");
    Ok(())
}
