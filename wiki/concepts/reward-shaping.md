---
concept: Reward Shaping
description: Techniques for designing effective reward signals across RLVR domains
---

# Reward Shaping for RLVR

## Binary vs. Continuous Rewards

**Binary** (0/1): Simplest, cleanest signal. All-or-nothing correctness.
- Pro: No reward hacking on partial credit
- Con: Sparse signal, especially for hard problems

**Continuous** (0.0–1.0): Partial credit for partial correctness.
- Pro: Denser gradient signal, faster learning
- Con: More susceptible to reward hacking

**Recommendation**: Start binary for each domain. Add continuous shaping only if learning is too slow.

## Partial Credit Strategies

### For Code
- Fraction of test cases passed: `passed / total`
- Compilation bonus: +0.1 if code compiles, +0.9 * (tests_passed / total)
- Graduated: first-test-pass bonus to encourage early progress

### For Math
- Binary for final answer
- Optional: partial credit for intermediate steps (requires step-level verification — possible with formal proofs, hard for natural language solutions)

### For Constraint Satisfaction
- Fraction of constraints satisfied
- Weighted by constraint importance
- Binary for hard constraints, partial for soft constraints

### For Agent Tasks
- Milestone rewards: partial credit for reaching intermediate states
- Distance-based: reward proportional to closeness to goal state
- Time penalty: small negative reward per step to encourage efficiency

## Reward Normalization

Across 121+ domains with different reward scales, normalization is critical:

```python
# Per-domain normalization
domain_rewards[domain].append(raw_reward)
mean = running_mean(domain_rewards[domain])
std = running_std(domain_rewards[domain])
normalized_reward = (raw_reward - mean) / (std + epsilon)
```

Or simpler: ensure all domains use [0, 1] rewards.

## Curriculum-Aware Rewards

Within each domain, calibrate difficulty so that the agent succeeds ~30-70% of the time:
- Too easy (>90% success): increase difficulty
- Too hard (<10% success): decrease difficulty
- Sweet spot: 30-70% success rate maximizes learning signal

This is more important than reward shaping — the right difficulty level IS the reward shaping.

## Multi-Domain Balancing

When training across all domains simultaneously:

1. **Equal time**: Allocate equal training steps to each domain → biases toward easy domains
2. **Equal difficulty**: Allocate steps to keep success rate ~50% in each domain → best for learning
3. **Capability-weighted**: Allocate more steps to domains where the agent is weakest → targeted improvement
4. **Information-weighted**: Allocate based on gradient magnitude → most efficient use of compute

Recommendation: Start with equal time, switch to capability-weighted once baselines are established.

## Anti-Reward-Hacking Measures

Common hacking vectors per domain type:

| Domain | Hack | Mitigation |
|--------|------|------------|
| Code | Hardcode expected outputs | Use hidden/random test cases |
| Math | Pattern match answer format without solving | Require intermediate steps or multiple equivalent phrasings |
| QA | Exploit dataset biases | Adversarial evaluation sets |
| Puzzles | Memorize solutions | Procedurally generate new instances |
| Agents | Take shortest path that bypasses intent | Multi-metric evaluation (goal + constraint satisfaction) |
| Format | Satisfy letter of constraints, not spirit | More comprehensive constraint sets |

The strongest defense: **procedurally generate new problems at training time** so the agent can't memorize.
