---
domain: Formal Theorem Proving
category: Math / Formal Verification
verification_type: execution
dataset_scale: ~100K theorems (Mathlib alone); expandable
difficulty_range: medium/hard/superhuman
modality: code
status: verified
---

## Overview

Formal theorem proving is the crown jewel of verifiable math: the model writes a proof in a formal language (Lean 4, Coq, Isabelle/HOL), and the type-checker either accepts or rejects it. There is zero ambiguity — a proof that type-checks is correct by construction. This makes it a perfect RLVR domain: the verification function is the compiler/type-checker itself, and the reward is binary (proof accepted or not).

The domain has been battle-tested in RLVR. AlphaProof (DeepMind, 2024) used RL to solve IMO problems via Lean formalization. HTPS (Lample et al., 2022), ReProver (Yang et al., 2023), and others have used RL or expert iteration on formal proof search. The key challenge is that formal proof is extremely hard — even simple mathematical statements require intricate tactic sequences.

## Verification Mechanism

**Primary method: Type-checker execution.**

The verification is a hard binary signal: either the proof compiles or it doesn't.

```python
import subprocess
import tempfile
import os

def verify_lean4_proof(theorem_statement: str, proof: str, timeout: int = 60) -> float:
    """
    Verify a Lean 4 proof by running the Lean type-checker.
    
    Args:
        theorem_statement: The Lean 4 theorem statement (e.g., "theorem foo : 1 + 1 = 2")
        proof: The model-generated proof body (tactic block or term)
        timeout: Seconds before killing the process
    
    Returns:
        1.0 if proof type-checks, 0.0 otherwise
    """
    # Construct the full Lean 4 file
    lean_code = f"""
import Mathlib

{theorem_statement} := by
{proof}
"""
    
    with tempfile.NamedTemporaryFile(suffix='.lean', mode='w', delete=False) as f:
        f.write(lean_code)
        f.flush()
        
        try:
            result = subprocess.run(
                ['lake', 'env', 'lean', f.name],
                capture_output=True,
                text=True,
                timeout=timeout,
                cwd='/path/to/mathlib4/project'
            )
            
            # No errors = proof accepted
            if result.returncode == 0 and 'error' not in result.stderr.lower():
                return 1.0
            else:
                return 0.0
        except subprocess.TimeoutExpired:
            return 0.0
        finally:
            os.unlink(f.name)


def verify_coq_proof(statement: str, proof: str, timeout: int = 60) -> float:
    """Verify a Coq proof via coqc."""
    coq_code = f"""
Require Import Arith.
Require Import Lia.

{statement}
Proof.
{proof}
Qed.
"""
    with tempfile.NamedTemporaryFile(suffix='.v', mode='w', delete=False) as f:
        f.write(coq_code)
        f.flush()
        try:
            result = subprocess.run(
                ['coqc', f.name],
                capture_output=True, text=True, timeout=timeout
            )
            return 1.0 if result.returncode == 0 else 0.0
        except subprocess.TimeoutExpired:
            return 0.0
        finally:
            os.unlink(f.name)
```

**Interactive tactic mode** (used by HTPS, ReProver):

Instead of generating the full proof at once, the model interacts with the proof assistant step-by-step:

```python
def verify_interactive_lean4(theorem_statement: str, tactic_generator, max_steps=100):
    """
    Interactive proof search: model generates one tactic at a time,
    receives the updated proof state, and continues until no goals remain.
    """
    env = Lean4Environment(theorem_statement)  # e.g., via LeanDojo
    state = env.initial_state()
    
    for step in range(max_steps):
        if state.is_solved():
            return 1.0  # All goals closed
        
        # Model generates next tactic given current proof state
        tactic = tactic_generator(state.goals, state.context)
        
        try:
            state = env.apply_tactic(tactic)
        except TacticError:
            return 0.0  # Invalid tactic
    
    return 0.0  # Ran out of steps
```

## Dataset Sources

| Dataset | Size | Language | Source | Notes |
|---------|------|----------|--------|-------|
| **Mathlib4** | ~170K theorems | Lean 4 | [github.com/leanprover-community/mathlib4](https://github.com/leanprover-community/mathlib4) | Largest formal math library. Active development. |
| **miniF2F** (Zheng et al., 2022) | 488 | Lean 4 / Isabelle | [github.com/openai/miniF2F](https://github.com/openai/miniF2F) | Formalized competition problems. Standard benchmark. |
| **ProofNet** (Azerbayev et al., 2023) | 371 | Lean 4 | [github.com/zhangir-azerbayev/ProofNet](https://github.com/zhangir-azerbayev/ProofNet) | Undergraduate math, harder than miniF2F |
| **LeanDojo** (Yang et al., 2023) | ~98K theorems | Lean 4 | [github.com/lean-dojo/LeanDojo](https://github.com/lean-dojo/LeanDojo) | Tooling for extracting training data from Lean repos |
| **FIMO** (Liu et al., 2023) | 149 | Lean 4 | GitHub | Formalized IMO problems |
| **CompLean** | ~1,000 | Lean 4 | Various | Competition math in Lean |
| **Isabelle AFP** | ~70K theorems | Isabelle | [isa-afp.org](https://www.isa-afp.org/) | Archive of Formal Proofs |
| **Coq stdlib + MathComp** | ~30K | Coq | [github.com/math-comp](https://github.com/math-comp) | Coq mathematical components |
| **HOL Light proofs** | ~20K | HOL Light | Various | Used by some DeepMind work |
| **PutnamBench** (Tsoukalas et al., 2024) | 1,697 | Lean 4 / Coq / Isabelle | [github.com/trishullab/PutnamBench](https://github.com/trishullab/PutnamBench) | Formalized Putnam problems |
| **ProofPile** (Azerbayev et al., 2023) | ~8.3B tokens | Mixed | HuggingFace | Pretraining data for math/formal |

### Synthetic data generation

```python
# Generate new theorems by composing existing Mathlib lemmas
def generate_synthetic_theorem():
    """
    Strategy: pick two related Mathlib lemmas, create a theorem
    that combines them. The proof is the composition.
    """
    lemma1 = random.choice(mathlib_lemmas)
    # Find lemmas whose conclusion matches lemma1's hypothesis
    compatible = [l for l in mathlib_lemmas if types_match(l.conclusion, lemma1.hypothesis)]
    if compatible:
        lemma2 = random.choice(compatible)
        # New theorem: lemma2.hypothesis -> lemma1.conclusion
        return compose_theorem(lemma2, lemma1)
```

## Task Format

**Input**: A Lean 4 theorem statement with `sorry` as the proof.

```lean
import Mathlib

theorem aime_2024_p3 (n : Nat) (h : n = 2024) :
    (Finset.sum (Finset.range n) (fun i => i ^ 2)) % 7 = 3 := by
  sorry
```

**Output**: The model replaces `sorry` with a valid tactic proof.

```lean
  subst h
  native_decide
```

Or a more complex proof:
```lean
  subst h
  simp only [Finset.sum_range_succ]
  ring_nf
  omega
```

## Difficulty Curriculum

| Level | Source | Current SOTA | Scale |
|-------|--------|-------------|-------|
| Easy | Mathlib easy lemmas (one-tactic proofs) | ~80% (ReProver) | ~30K |
| Medium | Mathlib medium (2-5 tactic proofs) | ~40% | ~50K |
| Hard | miniF2F, ProofNet | ~30% (whole-proof gen) | ~800 |
| Very Hard | FIMO (IMO formalized) | <10% | ~150 |
| Superhuman | Open Mathlib issues, unsolved conjectures | ~0% | Unbounded |

Curriculum: start with single-tactic proofs (where `simp`, `omega`, `ring`, or `exact?` suffice), progress to multi-tactic proofs, then to competition-level theorems.

## Limitations & Risks

1. **Sparse reward**: Most proofs are long, and the reward is only given at the end (proof compiles or not). Intermediate tactic feedback (via interactive mode) helps but doesn't fully solve the credit assignment problem.
2. **Slow verification**: Lean 4 type-checking can take seconds to minutes per proof, especially with Mathlib imports. This is orders of magnitude slower than string comparison. RL training requires thousands of rollouts — compilation speed is the bottleneck.
3. **Formalization gap**: Natural-language math and formal math are very different skills. A model that proves theorems in Lean may not explain math in English, and vice versa. Autoformalization (NL -> formal) is an active research area.
4. **Lean 4 ecosystem churn**: Mathlib is under active development. Theorems and tactic names change frequently. Training data has a shelf life.
5. **Overfitting to `simp`/`omega`**: RL agents may learn to try a small set of powerful tactics (`simp`, `omega`, `ring`, `norm_num`, `decide`) on every goal, rather than learning genuine mathematical reasoning. This "tactic spam" strategy has diminishing returns on harder problems.
6. **Limited to existing formalization**: Only formalized mathematics can be used. Vast areas of mathematics have no Lean/Coq formalization yet.

## Connections

- **math-competition.md**: Formalizing competition problems bridges natural-language math and formal proof. miniF2F and FIMO are exactly this bridge.
- **logic-first-order.md**: First-order logic theorem proving is a simpler version of the same idea — the prover is the verifier.
- **logic-formal-specification.md**: TLA+/Alloy model checking is analogous — write a spec, the tool checks it.
- **math-theorem-conjecturing.md**: Generating conjectures that can be formally verified is a natural extension.
- **automated-reasoning.md**: General automated reasoning overlaps with the tactic generation problem.
