---
domain: Test Generation
category: Code & Software
verification_type: execution-based (tests pass on correct code + fail on mutants + coverage measurement)
dataset_scale: 1K–100K functions/classes
difficulty_range: simple function unit tests → integration tests with mocks and fixtures
modality: code-to-tests
status: growing
---

## Overview

Test generation is the dual of code generation: given source code (a function, class, or module), the model must produce a test suite that (a) passes on the given correct implementation, (b) fails on known buggy versions (mutants), and (c) achieves high code coverage. Verification is fully automatic through test execution and mutation testing.

This is a natural RLVR domain because the reward signal is multi-dimensional and entirely computable: run the tests, measure coverage, run against mutants, check results. The practical value is high — automated test generation directly improves software quality and developer productivity.

## Verification Mechanism

```
def verify_test_suite(source_code, generated_tests, mutants=None,
                      coverage_target=0.9):
    # Phase 1: Tests must pass on correct code
    test_file = write_file(tmp_dir / "test_solution.py", generated_tests)
    source_file = write_file(tmp_dir / "solution.py", source_code)

    result = run_in_sandbox(
        command="python -m pytest test_solution.py -v --tb=short",
        cwd=tmp_dir,
        timeout=60
    )
    if result.returncode != 0:
        return 0.0  # tests fail on correct code → invalid tests

    # Phase 2: Measure code coverage
    cov_result = run_in_sandbox(
        command="python -m pytest test_solution.py --cov=solution "
                "--cov-report=json",
        cwd=tmp_dir,
        timeout=60
    )
    coverage_data = json.loads(read_file(tmp_dir / "coverage.json"))
    line_coverage = coverage_data["totals"]["percent_covered"] / 100.0
    branch_coverage = coverage_data["totals"].get("branch_percent", 0) / 100.0

    # Phase 3: Mutation testing — tests must FAIL on buggy versions
    if mutants:
        mutants_killed = 0
        for mutant in mutants:
            write_file(tmp_dir / "solution.py", mutant.code)
            mut_result = run_in_sandbox(
                command="python -m pytest test_solution.py -x --tb=no",
                cwd=tmp_dir,
                timeout=30
            )
            if mut_result.returncode != 0:
                mutants_killed += 1  # test suite detected the bug
            # Restore original
            write_file(tmp_dir / "solution.py", source_code)

        mutation_score = mutants_killed / len(mutants)
    else:
        # Auto-generate mutants if not provided
        mutants_auto = generate_mutants(source_code, n=20)
        mutants_killed = 0
        for mutant in mutants_auto:
            write_file(tmp_dir / "solution.py", mutant)
            mut_result = run_in_sandbox(
                command="python -m pytest test_solution.py -x --tb=no",
                cwd=tmp_dir, timeout=30
            )
            if mut_result.returncode != 0:
                mutants_killed += 1
            write_file(tmp_dir / "solution.py", source_code)
        mutation_score = mutants_killed / len(mutants_auto)

    # Phase 4: Composite reward
    # All three dimensions contribute
    reward = (
        0.3 * (1.0 if line_coverage >= coverage_target else
               line_coverage / coverage_target) +
        0.3 * mutation_score +
        0.4 * (1.0)  # base reward for passing on correct code
    )
    return reward
```

Mutant generation:

```
def generate_mutants(source_code, n=20):
    """
    Apply standard mutation operators to create buggy versions.
    """
    operators = [
        replace_operator("+", "-"),
        replace_operator(">=", ">"),
        replace_operator("==", "!="),
        replace_comparison("<", "<="),
        negate_condition,
        delete_statement,
        change_constant(c, c + 1),
        change_constant(c, c - 1),
        swap_arguments,
        replace_return_with_none,
        remove_else_branch,
    ]
    mutants = []
    for _ in range(n):
        op = random.choice(operators)
        mutant = op.apply(parse_ast(source_code))
        if mutant != source_code:  # non-trivial mutation
            mutants.append(mutant)
    return mutants
```

Key considerations:

- **False positives**: Tests that pass on correct code but for the wrong reason (e.g., `assert True`). Mutation testing catches this — such tests will also pass on mutants.
- **Test quality vs. quantity**: More tests don't necessarily mean better tests. Mutation score is the key quality metric.
- **Deterministic tests**: Generated tests should be deterministic. Random test inputs need fixed seeds.
- **Test isolation**: Each test should be independent. Shared state between tests causes flaky behavior.

## Dataset Sources

| Dataset | Size | Language | Notes |
|---------|------|----------|-------|
| **HumanEval** (reversed) | 164 functions | Python | Given the function, generate tests (reverse of normal use) |
| **MBPP** (reversed) | 974 functions | Python | Same approach — function given, generate tests |
| **EvoSuite benchmarks** | 100+ Java classes | Java | Standard test generation benchmark for Java |
| **SF110** | 110 Java projects | Java | Real-world Java projects for test generation evaluation |
| **Defects4J** | 835 buggy versions | Java | Known bugs serve as natural mutants for evaluation |
| **Methods2Test** | 780K pairs | Java | Method-to-test pairs mined from GitHub |
| **TestPilot benchmark** | 25 npm packages | JavaScript | JavaScript test generation benchmark |
| **CodaMOSA benchmark** | 486 Python functions | Python | Search-based + LLM test generation benchmark |
| **ChatUniTest data** | 1,752 focal methods | Java | Unit test generation with ChatGPT |

**Synthetic data generation**: Any code repository with existing tests can be used in reverse: the code is the input, the tests are the target. Millions of (function, test) pairs can be mined from GitHub. For RL, the existing tests serve as one reference point but the model is free to generate different (better) tests.

## Task Format

**Input prompt**:
```
Generate a comprehensive test suite for the following function.
Tests should cover normal cases, edge cases, and error cases.
Aim for high code coverage and bug-detection ability.

```python
def binary_search(arr, target):
    left, right = 0, len(arr) - 1
    while left <= right:
        mid = (left + right) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    return -1
```
```

**Expected output**:
```python
import pytest
from solution import binary_search

def test_found_middle():
    assert binary_search([1, 3, 5, 7, 9], 5) == 2

def test_found_first():
    assert binary_search([1, 3, 5, 7, 9], 1) == 0

def test_found_last():
    assert binary_search([1, 3, 5, 7, 9], 9) == 4

def test_not_found():
    assert binary_search([1, 3, 5, 7, 9], 4) == -1

def test_empty_array():
    assert binary_search([], 5) == -1

def test_single_element_found():
    assert binary_search([5], 5) == 0

def test_single_element_not_found():
    assert binary_search([5], 3) == -1

def test_duplicates():
    result = binary_search([1, 3, 3, 3, 5], 3)
    assert result in [1, 2, 3]  # any valid index

def test_negative_numbers():
    assert binary_search([-5, -3, -1, 0, 2], -3) == 1

def test_large_array():
    arr = list(range(0, 10000, 2))
    assert binary_search(arr, 5000) == 2500
```

**Verification**: Tests pass on the function; tests fail on mutants (e.g., `<` instead of `<=`, `mid + 1` instead of `mid`); line coverage >= 100%.

## Difficulty Curriculum

| Level | Code Complexity | Test Challenge |
|-------|----------------|---------------|
| 1 | Pure functions, no branching | Basic input/output tests |
| 2 | Conditional logic, loops | Edge cases, boundary testing |
| 3 | Exception handling, multiple paths | Error cases, branch coverage |
| 4 | Classes with state | Setup/teardown, stateful testing |
| 5 | External dependencies | Mocking, dependency injection |
| 6 | Concurrent/async code | Race conditions, async test patterns |

## Limitations & Risks

- **Tautological tests**: `assert binary_search([1,2,3], 2) == binary_search([1,2,3], 2)` passes on correct code AND mutants. Mutation testing is essential to catch these.
- **Overfitting to implementation**: Tests that are too tightly coupled to the specific implementation (e.g., checking internal state) may fail after valid refactoring. Black-box tests are preferred.
- **Mutant equivalence**: Some mutations produce programs that are semantically identical to the original (equivalent mutants). These cannot be "killed" by any test. The mutation score is thus an underestimate.
- **Execution cost**: Running tests against 20+ mutants per training sample multiplies execution cost. Efficient mutant generation and early termination (-x flag for pytest) help.
- **Test framework conventions**: Generated tests must follow framework conventions (pytest, JUnit, Jest). Import paths, test naming, and assertion style must be correct.

## Connections

- The dual of **Code Generation**: there, tests verify code; here, code verifies tests.
- Directly supports **Code Repair** by providing stronger test suites for bug detection.
- **Code Refactoring** depends on high-quality tests: better generated tests enable safer refactoring.
- Mutation operators used in verification overlap with synthetic bug generation for **Code Repair**.
- Coverage measurement infrastructure is shared with **Code Generation** verification.
