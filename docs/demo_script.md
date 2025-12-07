# Ask Pete: One-Hour Authoring Tool Demo Script

## Opening (5 minutes)

### Problem Statement

**Traditional Curriculum Design Issues:**

- Linear, one-size-fits-all approaches
- No cognitive load management
- Difficult to create branching narratives
- Time-intensive manual creation

**Ask Pete Solution:**

- AI-assisted curriculum generation
- Built-in cognitive load analysis (The Weigh Station)
- Story-driven, non-linear learning paths
- "Train Yard" metaphor makes abstract concepts tangible

---

## Live Demo: Blueprint Generation (15 minutes)

### Step 1: Access Teacher App

1. Navigate to `http://localhost:8082`
2. Show the "Blueprint Station" / "Curriculum Architect" interface

### Step 2: Configure Blueprint

**Example Subject**: "Newton's Laws of Motion"

Fill in parameters:

- **Subject**: Newton's Laws of Motion
- **Focus Slider**: 0.5 (balanced narrative/academic)
- **Literary Device**: Hero's Journey
- **Vocabulary**: force, acceleration, mass, velocity

**Talking Points:**

- Focus slider controls narrative vs. academic balance
- Literary devices provide proven story structures
- Vocabulary integration ensures key terms are woven into content

### Step 3: Generate Blueprint

1. Click "Generate Blueprint"
2. **While waiting (10-20 seconds):**
   - Explain AI prompt engineering
   - Mention JSON enum format for game logic
   - Discuss "words as symbols of power" design philosophy

### Step 4: Examine Generated Graph

**Point out:**

- **Nodes**: Story beats aligned with Newton's Laws
- **Connections**: Non-linear paths (branching)
- **Logic Blocks**: Conditional unlocks based on mastery
- **Train Yard Terminology**: "Stations", "Tracks", "Locomotive"

**Example Node Titles** (AI-generated):

- "The Engineer's Challenge: Understanding Inertia"
- "The First Station: Force Equals Mass Times Acceleration"
- "The Junction: Choosing Your Path to Momentum"

### Step 5: Save Blueprint

1. Click "Save Blueprint"
2. Show database ID confirmation
3. Explain persistence for future use

---

## Technical Deep Dive (10 minutes)

### Architecture Highlights

- **Rust Full-Stack**: Type safety from DB to browser
- **WASM Frontend**: Leptos for reactive UI
- **Hybrid AI**: Gemini (cloud) for reasoning + Candle (local, planned) for privacy
- **ECS Game Logic**: Bevy for simulation

### AI Prompt Design

**Show** ([`architect.rs`](file:///c:/Users/Trinity/Documents/daydream/Ask_Pete/crates/ask_pete_ai/src/architect.rs)):

- Lore context injection ("Train Yard" metaphor)
- JSON schema enforcement (lines 91-110)
- Literary device templates

**Recent Fix:**

- AI was generating Rust-like strings: `"GreaterThan { variable: 'Strength', value: 10.0 }"`
- Updated prompt to generate proper JSON: `{"GreaterThan": {"variable": "Strength", "value": 10.0}}`

### The Weigh Station (Cognitive Load)

**Design Philosophy:**

- Each vocabulary word has a "weight" (cognitive complexity)
- Nodes calculate total load based on words used
- System prevents overload (safety lockout)

---

## Student Experience Preview (5 minutes)

### From Blueprint to Gameplay

1. **Teacher** creates blueprint (what we just did)
2. **Student** loads quest in Player App (port 8080)
3. **Player** navigates non-linear story graph
4. **Physics Engine** tracks "Steam" (mastery) and "Coal" (effort)

### Demo Visuals

**Show:**

- Quest start endpoint (`POST /api/quest/start/:id`)
- Physics state tracking (velocity, steam accumulation)
- Quest completion with Steam rewards

**Run test script** (if time permits):

```powershell
.\test_mvp_game_loop.ps1
```

---

## Roadmap & Future Vision (10 minutes)

### Current State: MVP

- ✅ AI blueprint generation (Gemini)
- ✅ Non-linear graph editing
- ✅ Database persistence
- ✅ Quest system basics

### Short-Term (Next 3 Months)

- 🚧 Real-time collaborative editing (multiplayer authoring)
- 🚧 Local LLM integration (Candle + GPU acceleration)
- 🚧 Advanced Weigh Station analytics
- 🚧 Export to LMS formats (SCORM, xAPI)

### Long-Term Vision

- 🌍 "Phygital" Node Gardens (geolocation-based learning)
- 🎮 MMO Party System (class roles: Engineer, Conductor, Stoker)
- 🧠 RAG-powered content recommendations
- 📊 Research dashboard for learning analytics

---

## Q&A (15 minutes)

### Anticipated Questions

**Q: "Why the Train Yard metaphor?"**  
A: Cognitive Load Theory is abstract. Trains have limited capacity (like working memory), cargo has weight (like cognitive complexity). The metaphor makes invisible mental processes tangible.

**Q: "Why Rust instead of Python/JavaScript?"**  
A: Type safety prevents a large class of bugs. WASM enables near-native performance in browser. Shared types between frontend/backend eliminate API mismatches.

**Q: "What about accessibility?"**  
A: WASM supports screen readers. We're exploring text-based "Telegraph" mode for vision-impaired users.

**Q: "Can I self-host?"**  
A: Yes! Apache 2.0 licensed. Runs on Docker. PostgreSQL + Rust server + WASM frontend.

**Q: "Data privacy with AI?"**  
A: Current MVP uses Gemini (cloud), but architecture supports local LLM (Candle). Student data stays local, only anonymized telemetry goes cloud.

---

## Closing (5 minutes)

### Call to Action

- **Try it**: Open source on GitHub
- **Contribute**: Issues, PRs welcome (Apache 2.0)
- **Research**: Interested in learning analytics? Datasets available (anonymized)

### Contact

- **GitHub**: [Ask_Pete Repository](https://github.com/Joshua42atkinson/Ask_Pete)
- **Documentation**: See `ENGINEERING_BIBLE.md` for architecture details
- **Demo**: This walkthrough + recorded video

---

## Demo Notes

**Pre-Demo Checklist:**

- [ ] PostgreSQL running
- [ ] Server started (`cargo run -p ask_pete_server`)
- [ ] Teacher App loaded (`trunk serve` in `ask_pete_tools`)
- [ ] Browser open to `http://localhost:8082`
- [ ] Test script verified passing

**Backup Plan (if live demo fails):**

- Show pre-recorded video of blueprint generation
- Walk through code in `architect.rs`
- Show test script output logs

**Time Allocation:**

- Opening: 5 min
- Live Demo: 15 min
- Technical Deep Dive: 10 min
- Student Preview: 5 min
- Roadmap: 10 min
- Q&A: 15 min
- **Total**: 60 minutes
