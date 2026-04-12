---
domain: Functional Programming Challenges
category: code-paradigm
verification_type: execution
dataset_scale: 50K+ (from FP exercise platforms)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Functional Programming Challenges

## Overview
Solve programming problems in pure functional style (Haskell, OCaml, Racket, Clojure): implement higher-order functions, monadic transformations, algebraic data types, parser combinators, type-level programming. Verification: test suite execution in the target functional language.

## Verification Mechanism
```python
def verify(language: str, code: str, tests: list[dict]) -> float:
    if language == "haskell":
        result = run_ghc(code, tests, timeout=30)
    elif language == "ocaml":
        result = run_ocaml(code, tests, timeout=30)
    elif language == "racket":
        result = run_racket(code, tests, timeout=30)
    
    return result.passed / result.total if result.total > 0 else 0.0
```

## Dataset Sources
- **Exercism**: Haskell/OCaml/Clojure/Elixir tracks with test suites.
- **99 Haskell Problems**: Classic functional programming exercises.
- **Advent of Code in FP**: Solutions in functional languages with known outputs.
- **SICP exercises**: Structure and Interpretation of Computer Programs.
- **Real World Haskell/OCaml exercises**: Textbook problems.
- **Haskell Weekly challenges**: Community challenges.
- **CIS 194 (UPenn Haskell)**: Course exercises with tests.

## Task Format
- **Input**: "Implement `foldTree :: [a] -> Tree a` that builds a balanced binary tree from a list, in Haskell"
- **Output**: Haskell code that passes test suite

## Difficulty Curriculum
- Level 1: Map, filter, fold implementations
- Level 3: Algebraic data types, pattern matching, recursion
- Level 5: Monads, monad transformers, parser combinators
- Level 7: Type-level programming, GADTs, type families
- Level 9: Dependent types (Idris/Agda), effect systems

## Connections
- [[code-generation]] — code generation in FP languages
- [[type-theory-puzzles]] — type-level programming
- [[compiler-construction]] — FP languages often used for compilers
