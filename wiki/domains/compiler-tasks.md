---
domain: Compiler Tasks
category: Code & Software
verification_type: execution-based (compile + test equivalence, or output match)
dataset_scale: 1K–1M programs
difficulty_range: peephole optimizations → full decompilation of complex binaries
modality: code-to-code (IR/assembly/source transformations)
status: emerging
---

## Overview

Compiler tasks encompass a family of RLVR problems centered on program transformations that compilers perform: optimization pass selection, decompilation (binary → source), disassembly, superoptimization, and compiler flag selection. In every case, verification is execution-based: compile and run the transformed code, then check that it produces the same outputs as the original (semantic equivalence) and, for optimization tasks, runs faster or produces smaller binaries.

This domain is compelling for RLVR because (a) the verifier is the compiler and test suite combined — both deterministic, (b) the search space of valid transformations is enormous and poorly explored by fixed heuristics, and (c) the domain directly improves software infrastructure.

## Verification Mechanism

### Decompilation (binary → source)

```
def verify_decompilation(binary, decompiled_source, test_suite,
                         target_language="C"):
    # 1. Compile the decompiled source back to a new binary
    try:
        recompiled = compile_in_sandbox(
            source=decompiled_source,
            language=target_language,
            flags=["-O0"],  # avoid optimizer hiding bugs
            timeout=60
        )
    except CompilationError:
        return 0.0  # decompiled code doesn't compile

    # 2. Run test suite against ORIGINAL binary
    original_outputs = []
    for test in test_suite:
        out = run_in_sandbox(binary, stdin=test.input, timeout=10)
        original_outputs.append(out.stdout)

    # 3. Run same test suite against RECOMPILED binary
    recompiled_outputs = []
    for test in test_suite:
        out = run_in_sandbox(recompiled, stdin=test.input, timeout=10)
        recompiled_outputs.append(out.stdout)

    # 4. Compare outputs
    if all(o == r for o, r in zip(original_outputs, recompiled_outputs)):
        return 1.0
    return 0.0
```

### Optimization pass selection

```
def verify_optimization(source_code, model_flags, test_suite,
                        baseline_flags="-O0"):
    # 1. Compile with baseline flags
    baseline_bin = compile(source_code, flags=baseline_flags)

    # 2. Compile with model-selected flags
    try:
        optimized_bin = compile(source_code, flags=model_flags)
    except CompilationError:
        return 0.0  # invalid flag combination

    # 3. Correctness: outputs must match on all tests
    for test in test_suite:
        base_out = run(baseline_bin, test.input)
        opt_out  = run(optimized_bin, test.input)
        if base_out != opt_out:
            return 0.0  # optimization broke semantics

    # 4. Performance: measure speedup
    base_time = benchmark(baseline_bin, perf_input, n_runs=5)
    opt_time  = benchmark(optimized_bin, perf_input, n_runs=5)
    speedup = base_time / opt_time

    # 5. Binary size (optional)
    size_ratio = file_size(optimized_bin) / file_size(baseline_bin)

    # Combined reward
    return compute_reward(speedup, size_ratio, correctness=True)
```

### Superoptimization (instruction-level)

```
def verify_superopt(original_asm, optimized_asm, test_vectors):
    """
    Verify that optimized assembly produces identical outputs
    for all test input vectors.
    """
    for inputs in test_vectors:
        orig_state = emulate(original_asm, inputs)
        opt_state  = emulate(optimized_asm, inputs)
        if orig_state.registers != opt_state.registers:
            return 0.0
        if orig_state.memory != opt_state.memory:
            return 0.0
    # Optional: check instruction count reduction
    orig_count = count_instructions(original_asm)
    opt_count  = count_instructions(optimized_asm)
    if opt_count >= orig_count:
        return 0.0  # no improvement
    return 1.0
```

## Dataset Sources

| Dataset | Size | Task | Notes |
|---------|------|------|-------|
| **AnghaBench** | 1,042,466 C functions | Decompilation / compilation | Largest single-source C benchmark; extracted from GitHub |
| **ExeBench** | 4.3M functions | Decompilation | C functions with I/O examples for verification |
| **Decompilation benchmarks (Ghidra/IDA)** | Varies | Decompilation | Compare neural decompilation vs. Ghidra/IDA output |
| **LLVM opt benchmarks** | ~10K programs | Pass ordering | LLVM test suite + SPEC CPU benchmarks |
| **CompilerGym** | 1M+ programs | Compiler optimization | OpenAI/Facebook RL environment for LLVM pass ordering |
| **BinaryBench** | 3,195 binaries | Reverse engineering | Binary analysis benchmark with multiple tasks |
| **PROUST** | 105 programs | Superoptimization | Small but canonical x86 superoptimization benchmark |
| **Stoke benchmarks** | ~100 programs | Superoptimization | Stanford stochastic superoptimizer test suite |
| **cBench / MiBench** | 32 / 35 programs | Compiler optimization | Embedded systems benchmark suites |

**Synthetic scaling**: Compile any C/C++ code from GitHub at various optimization levels. The source ↔ binary pair with I/O tests creates a decompilation training instance. AnghaBench alone provides over 1M functions.

## Task Format

**Decompilation example**:

**Input prompt**:
```
Decompile the following x86-64 assembly into readable C code:

    push   rbp
    mov    rbp, rsp
    mov    DWORD PTR [rbp-4], edi
    mov    eax, DWORD PTR [rbp-4]
    imul   eax, DWORD PTR [rbp-4]
    pop    rbp
    ret
```

**Expected output**:
```c
int square(int x) {
    return x * x;
}
```

**Verification**: Compile the C code, run against test inputs, compare with original binary outputs.

**Optimization pass ordering example**:

**Input**: C source code + available LLVM passes.

**Expected output**: Ordered list of passes, e.g., `-mem2reg -instcombine -simplifycfg -gvn -licm`

**Verification**: Compile with selected passes, check correctness, measure performance improvement.

## Difficulty Curriculum

| Level | Task | Complexity |
|-------|------|-----------|
| 1 | Decompile simple arithmetic functions | 5–10 instructions |
| 2 | Decompile with control flow (if/else, loops) | 20–50 instructions |
| 3 | Decompile with data structures and pointers | 50–200 instructions |
| 4 | Select optimization passes for small programs | Single-file C programs |
| 5 | Decompile stripped binaries (no symbols) | Requires type/name recovery |
| 6 | Full binary reverse engineering | Multi-function, struct recovery, vtable reconstruction |

## Limitations & Risks

- **Semantic equivalence is hard**: For complex programs, test suites may not cover all behaviors. Formal verification (SMT solvers) is more rigorous but extremely expensive. Test-based verification is practical but has coverage gaps.
- **Non-determinism**: Programs with undefined behavior in C/C++ may behave differently when recompiled. Task selection should avoid UB-heavy code.
- **Toolchain dependency**: Results depend heavily on the specific compiler version, target architecture, and optimization level. The verification environment must be precisely specified.
- **Decompilation readability**: A model might produce correct but unreadable C code (e.g., single-letter variables, no structure). Readability metrics can supplement correctness verification.
- **Superoptimization scale**: Superoptimization verification via emulation is exact for tested inputs but not a proof of equivalence. Bounded verification (check all inputs up to bit-width N) provides stronger guarantees for small programs.

## Connections

- **Code Translation** is conceptually similar: decompilation is "translating" from assembly to C.
- **Code Optimization** at the source level is the high-level counterpart to compiler optimization passes.
- **Code Generation** shares the test-based verification infrastructure.
- CompilerGym provides an existing RL environment that could be adapted for LLM-based RLVR.
- **Build Configuration** interacts with compiler flags and settings.
