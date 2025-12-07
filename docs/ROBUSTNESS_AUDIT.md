# Ask Pete Robustness Audit

## Frontend Robustness Issues

### 1. Unsafe `unwrap()` Usage

The following locations use `unwrap()` which can cause the application to panic (crash) if the value is `None` or `Err`. These should be replaced with proper error handling (e.g., `match`, `if let`, or `unwrap_or`).

- **`src/pages/play_mode.rs`**: `window().history().unwrap().back().unwrap()`
  - **Risk**: High. If the browser history API is unavailable or empty, the app will crash when trying to go back.
  - **Fix**: Check if `history()` returns `Ok` and if `back()` succeeds.

- **`src/pages/ask_pete.rs`**: `engine.get().unwrap()`
  - **Risk**: Medium. If the engine signal is not initialized, this will panic.
  - **Fix**: Use `if let Some(engine) = engine.get()`.

- **`src/components/weigh_station.rs`**: `ev.target().unwrap().unchecked_into::<...>()`
  - **Risk**: Low. `ev.target()` is usually present in event handlers, but safer to check.

- **`src/components/reflection_form.rs`**: `save_reflection_action.value().get().unwrap()`
  - **Risk**: Medium. If the action hasn't run yet, this might panic depending on `Action` state.

- **`src/components/authoring/property_editor.rs`**: `doc.get_element_by_id(...).unwrap()`
  - **Risk**: High. If the DOM element with that ID is missing (e.g., due to a typo or conditional rendering), the app crashes.
  - **Fix**: Handle the `None` case gracefully (e.g., log error, skip operation).

- **`src/ai/tokenizer/encoder.rs`**: `GemmaTokenizer::from_embedded().unwrap()` and `tokenizer.encode(text).unwrap()`
  - **Risk**: Critical. If the embedded model is missing or encoding fails, the AI feature breaks completely.
  - **Fix**: Propagate errors using `Result` and show a UI error message.

### 2. Missing Implementations (TODOs)

- **`src/components/model_manager.rs`**: Hook up `pete_core::ai::gemma_client` download logic.
- **`src/components/authoring/train_yard.rs`**: Verify API endpoint (`/api/architect/blueprint` vs `/api/train_yard/dispatch`).
- **`src/components/authoring/node_canvas.rs`**: Save "Word Smithy" definitions to backend.

### 3. Error Handling Gaps

- **Async Blocks**: In `node_canvas.rs` and `train_yard.rs`, errors in `spawn_local` are logged to console but might not always show a user-visible toast or alert.
- **API Failures**: If the backend is down, `TrainYard` logs to console. It should probably show a "Network Error" toast.

## Recommendations

1. **Refactor `unwrap()`**: Systematically replace `unwrap()` with `if let` or `match` blocks, especially for DOM access and AI operations.
2. **Global Error Boundary**: Ensure a global error boundary catches panics to prevent the "White Screen of Death".
3. **User Feedback**: Add Toast notifications for all async failures (Network, Save, Generate).
