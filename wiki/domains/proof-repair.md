---
domain: Formal Proof Repair
category: formal-methods
verification_type: rule
dataset_scale: 50K+ (from Lean/Coq version migrations)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Formal Proof Repair

## Overview
Given a formal proof that was valid in a previous version of a library/system but now fails (due to API changes, definition changes, or tactic deprecation), repair the proof so it compiles again. Verification: the repaired proof type-checks. This is especially relevant for Lean4/Mathlib, which undergoes frequent refactoring.

## Verification Mechanism
```python
def verify(broken_proof: str, theorem_statement: str, repaired_proof: str) -> float:
    # The theorem statement must remain unchanged
    if extract_theorem_statement(repaired_proof) != theorem_statement:
        return 0.0
    
    # The repaired proof must type-check
    result = lean4_typecheck(repaired_proof)
    return 1.0 if result.success else 0.0
```

This is as strong as formal proof verification — the type-checker is the ultimate arbiter.

## Dataset Sources
- **Mathlib4 git history**: Every commit that breaks and fixes proofs. Thousands of proof repair examples.
- **Lean3 → Lean4 migration**: Massive corpus of proof repairs from the mathport project.
- **Coq version migration**: Breaking changes between Coq versions.
- **Isabelle AFP updates**: Archive of Formal Proofs version maintenance.
- **Procedural generation**: Take a valid proof, randomly break it (change tactic, modify definition), require repair.

## Task Format
- **Input**: Broken proof + error message from type-checker + (optionally) diff of library changes
- **Output**: Repaired proof that type-checks

## Difficulty Curriculum
- Level 1: Simple tactic renaming (e.g., `simp` → `simp only [...]`)
- Level 3: Tactic argument changes, import updates
- Level 5: Proof restructuring due to definition changes
- Level 7: Complete proof rewrite using new API
- Level 9: Repair proofs broken by deep definitional changes

## Limitations & Risks
- Requires specific Lean4/Coq/Isabelle versions and library versions for verification.
- Repair may be trivial (rename a tactic) or require deep mathematical understanding.
- Excellent complement to proof generation — repairs existing proofs rather than writing from scratch.

## Connections
- [[math-formal-proofs]] — formal proof generation
- [[code-repair]] — same concept in software
- [[code-refactoring]] — refactoring proof code
