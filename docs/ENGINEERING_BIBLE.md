# Ask Pete: Engineering Bible & Decision Log

> "The code tells you *how*, this document tells you *why*."

## Purpose

This document serves as the "Human-in-the-Loop" Logic File. It records the architectural decisions, strategic pivots, and the "why" behind the code for the *Ask Pete* project. It is designed to allow any new engineer (human or AI) to understand the project's evolution and current state immediately.

## Architectural Decisions

### 1. The "Three Frontends" Strategy (Workspace Architecture)

**Date:** 2025-11-29
**Context:** Initially, the project used a single `frontend` crate with port-based routing to separate Researcher, Teacher, and Player interfaces.
**Decision:** We pivoted to a **Workspace Architecture** with three separate application crates:

- `apps/researcher`: For data analysis and system configuration.
- `apps/teacher`: For lesson planning and student oversight.
- `apps/player`: For the student game experience.

**Logic & Reasoning:**

- **Security:** Strict separation ensures students never download admin/teacher code, preventing reverse-engineering of sensitive tools.
- **Scalability:** Allows independent scaling of the Player app (high concurrency, low compute) vs. Researcher app (low concurrency, high compute).
- **Organization:** Facilitates future team separation (e.g., a "Game Team" vs. a "Data Team") without merge conflicts.
- **SaaS Readiness:** Aligns with enterprise requirements for data isolation and role-based access control.

### 2. Rust-First Stack (The "Iron Chassis")

**Context:** The project requires high performance, type safety, and WASM capability.
**Decision:** We utilize a full Rust stack:

- **Backend:** Axum (Web Server), SQLx (Database), Bevy (ECS/Game Logic).
- **Frontend:** Leptos (WASM UI Framework).
- **Shared:** Common crate for shared types and logic.

**Logic:**

- **Type Safety:** Shared types between frontend and backend eliminate a vast class of bugs.
- **Performance:** Rust/WASM offers near-native performance for the game client.
- **Concurrency:** Rust's ownership model ensures thread safety for the complex backend simulation.

## Project Structure

- `crates/`: Contains all the application and library crates.
  - `ask_pete_researcher`: The Researcher App (formerly `apps/researcher`).
  - `ask_pete_trainyard`: The Teacher App (formerly `apps/teacher`).
  - `ask_pete_client`: The Player App (formerly `apps/player`).
  - `ask_pete_server`: The Backend Server.
  - `ask_pete_core`: Shared types and logic.

## System Architecture: The Iron Network

### 1. The Iron Chassis (Core Stack)

The foundation of the application is built on a high-performance Rust stack designed for reliability and speed.

- **Backend**: `Axum` (Web Server) + `Bevy` (ECS Simulation) + `SQLx` (Postgres).
- **Frontend**: `Leptos` (WASM UI) + `TailwindCSS`.
- **Communication**: Shared types in `common/` ensure the frontend and backend speak the exact same language.

### 2. The Socratic Engine (AI Core)

Located in `backend/src/ai/socratic_engine.rs`, this is the conversational heart of the system.

- **Role**: Manages dialogue with the student, acting as a Socratic tutor.
- **Integration**: Connects to Google's Gemini API (`gemini_client.rs`) for intelligence.
- **Memory**: Uses a local vector database (LanceDB) to recall past interactions.
- **Weigh Station**: Analyzes content for Cognitive Load to prevent burnout.
- **Hardware Scanner**: Detects local specs to optimize AI model selection (2B vs 9B).

### 3. The Antigravity Bridge (Cloud Sync)

- **Role**: Handles secure, anonymized telemetry synchronization with Google Cloud.
- **Privacy**: Ensures only "Coal & Steam" metrics leave the device; PII remains local.

### 4. The Event Bridge (Axum <-> Bevy)

To bridge the gap between the stateless Web Server (Axum) and the stateful Game Loop (Bevy), we use an **Inbox/Outbox Pattern**:

- **Inboxes (Axum -> Bevy)**: Shared resources (e.g., `QuestCommandInbox`, `PeteCommandInbox`) where Axum handlers push events. Bevy systems (e.g., `sync_quest_inbox`) drain these each frame and trigger internal ECS events.
- **Outboxes (Bevy -> Axum)**: Shared resources (e.g., `SharedPhysicsResource`) that Bevy systems update each frame. Axum handlers read these to serve real-time data to the frontend.

### 5. The Three Apps (Audience Separation)

We have physically separated the user interfaces to ensure security and scalability.

#### A. Researcher App (`apps/researcher`)

- **Port**: 8081
- **Audience**: Data Scientists, System Admins.
- **Key Features**:
  - `ResearchDashboard`: Real-time telemetry and system health.
  - `AiTestPanel`: Tools for testing and fine-tuning the AI models.
  - **Security**: This app has access to raw data and system configs. It should never be exposed to students.

#### B. Teacher App (`apps/teacher`)

- **Port**: 8082
- **Audience**: Instructors, Curriculum Designers.
- **Key Features**:
  - `TrainYard`: The visual graph editor for creating lesson plans (`StoryGraph`).
  - `KnowledgeLibrary`: Management of educational content and assets.
  - **Workflow**: Teachers design "Journeys" here, which are then saved to the database for students to play.

#### C. Player App (`apps/player`)

- **Port**: 8083
- **Audience**: Students (The "Players").
- **Key Features**:
  - `PlayMode`: The immersive game interface where students traverse the `StoryGraph`.
  - `EngineCab`: The dashboard showing their progress and "fuel" (cognitive load).
  - `GpsHud`: Mobile-responsive navigation aid.
  - **Design**: Optimized for low latency and high engagement. It contains *zero* admin code.

## Key Workflows

### The "Blueprint" Workflow (Content Creation)

1. **Teacher** uses `TrainYard` (Port 8082) to design a lesson.
2. **Weigh Station** analyzes the lesson in real-time.
3. **Safety Lockout**: If the Cognitive Load is too high for the target audience, the system prevents saving.
4. They use the **Curriculum Architect** (AI) to generate a `StoryGraph`.
5. The graph is saved to the Postgres database.
6. **Student** logs into `Player App` (Port 8083).
7. The backend serves the `StoryGraph` to the student as a playable "Journey".

### The "Socratic" Workflow (Learning)

1. **Student** encounters a challenge node in `PlayMode`.
2. They ask a question via the chat interface.
3. **Backend** routes the question to the `SocraticEngine`.
4. **AI** analyzes the student's `CognitiveLoad` and `VirtueTopology`.
5. **AI** responds with a hint or question, not just the answer.

## The Phygital Expansion

### Node Gardens & GPS

- **Concept**: The "Railway" extends into the physical world.
- **Implementation**: `libs/domain-physics` uses `osmpbfreader` to parse OpenStreetMap data.
- **Mechanic**: Students must physically travel to "Stations" (Geofenced zones) to unlock specific curriculum nodes.
141:
142: ### Game Logic & Mechanics
143:
144: **Location**: `libs/domain-physics`
145:
146: All core game mechanics (e.g., movement, inventory, stats, combat calculations) must be implemented in the `domain-physics` crate. This ensures that:
147: - Logic is shared between the Client (prediction) and Server (authority).
148: - It is isolated from UI code (`apps/player`) and Infrastructure code (`backend`).
149: - It can be unit tested in isolation.

## Maintenance & Troubleshooting

### Common Issues

- **"AI isn't generating triggers"**: Check `backend/src/ai/architect.rs`. Ensure the prompt includes the correct JSON schema for `logic`.
- **"Graph editor is broken"**: Check `apps/teacher/src/components/node_canvas.rs`. This component handles the complex SVG rendering.
- **"Build fails on new app"**: Ensure `Cargo.toml` in the app correctly points to `../../common` for shared types.

## Current Focus

- **MVP Polish & Verification:** Ensuring the end-to-end loop (Ask Pete -> Steam -> Coal) works seamlessly.
- **Cognitive Logistics:** Fine-tuning the `WeighStation` and `CognitiveLoad` integration.
- **Phygital Integration:** Expanding the Node Garden and GPS features.
