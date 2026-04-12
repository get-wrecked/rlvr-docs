---
domain: Natural Language Theorem Proving
category: math-reasoning
verification_type: execution
dataset_scale: 100K+ (from math competition solutions)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Natural Language Theorem Proving

## Overview
Given a mathematical claim, produce a step-by-step natural language proof. Unlike formal theorem proving (Lean4), this tests the ability to reason in natural language — closer to how mathematicians actually work. The challenge is verification: how do you check a natural language proof?

## Verification Mechanism
The key insight: decompose the proof into individual steps, then verify each step computationally.

```python
def verify(claim: str, proof_steps: list[str]) -> float:
    # Parse each step into a formal claim using a symbolic parser
    formalized = [sympy_formalize(step) for step in proof_steps]
    
    # Check each step follows from previous steps
    for i, step in enumerate(formalized):
        premises = formalized[:i]
        if not check_entailment(premises, step):
            return 0.0
    
    # Check final step matches the original claim
    if not equivalent(formalized[-1], formalize(claim)):
        return 0.0
    
    return 1.0
```

Alternative: use the proof steps to generate a formal proof (Lean4), verify that.

**Hybrid approach (most practical)**: 
1. Check that the final answer is correct (exact match — this is easy)
2. Check that each intermediate computation is correct (execute arithmetic/algebra steps)
3. Check logical connectives (each "therefore" follows from "because")

This gives partial credit and catches most errors.

## Dataset Sources
- MATH dataset solutions (12.5K with step-by-step)
- Art of Problem Solving (AoPS) wiki — thousands of detailed proofs
- ProofWiki — 20K+ formalized proofs with natural language versions
- Metamath / Natural Language Metamath correspondence
- Math StackExchange accepted answers

## Task Format
- **Input**: "Prove that for all positive integers n, n^2 + n is even."
- **Output**: "Step 1: n^2 + n = n(n+1). Step 2: n and n+1 are consecutive integers. Step 3: One of any two consecutive integers is even. Step 4: Therefore n(n+1) is even."

## Difficulty Curriculum
- Level 1-3: Algebraic identities, basic number theory
- Level 4-6: Induction proofs, intermediate contest problems
- Level 7-9: Olympiad-level proofs
- Level 10: Research-level conjectures

## Limitations & Risks
- Natural language proof verification is fundamentally harder than formal proof verification. The hybrid approach (check answer + check computations) catches most errors but not subtle logical gaps.
- Risk of rewarding plausible-sounding but flawed proofs. Mitigate by also training on formal proofs (Lean4) to build rigorous reasoning, then transferring to natural language.

## Connections
- [[math-formal-proofs]] — formal version of this task
- [[math-competition]] — source of proof problems
- [[automated-reasoning]] — machine reasoning more broadly
