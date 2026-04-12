---
domain: Logic Programming (Prolog/Datalog/ASP)
category: programming-paradigm
verification_type: execution
dataset_scale: 10K+ (from LP benchmarks + courses)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Logic Programming (Prolog/Datalog/ASP)

## Overview
Write programs in logic programming languages: Prolog (unification, backtracking, cut), Datalog (recursive queries, stratification), and Answer Set Programming (stable models, optimization). Verification: run the program, check query results against expected answers.

## Verification Mechanism
```python
def verify(language: str, program: str, queries: list[dict]) -> float:
    if language == "prolog":
        result = run_swipl(program, queries, timeout=30)
        correct = sum(1 for q, r in zip(queries, result.answers) 
                     if set(r) == set(q["expected"]))
        return correct / len(queries)
    
    elif language == "datalog":
        result = run_souffle(program, queries, timeout=30)
        correct = sum(1 for q, r in zip(queries, result.answers)
                     if set(map(tuple, r)) == set(map(tuple, q["expected"])))
        return correct / len(queries)
    
    elif language == "asp":
        result = run_clingo(program, timeout=30)
        if result.stable_models is None:
            return 0.0
        # Check if expected atoms are in some stable model
        for expected in queries:
            if not any(expected["atoms"].issubset(model) for model in result.stable_models):
                return 0.0
        return 1.0
```

## Dataset Sources
- **SWI-Prolog exercises**: Official tutorial exercises.
- **99 Prolog Problems**: Classic problem set (analogous to 99 Haskell Problems).
- **ASP Competition**: Annual Answer Set Programming competition instances.
- **Datalog benchmarks**: Soufflé benchmark suite, Doop (Java points-to analysis).
- **LP textbook exercises**: Sterling & Shapiro, Clocksin & Mellish.
- **Procedural generation**: Generate random logic programs, compute query results.

## Task Format
- **Input**: "Write a Prolog program that solves the N-Queens problem. Query: ?- queens(8, Qs)."
- **Output**: Prolog code that returns valid queen placements

## Difficulty Curriculum
- Level 1: Facts and simple rules, list operations
- Level 3: Recursive predicates, accumulator patterns
- Level 5: Meta-programming, definite clause grammars (DCGs)
- Level 7: Constraint logic programming (CLP(FD))
- Level 9: Complex ASP encodings, Datalog with stratified negation

## Limitations & Risks
- Prolog has non-deterministic execution — multiple valid answers may exist. Accept any valid answer.
- ASP can have exponentially many stable models — check desired properties rather than exact model.

## Connections
- [[functional-programming]] — different paradigm, similar level of abstraction
- [[logic-propositional]] — logic foundations
- [[formal-language-theory]] — DCGs for parsing
- [[constraint-programming]] — CLP bridges LP and CP
