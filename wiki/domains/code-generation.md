---
domain: Code Generation
category: Code & Software
verification_type: execution-based (unit test pass/fail)
dataset_scale: 10K–10M problems
difficulty_range: introductory → competitive-programming grand-master
modality: text-to-code
status: mature
---

## Overview

Code generation is the flagship RLVR domain: given a natural-language problem description and a function signature, the model must produce source code that satisfies a hidden test suite. Because the test suite is deterministic and machine-executable, the reward signal is fully verifiable with no human or LLM judge needed. This domain has the longest track record in RLVR research, beginning with early work on AlphaCode and continuing through DeepSeek-Coder-V2, CodeRL, and numerous open efforts.

The task is attractive for RL because (a) verification is cheap — run code, check tests; (b) difficulty scales smoothly from single-function string manipulation to multi-file algorithmic challenges; and (c) large, curated datasets already exist at multiple difficulty tiers.

## Verification Mechanism

The core verification loop:

```
def verify(problem, generated_code, test_suite):
    # 1. Write generated code to a temp file
    write_file(tmp_dir / "solution.py", generated_code)

    # 2. Execute each test case in a sandboxed environment
    results = []
    for test in test_suite:
        try:
            outcome = run_in_sandbox(
                command=f"python solution.py",
                stdin=test.input,
                timeout=10,          # hard wall-clock limit
                memory_limit="256MB"
            )
            passed = (outcome.stdout.strip() == test.expected_output.strip())
        except TimeoutError:
            passed = False
        except RuntimeError:
            passed = False
        results.append(passed)

    # 3. Reward: strict = all tests pass; partial = fraction
    strict_reward  = 1.0 if all(results) else 0.0
    partial_reward = sum(results) / len(results)
    return strict_reward  # most RLVR pipelines use strict
```

Key implementation concerns:

- **Sandboxing**: Code runs inside Docker containers, gVisor, or Firecracker microVMs. Network access is disabled. Filesystem is read-only except for a scratch directory.
- **Timeout calibration**: A generous timeout (e.g., 10x the reference solution's runtime) is set per problem to avoid penalizing correct but slow solutions unfairly, while still rejecting infinite loops.
- **Multi-language support**: The same problem can be verified for Python, C++, Java, Rust, etc. by swapping the compilation/execution step. Test cases are typically I/O-based (stdin/stdout) and therefore language-agnostic.
- **Determinism**: Floating-point comparison uses tolerance (1e-6). For non-deterministic problems (e.g., "print any valid answer"), a custom checker judges correctness.

## Dataset Sources

| Dataset | Size | Difficulty | Languages | Notes |
|---------|------|-----------|-----------|-------|
| **HumanEval** | 164 problems | Easy–Medium | Python | OpenAI's canonical benchmark; function-level with docstring tests |
| **HumanEval+** | 164 problems | Easy–Medium | Python | EvalPlus: 80x more tests per problem to reduce false positives |
| **MBPP** | 974 problems | Easy | Python | Mostly Basic Programming Problems; crowd-sourced |
| **APPS** | 10,000 problems | Intro → Competition | Python | Scraped from Codeforces, Kattis, etc. Three difficulty tiers |
| **CodeContests** | ~13,000 problems | Medium → Grandmaster | Multi-language | DeepMind's competitive programming dataset with extensive tests |
| **LiveCodeBench** | Continuously growing | Mixed | Multi-language | Rolling benchmark from recent contest problems to avoid contamination |
| **TACO** | 25,443 problems | Mixed | Python | Topics and difficulty-tagged competitive programming problems |
| **xCodeEval** | 5,600 problems | Mixed | 11 languages | Multilingual code generation with execution-based evaluation |

For RL training at scale, practitioners often mine Codeforces, LeetCode, AtCoder, and Project Euler problems programmatically and generate additional test cases via fuzzing or mutation to harden the test suites.

## Task Format

**Input prompt** (example):
```
Write a function `longest_palindrome(s: str) -> str` that returns
the longest palindromic substring in s.

Examples:
  longest_palindrome("babad") -> "bab"  (or "aba")
  longest_palindrome("cbbd")  -> "bb"
```

**Expected output**: A complete Python function body.

**Hidden test suite** (not shown to model):
```python
assert longest_palindrome("babad") in ["bab", "aba"]
assert longest_palindrome("cbbd") == "bb"
assert longest_palindrome("a") == "a"
assert longest_palindrome("") == ""
assert longest_palindrome("abcdefedcba") == "abcdefedcba"
# ... 50+ more edge cases
```

The model's output is spliced into a harness that imports the function and runs the assertions. Reward = 1 if all pass, 0 otherwise.

## Difficulty Curriculum

A natural curriculum exists thanks to competitive programming rating systems:

| Level | Description | Example | Approx. Codeforces Rating |
|-------|------------|---------|--------------------------|
| 1 | String/array basics | Reverse a string | < 800 |
| 2 | Simple algorithms | Two-sum, binary search | 800–1200 |
| 3 | Standard DP/graphs | Shortest path, knapsack | 1200–1600 |
| 4 | Advanced algorithms | Segment trees, flows | 1600–2100 |
| 5 | Competition hard | Combinatorics + optimization | 2100–2800 |
| 6 | Grandmaster | Multi-step reductions | > 2800 |

RL curricula typically start with levels 1–3 and progressively mix in harder problems as the policy improves, following a self-paced or threshold-gated schedule.

## Limitations & Risks

- **Test suite adequacy**: Weak test suites cause false positives — the model "passes" but produces incorrect code. EvalPlus showed HumanEval's original tests miss many bugs. Mitigations: generate extra tests via fuzzing, use property-based testing, include adversarial edge cases.
- **Reward hacking**: Models can exploit test oracles by hard-coding outputs, reading test files, or detecting test patterns. Mitigations: randomized test inputs, sandboxed execution, code-structure checks.
- **Benchmark contamination**: Popular benchmarks (HumanEval, MBPP) are almost certainly in pretraining data. LiveCodeBench addresses this with rolling fresh problems.
- **Execution cost**: Running thousands of sandboxed executions per RL step is expensive. Batching, caching, and lightweight sandboxes (Bubblewrap, nsjail) help.
- **Language bias**: Most datasets are Python-heavy. C++, Java, and Rust are underrepresented, leading to weaker RL signal for those languages.

## Connections

- Directly extends to **Code Repair** (same verification, different input framing).
- **Code Optimization** adds performance measurement on top of the same test-based verification.
- **SQL Generation** is an analogous domain with execution-based verification against databases.
- **Test Generation** is the dual problem: generate tests rather than code.
- Lessons from code generation RL (reward shaping, curriculum design) transfer to all other execution-verified domains.
