---
domain: Memory Management & Low-Level Systems
category: systems-programming
verification_type: execution
dataset_scale: 10K+ (from systems programming courses + CVE databases)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Memory Management & Low-Level Systems

## Overview
Tasks involving memory layout, pointer manipulation, unsafe code verification, memory leak detection, use-after-free identification, and low-level optimization. Verification: compile and run with sanitizers (ASan, MSan, TSan, UBSan), valgrind, or formal tools (KLEE).

## Verification Mechanism
```python
def verify(task_type: str, code: str, test_cases: list = None) -> float:
    if task_type == "fix_memory_bug":
        # Compile with sanitizers
        compile_result = run_gcc(code, flags=["-fsanitize=address,undefined", "-g"])
        if not compile_result.success:
            return 0.0
        
        # Run tests — must pass WITHOUT sanitizer errors
        for test in test_cases:
            result = run_with_timeout(compile_result.binary, test.input, timeout=30)
            if result.sanitizer_error:
                return 0.0
            if result.output != test.expected_output:
                return 0.0
        return 1.0
    
    elif task_type == "identify_bug":
        # Agent must identify the correct bug type and location
        buggy_code = code
        result = run_with_sanitizer(buggy_code, test_cases)
        agent_diagnosis = parse_agent_diagnosis(agent_response)
        return 1.0 if diagnosis_matches(agent_diagnosis, result.actual_bugs) else 0.0
    
    elif task_type == "optimize_memory":
        # Correctness + memory usage reduction
        original_mem = measure_peak_memory(original_code, test_input)
        optimized_mem = measure_peak_memory(code, test_input)
        
        correct = all(run_test(code, t).output == t.expected for t in test_cases)
        if not correct:
            return 0.0
        
        return min(1.0, original_mem / optimized_mem)
```

## Dataset Sources
- **CVE database**: Thousands of memory bugs with patches.
- **OSS-Fuzz bugs**: Publicly disclosed bugs found by fuzzing.
- **CWE examples**: Common Weakness Enumeration with code examples.
- **Juliet Test Suite**: NIST test suite for security vulnerabilities.
- **SystemsProgramming courses**: MIT 6.828, CMU 15-213 labs.
- **Rust vs C comparisons**: Unsafe C code rewritten in safe Rust.

## Task Format
- **Input**: "This C code has a use-after-free bug. Find and fix it." + code
- **Output**: Fixed code that passes sanitizer checks

## Difficulty Curriculum
- Level 1: Buffer overflow in simple array access
- Level 3: Use-after-free, double-free
- Level 5: Race conditions (with TSan), complex aliasing
- Level 7: Heap exploitation scenarios
- Level 9: Kernel-level memory management, custom allocators

## Limitations & Risks
- Sanitizer coverage is good but not exhaustive. Some bugs may slip through.
- Formal verification (KLEE, Frama-C) provides stronger guarantees but is slower.
- Combining sanitizers + test suite + formal analysis gives very strong verification.

## Connections
- [[code-repair]] — bug fixing
- [[cybersecurity-ctf]] — exploitation of memory bugs
- [[code-optimization]] — memory optimization
