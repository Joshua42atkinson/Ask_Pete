# Walkthrough - Pete AI & Branding Updates

I have successfully fixed the build, implemented the Model Manager, connected Pete AI, and updated the branding.

## Changes

### 1. Build Fixes

- **Issue**: `ort` compilation errors due to `fastembed` 3.5 pulling in a broken `ort` 2.0.0-rc.4.
- **Fix**: Upgraded `fastembed` to `5.3`, which uses a stable/working `ort` version (likely prebuilt binaries).
- **Fix**: Resolved syntax errors (dangling doc comments) in `backend/src/services/mod.rs`.

### 2. Model Manager

- **Implemented**: `backend/src/services/model_manager.rs`
- **Features**:
  - Lists available models with GGUF filenames (e.g., `gemma-2-2b-it-Q4_K_M.gguf`).
  - Downloads models from HuggingFace using `hf-hub`.
  - Caches models locally.

## Verification Results

### Build

- `cargo check` passes successfully (with some unused code warnings).

### Manual Verification Steps

1. **Start the App**: Run `cargo run` (backend) and `trunk serve` (frontend).
2. **Check Branding**: Verify the title is "Ask Pete" and the header says "Ask Pete".
3. **Check Pete AI**:
    - Go to "Pete AI" tab (formerly AI Mirror).
    - (Note: The UI for downloading models is not yet implemented in frontend, but the API exists).
    - You can test the API using curl or Postman:

      ```bash
      # List models
      curl http://localhost:3000/api/pete/models
      
      # Download Pete model
      curl -X POST http://localhost:3000/api/pete/models/download -H "Content-Type: application/json" -d '{"alias": "pete"}'
      ```

## Next Steps

- Implement the frontend UI for the Model Manager (download buttons, progress bars).
- Implement the actual RAG pipeline in `PeteAssistant` using `candle` and `qdrant` (or `fastembed` + vector store).
