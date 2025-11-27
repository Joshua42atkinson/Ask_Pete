# Implementation Plan - Pete AI Assistant & Model Manager

## Goal

Fix compilation errors, implement the Model Manager for downloading AI models, connect the Pete AI assistant, and update branding.

## User Review Required
>
> [!IMPORTANT]
> The build is currently broken due to `ort` (ONNX Runtime) or related dependencies. We might need to adjust features or install system dependencies.

## Proposed Changes

### Build Fixes

- Investigate and resolve `ort` / `fastembed` compilation errors.
- Ensure all new dependencies in `backend/Cargo.toml` are compatible.

### Model Manager

#### [NEW] [backend/src/model_manager.rs](file:///c:/Users/Trinity/Documents/daydream/Day_Dream/backend/src/model_manager.rs)

- Implement `ModelManager` struct.
- Use `hf-hub` to download models (e.g., Gemma 1B, embedding models).
- Handle caching and file paths using `dirs`.

### Pete AI Integration

#### [MODIFY] [backend/src/main.rs](file:///c:/Users/Trinity/Documents/daydream/Day_Dream/backend/src/main.rs)

- Initialize `ModelManager` on startup.
- Integrate with the existing AI handler.

### Branding

#### [MODIFY] [frontend/index.html](file:///c:/Users/Trinity/Documents/daydream/Day_Dream/frontend/index.html)

- Update title and meta tags.

#### [MODIFY] [frontend/src/app.rs](file:///c:/Users/Trinity/Documents/daydream/Day_Dream/frontend/src/app.rs)

- Update UI text/colors to reflect "Pete" branding.

## Verification Plan

### Automated Tests

- Run `cargo check` to verify build fixes.
- Run `cargo test` for the model manager logic.

### Manual Verification

- Launch the app.
- Verify model download starts/completes.
- Chat with Pete and verify responses.
