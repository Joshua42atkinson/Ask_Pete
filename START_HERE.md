# 🚂 ASK PETE - START HERE

**Last Updated:** 2025-12-03

## ⚠️ CRITICAL: What You Actually Have

You have a **FULL WORKING WEBSITE** with node editor and GPS integration.

**IT IS NOT** the simple React app at `ask_pete_trainyard_web`  
**IT IS** the Leptos app at `ask_pete_node_garden`

---

## 🎯 How to Launch Everything (One Command)

```powershell
.\start_full_stack.ps1
```

This launches:

1. **Backend** (Rust server on port 3000)
2. **Frontend** (Leptos app on port 8080)
3. **Opens browser** to the Train Yard authoring tool

---

## 🗺️ The Real Architecture

### The Actual Working Stack

```
┌─────────────────────────────────┐
│  ask_pete_node_garden (Leptos)  │ ← YOUR FULL WEBSITE
│  Port 8080                      │   (Node editor, GPS, grid)
│  /yard = Authoring Tool         │
│  /play = Student View           │
└──────────┬──────────────────────┘
           ↓ REST API
┌──────────┴──────────────────────┐
│  ask_pete_server (Axum)         │ ← BACKEND
│  Port 3000                      │
│  /api/expert/graph              │
└──────────┬──────────────────────┘
           ↓
┌──────────┴──────────────────────┐
│  Database + Local AI            │
└─────────────────────────────────┘
```

### The Dead Ends (Don't Use These)

- ❌ `ask_pete_trainyard_web` - Minimal React prototype, not your real app
- ❌ Port 5173 - Wrong frontend

---

## 📍 URL Routes

Once running, go to:

**Authoring Tool (Node Editor):**  
`http://127.0.0.1:8080/yard`

**Student View:**  
`http://127.0.0.1:8080/play`

**Teacher Dashboard:**  
`http://127.0.0.1:8080/` (if on port 8082)

---

## 🔧 If Something Breaks

### Backend won't start

```powershell
cd Ask_Pete
cargo run --bin ask_pete_server
```

### Frontend won't compile

```powershell
cd crates\ask_pete_node_garden
trunk serve --open
```

### Missing dependencies

```powershell
cargo build --workspace
cd crates\ask_pete_node_garden
npm install  # if needed
```

---

## 📂 Project Structure (THE TRUTH)

```
Ask_Pete/
├── crates/
│   ├── ask_pete_node_garden/    ← YOUR MAIN APP (Leptos)
│   │   ├── src/
│   │   │   ├── components/
│   │   │   │   └── authoring/
│   │   │   │       └── node_canvas.rs   ← NODE EDITOR
│   │   │   └── pages/
│   │   └── Trunk.toml (port 8080)
│   │
│   ├── ask_pete_server/          ← BACKEND
│   │   └── src/
│   │       └── handlers/
│   │
│   ├── ask_pete_trainyard_web/   ❌ IGNORE (old prototype)
│   └── ask_pete_ai/              ← LOCAL AI (Mistral 7B)
│
├── assets/models/                 ← AI MODELS (4.3GB)
│   ├── mistral-7b-instruct-v0.1.Q4_K_M.gguf
│   └── tokenizer.json
│
└── start_full_stack.ps1          ← LAUNCH EVERYTHING
```

---

## 🎨 Features You Built

Your `ask_pete_node_garden` has:

✅ **Node Canvas** - Visual graph editor  
✅ **GPS Integration** - Map nodes to real locations  
✅ **Grid System** - Background grid for layout  
✅ **Authoring Tools:**

- Blueprint Station (AI generation)
- Inspector Panel
- Word Smithy
- Owl Diagnostic

✅ **Student Experience:**

- Play Mode
- Character Creation
- Progress Tracking

✅ **AI Features:**

- Local Mistral 7B
- Blueprint generation
- Socratic dialogue

---

## 🚀 Next Session Checklist

When you start coding again:

1. ✅ Read this file first
2. ✅ Run `.\start_full_stack.ps1`
3. ✅ Go to `http://127.0.0.1:8080/yard`
4. ✅ Start creating nodes

**DO NOT** waste time on discovery - everything is documented here.

---

## 📝 For AI Assistants (Read This First)

When helping Trinity:

1. **Never** suggest the React app (`ask_pete_trainyard_web`) - it's a dead end
2. **Always** work with `ask_pete_node_garden` (Leptos)
3. **Always** check `START_HERE.md` before making suggestions
4. The node editor is at `/yard`, not `/`
5. Backend MUST be running on port 3000 for frontend to work
6. GPS/map integration is a planned feature, ask before implementing

---

**Last successful run:** [Update this when you successfully launch]
**Known issues:** [Document problems here]
