# Instructional Design Loop & Future Roadmap

## 🔄 The Instructional Design Loop

This document maps out the phases for the final instructional design loop, transforming "Ask Pete" from a tool into a self-improving educational ecosystem.

### Phase 1: The "Pete" AI Assistant (Current)

- **Goal**: Enable real-time assistance for users.
- **Features**:
  - "Pete" (Gemma 1B/Llama 3) available in the sidebar.
  - Context-aware help based on current screen.
  - RAG (Retrieval Augmented Generation) from the Technical Manual.

### Phase 2: The Authoring Loop

- **Goal**: Allow Instructional Designers (IDs) to create content without code.
- **Features**:
  - **Visual Node Editor**: Create dialogue trees and quest logic visually.
  - **Asset Manager**: Drag-and-drop 3D models and audio.
  - **"Play Mode"**: Instant preview of created content.

### Phase 3: The AI Co-Pilot ("Pete" as IDE)

- **Goal**: Empower IDs to extend the app using natural language.
- **Features**:
  - **Natural Language Coding**: IDs ask Pete "Make a module that tracks eye movement," and Pete generates the Rust/Leptos boilerplate.
  - **Virtual Environment**: Pete spins up a sandboxed environment to test generated code safely.
  - **Code Explanation**: Pete explains *why* the code works, teaching the ID basic concepts.

### Phase 4: The Feedback Loop (Research)

- **Goal**: Close the loop by using student data to improve content.
- **Features**:
  - **Heatmaps**: Visualize where students get stuck in the 3D world.
  - **Sentiment Trends**: Analyze aggregate student sentiment over time.
  - **Automated Iteration**: Pete suggests content tweaks based on data (e.g., "Students are failing this quiz; try simplifying the language").

## 🤖 AI IDE Integration ("Pete")

To realize the vision of an "Open Source AI IDE," we will integrate the following capabilities:

### 1. Code Generation & Reading

- **Large Language Model (LLM)**: Llama 3 (8B or 70B) for complex code generation.
- **Small Language Model (SLM)**: Gemma 2B or Phi-3 for fast, on-device code completion and explanation.
- **Context**: The models will have read access to the entire `src/` directory to understand the codebase context.

### 2. Virtual Environments

- **Sandboxing**: Use WebAssembly (WASM) or lightweight Docker containers to run generated code safely.
- **Hot Reloading**: Instant feedback when code is modified by the AI.

### 3. The "Pete" Interface

- **Chat-Driven Development**: A dedicated "IDE Mode" where the chat interface takes center stage.
- **Diff View**: Pete shows "Before" and "After" views of code changes for ID approval.
- **One-Click Deploy**: Approved changes are compiled and hot-swapped into the running application.

## 🗺️ Roadmap Timeline

| Phase | Description | Estimated Timeline |
|-------|-------------|--------------------|
| **1** | **MVP Release** (Current) | Q4 2025 |
| **2** | **Authoring Tools** | Q1 2026 |
| **3** | **AI IDE Alpha** | Q2 2026 |
| **4** | **Full Instructional Loop** | Q3 2026 |
