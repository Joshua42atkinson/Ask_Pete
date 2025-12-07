# Recommendations for Finalizing "Ask Pete" MVP

## Executive Summary

To align with Purdue's requirements for a student study tool and finalize the MVP, we need to bridge the gap between the current prototype and the desired "Mistral 7b + RAG" architecture. The current system relies on mocked AI or Cloud Gemini, and lacks the robust "Store" and "Mentorship" features required for the hybrid approach.

## 1. AI Architecture Alignment (The "Brain")

**Requirement**: Local Mistral 7b LLM + RAG (Retrieval-Augmented Generation).
**Current State**:

- Code references `Gemma` and `Gemini`.
- `LocalModel` defaults to `mistral-7b-instruct-v0.1.Q4_K_M.gguf` in `main.rs`, which is good.
- **Gap**: The model file likely doesn't exist in the repo (too large). We need to ensure the deployment process (Dockerfile) downloads this model or mounts it.
- **Gap**: RAG is stubbed. `knowledge.rs` does not chunk or embed documents. `LanceDB` is initialized but not connected to the `SocraticEngine`.

**Recommendation**:

- **Switch to Mistral 7b**: Confirm `assets/models/mistral-7b-instruct-v0.1.Q4_K_M.gguf` is the target.
- **Implement RAG**:
  - Use `fastembed-rs` for local embedding generation (no API costs).
  - Finish the `LanceDB` integration in `knowledge.rs` to store and query vectors.
  - Connect `SocraticEngine` to `LanceDB` to inject context into the Mistral prompt.

## 2. Blueprint Generation (The "Core Loop")

**Requirement**: AI-assisted blueprint creation.
**Current State**:

- Fails with 500 Error because AI generates Rust-like syntax instead of JSON.
- Prompt in `architect.rs` is fragile.

**Recommendation**:

- **Schema Enforcement**: Update the `architect.rs` prompt to use a "Grammar" (if using `llama.cpp`/`candle` locally) or a one-shot example that strictly enforces JSON.
- **Robust Parsing**: Implement a fallback parser that can handle "Rust-struct-style" output if the model refuses to speak JSON.

## 3. Hybrid Cloud Architecture (The "Network")

**Requirement**: Google Cloud Run + Google Auth + Student Store.
**Current State**:

- **Auth**: `oauth2` flow is implemented but uses **mocked JWTs** and hardcoded `localhost` redirects.
- **Store**: No data model or API exists for the "Student Store" (upgrades/economy).
- **Deployment**: `Dockerfile` is outdated (targets Leptos instead of React).

**Recommendation**:

- **Fix Auth**: Implement real JWT signing (using `jsonwebtoken` crate) and make the redirect URL configurable via `ENV` vars.
- **Build the Store**: Create `store_items` and `student_inventory` tables. Add API endpoints for purchasing upgrades (burning "Coal" currency).
- **Update Dockerfile**: Modify the build stage to compile the React App (Vite) and the Rust Backend, serving static files correctly.

## 4. Immediate Action Plan (Phase 1)

1. **Fix Dockerfile**: Ensure we can build the React+Rust bundle.
2. **Activate Mistral**: Ensure the local model loading works (or fallback to Gemini with RAG if local is too heavy for Cloud Run - *Note: Cloud Run has no GPU, so local 7b inference will be slow. We might need to recommend sticking to Gemini for the Cloud version, or using a smaller model like Danube 3 or Gemma 2B for local.*).
3. **Implement RAG**: Enable the "Brain" to read uploaded documents.

## 5. Critical Question for User

**Cloud Run vs. Local Inference**:
Running Mistral 7b *inside* a standard Google Cloud Run container (CPU only) will be very slow (tokens/sec < 1).

- **Option A**: Use Gemini (Cloud) for the Cloud Run instance, and Mistral (Local) only for the local student app.
- **Option B**: Deploy to a GPU-enabled service (more expensive).
- **Option C**: Use a very small model (Phi-3, Gemma 2B) for Cloud Run.

*Recommendation: Hybrid. Use Gemini for the Cloud Server (Teacher/Group logic) and Mistral for the Local Student App (Privacy/Offline).*
