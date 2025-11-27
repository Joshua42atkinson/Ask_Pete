# Ask Pete Migration Protocol

## Moving from "Daydream" to "Ask Pete" (Capstone Rebrand)

This document outlines the protocol for migrating the codebase and research from the "Daydream" prototype to the "Ask Pete" production candidate.

### Phase 1: The Safety Snapshot

Before renaming files, freeze the current state.

1. **Git Tag:** `git tag -a v0.9-daydream-legacy -m "Final state of Daydream before Ask Pete migration"`
2. **New Branch:** `git checkout -b feature/ask-pete-rebrand`
3. **Backup:** Zip the current folder and save to a separate drive.

### Phase 2: The Identity Shift (Configuration)

Update configuration files to reflect the new identity.

1. **`src-tauri/tauri.conf.json`**
    * Change `productName` from "Daydream" to "Ask Pete".
    * Change `identifier` from `com.daydream.app` to `edu.purdue.askpete`.
2. **`backend/Cargo.toml` & `frontend/Cargo.toml`**
    * Update `name` and `authors` fields.
3. **`package.json`**
    * Update the project name and description.

### Phase 3: The Surgical Rename (Code References)

The string "Daydream" is hardcoded in paths and logic.

1. **Android Package:**
    * Recursively rename `src-tauri/gen/android/app/src/main/java/com/daydream/app/` to `src-tauri/gen/android/app/src/main/java/edu/purdue/askpete/`.
2. **Global Find & Replace:**
    * Search for "Daydream" and replace with "Ask Pete" inside string literals.
    * **Caution:** Do not break variable names unless refactoring properly.

### Phase 4: Migrating the Research (The "Brain")

Move research from "Share Drive" to the repository.

1. **Create Directory:** `docs/capstone_research` (Already done).
2. **Ingestion:**
    * Move key text files/PDFs to `assets/research` (create if needed).
    * Use the "Weigh Station" script to ingest this research.
3. **Transformation:**
    * Use `llama_engine.rs` to read old notes and extract "Vocabulary Tokens" and "Graph Nodes".
    * Example: Old Doc: "Students struggle with entropy" -> New DB Entry: Node "Entropy" (High Weight).

### Phase 5: The Persona Update

Update `AGENTS.MD` (or equivalent system prompt files) to define Pete's personality.

* **Current:** "Daydream... a mirror for the user."
* **New:** "Pete... a Station Master and Instructional Design Mentor. He speaks in train metaphors. He prioritizes efficiency and cognitive load management."
