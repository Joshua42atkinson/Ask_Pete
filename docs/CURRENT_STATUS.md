# Current Status: Blueprint Generation & Logic System

## 1. The Core Issue: "Language Barrier"

The "Generate" button fails (500 Error) because the AI and the Backend are speaking different languages regarding **Game Logic**.

- **The Backend (`libs/core`)** expects strict JSON objects for `LogicBlock`:

  ```json
  {
    "condition": { "GreaterThan": { "variable": "Strength", "value": 10.0 } },
    "effect": { "GrantItem": { "item_id": "Key" } }
  }
  ```

- **The AI (`architect.rs` Prompt)** is instructed to generate "Rust-like" strings:

  ```json
  {
    "condition": "GreaterThan { variable: 'Strength', value: 10.0 }",
    "effect": "GrantItem { item_id: 'Key' }"
  }
  ```

- **Result**: The backend tries to read the string `"GreaterThan..."` as a JSON object and fails, or fails to find the expected fields.

## 2. Missing Components ("The Gap")

As you suspected, the "Translation Layer" is missing.

- **Missing**: A parser to convert the AI's "pseudo-code" strings into actual `TriggerCondition` and `TriggerEffect` Rust enums.
- **Alternative**: We could change the AI Prompt to generate strict JSON directly, but LLMs often struggle with the verbosity of `serde` enum JSON formats.

## 3. What IS Working

- **Gemini Integration**: We successfully connected to `gemini-2.0-flash` (after fixing model names and token limits).
- **JSON Extraction**: We fixed the fragile JSON parsing logic; we are now correctly extracting the JSON payload from the AI's response.
- **StoryGraph Structure**: The high-level structure (Nodes, Connections, Titles) is correct and matches the backend schema.

## 4. Next Steps (Options)

1. **Fix the Prompt**: Update `architect.rs` to ask for the exact JSON format the backend expects. (Easiest, but AI might make syntax errors).
2. **Implement the Parser**: Write a helper function to parse the "Rust-like" strings into Enums. (More robust, allows "sloppier" AI output).
3. **Simplify for MVP**: Temporarily make `condition` and `effect` simple Strings in the struct, and parse them later at runtime.
