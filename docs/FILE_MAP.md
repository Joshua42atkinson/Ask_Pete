# File Naming Map & Project Structure

This document serves as a guide for AI agents ("vibe coding") to understand the project structure and file naming conventions of the **Daydream / Ask Pete** repository.

## 1. Project Root

- `Cargo.toml`: Workspace definition.
- `apps/`: Application binaries.
- `libs/`: Shared libraries.
- `backend/`: (Legacy/Deleted) Old backend code.

## 2. Applications (`apps/`)

### `grand-central` (The Backend)

- **Purpose**: The main server application (Axum + Bevy).
- **Key Files**:
  - `src/main.rs`: Entry point, app assembly.
  - `src/state.rs`: Application state definition (`AppState`).
  - `src/handlers/`: API route handlers.
  - `src/routes/`: Route definitions (merging handlers).
  - `src/services/`: Business logic services (e.g., `model_manager.rs`, `pete.rs`).

### `frontend` (The Frontend)

- **Purpose**: The web interface (Leptos).
- **Key Files**:
  - `src/lib.rs`: App entry point.
  - `src/pages/`: UI pages (e.g., `home.rs`, `profile.rs`).
  - `src/components/`: Reusable UI components.

## 3. Libraries (`libs/`)

### `infra-ai`

- **Purpose**: AI inference and logic.
- **Key Files**:
  - `src/local_inference.rs`: Local LLM (Gemma) wrapper.
  - `src/socratic_engine.rs`: Main dialogue logic.
  - `src/llm/gemini_client.rs`: Google Gemini API client.

### `infra-db`

- **Purpose**: Database interactions.
- **Key Files**:
  - `src/conversation_memory.rs`: Postgres-backed memory.
  - `src/vector_store.rs`: LanceDB vector storage.

### `domain-physics`

- **Purpose**: Game mechanics and simulation state.
- **Key Files**:
  - `src/components.rs`: ECS Components (`SharedPhysicsResource`, `PlayerCharacter`).
  - `src/systems.rs`: Bevy systems for logic updates.

### `pete-core` (Legacy/Core)

- **Purpose**: Shared types and definitions (being migrated to domains).
- **Key Files**:
  - `src/lib.rs`: Core types (`UserRole`, `Quest`).

## 4. Naming Conventions

- **Structs**: PascalCase (e.g., `SocraticEngine`).
- **Functions**: snake_case (e.g., `generate_response`).
- **Files**: snake_case (e.g., `socratic_engine.rs`).
- **Modules**: snake_case, matching filename.

## 5. Key Concepts

- **Resources**: Shared state in Bevy/Axum (e.g., `SharedPhysicsResource`).
- **Inboxes/Outboxes**: Command pattern for async communication (e.g., `PeteCommandInbox`).
- **Handlers**: Async functions handling HTTP requests.
