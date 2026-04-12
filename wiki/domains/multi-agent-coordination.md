---
domain: Multi-Agent Coordination & Game Theory
category: games-strategy
verification_type: simulation
dataset_scale: unlimited (self-play + procedural)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Multi-Agent Coordination & Game Theory

## Overview
Solve game-theoretic problems: compute Nash equilibria, optimal strategies in extensive-form games, mechanism design, auction bidding, negotiation. Verification: check if proposed strategies are actually equilibria (no agent can improve by deviating unilaterally) or satisfy specified solution concepts.

## Verification Mechanism
```python
import nashpy
import numpy as np

def verify_nash_equilibrium(payoff_matrices: tuple, strategy_profile: tuple) -> float:
    """Check if a strategy profile is a Nash equilibrium."""
    game = nashpy.Game(*payoff_matrices)
    s1, s2 = strategy_profile  # Mixed strategies
    
    # Check if s1 is best response to s2
    payoffs_if_deviate_1 = payoff_matrices[0] @ s2
    best_response_payoff_1 = max(payoffs_if_deviate_1)
    current_payoff_1 = s1 @ payoff_matrices[0] @ s2
    
    # Check if s2 is best response to s1
    payoffs_if_deviate_2 = s1 @ payoff_matrices[1]
    best_response_payoff_2 = max(payoffs_if_deviate_2)
    current_payoff_2 = s1 @ payoff_matrices[1] @ s2
    
    epsilon = 1e-6
    is_ne = (best_response_payoff_1 - current_payoff_1 <= epsilon and
             best_response_payoff_2 - current_payoff_2 <= epsilon)
    
    return 1.0 if is_ne else 0.0

def verify_mechanism_design(mechanism: dict, properties: list[str]) -> float:
    """Check if a mechanism satisfies desired properties."""
    score = 0
    for prop in properties:
        if prop == "incentive_compatible":
            if check_ic(mechanism):
                score += 1
        elif prop == "individually_rational":
            if check_ir(mechanism):
                score += 1
        elif prop == "budget_balanced":
            if check_bb(mechanism):
                score += 1
        elif prop == "efficient":
            if check_efficiency(mechanism):
                score += 1
    return score / len(properties)
```

## Dataset Sources
- **Gambit library**: Extensive-form game solver with benchmark games.
- **Game theory textbooks**: Osborne & Rubinstein, Mas-Colell — exercises with solutions.
- **Auction theory benchmarks**: Standard auction formats with known optimal strategies.
- **ANAC (Automated Negotiation Agents Competition)**: Negotiation scenarios.
- **Procedural generation**: Generate random payoff matrices, compute equilibria, use as tasks.
- **StratmasticS/Mathigon**: Educational game theory exercises.

## Task Format
- **Input**: "Find all Nash equilibria of this 3x3 game: [payoff matrix]"
- **Output**: List of strategy profiles (potentially mixed strategies)

## Difficulty Curriculum
- Level 1: 2x2 pure strategy games (dominant strategies)
- Level 3: 2x2 mixed strategy Nash equilibria
- Level 5: 3x3+ games, multiple equilibria
- Level 7: Extensive-form games (backward induction)
- Level 9: Large multi-player games, mechanism design, coalition games

## Limitations & Risks
- Equilibrium computation is PPAD-hard in general. Verification is easier than finding.
- Multiple equilibria may exist — accepting any valid one is fine.
- Mechanism design verification requires checking properties across ALL possible type profiles (can be combinatorial). Focus on finite type spaces.

## Connections
- [[combinatorics-optimization]] — equilibrium computation as optimization
- [[planning-classical]] — strategic planning
- [[chess]] — specific two-player zero-sum game
