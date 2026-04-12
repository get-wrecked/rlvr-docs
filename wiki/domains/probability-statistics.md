---
domain: Probability & Statistics
category: Math
verification_type: exact_match
dataset_scale: ~20K existing + unlimited procedural
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

## Overview

Probability and statistics problems ask the model to compute exact probabilities, expected values, variances, distributions, confidence intervals, hypothesis test outcomes, and combinatorial counting results. The key RLVR advantage: most probability problems have exact numerical or symbolic answers (fractions, closed-form expressions) that can be verified by direct computation or Monte Carlo simulation.

This domain is partially validated: probability problems appear in GSM8K (simple), MATH (medium-hard), and competition benchmarks (hard). The unique feature is the availability of a secondary verification method — Monte Carlo simulation — which can independently estimate the correct answer even when no closed-form solution is known.

## Verification Mechanism

**Primary method: Exact numerical/symbolic comparison + Monte Carlo cross-check.**

```python
import sympy as sp
from fractions import Fraction
import random
import numpy as np

def verify_probability(model_answer: str, gold_answer: str, 
                       simulation_fn=None, n_simulations=1_000_000) -> float:
    """
    Verify a probability/statistics answer.
    
    Three verification strategies:
    1. Exact match (fraction/decimal comparison)
    2. Symbolic equivalence (for expressions)
    3. Monte Carlo simulation (independent verification)
    """
    # Strategy 1: Exact numerical match
    try:
        model_val = parse_as_fraction(model_answer)
        gold_val = parse_as_fraction(gold_answer)
        
        if model_val == gold_val:
            return 1.0
    except:
        pass
    
    # Strategy 2: Symbolic equivalence
    try:
        model_expr = sp.sympify(model_answer)
        gold_expr = sp.sympify(gold_answer)
        if sp.simplify(model_expr - gold_expr) == 0:
            return 1.0
    except:
        pass
    
    # Strategy 3: Numerical comparison with tolerance
    try:
        model_float = float(sp.sympify(model_answer).evalf())
        gold_float = float(sp.sympify(gold_answer).evalf())
        if abs(model_float - gold_float) < 1e-6:
            return 1.0
    except:
        pass
    
    # Strategy 4: Monte Carlo cross-check (if simulation function provided)
    if simulation_fn is not None:
        mc_estimate = monte_carlo_verify(simulation_fn, model_answer, n_simulations)
        if mc_estimate is not None:
            return mc_estimate
    
    return 0.0


def monte_carlo_verify(simulation_fn, model_answer: str, 
                       n_simulations: int = 1_000_000) -> float:
    """
    Verify an answer using Monte Carlo simulation.
    
    Args:
        simulation_fn: A function that runs one trial and returns 
                       the outcome (e.g., True/False for probability,
                       a number for expected value)
        model_answer: The model's claimed answer
        n_simulations: Number of simulation trials
    """
    model_val = float(sp.sympify(model_answer).evalf())
    
    outcomes = [simulation_fn() for _ in range(n_simulations)]
    
    if all(isinstance(o, bool) for o in outcomes):
        # Probability estimation
        mc_estimate = sum(outcomes) / n_simulations
        # 99.9% confidence interval
        std_err = (mc_estimate * (1 - mc_estimate) / n_simulations) ** 0.5
        margin = 3.29 * std_err  # z = 3.29 for 99.9%
        
        if abs(model_val - mc_estimate) < margin + 1e-4:
            return 1.0
        return 0.0
    else:
        # Expected value estimation
        mc_estimate = np.mean(outcomes)
        std_err = np.std(outcomes) / np.sqrt(n_simulations)
        margin = 3.29 * std_err
        
        if abs(model_val - mc_estimate) < margin + 1e-4:
            return 1.0
        return 0.0


def verify_counting(model_answer: int, gold_answer: int) -> float:
    """Verify a combinatorial counting answer (exact integer)."""
    return 1.0 if model_answer == gold_answer else 0.0


def verify_distribution(model_params: dict, gold_params: dict, 
                        dist_type: str) -> float:
    """
    Verify a distribution identification/fitting answer.
    E.g., "The distribution is Binomial(n=10, p=0.3)"
    """
    if model_params.get("type") != gold_params.get("type"):
        return 0.0
    
    if dist_type == "binomial":
        return 1.0 if (model_params["n"] == gold_params["n"] and 
                        abs(model_params["p"] - gold_params["p"]) < 1e-6) else 0.0
    elif dist_type == "normal":
        return 1.0 if (abs(model_params["mu"] - gold_params["mu"]) < 1e-6 and
                        abs(model_params["sigma"] - gold_params["sigma"]) < 1e-6) else 0.0
    # ... other distributions
```

### Simulation-Based Verification (especially powerful for this domain)

```python
# Example: verify "probability of getting exactly 3 heads in 5 fair coin flips"
def sim_3_heads_5_flips():
    """One trial of the simulation."""
    flips = [random.choice([0, 1]) for _ in range(5)]
    return sum(flips) == 3

# Gold answer: C(5,3) * (1/2)^5 = 10/32 = 5/16 = 0.3125
# Monte Carlo with 1M trials will estimate ~0.3125 ± 0.0005

# Example: verify expected value of max of 3 uniform[0,1] random variables
def sim_max_3_uniform():
    """One trial: return max of 3 uniform RVs."""
    return max(random.random() for _ in range(3))

# Gold answer: E[max(U1,U2,U3)] = 3/4 = 0.75
# Monte Carlo will confirm
```

## Dataset Sources

| Dataset | Size | Source | Notes |
|---------|------|--------|-------|
| **MATH** (probability subset) | ~700 | hendrycks/math | Competition-style probability |
| **MATH** (counting & probability) | ~1,200 | hendrycks/math | Combinatorics + probability |
| **GSM8K** (probability problems) | ~200 | openai/grade-school-math | Simple probability word problems |
| **AMC/AIME probability** | ~500 | AoPS | Competition probability |
| **AP Statistics practice** | ~2K | Various | Standardized test problems |
| **Khan Academy statistics** | ~3K | Khan Academy | Basic to intermediate |
| **StatQuest problems** | ~500 | Various | Applied statistics |
| **Probability textbook exercises** | ~10K | Crawled from various sources | Ross, Sheldon; Pitman; Blitzstein & Hwang |
| **Putnam probability** | ~200 | Various | Hard probability problems |
| **AMPS** (probability/stats) | ~100K | hendrycks | Procedurally generated |

### Procedural Generation

```python
def generate_probability_problem(difficulty="medium"):
    """Generate a probability problem with verified answer."""
    
    if difficulty == "easy":
        # Basic: probability of event in simple sample space
        n_dice = random.randint(1, 2)
        target_sum = random.randint(n_dice, 6 * n_dice)
        
        # Compute exact probability
        count = count_ways_to_sum(n_dice, 6, target_sum)
        total = 6 ** n_dice
        prob = Fraction(count, total)
        
        # Simulation function
        def sim():
            return sum(random.randint(1, 6) for _ in range(n_dice)) == target_sum
        
        return {
            "problem": f"What is the probability that the sum of {n_dice} fair dice equals {target_sum}?",
            "answer": str(prob),
            "simulation_fn": sim
        }
    
    elif difficulty == "medium":
        # Conditional probability, Bayes' theorem
        p_disease = Fraction(random.randint(1, 10), 1000)
        sensitivity = Fraction(random.randint(90, 99), 100)
        specificity = Fraction(random.randint(85, 99), 100)
        
        # P(disease | positive test) via Bayes
        p_positive = p_disease * sensitivity + (1 - p_disease) * (1 - specificity)
        p_disease_given_positive = (p_disease * sensitivity) / p_positive
        
        return {
            "problem": (f"A disease affects {p_disease} of the population. "
                       f"A test has sensitivity {sensitivity} and specificity {specificity}. "
                       f"What is the probability of having the disease given a positive test?"),
            "answer": str(p_disease_given_positive)
        }
    
    elif difficulty == "hard":
        # Expected value via indicator random variables, generating functions, etc.
        n = random.randint(5, 20)
        
        # Coupon collector: expected time to collect all n coupons
        ev = sum(Fraction(n, k) for k in range(1, n + 1))
        
        def sim():
            collected = set()
            draws = 0
            while len(collected) < n:
                collected.add(random.randint(0, n - 1))
                draws += 1
            return draws
        
        return {
            "problem": f"There are {n} distinct coupon types, each equally likely. What is the expected number of coupons you need to collect to get all {n} types?",
            "answer": str(ev),
            "simulation_fn": sim
        }
```

## Task Format

**Input**: Natural language probability/statistics problem.

```
Problem: A bag contains 5 red balls and 3 blue balls. You draw 3 balls 
without replacement. What is the probability that exactly 2 are red?

Expected answer: 15/28
```

```
Problem: Let X and Y be independent uniform random variables on [0, 1]. 
What is E[max(X, Y)]?

Expected answer: 2/3
```

**Output**: Exact probability/value as a fraction or simplified expression.

## Difficulty Curriculum

| Level | Type | Example | Scale |
|-------|------|---------|-------|
| Easy | Single event probability, basic counting | P(head on fair coin) | Unlimited |
| Medium | Conditional probability, Bayes, multi-step counting | Bayes theorem with medical tests | Unlimited |
| Hard | Expected value, indicator RVs, generating functions | Coupon collector, matching derangements | ~5K |
| Very Hard | Continuous distributions, transforms, stochastic processes | Gambler's ruin, Brownian motion hitting times | ~1K |
| Superhuman | Open problems in probability theory | Unsolved combinatorial probability | Open |

## Limitations & Risks

1. **Monte Carlo is slow**: Simulation-based verification requires millions of trials for high precision, taking seconds per problem. This limits RL training throughput compared to instant exact-match verification.
2. **Exact fractions are fragile**: The answer 15/28 must be parsed correctly. "0.5357..." is approximately correct but doesn't match exactly. The verifier must handle both representations.
3. **Ambiguous problem statements**: Probability problems are notoriously ambiguous ("what is the probability of at least one six" — in how many dice? how many rolls?). Problems must be unambiguous.
4. **Distribution assumptions**: Many statistics problems depend on distributional assumptions (normality, independence) that may not be explicitly stated. The model must make the same assumptions as the gold answer.
5. **Continuous vs. discrete**: Continuous probability problems (integrals, density functions) are harder to verify than discrete ones. They overlap with `math-symbolic.md`.

## Connections

- **math-numerical.md**: Probability computations are numerical math. Expected values, binomial coefficients, etc.
- **math-competition.md**: Competition probability/counting is a major category.
- **math-symbolic.md**: Continuous probability involves integrals and symbolic computation.
- **combinatorics-optimization.md**: Counting problems are combinatorics. Stochastic optimization bridges probability and optimization.
- **geometry-computation.md**: Geometric probability (random points in shapes) bridges both domains.
