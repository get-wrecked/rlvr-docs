---
domain: Metalogic & Logic Metatheory
category: logic-foundations
verification_type: execution
dataset_scale: 5K+ (from logic textbooks + formal proofs)
difficulty_range: hard/superhuman
modality: text
status: needs_validation
---

# Metalogic & Logic Metatheory

## Overview
Reason about logical systems themselves: prove completeness/soundness of proof systems, determine decidability of theories, compute model-theoretic properties (cardinality, saturation), verify Gödel numbering constructions. Verification via formal proof assistants (Lean4, Coq) or model-theoretic computation.

## Verification Mechanism
```python
def verify(task_type: str, theory: dict, answer: dict) -> float:
    if task_type == "decidability":
        # For decidable theories, we can verify by running the decision procedure
        if answer["claim"] == "decidable":
            # Check if the proposed decision procedure works
            for formula in theory["test_formulas"]:
                dp_result = run_decision_procedure(answer["procedure"], formula)
                actual = check_validity(theory, formula)
                if dp_result != actual:
                    return 0.0
            return 1.0
        elif answer["claim"] == "undecidable":
            # Verify reduction from a known undecidable problem
            return verify_formal_proof(answer["reduction_proof"]) if "reduction_proof" in answer else 0.5
    
    elif task_type == "model_counting":
        # Count models of a given size
        actual_count = enumerate_models(theory["axioms"], answer["domain_size"])
        return 1.0 if answer["count"] == actual_count else 0.0
    
    elif task_type == "completeness":
        # Verify a completeness proof (strongest: formal proof in Lean4)
        if "lean4_proof" in answer:
            return 1.0 if lean4_typecheck(answer["lean4_proof"]).success else 0.0
        return 0.5  # Can't fully verify NL completeness proofs
```

## Dataset Sources
- **Logic textbook exercises**: Enderton, Marker, Hodges — with solutions.
- **Lean4 mathlib logic**: Formalized metalogic results.
- **Metamath logic section**: Formalized logic proofs.
- **TPTP theorem rating system**: Difficulty-rated logic problems.
- **Model theory benchmarks**: Known models for finite theories.

## Task Format
- **Input**: "Is the theory of algebraically closed fields of characteristic 0 complete?"
- **Output**: "Yes, by quantifier elimination (Tarski's theorem)"

## Difficulty Curriculum
- Level 5: Basic soundness/completeness for propositional logic
- Level 7: Decidability proofs for specific theories
- Level 9: Incompleteness theorem applications, model theory

## Connections
- [[logic-first-order]] — the systems being meta-analyzed
- [[automated-theorem-proving]] — proving metatheorems
- [[math-formal-proofs]] — formal proofs of metalogic results
