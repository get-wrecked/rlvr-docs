---
domain: Type Theory Puzzles
category: programming-languages
verification_type: rule
dataset_scale: 10K+ (from type theory exercise sets + procedural generation)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Type Theory Puzzles

## Overview
Given a type signature, produce a term that inhabits that type (in a typed lambda calculus, Haskell, or Idris/Agda). Via the Curry-Howard correspondence, this is equivalent to proving a logical proposition. Verification: type-checker confirms the term has the stated type.

## Verification Mechanism
```python
def verify(type_signature: str, term: str, language: str = "haskell") -> float:
    if language == "haskell":
        # Write a Haskell file with the type annotation and term
        code = f"module Check where\n\ncheck :: {type_signature}\ncheck = {term}"
        result = run_ghc_typecheck(code)
        return 1.0 if result.success else 0.0
    elif language == "agda":
        # Agda type-checks and verifies totality
        result = run_agda_typecheck(type_signature, term)
        return 1.0 if result.success else 0.0
```

Type-checking is decidable for most practical type systems. The Agda/Idris variant additionally checks totality (no infinite loops), making verification even stronger.

## Dataset Sources
- **Haskell Typeclassopedia exercises**: Type-level programming challenges.
- **Software Foundations (Coq)**: Proof exercises = type inhabitation.
- **Agda standard library**: Types with known inhabitants.
- **Programming Language Foundations in Agda**: Full exercise set.
- **Procedural generation**: Generate random types from grammar, check inhabitability.
- **Djinn**: Haskell tool that generates terms from types. Use its outputs as ground truth.

## Task Format
- **Input**: "Provide a term of type `(a -> b) -> (b -> c) -> (a -> c)`"
- **Output**: `\f g x -> g (f x)`

## Difficulty Curriculum
- Level 1: Simple function types (`a -> a`, `a -> b -> a`)
- Level 3: Higher-order functions, polymorphism
- Level 5: GADTs, type families
- Level 7: Dependent types (Agda/Idris)
- Level 9: Complex proof terms (equivalence of mathematical structures)

## Limitations & Risks
- Trivial inhabitants (bottom/undefined, infinite loops) must be ruled out. Use total languages (Agda) or add runtime checks.
- Very abstract — may not develop broadly useful reasoning. But the Curry-Howard connection means this IS logical reasoning in disguise.

## Connections
- [[math-formal-proofs]] — Curry-Howard: types = propositions, programs = proofs
- [[type-inference]] — reverse direction: infer types for given programs
- [[logic-propositional]] — propositional logic ≅ simply-typed lambda calculus
