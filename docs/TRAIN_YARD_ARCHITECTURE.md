# The Train Yard: Narrative-ECS Architecture

## The Problem: Narrative-ECS Mismatch

You are trying to force a linear story (Step A -> Step B) into an Entity Component System (ECS) engine (Bevy) designed for parallel, non-linear physics. That is why it "isn't working." The ECS wants to update everything at once; the story wants to wait for a player choice.

## The Solution: Decoupling Authoring from Engine

To fix this and give you the "best tool" for authoring, we need to build **The Train Yard** as a separate, web-based Graph Editor that exports a strict JSON format your Rust engine can digest.

### 1. The Architecture

Do not try to build the authoring tool *inside* Bevy yet. It is too hard for non-technical users (teachers/students) and UI iteration is slow in Rust.

* **The Authoring Tool ("The Train Yard"):** A web app (React Flow or Svelte Flow) running in the browser. It allows users to visually drag-and-drop nodes to create story branches. It saves a `.json` file.
* **The Runtime ("The Locomotive"):** Your Bevy Rust game. It loads the `.json` file as an Asset and plays it.

### 2. The Data Schema (The "Contract")

This is the most important part. You need a strict Schema that both the Web Tool and the Rust Game understand. This resolves the "Dependency Hell" of data.

**Requirements:**

1. A unique `node_id`.
2. `speaker_name` and `dialogue_text`.
3. A list of `choices`. Each choice has a `text`, a `target_node_id`, and optional `requirements` (e.g., 'Requires Analysis > 5').
4. `events` triggered on entry (e.g., 'Add Item: Coal').

**JSON Structure:**

```json
{
  "graph_id": "quest_intro_01",
  "nodes": [
    {
      "id": "node_1",
      "speaker": "Pete",
      "text": "Hello there!",
      "choices": [
        { "text": "Hi", "target": "node_2" }
      ],
      "events": []
    }
  ]
}
```

### 3. The Authoring Tool ("The Train Yard")

You need a visual node editor. Writing JSON by hand is painful and error-prone.

**Tech Stack:** React + React Flow (or Svelte Flow).
**Why:** It provides "drag-and-drop" boxes out of the box.

**Key Feature:**

* **"The Weigh Station":** Add a "complexity check." If the text in a node is longer than 280 characters, turn the node border Red. This enforces the "Cognitive Load" rules.

### 4. The Runtime Integration (Bevy ECS)

You need a **State Machine** resource to manage the story, separate from the tick-rate of the game.

**The Fix:**

1. **Load the Asset:** Use `bevy_common_assets` to load the `.json` file into a `Handle<NarrativeGraph>`.
2. **The Resource:** Create a global resource `CurrentStoryState`.

    ```rust
    struct CurrentStoryState {
        current_node_id: String,
        graph_handle: Handle<NarrativeGraph>,
        waiting_for_input: bool,
    }
    ```

3. **The System:** Create a system that only runs when `waiting_for_input` is FALSE. It looks up the `current_node_id`, spawns the UI for that node, and sets `waiting_for_input` to TRUE.
4. **The Interaction:** When a player clicks a button, a separate system updates `current_node_id` to the new target and sets `waiting_for_input` to FALSE.

### 5. Best Practices for "Public Delivery"

To allow students to build their *own* stories:

1. **Host the Editor:** Host the React App ("Train Yard") on GitHub Pages or Vercel.
2. **Local Export:** Let them download their `.json` file.
3. **Drag-and-Drop Loading:** Update your Bevy game to accept a `.json` file drop (using `bevy_drop`) to instantly play their custom quest. This creates an immediate "Create -> Play" loop.
