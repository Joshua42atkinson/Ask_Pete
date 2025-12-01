pub mod antigravity;
pub mod architect;
pub mod knowledge_retrieval;
pub mod llm;
pub mod local_inference;
pub mod lore;
pub mod prompts;
pub mod socratic_engine;
pub mod vocabulary;
pub mod weigh_station;

pub use local_inference::{LocalConfigWrapper, LocalModel};
pub use socratic_engine::SocraticEngine;
