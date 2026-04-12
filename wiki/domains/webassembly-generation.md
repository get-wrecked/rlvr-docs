---
domain: WebAssembly Generation
category: Code & Software
verification_type: execution
dataset_scale: ~1K+ from WasmBench + PolyBenchC compiled
difficulty_range: medium/hard/superhuman
modality: code
status: strong_hypothesis
---

# WebAssembly Generation

## Overview
Generate WASM code or compile-to-WASM source (Rust/C/AssemblyScript). Verification by running in WASM runtime (Wasmtime, Wasmer) with test inputs.

## Verification Mechanism
1. Validate WASM binary with `wasm-validate`
2. Execute in Wasmtime/Wasmer with test inputs
3. Compare outputs to expected results
4. Optionally measure performance against reference

## Dataset Sources
- WasmBench
- PolyBenchC compiled to WASM
- WebAssembly spec test suite
- Emscripten test suite
- AssemblyScript examples

## Task Format
**Input**: Function specification + target (WAT text, or Rust/C source for WASM)
**Output**: WASM binary or source that compiles to WASM

## Difficulty Curriculum
1. Simple arithmetic functions in WAT
2. Memory management (linear memory)
3. WASI system calls
4. Component model / interface types
5. Performance-optimized WASM (SIMD)

## Connections
[[assembly-generation]], [[code-generation]], [[code-optimization]]
