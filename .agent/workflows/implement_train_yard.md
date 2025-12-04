---
description: Implement the Train Yard Narrative-ECS Architecture
---
# Implement Train Yard Architecture

This workflow guides the implementation of the "Train Yard" decoupled authoring tool and its integration with the Bevy engine.

## Reference

- [Architecture Document](file:///c:/Users/Trinity/Documents/daydream/Ask_Pete/docs/TRAIN_YARD_ARCHITECTURE.md)

## Steps

1. **Define Data Schema**
   - Create `NarrativeGraph` struct in `ask_pete_core` (or a new `ask_pete_format` crate).
   - Ensure it supports `node_id`, `speaker`, `text`, `choices`, and `events`.
   - Generate/Write matching TypeScript interfaces.

2. **Scaffold Web Editor ("The Train Yard")**
   - Initialize a new React project (e.g., in `crates/ask_pete_trainyard_web` or a separate repo).
   - Install `reactflow`.
   - Implement `DialogueNode` component.
   - Implement JSON export matching the schema.

3. **Implement Runtime Loader ("The Locomotive")**
   - Add `bevy_common_assets` dependency to `ask_pete_server` (or `ask_pete_physics`).
   - Implement `AssetLoader` for `NarrativeGraph`.
   - Create `CurrentStoryState` resource.

4. **Implement Narrative Systems**
   - Create `narrative_driver` system in Bevy.
   - Create `button_handler` system for UI interaction.
   - Verify "Create -> Export -> Play" loop.
