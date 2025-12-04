# Backend Source Code Index

## Exported Files

This `backend_source.txt` contains all backend source code from `ask_pete_server`, **excluding**:

- AI inference code
- Tokenizer code  
- Model loading code

## Included Modules

### Core

- `main.rs` - Server entry point, Grand Central Station
- `state.rs` - Application state management
- `error.rs` - Error handling

### Data Structures

- `train_yard.rs` - Station, Track, TrainYard definitions
- `models/` - Database models and types

### Handlers (Business Logic)

- `handlers/quest.rs` - Quest management
- `handlers/architect.rs` - Blueprint generation
- `handlers/weigh_station.rs` - Cognitive load analysis
- `handlers/expert.rs` - Expert system logic
- `handlers/persona_test_final.rs` - Persona testing

### Routes (API Endpoints)

- `routes/mod.rs` - Route definitions
- `routes/quest_routes.rs` - Quest endpoints
- `routes/expert_routes.rs` - Expert endpoints
- `routes/weigh_station_routes.rs` - Weigh Station endpoints
- `routes/debug.rs` - Debug/test endpoints

### Services

- `services/weigh_station.rs` - Cognitive load calculation
- `services/mod.rs` - Service layer coordination

## Key Architecture

**Grand Central Station** (`main.rs`):

- Axum web server on port 3000
- API routing
- Shared state management
- CORS configuration

**Train Yard** (`train_yard.rs`):

- Station = Learning node
- Track = Connection between stations
- LogicGate = Conditional routing

**Weigh Station** (`handlers/weigh_station.rs`):

- Calculates cognitive load
- Analyzes text complexity
- Word physics

## API Endpoints

```text
GET  /api/health              - Health check
GET  /api/expert/graph        - Load story graph
POST /api/expert/graph        - Save story graph
POST /api/architect/blueprint - Generate curriculum
POST /api/quest/start         - Start quest
POST /api/quest/complete/:id  - Complete quest
POST /api/weigh_station/weigh - Analyze cognitive load
```

## Usage

To review specific functionality, search for:

- "pub fn" - Function definitions
- "pub struct" - Data structures
- ".route(" - API endpoint definitions
- "TODO" or "FIXME" - Areas needing work

---

**Generated:** 2025-12-03  
**Purpose:** MVP code review and cleanup planning
