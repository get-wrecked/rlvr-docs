---
domain: Code Documentation Verification
category: code-understanding
verification_type: execution
dataset_scale: unlimited (from any codebase)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Code Documentation Verification

## Overview
Given code and its documentation (docstrings, API docs, README), verify whether the documentation accurately describes the code's behavior. Alternatively: generate documentation that accurately describes what the code does, verified by generating test cases from the documentation and checking they pass.

## Verification Mechanism
```python
def verify_doc_accuracy(code: str, documentation: str) -> float:
    # Strategy: Use documentation to generate test cases,
    # then check if code passes them.
    
    # Extract claims from documentation
    claims = extract_testable_claims(documentation)
    # e.g., "Returns the sum of two numbers" -> test: f(2, 3) == 5
    
    test_code = generate_tests_from_claims(claims, code)
    
    # Run tests against the actual code
    results = execute_tests(code, test_code, timeout=30)
    
    return results.passed / results.total if results.total > 0 else 0.0

# Alternative: verify docstring examples
def verify_docstring_examples(code: str) -> float:
    """Check that docstring examples (doctests) actually work."""
    import doctest
    results = doctest.testmod(code, verbose=False)
    if results.attempted == 0:
        return 0.0  # No examples to check
    return (results.attempted - results.failed) / results.attempted
```

## Dataset Sources
- **Python stdlib**: All functions with docstrings — check doctest examples.
- **NumPy/SciPy/pandas docstrings**: Extensive examples that can be verified.
- **ReadTheDocs projects**: Thousands of documented Python projects.
- **Javadoc**: Java documentation with @example tags.
- **GitHub code-docstring pairs**: Extract from top-starred repos.
- **Procedural generation**: Take code, deliberately introduce documentation errors, ask agent to identify them.

## Task Format
- **Input**: Function code + docstring → "Is this documentation accurate?"
- **Output**: True/False + specific inaccuracies identified

Or generative:
- **Input**: Function code → "Write accurate documentation"
- **Output**: Docstring with examples that pass when tested

## Difficulty Curriculum
- Level 1: Check if basic function description matches behavior
- Level 3: Verify parameter descriptions, return type docs
- Level 5: Check edge case documentation accuracy
- Level 7: Verify complex API documentation across multiple functions
- Level 9: Verify architectural documentation against codebase behavior

## Limitations & Risks
- Extracting testable claims from natural language docs is imperfect.
- Doctest verification is straightforward and reliable.
- Focus on the doctest/example verification path for strongest signal.

## Connections
- [[code-generation]] — generating code from docs (reverse direction)
- [[test-generation]] — generating tests from docs
- [[fact-verification]] — verifying claims against evidence
