---
domain: Theorem Formalization (Natural Language → Lean/Coq)
category: formal-methods
verification_type: rule
dataset_scale: 10K+ (from math corpora)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Theorem Formalization (Natural Language → Lean/Coq)

## Overview
Translate a mathematical theorem stated in natural language into a formal statement in Lean4/Coq/Isabelle. Verification: the formalized statement must (1) type-check, (2) be logically equivalent to the intended statement (checked by proving the equivalence or by checking it on test cases).

## Verification Mechanism
```python
def verify(nl_statement: str, formal_statement: str, 
           reference_formal: str = None) -> float:
    # Step 1: Type-check (syntactic validity)
    typecheck = lean4_typecheck(formal_statement)
    if not typecheck.success:
        return 0.0
    
    score = 0.3  # Passes type-check
    
    # Step 2: If reference formalization exists, check equivalence
    if reference_formal:
        # Try to prove forward implication
        fwd = lean4_prove(f"{formal_statement} → {reference_formal}", timeout=30)
        bwd = lean4_prove(f"{reference_formal} → {formal_statement}", timeout=30)
        
        if fwd and bwd:
            score = 1.0  # Proven equivalent
        elif fwd or bwd:
            score = 0.6  # One direction proven
    
    # Step 3: Spot-check with concrete instances
    if "∀" in formal_statement or "forall" in formal_statement:
        # Test with random values
        instances_correct = test_instances(formal_statement, n=100)
        score = max(score, 0.3 + 0.4 * instances_correct)
    
    return score
```

## Dataset Sources
- **ProofNet**: 371 theorem-proof pairs with NL and Lean formulations.
- **miniF2F**: 488 math competition problems formalized in Lean and Isabelle.
- **Mathlib4**: Compare informal comments/docstrings with formal statements.
- **LeanDojo**: Lean theorem proving environment with formalized theorems.
- **Natural Language Premise Selection**: Mapping NL to formal.
- **ITP textbooks**: Software Foundations, CPDT — have parallel NL/formal.

## Task Format
- **Input**: "For every prime number p greater than 2, p is odd."
- **Output**: `theorem prime_gt_two_odd (p : ℕ) (hp : Nat.Prime p) (h : p > 2) : ¬ Even p`

## Difficulty Curriculum
- Level 1: Basic algebraic identities
- Level 3: Number theory statements with quantifiers
- Level 5: Analysis (limits, continuity)
- Level 7: Abstract algebra (group theory)
- Level 9: Research-level statements with complex type dependencies

## Limitations & Risks
- Equivalence checking is undecidable in general. Use timeout-bounded proof search + instance testing.
- There are many valid formalizations of the same NL statement (different but equivalent type signatures). Accept any that are proven equivalent.
- Requires deep knowledge of both mathematics and the formal proof system's type theory.

## Connections
- [[math-formal-proofs]] — proving the formalized theorems
- [[proof-repair]] — fixing broken formalizations
- [[semantic-parsing]] — general NL-to-formal translation
