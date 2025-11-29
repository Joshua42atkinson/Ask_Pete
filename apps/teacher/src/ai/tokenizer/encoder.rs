use super::vocab::Vocabulary;
use anyhow::Result;

/// Pure-Rust tokenizer for Gemma 3 1B (WASM-compatible)
pub struct GemmaTokenizer {
    vocab: Vocabulary,
}

impl GemmaTokenizer {
    /// Creates a new tokenizer from embedded vocabulary
    ///
    /// tokenizer.json is embedded at compile time for WASM compatibility
    pub fn from_embedded() -> Result<Self> {
        const TOKENIZER_JSON: &str = include_str!("../../../public/models/tokenizer.json");
        let vocab = Vocabulary::from_json_data(TOKENIZER_JSON)?;
        Ok(Self { vocab })
    }

    /// Creates tokenizer from JSON string (for testing)
    pub fn from_json_str(json: &str) -> Result<Self> {
        let vocab = Vocabulary::from_json_data(json)?;
        Ok(Self { vocab })
    }

    /// Encodes text into token IDs
    ///
    /// Strategy:
    /// 1. Normalize text (lowercase, basic cleanup)
    /// 2. Split on whitespace
    /// 3. For each word, find longest matching token in vocabulary
    /// 4. Fall back to character-level for unknowns (using <unk> token)
    /// 5. Add special tokens as needed
    pub fn encode(&self, text: &str) -> Result<Vec<u32>> {
        let mut tokens = Vec::new();

        // Normalize: basic whitespace cleanup
        let normalized = text.trim();

        if normalized.is_empty() {
            return Ok(tokens);
        }

        // Split on whitespace
        for word in normalized.split_whitespace() {
            // Try to find exact match in vocabulary
            if let Some(id) = self.vocab.get_id(word) {
                tokens.push(id);
            } else {
                // Try lowercase
                let lowercase = word.to_lowercase();
                if let Some(id) = self.vocab.get_id(&lowercase) {
                    tokens.push(id);
                } else {
                    // Greedy longest-match tokenization
                    let mut remaining = word;
                    while !remaining.is_empty() {
                        let mut found = false;

                        // Try progressively smaller substrings
                        for len in (1..=remaining.len()).rev() {
                            let substr = &remaining[..len];
                            if let Some(id) = self.vocab.get_id(substr) {
                                tokens.push(id);
                                remaining = &remaining[len..];
                                found = true;
                                break;
                            }
                        }

                        if !found {
                            // No match found, use unknown token and skip one character
                            tokens.push(self.vocab.special_tokens.unk);
                            remaining = &remaining[1..];
                        }
                    }
                }
            }
        }

        Ok(tokens)
    }

    /// Decodes token IDs back into text
    ///
    /// Handles special tokens by either skipping them or replacing with markers
    pub fn decode(&self, tokens: &[u32]) -> Result<String> {
        let mut result = String::new();

        for &token_id in tokens {
            // Skip or handle special tokens
            if token_id == self.vocab.special_tokens.bos {
                continue; // Skip BOS
            } else if token_id == self.vocab.special_tokens.eos {
                continue; // Skip EOS
            } else if token_id == self.vocab.special_tokens.pad {
                continue; // Skip padding
            } else if token_id == self.vocab.special_tokens.start_of_turn {
                result.push_str("<start_of_turn>");
            } else if token_id == self.vocab.special_tokens.end_of_turn {
                result.push_str("<end_of_turn>");
            } else if let Some(token_str) = self.vocab.get_token(token_id) {
                // Add space before token if not at start and not punctuation
                if !result.is_empty() && !token_str.starts_with(|c: char| c.is_ascii_punctuation())
                {
                    result.push(' ');
                }
                result.push_str(token_str);
            } else {
                // Unknown token ID
                result.push_str("<unk>");
            }
        }

        Ok(result)
    }

    /// Encodes with special tokens for chat format
    ///
    /// Gemma expects: <start_of_turn>user\n{prompt}<end_of_turn><start_of_turn>model\n
    pub fn encode_chat(&self, user_prompt: &str) -> Result<Vec<u32>> {
        let mut tokens = Vec::new();

        // Add start of turn marker
        tokens.push(self.vocab.special_tokens.start_of_turn);

        // Add "user\n"
        if let Some(user_id) = self.vocab.get_id("user") {
            tokens.push(user_id);
        }
        if let Some(newline_id) = self.vocab.get_id("\n") {
            tokens.push(newline_id);
        }

        // Encode the actual prompt
        tokens.extend(self.encode(user_prompt)?);

        // Add end of turn
        tokens.push(self.vocab.special_tokens.end_of_turn);

        // Add start of model response
        tokens.push(self.vocab.special_tokens.start_of_turn);
        if let Some(model_id) = self.vocab.get_id("model") {
            tokens.push(model_id);
        }
        if let Some(newline_id) = self.vocab.get_id("\n") {
            tokens.push(newline_id);
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_creation() {
        let result = GemmaTokenizer::from_embedded();
        assert!(result.is_ok()); // Should succeed now that tokenizer.json is embedded
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let tokenizer = GemmaTokenizer::from_embedded().unwrap();
        let text = "Hello world";
        let tokens = tokenizer.encode(text).unwrap();
        assert!(tokens.len() > 0);

        let decoded = tokenizer.decode(&tokens).unwrap();
        assert!(decoded.to_lowercase().contains("hello"));
    }
}
