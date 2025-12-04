---
description: Repair Plan for Ask Pete
---

# Repair Plan: Transition to React & Vite

This plan outlines the steps to fully transition the "Ask Pete" authoring tool to a React/Vite architecture, removing the legacy Leptos frontend, and ensuring a stable, high-performance development environment.

## 1. Stabilization (Current Status: In Progress)

- [x] **Fix Critical React Errors**: Resolved `nodeTypes` memoization and `ReactFlow` container height issues in `App.tsx`.
- [x] **Verify UI Rendering**: Confirmed the React editor loads correctly without error overlays.
- [ ] **Verify Console Cleanliness**: Ensure no persistent warnings remain in the browser console.

## 2. Architecture Consolidation

- [ ] **Remove Leptos Frontend**:
  - Delete `crates/ask_pete_node_garden` (the Leptos app).
  - Remove `leptos` dependencies from the workspace `Cargo.toml`.
  - Clean up any shared code in `ask_pete_core` that was specific to Leptos (if any).
- [ ] **Port Player to React**:
  - The current "Play Story" button redirects to `localhost:8080` (Leptos).
  - We need to build a "Play Mode" within the React application (`ask_pete_trainyard_web`) or a separate React app for the player.
  - **Recommendation**: Add a `/play/:graphId` route to the existing React app to serve as the player. This keeps everything in one modern codebase.

## 3. Tooling & Environment

- [ ] **Standardize Dev Command**: Update `start_full_stack.ps1` to run `cargo run --bin ask_pete_server` and `npm run dev` (for React), removing `trunk serve`.
- [ ] **Linting & Formatting**: Ensure `eslint` and `prettier` are correctly configured for the React project to catch issues early.

## 4. Feature Parity & Enhancements

- [ ] **Persona Engine**: Fully implement the Persona Engine modal (currently a placeholder).
- [ ] **Word Smithy**: Ensure vocabulary data is correctly saved and retrieved from the backend.
- [ ] **Industrial Theme**: Continue refining the UI to match the "Boilermaker" aesthetic.

## 5. Next Session Goals

- Execute the removal of `ask_pete_node_garden`.
- Implement the React-based Player route.
- Verify the full "Create -> Save -> Play" workflow within the single React application.
