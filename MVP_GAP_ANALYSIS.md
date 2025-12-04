# MVP Gap Analysis: Heavy Lifting Remaining

This document outlines the "soft" or "lite" areas of the codebase that require significant engineering effort ("heavy lifting") to reach a production-ready MVP.

## 1. AI Inference (The "Brain")

- **Current State**: `libs/infra-ai/src/local_inference.rs` uses a **simulation** (sleeps for 50ms and returns a mock string).
- **Heavy Lifting**: Implement the actual `candle` or `ort` (ONNX Runtime) integration to load and run the Gemma 2B/9B models locally. This involves managing GPU buffers, tokenizer streams, and memory safety.

## 2. Cognitive Logistics (The "Heart")

- **Current State**: `apps/grand-central/src/handlers/quest.rs` uses a **hardcoded value** (`intrinsic_load = 5.0`) when starting a quest.
- **Heavy Lifting**:
  - Connect `start_quest` to the `WeighStation` analysis stored in the database.
  - Implement the "Dynamic Load Adjustment" system in Bevy (`domain-physics`) that modifies `Coal` burn rates based on real-time student performance (e.g., answering questions correctly/incorrectly).

## 3. Authentication (The "Keys")

- **Current State**: Google Login is **mocked/bypassed** for developer velocity.
- **Heavy Lifting**: Implement the full OAuth 2.0 flow with secure session management, token refreshment, and role-based access control (RBAC) enforcement in `axum` middleware.

## 4. Phygital/GPS (The "World")

- **Current State**: `apps/player/src/app.rs` has basic geolocation polling and a "mock zone" check.
- **Heavy Lifting**:
  - Implement robust Haversine distance calculations in the backend (`VaamService`).
  - Create a reliable "Unlock" mechanism that persists discovered locations to the `StudentProfile`.
  - Handle edge cases (GPS drift, signal loss).

## 5. Economy Loop (The "Fuel")

- **Current State**: We can *start* a quest and *burn* coal, but we lack a robust **Completion Handler**.
- **Heavy Lifting**: Implement the logic to:
  - Detect when a quest is finished (last node reached).
  - Calculate the final "Steam" reward based on performance and difficulty.
  - Commit these changes to the Postgres database (currently only exists in ephemeral ECS state).

## 6. Error Handling & Resilience

- **Current State**: The codebase contains numerous `unwrap()` calls and basic error logging.
- **Heavy Lifting**: Implement a comprehensive error recovery strategy (e.g., retry logic for AI failures, graceful degradation if the GPU is unavailable).
