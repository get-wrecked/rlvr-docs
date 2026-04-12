---
domain: Cellular Automata & Discrete Dynamical Systems
category: computational-science
verification_type: execution
dataset_scale: unlimited (procedurally generated)
difficulty_range: easy/medium/hard/superhuman
modality: multimodal
status: verified
---

# Cellular Automata & Discrete Dynamical Systems

## Overview
Predict the evolution of cellular automata (Conway's Game of Life, Rule 110, etc.) for N steps, classify rules by behavior type, find initial configurations that produce desired outcomes (inverse CA problem), and analyze properties (periodic, chaotic, etc.).

## Verification Mechanism
```python
import numpy as np

def verify(task_type: str, rule: int, initial_state: np.array, answer: Any) -> float:
    if task_type == "predict_state":
        # Simulate CA for N steps, compare to prediction
        final_state = simulate_ca(rule, initial_state, steps=answer["steps"])
        return 1.0 if np.array_equal(answer["state"], final_state) else 0.0
    
    elif task_type == "find_initial":
        # Given target state, find initial state that leads to it
        evolved = simulate_ca(rule, answer["initial_state"], steps=answer["steps"])
        target = answer["target_state"]
        return 1.0 if np.array_equal(evolved, target) else 0.0
    
    elif task_type == "period_detection":
        # Find the period of a cyclic CA
        states = simulate_ca_until_repeat(rule, initial_state, max_steps=10000)
        actual_period = detect_period(states)
        return 1.0 if answer == actual_period else 0.0
    
    elif task_type == "still_life":
        # Verify that a Game of Life pattern is a still life
        next_state = game_of_life_step(answer["pattern"])
        return 1.0 if np.array_equal(answer["pattern"], next_state) else 0.0
    
    elif task_type == "oscillator":
        # Verify pattern is an oscillator with claimed period
        state = answer["pattern"]
        for _ in range(answer["period"]):
            state = game_of_life_step(state)
        return 1.0 if np.array_equal(state, answer["pattern"]) else 0.0
```

## Dataset Sources
- **LifeWiki**: Thousands of documented Game of Life patterns with properties.
- **Wolfram Atlas of Simple Programs**: Catalog of elementary CA.
- **Catagolue**: Census of Game of Life objects found by random search.
- **Golly pattern collections**: Game of Life pattern files.
- **Procedural generation**: Generate random CA rules and initial states. Unlimited.

## Task Format
- **Input**: "Given this 8x8 Game of Life grid [pattern], what is the state after 5 steps?"
- **Output**: Updated 8x8 grid

## Difficulty Curriculum
- Level 1: Predict 1-2 steps of small CA
- Level 3: Predict 5-10 steps, identify still lifes and oscillators
- Level 5: Predict long-term behavior (periodicity, growth)
- Level 7: Inverse problems (find initial state for target)
- Level 9: Design CA rules with specific computational properties

## Limitations & Risks
- CA simulation is deterministic — verification is exact.
- Long-term prediction is computationally easy to verify but cognitively hard to perform.
- The inverse CA problem is in general undecidable (for Turing-complete CA like Rule 110).

## Connections
- [[program-synthesis-from-io]] — inverse CA as program synthesis
- [[simulation-modeling]] — simulation of discrete systems
- [[automated-reasoning]] — reasoning about evolving patterns
