# UI Component → Backend Function Mapping

**Last Updated**: 2025-11-29  
**Purpose**: Track which UI elements connect to which backend functions

---

## Teacher App (Port 8082)

### NodeCanvas (`apps/teacher/src/components/authoring/node_canvas.rs`)

| UI Element | Line | User Action | Frontend Function | Backend Endpoint | Status |
|------------|------|-------------|-------------------|------------------|--------|
| 💾 Save Button | 244-250 | Click | `save_graph_handler` (L68) | POST `/api/expert/graph` | ✅ Working |
| 📐 Blueprint Button | 261-268 | Click | `set_show_blueprint_station(true)` | Opens modal | ✅ Working |
| ▶ Play Button | 229-242 | Click | `navigate("/play/{id}")` | None (routing) | ✅ Working |
| 🦉 O.W.L. Button | 252-259 | Click | `set_show_owl_diagnostic(true)` | Opens modal | ✅ Working |
| Node Card | - | Mouse Down | `set_dragging_id` (L352) | None | ✅ Working |
| Node Card | - | Mouse Move | Updates `node.x, node.y` (L141-147) | None | ✅ Working |
| Output Port | - | Mouse Down | `on_port_mousedown` (L184) | None | ✅ Working |
| Output Port → Input Port | - | Drag release | `on_port_mouseup` (L190) creates Connection | None | ✅ Working |
| Canvas Background | - | Wheel scroll | `on_wheel` (L162) updates zoom/pan | None | ✅ Working |
| Canvas Background | - | Mouse drag | `on_mouse_move` (L121) pans canvas | None | ✅ Working |

**Data Flow (Save)**:

```
[Save Button Click] 
  → save_graph_handler (L68)
  → Collects nodes/connections from signals
  → Calls save_graph() in api.rs (L117)
  → POST /api/expert/graph
  → backend/handlers/expert.rs::save_graph (L69)
  → INSERT/UPDATE story_graphs table
  → Returns saved graph
  → Shows toast notification
```

---

### BlueprintStation (`apps/teacher/src/components/authoring/blueprint_station.rs`)

| UI Element | Line | User Action | Frontend Function | Backend Endpoint | Status |
|------------|------|-------------|-------------------|------------------|--------|
| Subject Input | ~50 | Type | Updates `subject` signal | None | ✅ Working |
| Literary Device Dropdown | ~60 | Select | Updates `literary_device` signal | None | ✅ Working |
| Focus Slider | ~70 | Drag | Updates `focus` signal (0.0-1.0) | None | ✅ Working |
| Vocabulary Input | ~80 | Type+Enter | Adds to `current_vocabulary` signal | None | ✅ Working |
| Generate Blueprint Button | ~100 | Click | `generate_handler` | POST `/api/architect/blueprint` | ✅ Working |
| Close Button | - | Click | Calls `on_close` callback | None | ✅ Working |

**Data Flow (Generate)**:

```
[Generate Button Click]
  → generate_handler
  → Creates BlueprintRequest{ subject, focus, literary_device, vocabulary }
  → POST /api/architect/blueprint
  → backend/handlers/architect.rs::generate_blueprint
  → SocraticEngine.generate_blueprint()
  → Returns StoryGraph
  → Calls on_generate callback
  → NodeCanvas receives graph and displays nodes
```

---

### Inspector (`apps/teacher/src/components/authoring/inspector.rs`)

| UI Element | User Action | Frontend Function | Backend Endpoint | Status |
|------------|-------------|-------------------|------------------|--------|
| Close Button | Click | Calls `on_close` callback | None | ✅ Working |
| Delete Button | Click | Calls `on_delete` callback | None | ✅ Working |
| Title Input | Edit | Updates `node.title` | None | ✅ Working |
| Content Textarea | Edit | Updates `node.content` | None | ✅ Working |
| AI Suggestions Button | Click | Opens AI panel | POST `/api/ai-mirror/send-message` | ⚠️ TODO |

---

## Player App (Port 8083)

### StudentDashboard (`apps/player/src/pages/student_dashboard.rs`)

| UI Element | Line | User Action | Frontend Function | Backend Endpoint | Status |
|------------|------|-------------|-------------------|------------------|--------|
| Page Load | - | Component mount | `fetch_scenarios()` (async) | GET `/api/scenarios` | ✅ Working |
| Scenario Card | ~80 | Click | `navigate("/journey/{id}")` | None (routing) | ✅ Working |

**Data Flow (Dashboard Load)**:

```
[Page Loads]
  → fetch_scenarios() spawned
  → GET /api/scenarios
  → backend/routes/scenarios.rs::list_scenarios
  → SELECT * FROM scenarios
  → Returns Vec<Scenario>
  → Renders scenario cards
```

---

### PlayMode (`apps/player/src/pages/play_mode.rs`)

| UI Element | Line | User Action | Frontend Function | Backend Endpoint | Status |
|------------|------|-------------|-------------------|------------------|--------|
| Page Load | - | Component mount | Loads scenario by ID from route | GET `/api/scenarios/:id` | ⚠️ TODO |
| Choice Button | - | Click | `handle_choice` | POST `/api/progress` | ⚠️ TODO |
| Save Progress | - | Auto | `save_progress` | POST `/api/progress` | ⚠️ TODO |

**Expected Data Flow (Not Yet Implemented)**:

```
[Play Mode Loads]
  → Extract :id from route params
  → GET /api/scenarios/:id
  → Should return Scenario + StoryGraph
  → Render first node
  → User clicks choice
  → Update current node
  → POST progress to backend
```

---

## Backend API Reference

### Story Graph Endpoints

| Method | Endpoint | Handler | Description | Status |
|--------|----------|---------|-------------|--------|
| GET | `/api/expert/graph` | `handlers/expert.rs::get_graph` | Load demo_graph | ✅ Working |
| POST | `/api/expert/graph` | `handlers/expert.rs::save_graph` | Save/Update graph | ✅ Working |

### Scenario Endpoints

| Method | Endpoint | Handler | Description | Status |
|--------|----------|---------|-------------|--------|
| GET | `/api/scenarios` | `routes/scenarios.rs::list_scenarios` | List all scenarios | ✅ Working |
| GET | `/api/scenarios/:id` | `routes/scenarios.rs::get_scenario` | Get single scenario | ⚠️ Needs graph data |

### Blueprint/AI Endpoints

| Method | Endpoint | Handler | Description | Status |
|--------|----------|---------|-------------|--------|
| POST | `/api/architect/blueprint` | `handlers/architect.rs::generate_blueprint` | Generate story graph with AI | ✅ Working |
| POST | `/api/ai-mirror/send-message` | `routes/ai_mirror.rs` | Chat with AI Mirror | ✅ Working |
| POST | `/api/ai-mirror/create-session` | `routes/ai_mirror.rs` | Create chat session | ✅ Working |

---

## Port Reference

| Service | Port | Command | URL |
|---------|------|---------|-----|
| Backend | 3000 | `cd backend && cargo run` | <http://localhost:3000> |
| Researcher App | 8081 | `cd apps/researcher && trunk serve` | <http://localhost:8081> |
| Teacher App | 8082 | `cd apps/teacher && trunk serve` | <http://localhost:8082> |
| Player App | 8083 | `cd apps/player && trunk serve` | <http://localhost:8083> |

---

## Common Flows

### Teacher Creates and Saves Blueprint

1. Visit <http://localhost:8082>
2. Click "📐 Blueprint" button
3. Fill in subject, focus, vocabulary
4. Click "Generate Blueprint"
5. AI generates graph → displays in NodeCanvas
6. Click "💾 Save" button
7. Graph saved to `story_graphs` table with id="demo_graph"

### Student Plays Scenario

1. Visit <http://localhost:8083>
2. Dashboard loads scenarios from `/api/scenarios`
3. Click scenario card
4. Navigate to `/journey/:id`
5. **TODO**: PlayMode should load scenario + linked story_graph
6. **TODO**: Render nodes and handle choices

---

## Quick Debugging Tips

### "Save button doesn't work"

- Check browser console for errors
- Check backend logs: `POST /api/expert/graph` should appear
- Verify database has `story_graphs` table: `psql -d daydream_db -c "\dt"`

### "Blueprint doesn't generate"

- Check backend logs for AI errors
- Verify `GEMINI_API_KEY` is set in `backend/.env`
- Check network tab: Should see `POST /api/architect/blueprint`

### "Scenarios don't load"

- Check backend logs: `GET /api/scenarios` should appear
- Verify `scenarios` table has data
- Check browser console for fetch errors
