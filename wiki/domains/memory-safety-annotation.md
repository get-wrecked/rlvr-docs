---
domain: Memory Safety Annotation
category: Code & Software
verification_type: execution
dataset_scale: unlimited (from c2rust output + unsafe Rust code)
difficulty_range: medium/hard/superhuman
modality: code
status: strong_hypothesis
---

# Memory Safety Annotation

## Overview
Given unsafe Rust code or C/C++ code, add lifetime annotations, borrow checker fixes, or smart pointer conversions to achieve memory safety. The Rust compiler itself is the verifier.

## Verification Mechanism
1. Code compiles with `rustc` (borrow checker passes)
2. Miri detects no undefined behavior
3. AddressSanitizer/MemorySanitizer clean (for C/C++)
4. Functional equivalence verified via test execution

## Dataset Sources
- c2rust translation outputs (need manual lifetime annotation)
- CRustS dataset (C to safe Rust)
- Rust compiler error messages dataset
- Rust clippy lint examples
- unsafe-code-analysis benchmarks

## Task Format
**Input**: Unsafe Rust code or C code
**Output**: Safe Rust code with proper lifetime annotations and ownership

## Difficulty Curriculum
1. Simple ownership fixes (clone vs. borrow)
2. Lifetime annotations on function signatures
3. Complex lifetime relationships (self-referential structs)
4. Interior mutability patterns (RefCell, Mutex)
5. Lock-free concurrent data structures with safe APIs

## Connections
[[memory-management]], [[code-translation]], [[formal-verification-software]]
