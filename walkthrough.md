# Game Loop Verification Walkthrough

## Goal

Verify the core "Ask Pete" game loop:

1. **Teacher/Author**: Generate a Blueprint (StoryGraph) from a prompt.
2. **Student/Player**: Start the Quest generated from the Blueprint.
3. **Physics Engine**: Verify "Steam" (Kinetic Energy) accumulates as the student progresses.

## Test Log

### 1. Unit Tests

- [ ] `cargo test --workspace`

### 2. Integration Tests (Simulated)

- [ ] **Blueprint Generation**: Send POST to `/api/architect/blueprint`
- [ ] **Quest Start**: Send POST to `/api/quest/start/:id` (or equivalent)
- [ ] **Telemetry**: Check `/api/weigh_station` or logs for Steam updates.

## Results

*(To be populated)*
