---
domain: Code Refactoring
category: Code & Software
verification_type: execution-based (all tests pass + structural quality improvement metrics)
dataset_scale: 1K–50K refactoring tasks
difficulty_range: rename variables → extract microservices
modality: code-to-code (quality improvement)
status: emerging
---

## Overview

Code refactoring tasks the model with transforming source code to improve its structure, readability, or maintainability without changing its observable behavior. The RLVR verification has two components: (1) behavioral preservation — the refactored code must pass the entire test suite, and (2) structural improvement — the refactored code must demonstrably improve on measurable quality metrics (cyclomatic complexity, code duplication, line count, naming quality, etc.).

This domain is attractive for RLVR because (a) the behavioral gate is the familiar test-execution verifier, (b) structural metrics can be computed automatically by static analysis tools, and (c) the task is practically valuable and common in software engineering.

## Verification Mechanism

```
def verify_refactoring(original_code, refactored_code, test_suite,
                       quality_criteria):
    # Phase 1: Behavioral preservation — ALL tests must pass
    write_file(tmp_dir / "solution.py", refactored_code)
    result = run_in_sandbox(
        command="python -m pytest tests/ -v",
        cwd=tmp_dir,
        timeout=120
    )
    if result.returncode != 0:
        return 0.0  # refactoring broke something

    # Phase 2: Structural quality measurement
    # Compute metrics for BOTH original and refactored
    orig_metrics = compute_metrics(original_code)
    ref_metrics  = compute_metrics(refactored_code)

    improvements = []

    # Cyclomatic complexity
    if "reduce_complexity" in quality_criteria:
        cc_improvement = orig_metrics.cyclomatic - ref_metrics.cyclomatic
        improvements.append(cc_improvement > 0)

    # Code duplication
    if "reduce_duplication" in quality_criteria:
        dup_improvement = orig_metrics.duplication_ratio - ref_metrics.duplication_ratio
        improvements.append(dup_improvement > 0)

    # Function length
    if "reduce_function_length" in quality_criteria:
        avg_orig = mean(orig_metrics.function_lengths)
        avg_ref  = mean(ref_metrics.function_lengths)
        improvements.append(avg_ref < avg_orig)

    # Line count (if specified)
    if "reduce_lines" in quality_criteria:
        improvements.append(ref_metrics.total_lines < orig_metrics.total_lines)

    # Code style / linting
    if "improve_style" in quality_criteria:
        orig_lint = run_linter(original_code)   # pylint, flake8
        ref_lint  = run_linter(refactored_code)
        improvements.append(ref_lint.score > orig_lint.score)

    # Phase 3: Composite reward
    if not improvements:
        return 1.0  # tests pass, no specific criteria → base reward

    improvement_ratio = sum(improvements) / len(improvements)
    return improvement_ratio  # 0.0–1.0 based on criteria met


def compute_metrics(code):
    """Use static analysis to compute quality metrics."""
    tree = ast.parse(code)
    return Metrics(
        cyclomatic=compute_cyclomatic_complexity(tree),
        function_lengths=[len_func(f) for f in extract_functions(tree)],
        duplication_ratio=detect_clones(code),
        total_lines=len(code.splitlines()),
        num_functions=count_functions(tree),
        max_nesting_depth=compute_nesting_depth(tree),
        naming_score=evaluate_naming(tree)
    )
```

Specific refactoring type verification:

```
def verify_extract_function(original, refactored, test_suite):
    """Verify 'Extract Function' refactoring specifically."""
    # 1. Tests pass
    if not run_tests(refactored, test_suite):
        return 0.0

    # 2. New function exists that wasn't in original
    orig_funcs = set(extract_function_names(original))
    ref_funcs  = set(extract_function_names(refactored))
    new_funcs  = ref_funcs - orig_funcs

    if not new_funcs:
        return 0.0  # no function was actually extracted

    # 3. Original long function is shorter
    # (find the function that was refactored)
    for func_name in orig_funcs & ref_funcs:
        orig_len = get_function_length(original, func_name)
        ref_len  = get_function_length(refactored, func_name)
        if ref_len < orig_len:
            return 1.0  # successfully extracted and shortened

    return 0.0
```

Key considerations:

- **Behavioral equivalence is non-negotiable**: The test suite is the source of truth. Any test failure means the refactoring is rejected, regardless of quality improvements.
- **Metric gaming**: A model might minimize cyclomatic complexity by inlining everything into one giant function. Multiple complementary metrics prevent this.
- **Subjective quality**: Some refactoring quality is inherently subjective (naming, code organization). Linter scores and complexity metrics are imperfect proxies.
- **Test suite strength**: If the test suite is weak, a refactoring might break behavior in ways tests don't catch. Stronger test suites (from **Test Generation**) improve verification.

## Dataset Sources

| Dataset | Size | Language | Notes |
|---------|------|----------|-------|
| **RefactorBench** | Emerging | Multi-language | Academic benchmark for code refactoring |
| **Fowler's Refactoring Catalog** | 72 refactoring types | Conceptual | Martin Fowler's canonical catalog; can be operationalized |
| **GitHub refactoring commits** | 100K+ | Multi-language | Mine commits with messages containing "refactor" |
| **RefactoringMiner dataset** | 10,000+ | Java | Automatically detected refactorings from Git history |
| **R-CPatMiner** | 37K refactoring patterns | Java | Mined code change patterns from 100K commits |
| **Code smell datasets** | 1K–5K | Java/Python | Code with known smells: God Class, Long Method, Feature Envy |
| **Pylint/SonarQube outputs** | Scalable | Multi-language | Run static analysis on code to find improvement opportunities |
| **Technical debt datasets** | Varies | Multi-language | Code annotated with technical debt indicators |

**Synthetic data generation**:
1. Start with clean, well-refactored code from high-quality repositories.
2. Apply "anti-refactorings" — inline functions, duplicate code, introduce long methods, use poor variable names.
3. The degraded version is the input; the original is the reference (but the model need not match it exactly — just pass tests and improve metrics).

## Task Format

**Input prompt**:
```
Refactor the following code to reduce complexity and improve
readability. The code must maintain identical behavior — all
existing tests must continue to pass.

```python
def process_order(order):
    if order['type'] == 'standard':
        if order['quantity'] > 100:
            discount = 0.1
            if order['customer_type'] == 'premium':
                discount = 0.15
            total = order['price'] * order['quantity'] * (1 - discount)
            if total > 10000:
                total *= 0.95  # bulk bonus
        else:
            discount = 0
            if order['customer_type'] == 'premium':
                discount = 0.05
            total = order['price'] * order['quantity'] * (1 - discount)
    elif order['type'] == 'express':
        total = order['price'] * order['quantity'] * 1.2
        if order['customer_type'] == 'premium':
            total *= 0.9
    else:
        raise ValueError(f"Unknown order type: {order['type']}")
    return round(total, 2)
```
```

**Expected output**: Refactored version with reduced nesting, extracted helper functions, and clearer structure.

**Verification**: (1) All tests pass, (2) cyclomatic complexity decreases, (3) max nesting depth decreases.

## Difficulty Curriculum

| Level | Refactoring Type | Example |
|-------|-----------------|---------|
| 1 | Rename variables/functions | `x` → `total_price`, `f` → `calculate_tax` |
| 2 | Simplify conditionals | Remove redundant branches, use guard clauses |
| 3 | Extract function/method | Pull out repeated logic into named function |
| 4 | Restructure class hierarchy | Extract interface, replace inheritance with composition |
| 5 | Design pattern introduction | Apply Strategy, Factory, Observer patterns |
| 6 | Architectural refactoring | Decompose monolith into modules, separate concerns |

## Limitations & Risks

- **Test coverage dependency**: If the test suite is weak, a "refactoring" might change behavior in untested areas. This is a fundamental limit — refactoring without tests is always risky.
- **Metric validity**: Cyclomatic complexity and similar metrics are crude proxies for code quality. A low-complexity function can still be confusing; a high-complexity function can be clear.
- **Style subjectivity**: What constitutes "good" code varies across teams and languages. Linter configurations should be explicit and match the project's style guide.
- **Over-refactoring**: A model might aggressively restructure code, making it unfamiliar to the original developers. This is "correct" by metrics but practically harmful. A diff-size limit can constrain changes.
- **Performance regression**: Refactoring for readability might hurt performance (e.g., extracting functions adds call overhead). For performance-sensitive code, benchmark verification (from **Code Optimization**) should be added.

## Connections

- **Code Optimization** is the performance-focused counterpart: optimize for speed rather than readability, with the same behavioral preservation gate.
- **Code Repair** fixes bugs; refactoring fixes structure. Both use test suites for verification.
- **Test Generation** provides the tests that make refactoring verification possible.
- **Type Inference** is a specific kind of refactoring: adding type annotations to improve code quality.
- **Code Translation** can be viewed as extreme refactoring: completely rewriting in a different language while preserving behavior.
