---
domain: Code Optimization
category: Code & Software
verification_type: execution-based (correctness tests + performance benchmarking)
dataset_scale: 1K–50K programs
difficulty_range: constant-factor speedups → algorithmic complexity improvements
modality: slow-code-to-fast-code
status: emerging
---

## Overview

Code optimization tasks the model with taking functionally correct but slow (or memory-hungry) code and producing an improved version that is both still correct and measurably faster or more memory-efficient. This is a two-part verification: (1) the optimized code must pass the full test suite (correctness preserved), and (2) it must demonstrate a quantifiable performance improvement on benchmark inputs.

This domain is a compelling RLVR target because the reward signal is rich — it combines a binary correctness gate with a continuous performance metric. RL can discover non-obvious optimization strategies (algorithmic improvements, cache-friendly access patterns, SIMD-friendly code) that supervised learning might not capture.

## Verification Mechanism

```
def verify_optimization(original_code, optimized_code, test_suite,
                        benchmark_inputs, language):
    # Phase 1: Correctness — optimized code must pass all tests
    for test in test_suite:
        try:
            result = run_in_sandbox(
                code=optimized_code, stdin=test.input,
                timeout=original_timeout * 2,  # generous timeout
                language=language
            )
            if result.stdout.strip() != test.expected_output.strip():
                return 0.0  # correctness failure → reward 0
        except (TimeoutError, RuntimeError):
            return 0.0

    # Phase 2: Performance measurement
    # Run original code on benchmark inputs
    original_times = []
    for inp in benchmark_inputs:
        t = measure_execution_time(original_code, inp, language, n_runs=5)
        original_times.append(median(t))

    # Run optimized code on same benchmark inputs
    optimized_times = []
    for inp in benchmark_inputs:
        t = measure_execution_time(optimized_code, inp, language, n_runs=5)
        optimized_times.append(median(t))

    # Phase 3: Compute reward
    speedup_ratios = [orig / opt for orig, opt in
                      zip(original_times, optimized_times)
                      if opt > 0]
    avg_speedup = geometric_mean(speedup_ratios)

    if avg_speedup < 1.05:
        return 0.0   # less than 5% improvement → no reward
    # Log-scaled reward: diminishing returns for extreme speedups
    reward = min(1.0, log2(avg_speedup) / log2(target_speedup))
    return reward
```

Additional measurement options:

```
def measure_memory(code, input_data, language):
    """Track peak RSS memory usage."""
    result = run_in_sandbox(
        code=code, stdin=input_data, language=language,
        measure_memory=True
    )
    return result.peak_memory_bytes

def verify_with_memory(original, optimized, test_suite, benchmark_inputs):
    # ... correctness check as above ...
    orig_mem = [measure_memory(original, inp) for inp in benchmark_inputs]
    opt_mem  = [measure_memory(optimized, inp) for inp in benchmark_inputs]
    mem_reduction = mean(orig / opt for orig, opt in zip(orig_mem, opt_mem))
    # Combine time and memory into composite reward
    return alpha * time_reward + (1 - alpha) * memory_reward
```

Key considerations:

- **Measurement noise**: CPU frequency scaling, caching, and OS scheduling cause variance. Mitigations: median of 5+ runs, pin CPU frequency, use performance counters (perf_event) instead of wall-clock time.
- **Benchmark input scaling**: Use multiple input sizes to ensure optimization works across scales (not just for small n). Include n=100, n=10K, n=1M.
- **Minimum improvement threshold**: Requiring at least 5–10% speedup avoids rewarding trivial or noise-level "improvements."

## Dataset Sources

| Dataset | Size | Languages | Notes |
|---------|------|-----------|-------|
| **PIE (Performance-Improving Edits)** | 77,967 pairs | C++ | Competitive programming submissions: slow → fast pairs for same problem |
| **CodeNet** | 14M submissions | 50+ languages | Multiple submissions per problem with different runtimes; mine slow→fast pairs |
| **APPS / CodeContests** | 10K–13K problems | Multi-language | Use slowest-passing and fastest-passing solutions as pairs |
| **Can LLMs Reason about Program Efficiency?** | 121 problems | Python | EfficientCode benchmark with efficiency annotations |
| **Mercury** | 1,889 problems | Python | Code efficiency benchmark with runtime percentile targets |
| **HPC-oriented datasets** | Varies | C/C++/Fortran | Loop optimization, vectorization, parallelization tasks |

**Synthetic data generation**: For any competitive programming problem with multiple correct submissions, rank by runtime and pair slow solutions with fast ones. CodeNet alone yields millions of such pairs across 50+ languages.

## Task Format

**Input prompt**:
```
Optimize the following Python function. It must produce identical
outputs but run significantly faster on large inputs.

Current runtime on n=100,000: ~4.2 seconds

```python
def two_sum_count(arr, target):
    count = 0
    for i in range(len(arr)):
        for j in range(i + 1, len(arr)):
            if arr[i] + arr[j] == target:
                count += 1
    return count
```
```

**Expected output**: An O(n) or O(n log n) solution using a hash map.

**Verification**: (1) same outputs on all test inputs, (2) measured runtime on n=100,000 drops from ~4.2s to <0.1s.

## Difficulty Curriculum

| Level | Optimization Type | Typical Speedup | Example |
|-------|------------------|----------------|---------|
| 1 | Minor cleanup | 1.1–1.5x | Remove redundant copies, use built-ins |
| 2 | Data structure swap | 2–10x | List → set for membership checks |
| 3 | Algorithm improvement | 10–100x | O(n^2) → O(n log n) sort-based |
| 4 | Algorithmic breakthrough | 100–1000x | O(n^2) → O(n) with hash map |
| 5 | Low-level optimization | 2–10x on top of good algorithm | Cache-friendly access, SIMD, bit tricks |
| 6 | Parallelization | Nx for N cores | Threading, GPU offload |

The RL curriculum should start with levels 1–3 where the model learns that correctness is non-negotiable, then advance to levels where algorithmic insight is required.

## Limitations & Risks

- **Measurement reliability**: Performance benchmarking is inherently noisy. Sandboxed environments may not reflect real hardware characteristics. Dedicated benchmark machines with pinned CPUs help.
- **Gaming the metric**: A model might "optimize" by caching test-case-specific results or exploiting patterns in benchmark inputs. Using randomized inputs and checking correctness on held-out tests mitigates this.
- **Correctness-performance tradeoff**: Some optimizations sacrifice numerical precision (e.g., fast math). The test suite must be designed to catch this.
- **Language-specific optimizations**: Python optimization often means "use NumPy" or "use C extensions," which is valid but may not exercise algorithmic reasoning. Multi-language setups (C++, Rust) force more genuine optimization.
- **Hardware dependence**: An optimization that helps on x86 might not help on ARM. Verification should ideally be hardware-specific or use architecture-independent metrics (instruction count).

## Connections

- Extends **Code Generation** with a performance dimension on top of the correctness gate.
- **Compiler Tasks** overlap: the model is essentially performing source-to-source compiler optimization.
- **Code Translation** to a faster language (Python → C++) is one optimization strategy.
- **Code Refactoring** shares the "preserve behavior, improve quality" structure but targets readability rather than performance.
- Performance measurement infrastructure can be reused for **Data Wrangling** (optimizing pandas pipelines).
