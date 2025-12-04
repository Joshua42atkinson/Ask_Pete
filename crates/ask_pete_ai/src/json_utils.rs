// use regex::Regex;
use anyhow::Result;
use serde::de::DeserializeOwned;

/// Extracts a JSON object or array from a string that might contain other text.
///
/// Strategies:
/// 1. Look for markdown code blocks: ```json ... ```
/// 2. Look for the first outer `{` and last `}` (Object)
/// 3. Look for the first outer `[` and last `]` (Array) - *Future proofing*
///
/// Returns `Option<String>` containing the cleaned JSON string.
pub fn extract_json_from_text(text: &str) -> Option<String> {
    // Strategy 1: Markdown Code Blocks
    if let Some(start) = text.find("```json") {
        if let Some(_end) = text[start..].find("```") {
            // Found a block, but we need to find the *closing* fence
            // The `find` above finds the *start* of the closing fence relative to `start`
            // Wait, `text[start..]` starts with ```json, so the first ``` found will be that one.
            // We need to search *after* the opening tag.
            let content_start = start + 7; // skip ```json
            if let Some(close_relative) = text[content_start..].find("```") {
                let json_content = &text[content_start..content_start + close_relative];
                return Some(json_content.trim().to_string());
            }
        }
    }

    // Fallback for generic code blocks ``` ... ```
    if let Some(start) = text.find("```") {
        let content_start = start + 3;
        if let Some(close_relative) = text[content_start..].find("```") {
            let json_content = &text[content_start..content_start + close_relative];
            // Check if it looks like JSON (starts with { or [)
            let trimmed = json_content.trim();
            if trimmed.starts_with('{') || trimmed.starts_with('[') {
                return Some(trimmed.to_string());
            }
        }
    }

    // Strategy 2: Brute Force Braces (Object)
    if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            if end > start {
                return Some(text[start..=end].to_string());
            }
        }
    }

    // Strategy 3: Brute Force Brackets (Array)
    if let Some(start) = text.find('[') {
        if let Some(end) = text.rfind(']') {
            if end > start {
                return Some(text[start..=end].to_string());
            }
        }
    }

    None
}

/// Extracts JSON from text and deserializes it into type T.
pub fn extract_and_parse_json<T: DeserializeOwned>(text: &str) -> Result<T> {
    let json_text =
        extract_json_from_text(text).ok_or_else(|| anyhow::anyhow!("No JSON found in text"))?;

    let result = serde_json::from_str(&json_text)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_clean_json() {
        let input = r#"{"key": "value"}"#;
        assert_eq!(extract_json_from_text(input), Some(input.to_string()));
    }

    #[test]
    fn test_extract_markdown_json() {
        let input = r#"Here is the data:
```json
{
  "key": "value"
}
```
Hope that helps!"#;
        let expected = r#"{
  "key": "value"
}"#;
        assert_eq!(extract_json_from_text(input), Some(expected.to_string()));
    }

    #[test]
    fn test_extract_embedded_json() {
        let input = r#"Sure, I can help. {"key": "value"} is the answer."#;
        assert_eq!(
            extract_json_from_text(input),
            Some(r#"{"key": "value"}"#.to_string())
        );
    }
}
