---
domain: Type Inference
category: Code & Software
verification_type: execution-based (type-checker accepts annotated code)
dataset_scale: 10K–500K functions
difficulty_range: simple primitives → complex generics and dependent types
modality: untyped-code-to-typed-code
status: emerging
---

## Overview

Type inference tasks the model with adding type annotations to untyped or partially-typed code such that a type-checker (e.g., mypy for Python, tsc for TypeScript) accepts the result without errors. The verification is clean and fully automatic: run the type-checker on the annotated code and check that it reports zero errors.

This domain is attractive for RLVR because (a) the type-checker is a deterministic, fast verifier; (b) the problem is practically valuable — gradual typing adoption in Python and JavaScript codebases is a real industry need; and (c) the difficulty scales naturally from simple primitives to complex generic types.

## Verification Mechanism

```
def verify_type_annotations(original_code, annotated_code, language,
                            test_suite=None):
    # 1. Write annotated code to file
    write_file(tmp_dir / filename(language), annotated_code)

    # 2. Run the type-checker
    if language == "python":
        result = run_in_sandbox(
            command="mypy --strict --no-error-summary " + filename,
            timeout=60
        )
    elif language == "typescript":
        result = run_in_sandbox(
            command="tsc --strict --noEmit " + filename,
            timeout=60
        )

    type_check_passed = (result.returncode == 0)

    # 3. (Optional) Verify runtime behavior is preserved
    if test_suite and type_check_passed:
        for test in test_suite:
            outcome = run_in_sandbox(
                command=f"python {filename}",
                stdin=test.input, timeout=10
            )
            if outcome.stdout.strip() != test.expected.strip():
                return 0.0  # annotations changed behavior (shouldn't happen
                             # but guards against model rewriting logic)

    # 4. (Optional) Check annotation specificity
    #    Penalize overly broad types like 'Any'
    any_count = count_occurrences(annotated_code, ": Any")
    if any_count > 0:
        specificity_penalty = any_count * 0.1
    else:
        specificity_penalty = 0.0

    if type_check_passed:
        return max(0.0, 1.0 - specificity_penalty)
    return 0.0
```

Stricter verification:

```
def verify_strict(annotated_code, reference_types=None):
    """
    Beyond type-checker acceptance, compare inferred types against
    known reference annotations for key functions/variables.
    """
    # Parse type annotations from model output
    model_types = extract_type_annotations(annotated_code)

    # Compare against reference (if available)
    if reference_types:
        exact_matches = sum(
            1 for name, mtype in model_types.items()
            if is_type_equivalent(mtype, reference_types.get(name))
        )
        accuracy = exact_matches / len(reference_types)
        return accuracy  # continuous reward based on type accuracy

    # Fallback: just check that mypy --strict passes
    return run_mypy_strict(annotated_code)
```

Key considerations:

- **`Any` escape hatch**: A trivial solution is to annotate everything as `Any`, which satisfies most type-checkers. Running with `--strict` and `--disallow-any-explicit` flags prevents this. A reward penalty for `Any` usage is also effective.
- **Type-checker strictness levels**: mypy has `--strict`, `--warn-return-any`, `--disallow-untyped-defs`, etc. The verification should use the strictest relevant settings.
- **Behavioral preservation**: Type annotations in Python and TypeScript should not change runtime behavior. A test suite run after annotation catches any accidental logic changes.
- **Third-party stubs**: Type-checking code that imports third-party libraries requires type stubs. The sandbox must have relevant stubs installed (e.g., `types-requests`, `@types/node`).

## Dataset Sources

| Dataset | Size | Language | Notes |
|---------|------|----------|-------|
| **TypeWeaver** | 513 Python projects | Python | Open-source projects with partial type annotations |
| **ManyTypes4Py** | 5,382 Python files | Python | Type annotations mined from GitHub, validated with mypy |
| **InferTypes4Py** | 4,549 functions | Python | Function-level type inference benchmark |
| **TypeBugs** | 93 bugs | Python | Type-related bugs in real projects |
| **DefinitelyTyped** | 8,000+ packages | TypeScript | Type definitions for npm packages; can be used in reverse |
| **LambdaNet** | 300 TypeScript projects | TypeScript | Graph-based type inference training data |
| **DeepTyper** | 10,000+ files | JavaScript/TypeScript | Untyped JS paired with typed TS declarations |
| **Type4Py** | 4,621 projects | Python | Large-scale Python type annotation dataset |
| **HeaderGen** | 1,200 Python files | Python | Focuses on function signatures and return types |

**Synthetic data generation**: Strip type annotations from well-typed TypeScript/Python codebases to create (untyped, typed) pairs. This scales to millions of functions from GitHub.

## Task Format

**Input prompt**:
```
Add type annotations to the following Python function so that
`mypy --strict` passes with zero errors.

```python
def merge_dicts(base, override):
    result = {}
    for key in base:
        result[key] = base[key]
    for key in override:
        result[key] = override[key]
    return result
```
```

**Expected output**:
```python
def merge_dicts(base: dict[str, Any], override: dict[str, Any]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key in base:
        result[key] = base[key]
    for key in override:
        result[key] = override[key]
    return result
```

Better (more specific, earns higher reward):
```python
from typing import TypeVar
V = TypeVar('V')
def merge_dicts(base: dict[str, V], override: dict[str, V]) -> dict[str, V]:
    result: dict[str, V] = {}
    for key in base:
        result[key] = base[key]
    for key in override:
        result[key] = override[key]
    return result
```

## Difficulty Curriculum

| Level | Type Complexity | Example |
|-------|----------------|---------|
| 1 | Primitives | `int`, `str`, `bool`, `float` |
| 2 | Collections | `list[int]`, `dict[str, float]`, `tuple[int, ...]` |
| 3 | Optional and Union | `Optional[str]`, `int \| str` |
| 4 | Callable and generics | `Callable[[int, str], bool]`, `TypeVar` |
| 5 | Protocols and structural types | `Protocol` with required methods |
| 6 | Advanced generics | `ParamSpec`, `TypeVarTuple`, bounded TypeVars, overloads |

## Limitations & Risks

- **Type system expressiveness**: Some runtime patterns cannot be expressed in static types. The model may need to use `cast()` or `type: ignore` pragmatically. Penalizing these is desirable but requires careful calibration.
- **`Any` gaming**: Without explicit penalties, models will gravitate toward `Any` annotations. Strict mode and `Any`-counting in the reward function are essential.
- **Third-party dependencies**: Real-world code depends on libraries that may or may not have type stubs. Missing stubs cause false type-checker failures. The sandbox must include relevant stubs or the task must be scoped to code with available stubs.
- **Version sensitivity**: Type-checking behavior differs between mypy versions, pyright, and other checkers. The verification environment must pin specific versions.
- **Gradual typing semantics**: In Python, adding types to one function can cause cascading type errors in callers. Tasks should be scoped appropriately (single function, single file, or provide stubs for context).

## Connections

- **Code Translation** from dynamically-typed to statically-typed languages (Python → Rust) requires type inference as a subtask.
- **Code Refactoring** often involves adding or improving type annotations as part of code quality improvements.
- **Code Generation** can integrate type annotations into the generated code, verified by the same type-checker pipeline.
- **Compiler Tasks** share the formal verification aspect — type-checkers are essentially lightweight compilers.
- Type annotations improve **Code Repair** by catching type-related bugs statically.
