# Task: Ask Pete MVP Finalization

## Phase 1: MVP Core Loop (Current Focus)

- [x] **Fix Blueprint Generation** (Critical for Game Loop) <!-- id: 4 -->
  - [x] Fix Weigh Station AI Mock (Resolved JSON error)
  - [x] Update `architect.rs` prompt for strict JSON
  - [x] Improve `extract_json` parser (Implemented `json_utils`)
- [x] **Integrate Iron Split** (From Session Handoff)
  - [x] Initialize `IronSplitSystem` in `main.rs`
  - [x] Replace mock AI in `SocraticEngine`
- [ ] **Optimize AI Performance**
  - [x] Investigate slowness (CPU vs GPU)
  - [ ] Explore optimization options (quantization, threads)
- [x] **Create Desktop Launcher**
  - [x] Create a script/shortcut for easy testing
- [ ] **Fix Frontend (train_yard.rs)** (Blocking UI)
  - [ ] Diagnose type mismatch error (Could not reproduce, waiting for user input)
  - [ ] Apply fix
- [x] **Audit App Robustness**
  - [x] Identify weak points in Player UI (See ROBUSTNESS_AUDIT.md)
  - [x] Identify weak points in Error Handling (See ROBUSTNESS_AUDIT.md)
- [/] **Test Game Loop** <!-- id: 10 -->
  - [ ] Run Unit Tests (`cargo test`)
  - [ ] Generate Blueprint (Teacher)
  - [ ] Start Quest (Student)
  - [ ] Verify "Steam" accumulation

## Phase 2: Infrastructure & Alignment

- [ ] Create `GPU_INTEGRATION.md` (Candle/Metal/CUDA recommendations) <!-- id: 11 -->
- [ ] Update `Dockerfile` for React + Rust hybrid build <!-- id: 1 -->
- [ ] Implement RAG (Chunking & Embedding) in `knowledge.rs` <!-- id: 3 -->

## Phase 3: Hybrid Cloud & Auth

- [ ] Implement Google Auth with real JWTs <!-- id: 7 -->
- [ ] Configure Cloud Run deployment <!-- id: 8 -->
- [ ] Add Mentorship Flags & Group Logic <!-- id: 9 -->
