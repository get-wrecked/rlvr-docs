---
domain: Voting Theory & Social Choice Computation
category: social-science-formal
verification_type: execution
dataset_scale: 10K+ (PrefLib + procedural)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Voting Theory & Social Choice Computation

## Overview
Compute election outcomes under different voting rules (plurality, Borda, Condorcet, STV/IRV, approval, Schulze), determine properties of preference profiles (Condorcet winners, Pareto optimality), and analyze strategic voting. Verification: compute the winner/ranking using the specified voting rule — single correct answer.

## Verification Mechanism
```python
def verify(task_type: str, profile: dict, answer: Any) -> float:
    if task_type == "compute_winner":
        rule = profile["voting_rule"]
        ballots = profile["ballots"]
        
        if rule == "plurality":
            correct = plurality_winner(ballots)
        elif rule == "borda":
            correct = borda_winner(ballots)
        elif rule == "condorcet":
            correct = condorcet_winner(ballots)  # May be None
        elif rule == "stv":
            correct = stv_winner(ballots, profile["seats"])
        elif rule == "schulze":
            correct = schulze_winner(ballots)
        elif rule == "copeland":
            correct = copeland_winner(ballots)
        
        return 1.0 if answer == correct else 0.0
    
    elif task_type == "manipulation":
        # Check if proposed strategic vote actually changes the outcome
        original_winner = compute_winner(profile)
        manipulated = apply_strategic_vote(profile, answer["strategic_ballot"])
        new_winner = compute_winner(manipulated)
        return 1.0 if new_winner == answer["desired_winner"] and new_winner != original_winner else 0.0
    
    elif task_type == "axiomatic":
        # Check if a voting rule satisfies a given axiom
        return 1.0 if answer == check_axiom(profile, answer["axiom"]) else 0.0
```

## Dataset Sources
- **PrefLib**: Thousands of real-world preference profiles (elections, surveys, competitions).
- **Voting theory textbooks**: Saari, Pacuit, Brandt et al.
- **Procedural generation**: Generate random preference profiles. Unlimited.
- **Real elections**: Australian STV data, Scottish council elections.
- **American Mathematical Monthly voting puzzles**: With solutions.

## Task Format
- **Input**: "5 voters rank candidates A,B,C. Ballots: [ABC, ABC, BCA, BCA, CAB]. Who wins under Borda count?"
- **Output**: "B (scores: A=5, B=7, C=3)"

## Difficulty Curriculum
- Level 1: Plurality, majority winner
- Level 3: Borda count, Condorcet winner
- Level 5: STV/IRV elimination, Schulze method
- Level 7: Strategic manipulation, Gibbard-Satterthwaite examples
- Level 9: Axiomatic analysis, impossibility theorem instances

## Connections
- [[multi-agent-coordination]] — social choice as multi-agent aggregation
- [[combinatorics-optimization]] — computational social choice
- [[legal-reasoning]] — electoral law
