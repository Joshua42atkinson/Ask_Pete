# Hermetic AI Loop Implementation Plan

## Goal

Create a fully testable MVP of a unified AI dashboard that integrates local AI model generation, RAG, blueprint visualization, and a playtesting loop.

## Status

- [x] **Unified Dashboard (React)**: Created `UnifiedDashboard.tsx` consolidating Architect, Visualization, and Playtest.
- [x] **RAG Integration**: `CurriculumArchitect` and `SocraticEngine` now use `db_pool` for knowledge retrieval.
- [x] **Local AI Integration**:
  - Attempted `candle` (Rust) integration but failed due to Vulkan/GPU compilation issues.
  - [NEW] Integrated **LM Studio** client (port 1234) for reliable GPU acceleration.
- [ ] **Verification**:
  - Test loop with LM Studio timed out (generation slow or connection issue).
  - Need to optimize prompt or model settings for faster response.

## Architecture

- **Frontend**: React (Vite) + `gloo_net` for API calls.
- **Backend**: Rust (Axum + Bevy).
- **AI**:
  - **Primary**: LM Studio (OpenAI-compatible API at `http://localhost:1234/v1`).
  - **Fallback**: Google Gemini (Cloud).
  - **Safety**: `WeighStation` (requires local model, currently mocked or using CPU-loaded model if available).

## Next Steps

1. **Optimize LM Studio**: Ensure model is loaded and responsive.
2. **Retest Loop**: Run the browser test again to confirm end-to-end functionality.
3. **Telemetry**: Implement backend storage for playtest metrics.
