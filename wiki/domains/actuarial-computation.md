---
domain: Actuarial & Insurance Computation
category: domain-expert
verification_type: execution
dataset_scale: 50K+ (from SOA exam banks + mortality tables)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Actuarial & Insurance Computation

## Overview
Compute actuarial quantities: life insurance premiums, annuity present values, loss reserves, mortality rates from life tables, ruin probabilities. Verification: numerical computation against known actuarial formulas (commutation functions, survival models).

## Verification Mechanism
```python
import numpy as np

def verify(task_type: str, problem: dict, answer: float) -> float:
    if task_type == "life_insurance":
        # Compute net single premium using commutation functions
        mortality_table = load_mortality_table(problem["table"])
        x = problem["age"]
        n = problem["term"]
        i = problem["interest_rate"]
        
        correct = compute_insurance_premium(mortality_table, x, n, i, problem["type"])
        return 1.0 if abs(answer - correct) / max(abs(correct), 0.001) < 0.001 else 0.0
    
    elif task_type == "annuity":
        correct = compute_annuity_value(problem["table"], problem["age"], 
                                        problem["term"], problem["rate"])
        return 1.0 if abs(answer - correct) / max(abs(correct), 0.001) < 0.001 else 0.0
    
    elif task_type == "loss_model":
        # Expected loss computation
        correct = compute_expected_loss(problem["severity_dist"], problem["frequency_dist"],
                                       problem.get("deductible", 0), problem.get("limit", float('inf')))
        return 1.0 if abs(answer - correct) / max(abs(correct), 0.001) < 0.01 else 0.0
```

## Dataset Sources
- **SOA (Society of Actuaries) exam problems**: Thousands of practice problems (Exams P, FM, IFM, STAM, LTAM).
- **Illustrative Life Table**: Standard actuarial teaching table.
- **Human Mortality Database**: 40+ countries, detailed mortality data.
- **CAS (Casualty Actuarial Society) exam problems**: P&C insurance problems.
- **Actuarial textbook exercises**: Bowers et al., Klugman et al.

## Task Format
- **Input**: "Using the Illustrative Life Table at 5% interest, compute the net single premium for a 20-year term insurance issued to age 30."
- **Output**: "A_{30:20} = 0.0423" (or equivalent numerical answer)

## Difficulty Curriculum
- Level 1: Simple present value, annuity computations
- Level 3: Life insurance premiums, reserves
- Level 5: Loss models with deductibles and limits
- Level 7: Multi-state models, pension valuations
- Level 9: Stochastic reserving, risk measures (VaR, TVaR)

## Connections
- [[financial-calculation]] — financial mathematics
- [[probability-statistics]] — probability theory
- [[survival-analysis]] — survival models
- [[economic-modeling]] — economic valuation
