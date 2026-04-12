---
domain: Software Formal Verification
category: formal-methods-applied
verification_type: rule
dataset_scale: 10K+ (from verification benchmarks)
difficulty_range: hard/superhuman
modality: text
status: strong_hypothesis
---

# Software Formal Verification

## Overview
Write formal specifications and proofs that software satisfies its specification. Use Dafny, Frama-C, SPARK Ada, or Rust's type system to verify program properties: memory safety, functional correctness, termination, absence of runtime errors. Verification: the proof checker / verifier accepts.

## Verification Mechanism
```python
def verify(tool: str, code_with_annotations: str) -> float:
    if tool == "dafny":
        result = run_dafny(code_with_annotations, timeout=120)
        if result.all_verified:
            return 1.0
        return result.verified_count / result.total_obligations
    
    elif tool == "frama_c":
        result = run_frama_c_wp(code_with_annotations, timeout=120)
        return result.proved / result.total if result.total > 0 else 0.0
    
    elif tool == "klee":
        # Symbolic execution — check no assertion violations
        result = run_klee(code_with_annotations, timeout=300)
        return 1.0 if result.assertion_violations == 0 else 0.0
    
    elif tool == "rust_miri":
        # Check for undefined behavior in unsafe Rust
        result = run_miri(code_with_annotations, timeout=60)
        return 1.0 if result.no_ub else 0.0
```

## Dataset Sources
- **SV-COMP (Software Verification Competition)**: Annual benchmark with 10K+ verification tasks.
- **Dafny tutorials and exercises**: Verification challenges.
- **Frama-C tutorials**: ACSL-annotated C code.
- **SPARK Ada examples**: Formally verified Ada code.
- **Prusti examples**: Rust verification examples.
- **seL4 proofs**: Formally verified microkernel (Isabelle/HOL).
- **CompCert**: Verified C compiler (Coq).

## Task Format
- **Input**: "Add loop invariants and pre/post-conditions to this Dafny method so that it verifies: method BinarySearch(a: array<int>, key: int) returns (index: int) {...}"
- **Output**: Annotated code that passes Dafny verifier

## Difficulty Curriculum
- Level 3: Simple loop invariants, basic pre/post-conditions
- Level 5: Array manipulation with bounds checking
- Level 7: Data structure correctness (linked lists, trees)
- Level 9: Complex system properties (concurrent data structures)

## Connections
- [[math-formal-proofs]] — mathematical proof verification
- [[code-generation]] — generating verified code
- [[smt-solving]] — underlying verification technology
- [[distributed-systems]] — protocol verification
