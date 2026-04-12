---
domain: Negotiation & Bargaining
category: strategic-interaction
verification_type: outcome
dataset_scale: 100K+ (from deal/no-deal datasets, procedural)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Negotiation & Bargaining

## Overview
Negotiate with a counterparty (another agent or rule-based bot) to reach a deal that maximizes your utility. Verification: measure the utility of the agreed deal against the agent's value function.

## Verification Mechanism
```python
def verify(agent_values: dict, opponent_values: dict, 
           dialogue: list[str], deal: dict) -> float:
    if deal is None:
        return 0.0  # No deal reached
    
    # Check deal validity (items allocated correctly)
    for item in agent_values:
        assert deal.get(f"agent_{item}", 0) + deal.get(f"opponent_{item}", 0) == total[item]
    
    # Compute agent utility
    agent_utility = sum(agent_values[item] * deal[f"agent_{item}"] for item in agent_values)
    max_possible = sum(agent_values[item] * total[item] for item in agent_values)
    
    # Normalize to [0, 1]
    return agent_utility / max_possible if max_possible > 0 else 0.0
```

For more sophisticated scoring:
```python
def verify_pareto(deal, agent_values, opponent_values):
    """Check if deal is Pareto efficient."""
    agent_u = compute_utility(deal, agent_values)
    opp_u = compute_utility(deal, opponent_values)
    
    # Check if any reallocation improves both
    for alt_deal in enumerate_deals():
        alt_agent_u = compute_utility(alt_deal, agent_values)
        alt_opp_u = compute_utility(alt_deal, opponent_values)
        if alt_agent_u >= agent_u and alt_opp_u >= opp_u and \
           (alt_agent_u > agent_u or alt_opp_u > opp_u):
            return 0.5  # Not Pareto efficient
    return 1.0  # Pareto efficient
```

## Dataset Sources
- **DealOrNoDeal**: 5K negotiation dialogues with deal outcomes.
- **CraigslistBargain**: 6K price negotiations over Craigslist items.
- **ANAC (Automated Negotiation Agents Competition)**: Competition domains.
- **Multi-issue bargaining datasets**: Academic research datasets.
- **Procedural generation**: Generate random value assignments, have agents negotiate. Unlimited.
- **Self-play**: Agent negotiates with copies of itself or with rule-based opponents of varying strategy.

## Task Format
- **Input**: Your values for items: {books: 3, hats: 1, balls: 2}. Total available: {books: 3, hats: 2, balls: 2}. Negotiate with opponent.
- **Output**: Dialogue turns + final deal proposal

## Difficulty Curriculum
- Level 1: Single-item price negotiation with cooperative opponent
- Level 3: Multi-item negotiation, need to discover opponent's preferences
- Level 5: Partial information, strategic opponents
- Level 7: Multi-party negotiation, coalition formation
- Level 9: Open-ended negotiation with natural language (harder to score)

## Limitations & Risks
- Reward depends on opponent strategy — a weak opponent gives easy high rewards, a strong opponent makes it hard. Need consistent opponent calibration.
- Self-play can lead to degenerate strategies (overly aggressive or overly cooperative).
- Natural language negotiation is harder to score than structured deals. Start with structured.

## Connections
- [[multi-agent-coordination]] — game-theoretic foundations
- [[interactive-fiction]] — dialogue-based interaction
- [[email-tasks]] — professional communication
