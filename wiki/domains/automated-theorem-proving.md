---
domain: Automated Theorem Proving (First-Order)
category: formal-methods
verification_type: rule
dataset_scale: 25K+ (TPTP library)
difficulty_range: medium/hard/superhuman
modality: text
status: verified
---

# Automated Theorem Proving (First-Order)

## Overview
Prove first-order logic theorems using resolution, tableaux, superposition, or other automated reasoning methods. Verification: check the proof using an independent proof checker (TSTP proof format, Ivy, GDV).

## Verification Mechanism
```python
def verify(problem_tptp: str, proof: str) -> float:
    # Method 1: Use GDV (Global Distributed Verifier)
    result = run_gdv(problem_tptp, proof, timeout=60)
    if result.verified:
        return 1.0
    
    # Method 2: Run independent prover to confirm theorem is valid
    # then check proof steps
    eprover_result = run_eprover(problem_tptp, timeout=300)
    if eprover_result.proved:
        # Problem is provable; check if the submitted proof is correct
        steps = parse_tstp(proof)
        for step in steps:
            if not verify_inference_step(step, steps):
                return 0.5  # Theorem is true but proof is wrong
        return 1.0
    
    return 0.0
```

## Dataset Sources
- **TPTP (Thousands of Problems for Theorem Provers)**: 25K+ problems in standardized format. The gold standard.
- **TPTP rating**: Each problem has a difficulty rating (0.0-1.0).
- **CASC (CADE ATP System Competition)**: Annual competition benchmarks.
- **Mizar Mathematical Library**: 60K+ formalized theorems.
- **Metamath**: 40K+ proofs in a simple formal system.
- **SET.mm**: Metamath set theory database.
- **Procedural generation**: Generate random first-order statements, classify as theorem/non-theorem.

## Task Format
- **Input**: TPTP-format problem: "fof(ax1, axiom, ![X]: (p(X) => q(X))). fof(conj, conjecture, ![X]: (p(X) => q(X)))."
- **Output**: TSTP-format proof

## Difficulty Curriculum
- Level 1: Propositional tautologies
- Level 3: Simple first-order proofs (universal instantiation, modus ponens)
- Level 5: Proofs requiring equality reasoning
- Level 7: Set theory theorems
- Level 9: Open TPTP problems (rating > 0.9)

## Limitations & Risks
- Proof verification is decidable (just check each step). Proof *finding* is undecidable.
- TPTP format is standardized but complex. The agent needs to learn the format.
- Some TPTP problems remain unsolved by any prover — genuine research challenges.

## Connections
- [[logic-first-order]] — first-order logic
- [[math-formal-proofs]] — formal proofs in Lean/Coq (higher-order)
- [[smt-solving]] — related verification technology
