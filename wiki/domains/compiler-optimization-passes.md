---
domain: Compiler Optimization Pass Ordering
category: code-optimization
verification_type: execution
dataset_scale: unlimited (from any codebase)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Compiler Optimization Pass Ordering

## Overview
Given a program and a set of compiler optimization passes, determine the optimal ordering of passes to minimize code size, execution time, or energy consumption. The "phase ordering problem" is a long-standing challenge in compiler research. Verification is simple: compile with the proposed pass ordering, measure the result.

## Verification Mechanism
```python
def verify(source_code: str, pass_ordering: list[str], metric: str = "runtime") -> float:
    # Compile with proposed ordering
    binary = compile_with_passes(source_code, pass_ordering, compiler="llvm")
    if binary is None:  # compilation failed
        return 0.0
    
    # Compile with default O2 as baseline
    baseline = compile_with_passes(source_code, default_O2_passes, compiler="llvm")
    
    if metric == "runtime":
        proposed_time = benchmark(binary, iterations=10)
        baseline_time = benchmark(baseline, iterations=10)
        # Reward = improvement ratio (>1 = better than baseline)
        return min(1.0, baseline_time / proposed_time)
    elif metric == "size":
        return min(1.0, os.path.getsize(baseline) / os.path.getsize(binary))
```

## Dataset Sources
- **LLVM test suite**: Standard benchmark programs.
- **SPEC CPU**: Industry-standard benchmarks.
- **Polybench**: Polyhedral optimization benchmarks.
- **cBench/MiBench**: Embedded systems benchmarks.
- **Any open-source codebase**: Extract functions, use as optimization targets.

## Task Format
- **Input**: LLVM IR (or C/C++ source) + list of available passes + optimization target (speed/size)
- **Output**: Ordered sequence of passes to apply

## Difficulty Curriculum
- Level 1: Choose between 3-5 passes for small functions
- Level 5: Order 20+ passes for moderate programs
- Level 9: Full LLVM pass pipeline optimization for complex programs

## Limitations & Risks
- Benchmarking is noisy (variance in runtime). Need multiple runs and statistical testing.
- Pass interactions are complex — some orderings may crash the compiler. Need robustness to compilation failures.
- Transferability: optimal ordering is often program-specific.

## Connections
- [[code-optimization]] — higher-level code optimization
- [[compiler-tasks]] — broader compiler tasks
- [[combinatorics-optimization]] — this is a permutation optimization problem
