---
domain: Economic Modeling & Market Simulation
category: social-science
verification_type: execution
dataset_scale: 100K+ (from economics datasets + simulation)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Economic Modeling & Market Simulation

## Overview
Solve economics problems with verifiable numerical answers: supply-demand equilibria, optimal pricing, game-theoretic market analysis, portfolio optimization, macroeconomic model solutions. Verification: compute equilibrium conditions, check first-order conditions, verify numerical solutions.

## Verification Mechanism
```python
def verify(task_type: str, problem: dict, answer: dict) -> float:
    if task_type == "equilibrium":
        # Check supply = demand at proposed price
        supply_at_p = eval_supply(problem["supply_fn"], answer["price"])
        demand_at_p = eval_demand(problem["demand_fn"], answer["price"])
        if abs(supply_at_p - demand_at_p) < 0.01:
            return 1.0
        return 0.0
    
    elif task_type == "optimization":
        # Check first-order conditions
        objective = sympy.sympify(problem["objective"])
        for var, value in answer["optimal_point"].items():
            derivative = sympy.diff(objective, var)
            foc_value = float(derivative.subs(answer["optimal_point"]))
            if abs(foc_value) > 0.01:
                return 0.0
        
        # Check second-order conditions (is it actually a maximum/minimum?)
        hessian = compute_hessian(objective, list(answer["optimal_point"].keys()))
        hess_at_point = eval_hessian(hessian, answer["optimal_point"])
        if problem["type"] == "maximize" and not is_negative_definite(hess_at_point):
            return 0.5  # FOC satisfied but not confirmed maximum
        return 1.0
    
    elif task_type == "portfolio":
        # Check portfolio weights sum to 1, expected return/risk matches
        weights = np.array(answer["weights"])
        if abs(weights.sum() - 1.0) > 0.01:
            return 0.0
        expected_return = weights @ problem["expected_returns"]
        risk = np.sqrt(weights @ problem["cov_matrix"] @ weights)
        
        score = 0.0
        if abs(expected_return - answer["claimed_return"]) < 0.001:
            score += 0.5
        if abs(risk - answer["claimed_risk"]) < 0.001:
            score += 0.5
        return score
```

## Dataset Sources
- **Economics textbook problems**: Mankiw, Varian — with numerical solutions.
- **FRED (Federal Reserve Economic Data)**: Real economic data for empirical problems.
- **Kaggle economics datasets**: Stock data, housing prices, etc.
- **Kenneth French data library**: Factor data for financial economics.
- **GRE/GMAT quantitative reasoning**: Economics-flavored problems.
- **Competition economics problems**: Economics olympiad problems.

## Task Format
- **Input**: "Given demand function Q = 100 - 2P and supply function Q = 3P - 50, find the equilibrium price and quantity."
- **Output**: "P = 30, Q = 40"

## Difficulty Curriculum
- Level 1: Simple supply-demand equilibrium
- Level 3: Consumer/producer surplus, tax incidence
- Level 5: General equilibrium, portfolio optimization
- Level 7: Dynamic programming (Bellman equations), mechanism design
- Level 9: DSGE models, structural estimation

## Limitations & Risks
- Economic models are simplifications of reality. Verification checks internal consistency, not real-world accuracy.
- Some problems have multiple equilibria. Accept any valid equilibrium.
- Optimization verification via FOC/SOC is mathematically rigorous.

## Connections
- [[financial-calculation]] — financial computations
- [[multi-agent-coordination]] — game theory in economics
- [[mathematical-modeling]] — economic modeling is applied math
- [[combinatorics-optimization]] — optimization problems
