pub mod antigravity;
pub mod architect;
pub mod llm;
pub mod local_inference;
pub mod lore;
pub mod prompts;
pub mod socratic_engine;
pub mod vocabulary;

pub use local_inference::{GemmaConfigWrapper, GemmaModel};
pub use socratic_engine::SocraticEngine;
