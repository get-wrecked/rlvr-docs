---
domain: Code Translation
category: Code & Software
verification_type: execution-based (translated code passes original test suite)
dataset_scale: 1K–50K translation pairs
difficulty_range: simple scripts → complex library ports
modality: code-to-code (cross-language)
status: emerging
---

## Overview

Code translation tasks the model with converting source code from one programming language to another while preserving functional behavior. The RLVR verification is elegant: if the original code has a test suite, run that same test suite (adapted for the target language's test framework) against the translated code. If all tests pass, the translation is correct.

This domain is particularly well-suited for RLVR because (a) the verification is execution-based and binary, (b) the problem is underconstrained enough that RL can explore diverse translation strategies, and (c) the practical value is high — organizations regularly need to migrate codebases between languages.

## Verification Mechanism

```
def verify_translation(source_code, source_lang, target_lang,
                       translated_code, test_suite):
    # 1. Adapt test suite to target language (pre-computed or templated)
    #    For I/O-based tests, no adaptation needed
    target_tests = adapt_tests(test_suite, source_lang, target_lang)

    # 2. Write translated code to project
    write_file(tmp_dir / target_filename(target_lang), translated_code)

    # 3. Build if needed (C++, Java, Rust, Go)
    if needs_compilation(target_lang):
        build = run_in_sandbox(
            command=compile_command(target_lang, target_filename(target_lang)),
            timeout=60
        )
        if build.returncode != 0:
            return 0.0  # compilation failure

    # 4. Run tests
    results = []
    for test in target_tests:
        try:
            outcome = run_in_sandbox(
                command=run_command(target_lang, test),
                timeout=15
            )
            passed = check_output(outcome, test.expected)
        except (TimeoutError, RuntimeError):
            passed = False
        results.append(passed)

    # 5. Strict: all tests pass
    return 1.0 if all(results) else 0.0
```

Additional verification layers:

- **I/O equivalence**: For competitive-programming-style problems, tests are stdin/stdout pairs — completely language-agnostic. This is the simplest and most reliable approach.
- **Function-level equivalence**: For function-level translation, wrap both original and translated functions in a test harness that feeds the same inputs and compares outputs.
- **Property-based cross-checking**: Generate random inputs, run both original and translated code, compare outputs. This catches edge cases that fixed test suites miss.

```
def cross_validate(source_code, source_lang, translated_code, target_lang,
                   input_generator, n_trials=1000):
    for _ in range(n_trials):
        inp = input_generator.generate()
        source_out = run_in_sandbox(source_lang, source_code, inp)
        target_out = run_in_sandbox(target_lang, translated_code, inp)
        if source_out != target_out:
            return 0.0
    return 1.0
```

## Dataset Sources

| Dataset | Size | Language Pairs | Notes |
|---------|------|---------------|-------|
| **CodeTrans** | 11,800 pairs | Java ↔ C# | Parallel corpus from open-source projects |
| **TransCoder** | 852 functions | C++ ↔ Java ↔ Python | Facebook Research; competitive programming functions |
| **TransCoder-ST** | Same + self-training data | C++ ↔ Java ↔ Python | Extended with automated test generation |
| **Avatar** | 9,515 pairs | Java ↔ Python | Curated from GeeksforGeeks with test cases |
| **XLCoST** | 10,000+ snippets | 7 languages | C++, Java, Python, C#, JS, PHP, C — parallel corpus |
| **CodeNet** | 14M submissions | 50+ languages | IBM; same problems solved in many languages — mine parallel pairs |
| **HumanEval-X** | 164 × 5 languages | Python, C++, Java, JS, Go | Multilingual HumanEval with per-language tests |
| **MultiPL-E** | 164+ problems | 18 languages | HumanEval/MBPP translated to many languages |
| **MBXP** | 974 × 10+ languages | Multi-language | MBPP translated with execution-verified tests |

**Scaling strategy**: CodeNet is a goldmine — 14 million submissions to ~4,000 problems in 50+ languages. By identifying correct submissions in language A and language B for the same problem, one can construct massive parallel corpora with shared I/O-based test suites.

## Task Format

**Input prompt**:
```
Translate the following Python function to Rust. The translated
function must produce identical outputs for all inputs.

```python
def longest_common_subsequence(s1: str, s2: str) -> int:
    m, n = len(s1), len(s2)
    dp = [[0] * (n + 1) for _ in range(m + 1)]
    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if s1[i-1] == s2[j-1]:
                dp[i][j] = dp[i-1][j-1] + 1
            else:
                dp[i][j] = max(dp[i-1][j], dp[i][j-1])
    return dp[m][n]
```
```

**Expected output**: Equivalent Rust code.

**Verification**: Run shared I/O tests (or function-call tests adapted for Rust) against the translated code.

## Difficulty Curriculum

| Level | Complexity | Translation Challenge |
|-------|-----------|----------------------|
| 1 | Simple arithmetic/string functions | Direct syntax mapping |
| 2 | Standard data structures | Map idioms (Python list → Rust Vec) |
| 3 | Language-specific patterns | Generators → iterators, exceptions → Result types |
| 4 | Concurrency/async | Threading models differ fundamentally |
| 5 | Memory management | GC language → Rust ownership/borrowing |
| 6 | Framework-dependent code | Django → Spring; ecosystem knowledge required |

For RL curriculum, start with levels 1–2 using I/O-verified competitive programming problems and progressively introduce language-specific idiom challenges.

## Limitations & Risks

- **Test suite adaptation**: Translating the test harness itself between languages can introduce bugs. I/O-based tests avoid this entirely.
- **Idiomatic vs. literal translation**: A model might produce correct but non-idiomatic code (e.g., Python translated to Java but written in a Python style). Tests don't measure idiomaticity. Static analysis or style checkers can provide supplementary signals.
- **Standard library differences**: Some operations trivial in one language are complex in another (e.g., Python's `defaultdict` → C). The model must generate more code, and tests may not cover all edge cases of the reimplemented functionality.
- **Floating-point divergence**: Subtle numerical differences between language runtimes can cause false negatives. Use tolerance-based comparison.
- **Asymmetric difficulty**: Python → C++ is much harder than C++ → Python because of memory management, types, etc. Curriculum should account for this.

## Connections

- Builds on **Code Generation** infrastructure (same sandbox, same test execution).
- **Compiler Tasks** (decompilation) is a special case: "translate" from assembly/IR back to source.
- **Type Inference** is often needed as a subtask when translating from dynamically-typed to statically-typed languages.
- Cross-language **Code Repair** can be framed as: translate the fix pattern from one language to another.
- **Code Optimization** may benefit from translation: translate Python to C++ for performance, verified by test equivalence + benchmarking.
