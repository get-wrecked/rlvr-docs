---
domain: Concurrency & Parallel Programming
category: systems-programming
verification_type: execution
dataset_scale: 10K+ (from concurrency courses + benchmarks)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Concurrency & Parallel Programming

## Overview
Write concurrent programs that are correct (no data races, deadlocks, livelocks) and efficient (good parallelism). Verification: run with thread sanitizer (TSan), model checker (CHESS, CDSChecker), or stress test for race conditions.

## Verification Mechanism
```python
def verify(code: str, tests: list[dict], concurrency_level: int = 10) -> float:
    # Compile with thread sanitizer
    binary = compile_with_tsan(code)
    if binary is None:
        return 0.0
    
    # Run tests multiple times to expose races
    race_detected = False
    all_correct = True
    
    for test in tests:
        for _ in range(100):  # Repeat to expose non-deterministic bugs
            result = run_with_timeout(binary, test["input"], 
                                     threads=concurrency_level, timeout=30)
            if result.tsan_error:
                race_detected = True
            if result.output != test["expected_output"]:
                all_correct = False
    
    if race_detected:
        return 0.0  # Data race = fail
    if not all_correct:
        return 0.3  # No races but wrong output
    return 1.0
```

## Dataset Sources
- **Concurrency courses**: MIT 6.005, CMU 15-213 labs.
- **Go concurrency exercises**: Go Tour concurrency section, Exercism Go track.
- **Java Concurrency in Practice**: Textbook exercises.
- **CHESS benchmarks**: Concurrency testing benchmarks.
- **Procedural generation**: Generate producer-consumer, reader-writer, dining philosophers variants.
- **Erlang/Elixir exercises**: Actor-model concurrency challenges.

## Task Format
- **Input**: "Implement a thread-safe bounded buffer supporting multiple producers and consumers in Java/Go/Rust"
- **Output**: Concurrent code that passes TSan + stress tests

## Difficulty Curriculum
- Level 1: Simple mutex usage, thread-safe counter
- Level 3: Producer-consumer, reader-writer locks
- Level 5: Lock-free data structures, condition variables
- Level 7: Complex concurrent algorithms (concurrent skip list)
- Level 9: Wait-free algorithms, formal verification of concurrent code

## Connections
- [[memory-management]] — shared memory concurrency
- [[distributed-systems]] — distributed concurrency
- [[code-generation]] — generating concurrent code
- [[formal-verification-software]] — verifying concurrent programs
