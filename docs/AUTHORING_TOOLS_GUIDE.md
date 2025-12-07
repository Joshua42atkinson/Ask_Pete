# Authoring Tools Location Guide

## 📍 Where Are The Functions?

Good news! **All your authoring tools are present**. They're just distributed across multiple crates. Here's the complete map:

---

## 🚂 The Train Yard Authoring System

### **1. Web-Based Visual Editor** (React/TypeScript)

**Location:** `crates/ask_pete_trainyard_web/`

**What it does:**

- Visual node-based graph editor using ReactFlow
- Drag-and-drop dialogue/narrative creation
- Export to JSON format for the game engine

**Key Files:**

- `src/App.tsx` - Main editor interface with graph visualization
- `src/components/DialogueNode.tsx` - Node component for dialogue
- `src/types/NarrativeGraph.ts` - Type definitions

**How to Run:**

```powershell
cd crates\ask_pete_trainyard_web
npm install
npm run dev
```

**Features:**

- ✅ Export JSON button
- ✅ ReactFlow graph editor
- ✅ Visual node connections
- ✅ MiniMap and Controls

---

### **2. Rust Backend Authoring Framework**

**Location:** `crates/ask_pete_trainyard/src/authoring/`

**Components:**

1. **`blueprint_station.rs`** (10.5 KB) - Blueprint generation system
2. **`node_canvas.rs`** (23.2 KB) - Visual canvas for node editing
3. **`inspector.rs`** (25.3 KB) - Property inspector panel
4. **`property_editor.rs`** (8.6 KB) - Edit node properties
5. **`story_node.rs`** (5.1 KB) - Story node data structures
6. **`template_selector.rs`** (12.2 KB) - Pre-built templates
7. **`word_smithy.rs`** (4.7 KB) - Text/vocabulary authoring
8. **`owl_diagnostic.rs`** (7.4 KB) - AI diagnostic tools
9. **`toolbar/`** - Toolbar components

---

### **3. Distributed Authoring UI Components**

These exist in **multiple frontend crates** (likely for different views/apps):

```
ask_pete_client/src/components/authoring/
ask_pete_frontend/src/components/authoring/
ask_pete_node_garden/src/components/authoring/
ask_pete_researcher/src/components/authoring/
ask_pete_tools/src/components/authoring/
```

Each also has `pages/train_yard_layout.rs` for layout management.

---

## 🔧 Quick Launch Scripts

### **Option A: Web Authoring Tool (Recommended for Quick Start)**

Create `launch_trainyard_web.ps1`:

```powershell
Write-Host "🚂 Launching Train Yard Web Editor..." -ForegroundColor Cyan
cd crates\ask_pete_trainyard_web
npm install
npm run dev
```

### **Option B: Full Integrated System**

The authoring tools are integrated into the main Ask Pete server. Check:

- `crates/ask_pete_server/src/train_yard.rs` - Server-side graph logic

---

## 🎯 What Functions Are Available?

### **From ask_pete_trainyard (Rust)**

```rust
pub mod authoring {
    pub use blueprint_station::BlueprintStation;
    
    // Other modules:
    // - node_canvas: Visual graph editing
    // - inspector: Property inspection
    // - property_editor: Edit properties
    // - story_node: Story data structures
    // - template_selector: Templates
    // - word_smithy: Vocabulary authoring
    // - owl_diagnostic: AI diagnostics
}
```

### **From ask_pete_server/train_yard.rs**

```rust
pub struct TrainYard {
    pub stations: HashMap<Uuid, Station>,
    pub tracks: Vec<Track>,
}

// Functions:
- add_station(title, content, station_type, x, y) -> Uuid
- connect_stations(source, target, logic_gate)
```

### **From trainyard_web (React)**

```typescript
// App.tsx
- exportJson(): Export narrative graph to JSON
- useNodesState: Manage node state
- useEdgesState: Manage edge connections
- onConnect: Handle node connections
```

---

## 📝 Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│  WEB AUTHORING TOOL (React)                             │
│  crates/ask_pete_trainyard_web/                         │
│  → Visual graph editor                                  │
│  → Export JSON                                          │
└──────────────────────┬──────────────────────────────────┘
                       │
                       ↓ JSON Export
┌──────────────────────────────────────────────────────────┐
│  RUST BACKEND (Bevy ECS)                                 │
│  crates/ask_pete_trainyard/                              │
│  → Load JSON as Asset                                    │
│  → Execute narrative graph                               │
│  → Physics/game integration                              │
└──────────────────────────────────────────────────────────┘
```

---

## 🚀 Next Steps

1. **To use the visual editor:**

   ```powershell
   cd crates\ask_pete_trainyard_web
   npm run dev
   ```

   Open browser to `http://localhost:5173`

2. **To integrate with the game:**
   - Export JSON from web editor
   - Place in `assets/narratives/`
   - Load via Bevy asset system

3. **To extend functionality:**
   - Modify `ask_pete_trainyard/src/authoring/` modules
   - Update web UI in `ask_pete_trainyard_web/src/`

---

## 📚 Related Documentation

- [TRAIN_YARD_ARCHITECTURE.md](docs/TRAIN_YARD_ARCHITECTURE.md) - Architecture overview
- [TRAIN_YARD_TECHNICAL_SPEC.md](docs/TRAIN_YARD_TECHNICAL_SPEC.md) - Technical specs
- [The Field Manual](#) - "The Train Yard: Authoring Tools" (Section 7.3)

---

## 🆘 Troubleshooting

**Q: Nothing shows up when I run npm run dev?**

- Check if dependencies are installed: `npm install`
- Check ports: Default is 5173
- Check console logs for errors

**Q: Where do I find the exported JSON files?**

- They auto-download to your Downloads folder
- Named: `narrative_graph.json`

**Q: How do I integrate with the Rust backend?**

- See `docs/TRAIN_YARD_ARCHITECTURE.md` Section 4

---

**Status:** ✅ All authoring tools are present and accounted for!
