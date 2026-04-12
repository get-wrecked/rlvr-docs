---
domain: text-to-code-nl
category: language
verified: true
verification_type: exact
scalability: high
maturity: medium
tags: [code-generation, nl-to-code, execution, test-suite]
---

# Text-to-Code from Natural Language

## Overview

This domain covers generating code from natural language descriptions outside of competitive programming contexts. The model receives a natural language specification (a docstring, a Stack Overflow question, a notebook cell description) and must produce working code. Verification is execution-based: the generated code must pass a test suite.

This is distinct from competitive programming (which focuses on algorithmic problem-solving) in that it emphasizes practical, real-world code: data manipulation, API usage, library calls, string processing, and scripting tasks. The verification is exact — code either passes the tests or it does not.

## Verification Mechanism

**Verification type: Exact (execution-based)**

The verification pipeline:

1. **Syntax check**: Parse the generated code. If it fails to parse, reward = 0.
2. **Execution**: Run the code in a sandboxed environment.
3. **Test suite**: Execute predefined test cases against the code. Compare actual outputs to expected outputs.

```python
def verify(generated_code: str, test_cases: list[dict]) -> int:
    try:
        exec_env = create_sandbox()
        exec(generated_code, exec_env)
    except SyntaxError:
        return 0
    
    for test in test_cases:
        try:
            result = exec_env[test['function']](*test['inputs'])
            if result != test['expected_output']:
                return 0
        except Exception:
            return 0
    return 1
```

**Partial credit**: Can award fractional reward based on fraction of tests passed:
```python
passed = sum(1 for t in test_cases if run_test(code, t))
return passed / len(test_cases)
```

The execution-based verification is fully automated, deterministic, and binary per test case. It is one of the strongest RLVR verification mechanisms because it tests actual functional correctness, not surface-level similarity.

## Dataset Sources

| Dataset | Size | Language | Source | Notes |
|---------|------|----------|--------|-------|
| CoNaLa | 2.9K annotated + 600K mined | Python | Stack Overflow | NL intent to Python snippet |
| JuICe | 1.5M cells | Python | Jupyter notebooks | Notebook context to code cell |
| DS-1000 | 1K problems | Python | Data science libraries | NumPy, Pandas, Sklearn, etc. |
| MBPP | 974 problems | Python | Crowdsourced | Basic Python programming |
| HumanEval | 164 problems | Python | Hand-written | Function-level code generation |
| APPS | 10K problems | Python | Coding competitions | Ranges from introductory to competition |
| NL2Bash | 10K pairs | Bash | Various | Natural language to bash commands |
| CodeContests | 13K problems | Multiple | Competitive programming | Competition-level |
| Arcade | 1K examples | Python | Data science tasks | Table manipulation and analysis |
| ExcelFormulaNet | 100K+ | Excel | Spreadsheets | NL to Excel formulas |

**Scalability**: Stack Overflow provides millions of question-answer pairs with code. GitHub has billions of lines of code with associated comments and docstrings. Jupyter notebooks provide rich natural language context paired with code. The key challenge is generating or collecting reliable test suites, not the code itself.

## Task Format

**Input**: A natural language description of the desired function or code behavior.

**Output**: Working code (typically Python) that implements the described behavior.

**Prompt template example**:
```
Write a Python function that implements the following:

{natural_language_description}

def {function_name}({parameters}):
```

For notebook-style:
```
# Previous code:
{context_cells}

# Task: {natural_language_instruction}
# Write the next code cell:
```

## Difficulty Curriculum

1. **Level 1 — Single-line operations**: "Convert a list of strings to integers." Simple API calls or list comprehensions.
2. **Level 2 — Short functions (5-10 lines)**: "Write a function that removes duplicates from a list while preserving order."
3. **Level 3 — Library usage**: "Read a CSV file with Pandas and compute the mean of column 'price'." Requires knowing library APIs.
4. **Level 4 — Multi-step data processing**: "Load JSON, filter records where age > 30, group by city, and compute average salary." Chains multiple operations.
5. **Level 5 — Error handling and edge cases**: Code must handle edge cases (empty input, invalid data) and include proper error handling.
6. **Level 6 — Complex API interactions**: "Write a function that paginates through a REST API and collects all results." Requires understanding API patterns.
7. **Level 7 — Full module or class**: Generate a complete class with multiple methods, proper initialization, and interconnected functionality.

## Limitations & Risks

- **Test suite coverage**: If the test suite is incomplete, a function that passes all tests may still be incorrect for untested inputs. Test coverage is the bottleneck for verification quality.
- **Sandboxing requirements**: Code execution requires sandboxing to prevent malicious or destructive operations. This adds infrastructure complexity.
- **Non-deterministic code**: Code involving randomness, time, or external resources may produce different results across runs. Tests must account for this.
- **Multiple valid implementations**: Many NL descriptions admit multiple correct implementations. Test-based verification handles this well (unlike string comparison), but tests must be comprehensive.
- **Library version sensitivity**: Code using specific library versions may break with updates. Pinning dependencies in the execution environment is important.
- **Security risks**: RL-trained models may learn to generate code that technically passes tests but contains security vulnerabilities, backdoors, or obfuscated logic.

## Connections

- **[semantic-parsing](semantic-parsing.md)**: Text-to-SQL is a form of text-to-code. Semantic parsing and code generation share core techniques.
- **[structured-output-generation](structured-output-generation.md)**: Code must be syntactically valid, similar to structured output generation.
- **[table-understanding](table-understanding.md)**: Data science code generation often involves table manipulation.
- **[translation-reference](translation-reference.md)**: NL-to-code can be viewed as translation between natural and programming languages.
- **[spelling-grammar](spelling-grammar.md)**: Code repair (fixing bugs) is analogous to grammar correction.
