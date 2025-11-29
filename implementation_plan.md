# Implementation Plan - The Iron Network (Sector HSI)

## Goal

Elevate the "Authoring Tool" from a simple graph editor to a **"Human Systems Integration" (HSI)** platform. We will implement the **"O.W.L. Diagnostic"** (Instructional Design Optimizer) and deepen the **"Foundry"** (Node Canvas) visuals to reflect the "Iron Network" lore (Steam, Pressure, Torque).

## User Review Required
>
> [!IMPORTANT]
> **The O.W.L. Protocol**: We are adding a "Diagnostic" button that analyzes node content. Currently, this will be a heuristic check (word count, active voice proxies), but it sets the stage for AI analysis.
> **Terminology Shift**: "Cognitive Load" -> "Boiler Pressure". "Connections" -> "Tracks". "Nodes" -> "Stations".

## Proposed Changes

### Phase 1: Sector HSI (Instructional Design Tools)

#### [NEW] [frontend/src/components/authoring/owl_diagnostic.rs](file:///frontend/src/components/authoring/owl_diagnostic.rs)

- **The O.W.L. Node**: A floating panel or modal that runs "Diagnostics" on the graph.
- **Functionality**:
  - **Torque Check**: Analyzes text length and keyword density (proxy for "Active Voice").
  - **Friction Check**: Identifies nodes with too many connections (> 4) or "Scale Buildup" (too much text).
  - **Signal-to-Noise**: Highlights nodes with empty content.

#### [MODIFY] [frontend/src/components/authoring/node_canvas.rs](file:///frontend/src/components/authoring/node_canvas.rs)

- **Integration**: Add the "O.W.L. Diagnostic" button to the toolbar (The Foundry Control Panel).
- **Visuals**: Update the background and UI chrome to match "The Foundry" (Sector 0) aesthetic more closely.

### Phase 2: The Foundry (Visuals & Mechanics)

#### [MODIFY] [frontend/src/components/authoring/story_node.rs](file:///frontend/src/components/authoring/story_node.rs)

- **Boiler Pressure**: Rename "Cognitive Load" to "Boiler Pressure".
- **Visual Feedback**:
  - **Green**: "Optimal Pressure" (Safe).
  - **Yellow**: "High Pressure" (Warning).
  - **Red**: "Rupture Risk" (Overload).
- **Scale Buildup**: Add a subtle texture overlay to nodes that are "stale" or "heavy".

#### [MODIFY] [common/src/models/triggers.rs](file:///common/src/models/triggers.rs)

- **Signal Logic**: Ensure triggers are conceptually aligned with "Railway Signals" (Green/Red lights).

## Verification Plan

### Automated Tests

- `cargo test` for the new `owl_diagnostic` logic (if extracted to common/utils).

### Manual Verification

- **The O.W.L.**:
    1. Open the Authoring Tool.
    2. Create a node with very little text ("Low Torque").
    3. Click "Run O.W.L. Diagnostic".
    4. Verify the node is highlighted/flagged.
- **Boiler Pressure**:
    1. Add > 4 connections to a node.
    2. Verify the "Pressure Gauge" (Passenger Count) turns Red.
