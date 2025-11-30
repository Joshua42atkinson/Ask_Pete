use anyhow::{Context, Result};
use async_trait::async_trait;
// use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
// use lancedb::{connect, Table}; // Removed TableRef and arrow imports for now
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex}; // Added Mutex for interior mutability

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub id: String,
    pub text: String,
    pub metadata: String, // JSON string
}

#[async_trait]
pub trait VectorStore: Send + Sync {
    async fn add_document(&self, doc: Document) -> Result<()>;
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<Document>>;
}

pub struct LanceDbConnection {
    // conn: lancedb::Connection,
    // embedding_model: Mutex<TextEmbedding>, // Wrap in Mutex because embed needs &mut self
    table_name: String,
}

impl LanceDbConnection {
    pub async fn new(db_path: &str) -> Result<Self> {
        // Fix InitOptions construction

        let _uri = format!("data/{}", db_path);
        // let conn = connect(&uri).execute().await?;

        Ok(Self {
            // conn,
            // embedding_model: Mutex::new(model),
            table_name: "memory_bank".to_string(),
        })
    }
}

#[async_trait]
impl VectorStore for LanceDbConnection {
    async fn add_document(&self, doc: Document) -> Result<()> {
        // 1. Generate Embedding
        let embedding: Vec<f32> = vec![0.0; 384]; // Dummy embedding

        println!(
            "🧠 [Memory] (MOCKED) Embedding generated (Dim: {}). Storing: {}",
            embedding.len(),
            doc.id
        );

        // TODO: Actual LanceDB Insert
        // For now, we just print to verify the pipeline works.

        Ok(())
    }

    async fn search(&self, query: &str, limit: usize) -> Result<Vec<Document>> {

        println!("🧠 [Memory] (MOCKED) Searching for: '{}'", query);

        Ok(vec![])
    }
}
