use anyhow::{Error, Result};
use std::collections::HashMap;

/// Special tokens used by Gemma 3
#[derive(Debug, Clone)]
pub struct SpecialTokens {
    pub bos: u32,           // Beginning of sequence: <bos> = 2
    pub eos: u32,           // End of sequence: <eos> = 1
    pub pad: u32,           // Padding token
    pub unk: u32,           // Unknown token
    pub start_of_turn: u32, // <start_of_turn>
    pub end_of_turn: u32,   // <end_of_turn>
}

impl Default for SpecialTokens {
    fn default() -> Self {
        Self {
            bos: 2,
            eos: 1,
            pad: 0,
            unk: 3,
            start_of_turn: 106, // These are typical Gemma values
            end_of_turn: 107,   // Will be updated from tokenizer.json
        }
    }
}

/// Vocabulary mapping between tokens and IDs
#[derive(Debug, Clone)]
pub struct Vocabulary {
    /// Maps token strings to their IDs
    token_to_id: HashMap<String, u32>,
    /// Maps IDs back to token strings
    id_to_token: HashMap<u32, String>,
    /// Special tokens with known IDs
    pub special_tokens: SpecialTokens,
}

impl Vocabulary {
    /// Creates a new empty vocabulary
    pub fn new() -> Self {
        Self {
            token_to_id: HashMap::new(),
            id_to_token: HashMap::new(),
            special_tokens: SpecialTokens::default(),
        }
    }

    /// Loads vocabulary from embedded tokenizer.json
    ///
    /// This will be called with the token data once tokenizer.json is available
    pub fn from_json_data(json_str: &str) -> Result<Self> {
        use serde_json::Value;

        let data: Value = serde_json::from_str(json_str)?;

        let mut vocab = Self::new();

        // Parse vocabulary from tokenizer.json structure
        // Typical structure: data["model"]["vocab"] is an object mapping tokens to IDs
        if let Some(model) = data.get("model") {
            if let Some(vocab_obj) = model.get("vocab") {
                if let Some(vocab_map) = vocab_obj.as_object() {
                    for (token, id) in vocab_map {
                        if let Some(id_num) = id.as_u64() {
                            let id_u32 = id_num as u32;
                            vocab.token_to_id.insert(token.clone(), id_u32);
                            vocab.id_to_token.insert(id_u32, token.clone());
                        }
                    }
                }
            }
        }

        // Extract special tokens from added_tokens array
        if let Some(added_tokens) = data.get("added_tokens") {
            if let Some(tokens_array) = added_tokens.as_array() {
                for token_obj in tokens_array {
                    if let (Some(content), Some(id)) = (
                        token_obj.get("content").and_then(|v| v.as_str()),
                        token_obj.get("id").and_then(|v| v.as_u64()),
                    ) {
                        let id_u32 = id as u32;

                        // Update special tokens
                        match content {
                            "<bos>" => vocab.special_tokens.bos = id_u32,
                            "<eos>" => vocab.special_tokens.eos = id_u32,
                            "<pad>" => vocab.special_tokens.pad = id_u32,
                            "<unk>" => vocab.special_tokens.unk = id_u32,
                            "<start_of_turn>" => vocab.special_tokens.start_of_turn = id_u32,
                            "<end_of_turn>" => vocab.special_tokens.end_of_turn = id_u32,
                            _ => {}
                        }
                    }
                }
            }
        }

        if vocab.token_to_id.is_empty() {
            return Err(Error::msg("No vocabulary found in tokenizer.json"));
        }

        Ok(vocab)
    }

    /// Gets the token ID for a given string
    pub fn get_id(&self, token: &str) -> Option<u32> {
        self.token_to_id.get(token).copied()
    }

    /// Gets the token string for a given ID
    pub fn get_token(&self, id: u32) -> Option<&str> {
        self.id_to_token.get(&id).map(|s| s.as_str())
    }

    /// Returns the total vocabulary size
    pub fn size(&self) -> usize {
        self.token_to_id.len()
    }

    /// Checks if a token exists in the vocabulary
    pub fn contains(&self, token: &str) -> bool {
        self.token_to_id.contains_key(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vocab_creation() {
        let vocab = Vocabulary::new();
        assert_eq!(vocab.size(), 0);
        assert_eq!(vocab.special_tokens.bos, 2);
        assert_eq!(vocab.special_tokens.eos, 1);
    }

    #[test]
    fn test_special_tokens_default() {
        let tokens = SpecialTokens::default();
        assert_eq!(tokens.bos, 2);
        assert_eq!(tokens.eos, 1);
        assert_eq!(tokens.pad, 0);
    }
}
