---
domain: Theorem & Lemma Discovery
category: math-research
verification_type: rule
dataset_scale: 10K+ (from mathematical databases)
difficulty_range: hard/superhuman
modality: text
status: needs_validation
---

# Theorem & Lemma Discovery

## Overview
Given a mathematical domain and existing theorems, discover new true and non-trivial theorems. This is the frontier of AI mathematics. Verification requires: (1) the statement is syntactically valid, (2) the statement is true (verified by formal proof or CAS), (3) the statement is non-trivial (not a direct consequence of existing theorems).

## Verification Mechanism
```python
def verify(domain: str, statement: str, proof: str = None) -> float:
    score = 0.0
    
    # 1. Syntactic validity
    if not is_valid_mathematical_statement(statement):
        return 0.0
    score += 0.2
    
    # 2. Truth (multiple approaches)
    if proof:
        # Formal proof provided — strongest verification
        if verify_formal_proof(statement, proof, system="lean4"):
            score += 0.5
    else:
        # Computational verification for concrete statements
        if verify_computationally(statement, num_tests=10000):
            score += 0.3  # Less credit than formal proof
    
    # 3. Non-triviality
    if not is_trivial(statement, existing_theorems=domain_theorems[domain]):
        score += 0.3
    
    return score

def is_trivial(statement, existing_theorems):
    """Check if statement is a trivial consequence of known theorems."""
    # Try to prove automatically with short proof search
    proof = auto_prove(statement, premises=existing_theorems, timeout=10)
    if proof and proof.length <= 3:  # Very short proof = trivial
        return True
    # Check if it's a direct instantiation of a known theorem
    for thm in existing_theorems:
        if is_instantiation(statement, thm):
            return True
    return False
```

## Dataset Sources
- **Mathlib (Lean4)**: 100K+ formalized theorems — the "known theorems" baseline.
- **OEIS conjectures**: The OEIS flags many sequences where properties are conjectured but unproven.
- **Open problem lists**: Unsolved problems in various mathematical fields.
- **Proceedings of mathematical societies**: Published new theorems to calibrate what counts as interesting.

## Task Format
- **Input**: "In the domain of elementary number theory, with access to the following known results: [list]. State a true and non-trivial theorem."
- **Output**: A mathematical statement (+ optionally a proof)

## Difficulty Curriculum
- Level 5: Discover lemmas in well-explored areas (likely already known, but the agent doesn't know that)
- Level 7: Discover non-trivial theorems in active research areas
- Level 9: Discover theorems that connect different mathematical domains
- Level 10: Solve open problems

## Limitations & Risks
- **Non-triviality is the hard part.** Easy to generate infinitely many true but useless statements (e.g., "2 + 2 = 4", "for all n, n + 0 = n"). The triviality filter is crucial and imperfect.
- **Truth verification for novel conjectures may be undecidable.** Limit to domains where CAS or formal methods can verify.
- **This is genuinely research-level.** May need to start with "rediscovery" (find known but non-obvious theorems).
- Most suitable as a later-stage RLVR task for models that already have strong mathematical reasoning.

## Connections
- [[math-formal-proofs]] — formal proof infrastructure
- [[math-theorem-conjecturing]] — generating conjectures (the hypothesis step)
- [[mathematical-conjecture-testing]] — testing whether conjectures are true
