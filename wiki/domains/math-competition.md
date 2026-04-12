---
domain: Math Competition
category: Math
verification_type: exact_match
dataset_scale: ~50,000 problems (expandable to ~200K with augmentation)
difficulty_range: medium/hard/superhuman
modality: text
status: verified
---

## Overview

Math competition problems span AMC 8/10/12, AIME, USAMO/IMO, Putnam, and similar national/international olympiads. These are the flagship RLVR domain: problems have unambiguous answers, large archives exist online, and multiple RLVR papers have demonstrated training gains on them. The problems test creative problem-solving across algebra, combinatorics, geometry, and number theory at escalating difficulty levels.

This domain is the single most battle-tested RLVR environment. DeepSeek-R1, Qwen-2.5-Math, and numerous other systems were trained with RL on math competition data using exact-match verification.

## Verification Mechanism

**Primary method: Exact numerical/symbolic match.**

Most competition problems (AMC, AIME) have a single numerical answer. Olympiad problems that request a proof are harder to verify and are typically excluded or reduced to their final numerical answer.

```python
def verify_math_competition(problem, model_answer, gold_answer):
    # Step 1: Extract the final answer from model output
    # Look for \boxed{...}, "the answer is ...", or last numeric expression
    extracted = extract_final_answer(model_answer)
    
    # Step 2: Normalize both answers
    # - Parse as sympy expressions
    # - Expand and simplify
    # - Handle equivalent forms: 1/2 == 0.5, sqrt(2)/2 == 1/sqrt(2)
    normalized_model = normalize_math_expr(extracted)
    normalized_gold = normalize_math_expr(gold_answer)
    
    # Step 3: Check symbolic equivalence
    if sympy.simplify(normalized_model - normalized_gold) == 0:
        return 1.0
    
    # Step 4: Fallback — numerical evaluation with tolerance
    try:
        val_model = float(normalized_model.evalf())
        val_gold = float(normalized_gold.evalf())
        if abs(val_model - val_gold) < 1e-6:
            return 1.0
    except:
        pass
    
    return 0.0
```

**AIME-specific**: Answers are always integers in [0, 999], making verification trivial — parse the integer and compare.

**AMC-specific**: Multiple choice (A-E), so verification is string match on the letter or the corresponding value.

**Olympiad proofs**: Cannot be verified by exact match. Two approaches:
1. Exclude proof problems and keep only "find the value" / "compute" problems.
2. Convert to formal proofs in Lean4 (see `math-formal-proofs.md`) — this is the gold standard but requires formalization effort.

## Dataset Sources

| Dataset | Size | Source | Notes |
|---------|------|--------|-------|
| **MATH** (Hendrycks et al., 2021) | 12,500 | [github.com/hendrycks/math](https://github.com/hendrycks/math) | 7 subjects, 5 difficulty levels. Gold standard. |
| **AMC/AIME archives** | ~3,000 | [artofproblemsolving.com](https://artofproblemsolving.com/wiki/index.php/AMC_Problems_and_Solutions) | AMC 8/10/12 + AIME from 1950s–present |
| **AoPS (Art of Problem Solving)** | ~100,000+ | artofproblemsolving.com community | Crawlable. Mixed quality. Needs dedup vs MATH. |
| **Omni-MATH** (Gao et al., 2024) | ~4,400 | [github.com/KbsdJames/Omni-MATH](https://github.com/KbsdJames/Omni-MATH) | Olympiad-level, harder than MATH |
| **AIME 1983-2024** | ~630 | AoPS wiki | All answers are integers 0-999 |
| **Putnam archive** | ~1,200 | Various | Undergraduate competition, proof-heavy |
| **IMO Shortlist** | ~2,000 | Various compilations | Very hard. Many are proof problems. |
| **NuminaMath** (Numina, 2024) | ~860K | [huggingface.co/datasets/AI-MO/NuminaMath-CoT](https://huggingface.co/datasets/AI-MO/NuminaMath-CoT) | Large-scale aggregation of competition math |
| **AMPS** (Hendrycks et al., 2021) | ~5M (Khan+Mathematica) | [github.com/hendrycks/math](https://github.com/hendrycks/math) | Procedurally generated, easier tier |
| **GSM-Hard** | ~1,300 | HuggingFace | GSM8K with larger numbers |
| **OlympiadBench** (He et al., 2024) | ~8,500 | [github.com/OpenBMB/OlympiadBench](https://github.com/OpenBMB/OlympiadBench) | Physics + Math olympiad |

## Task Format

**Input**: Natural language problem statement, possibly with LaTeX formatting.

```
Problem: Let $S$ be the set of integers between 1 and $2^{40}$ whose binary
expansions have exactly two 1's. If a number is chosen at random from $S$,
the probability that it is divisible by 9 is $p/q$ where $p$ and $q$ are
relatively prime positive integers. Find $p + q$.

Expected answer: 913
```

**Output**: The model produces a chain-of-thought solution ending with a boxed answer.

```
[chain of thought reasoning]
...
The answer is \boxed{913}.
```

## Difficulty Curriculum

| Level | Source | Approx. Solve Rate (GPT-4) | Scale |
|-------|--------|-----------------------------|-------|
| Easy | AMC 8, MATH Level 1-2, GSM8K | 90%+ | ~10K |
| Medium | AMC 10/12, MATH Level 3-4 | 50-80% | ~10K |
| Hard | AIME, MATH Level 5 | 20-50% | ~5K |
| Very Hard | USAMO, Putnam | 5-15% | ~2K |
| Superhuman | IMO P3/P6, Omni-MATH hard | <5% | ~1K |

A natural curriculum: start RL on AMC 8 / MATH Level 1-3, then anneal toward AIME/Olympiad.

## Limitations & Risks

1. **Answer extraction fragility**: Models may produce correct reasoning but format the answer incorrectly. Robust answer extraction is essential — regex for `\boxed{}`, fallback to last number in output, etc. Misgraded examples corrupt the RL signal.
2. **Proof problems excluded**: Many of the hardest olympiad problems require proofs, not numerical answers. Exact-match verification cannot handle these. This biases the domain toward "compute" problems.
3. **Data contamination**: MATH, AMC, AIME problems are heavily represented in pretraining corpora. Models may memorize answers. Mitigation: use held-out recent competitions (AIME 2024+), or procedurally generate variants.
4. **Narrow transfer**: Training on competition math may not transfer well to applied mathematics, scientific reasoning, or real-world problem-solving. The problems are "puzzles" by design.
5. **Reward hacking via answer guessing**: For AIME (integer 0-999), random guessing has ~0.1% success rate. For AMC (5 choices), it's 20%. The RL agent could learn to guess rather than reason, especially early in training.

## Connections

- **math-numerical.md**: Simpler numerical computation. Competition math is a superset in difficulty.
- **math-formal-proofs.md**: Formal verification of olympiad proofs is the "holy grail" — combines the hardness of competition math with the rigor of type-checked proofs. The AI Mathematical Olympiad (AIMO) prize targets this.
- **math-symbolic.md**: Many competition problems involve symbolic manipulation as a subroutine.
- **combinatorics-optimization.md**: Competition combinatorics problems overlap with optimization.
- **number-theory-computation.md**: Number theory competition problems overlap heavily.
