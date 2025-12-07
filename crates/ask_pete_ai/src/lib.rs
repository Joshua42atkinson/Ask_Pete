pub mod antigravity;
pub mod architect;
pub mod error;
pub mod iron_split;
pub mod json_utils;
pub mod knowledge_retrieval;
pub mod llm;
pub mod local_inference;
pub mod lore;
pub mod prompts;
pub mod socratic_engine;
pub mod vocabulary;

pub use local_inference::{LocalModel, LocalModelConfig as LocalConfigWrapper};
pub use socratic_engine::SocraticEngine;
