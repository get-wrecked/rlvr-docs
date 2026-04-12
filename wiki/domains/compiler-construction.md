---
domain: Compiler / Interpreter Construction
category: systems
verification_type: execution
dataset_scale: 10K+ (from compiler courses + language specs)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Compiler / Interpreter Construction

## Overview
Build compilers, interpreters, or language processing tools for specified languages or DSLs. Verification: the resulting compiler/interpreter correctly processes a test suite of programs — executing them and comparing outputs.

## Verification Mechanism
```python
def verify(language_spec: str, compiler_code: str, test_programs: list[dict]) -> float:
    # Build the compiler/interpreter
    try:
        compiler = build_and_run(compiler_code, timeout=120)
    except Exception:
        return 0.0
    
    passed = 0
    for test in test_programs:
        try:
            # Run a test program through the compiled compiler
            output = run_program_through_compiler(
                compiler, test["source_code"], 
                stdin=test.get("stdin", ""),
                timeout=30
            )
            if output.strip() == test["expected_output"].strip():
                passed += 1
        except (TimeoutError, RuntimeError):
            pass  # Test failed
    
    return passed / len(test_programs)
```

## Dataset Sources
- **Compiler courses**: MIT 6.035, Stanford CS143, CMU 15-411 — course projects with test suites.
- **PLZoo**: Collection of small language implementations with specifications.
- **Crafting Interpreters**: Book with progressively complex interpreter projects.
- **LLVM test suite**: Tests for language frontends.
- **Language specifications**: Formal language specs (R7RS Scheme, Lua 5.4, etc.) with conformance test suites.
- **DSL benchmarks**: Domain-specific language implementation challenges.

## Task Format
- **Input**: "Implement an interpreter for a language with: integer arithmetic, variables, if-else, while loops, and print statements. Here is the grammar: [BNF]"
- **Output**: Complete interpreter source code

## Difficulty Curriculum
- Level 1: Calculator (arithmetic expressions only)
- Level 3: Simple imperative language (variables, loops, conditionals)
- Level 5: Functions, closures, first-class functions
- Level 7: Type checker, static analysis, optimization passes
- Level 9: Full language implementation (Scheme, subset of C)

## Limitations & Risks
- Compiler construction is a large task — may need to break into subtasks (lexer, parser, evaluator).
- Test suite coverage may miss edge cases. Use extensive test suites from compiler courses.
- Bootstrapping: if the compiler is written in its own language, we need a bootstrap compiler.

## Connections
- [[code-generation]] — generating compiler code
- [[formal-language-theory]] — parsing theory
- [[compiler-tasks]] — compiler-related tasks
- [[type-inference]] — type system implementation
