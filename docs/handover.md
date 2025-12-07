# Handover: Iron Split Implementation

## Current Status

- **Stability Fixes**: Implemented and verified (panic resilience, timeouts).
- **Assets**:
  - Mistral 7B: `assets/models/mistral-7b-instruct-v0.1.Q4_K_M.gguf` (Active)
  - Gemma 2B: `assets/models/gemma-2b-it-Q6_K.gguf` (Ready for integration)
- **Codebase**: Reverted to use Mistral/QLlama to ensure compilation.

## The "Iron Split" Plan

The goal is to implement a dual-model architecture:

1. **The Architect (Mistral)**: Complex logic, blueprint generation.
2. **The Navigator (Gemma)**: Fast dialogue, persona.

## Blockers & Next Steps

1. **Dependency Conflict**:
    - To support Gemma 2, we need `candle` crates version `0.8.4` (or at least `0.8.2`).
    - Attempting to upgrade `candle` to `0.8.4` caused a version conflict with the `half` crate, likely due to `bevy` or `ask_pete_node_garden` dependencies.
    - **Action**: Resolve the `half` crate conflict. You may need to use `cargo tree` to find the conflicting dependency and potentially use a `[patch.crates-io]` override or upgrade the conflicting crate.
2. **Implementation**:
    - Once dependencies are resolved, implement `crates/ask_pete_ai/src/iron_split.rs`.
    - Use `candle_transformers::models::quantized_gemma` for the Navigator.
    - Use `candle_transformers::models::quantized_llama` for the Architect.
