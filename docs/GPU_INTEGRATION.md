# GPU Integration & "Close to Metal" Recommendations

## Overview

To transform the "Train Yard" into a high-performance Instructional Design IDE, we need to move beyond simple CPU inference and leverage the full power of the hardware. The user has requested a "close to metal" integration using `candle` (Rust's ML framework).

## 1. The Framework: Candle (by Hugging Face)

`candle` is the ideal choice for this project because:

- **Rust Native**: No Python overhead. Matches our backend stack.
- **Backends**: Supports CUDA (NVIDIA), Metal (Apple Silicon), and WASM (Browser).
- **Zero-Copy**: Efficient memory management.

## 2. GPU Acceleration Strategy

### A. NVIDIA (CUDA) - "The Heavy Lifter"

For students/teachers with gaming PCs or dedicated servers.

- **Requirement**: Install CUDA Toolkit 12.x.
- **Cargo Feature**: Enable `cuda` feature in `candle-core`.
- **Implementation**:

  ```rust
  // libs/infra-ai/src/local_engine.rs
  #[cfg(feature = "cuda")]
  let device = Device::new_cuda(0)?;
  ```

### B. Apple Silicon (Metal) - "The Creative Pro"

For users on MacBooks (common in design/education).

- **Requirement**: macOS 13+.
- **Cargo Feature**: Enable `metal` feature.
- **Implementation**:

  ```rust
  #[cfg(feature = "metal")]
  let device = Device::new_metal(0)?;
  ```

### C. Vulkan - "The Universal Soldier"

For AMD GPUs or cross-platform compatibility.

- **Status**: Candle support is experimental but growing.

## 3. Recommended Architecture for "Instructional Design IDE"

To support real-time blueprint generation and "Vibe Coding":

1. **Model**: **Mistral 7b Instruct v0.3** (Quantized to 4-bit).
    - *Why*: Best balance of reasoning and speed. 7B fits in 6GB VRAM.
2. **Quantization**: Use `gguf` format (supported by `candle-transformers`).
3. **Speculative Decoding**: Implement a small draft model (e.g., Gemma 2B) to "guess" tokens and the large model (Mistral 7b) to verify. This doubles inference speed.

## 4. Implementation Steps (Roadmap)

1. **Dependency Update**:
    Add `candle-core`, `candle-nn`, `candle-transformers` to `libs/infra-ai/Cargo.toml`.
2. **Feature Flags**:
    Update `Cargo.toml` to expose `cuda` and `metal` features so the build script can detect the hardware.
3. **Kernel Integration**:
    Write custom CUDA kernels (if needed) for specific "Instructional Logic" operations, though standard kernels usually suffice.

## 5. "Close to Metal" Optimization

To truly be "close to metal":

- **Pinned Memory**: Use pinned host memory for faster CPU->GPU transfers.
- **Flash Attention**: Enable Flash Attention v2 in Candle for long-context RAG (critical for reading entire textbooks).

## 6. Immediate Action

For the MVP, we will stick to **CPU (Quantized)** to ensure it runs everywhere, but we will structure the `LocalModel` struct to accept a `Device` trait, making the switch to GPU a simple config change.
