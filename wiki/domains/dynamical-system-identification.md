---
domain: Dynamical System Identification
category: synthetic-rule-learning
verification_type: execution
dataset_scale: unlimited (procedurally generated)
difficulty_range: medium/hard/superhuman
modality: text
status: verified
---

# Dynamical System Identification

## Overview
Given time-series observations of a dynamical system (trajectories, phase portraits, Poincaré sections), identify the governing equations. This is "symbolic regression for dynamics" — the NCA paper's insight applied to continuous systems. The agent observes system behavior and must infer the underlying differential equations or discrete maps.

This develops the same "latent rule inference" capability as NCA pre-training, but for continuous/physical systems rather than discrete automata.

## Verification Mechanism
```python
import numpy as np
from scipy.integrate import solve_ivp

def verify(task_type: str, true_system: DynSystem, observations: np.array, answer: Any) -> float:
    if task_type == "identify_equations":
        # Agent proposes equations — verify by simulating and comparing
        proposed_system = parse_equations(answer["equations"])
        
        # Simulate proposed system from same initial conditions
        t_span = observations["t_span"]
        y0 = observations["initial_condition"]
        
        proposed_traj = solve_ivp(proposed_system.rhs, t_span, y0, 
                                  t_eval=observations["t_eval"]).y
        actual_traj = observations["trajectory"]
        
        # Compare trajectories
        rel_error = np.mean(np.abs(proposed_traj - actual_traj) / (np.abs(actual_traj) + 1e-10))
        return max(0, 1 - rel_error)
    
    elif task_type == "predict_trajectory":
        # Predict future states from past observations
        actual_future = true_system.simulate(observations["last_state"], 
                                              answer["prediction_horizon"])
        rel_error = np.mean(np.abs(answer["prediction"] - actual_future) / 
                           (np.abs(actual_future) + 1e-10))
        return max(0, 1 - rel_error)
    
    elif task_type == "classify_behavior":
        # Classify as: fixed point, limit cycle, quasi-periodic, chaotic, divergent
        actual = classify_dynamics(observations)
        return 1.0 if answer == actual else 0.0
    
    elif task_type == "find_fixed_points":
        # Find equilibria of the system
        actual_fps = true_system.find_fixed_points()
        found = sum(1 for fp in actual_fps 
                   if any(np.allclose(fp, afp, atol=0.01) for afp in answer["fixed_points"]))
        return found / len(actual_fps) if actual_fps else 1.0
```

## Dataset Sources
- **Procedural generation**: Generate random ODEs/PDEs with known structure:
  - Linear systems: random matrices A for dx/dt = Ax
  - Nonlinear: Lotka-Volterra, Lorenz, Rössler, Van der Pol with random parameters
  - Hamiltonian systems: random potential functions
  - Coupled oscillators: random coupling matrices
- **DySyS benchmark**: Dynamical system identification dataset.
- **SINDy datasets**: Sparse identification of nonlinear dynamics benchmarks.
- **Strogatz textbook**: Classic dynamical systems with known equations.
- **Physics simulation data**: Trajectories from known physical systems.

## Task Format
- **Input**: "Time series of (x, y) coordinates: [(0, 1.0, 0.0), (0.1, 0.98, 0.20), ...]. Identify the governing differential equations."
- **Output**: "dx/dt = -y, dy/dt = x" (simple harmonic oscillator)

## Difficulty Curriculum
- Level 1: Linear 2D systems (stable/unstable nodes, spirals)
- Level 3: Simple nonlinear systems (logistic, pendulum)
- Level 5: Chaotic systems (Lorenz, Rössler) — identify equations from trajectory
- Level 7: High-dimensional systems (N-body, coupled oscillators)
- Level 9: Partial differential equations from spatiotemporal data
- Level 10: Identify governing equations with noise and missing variables

## Connections
- [[symbolic-regression]] — finding expressions from data (this applies to dynamics)
- [[physics-simulation]] — physics as dynamical system
- [[differential-equations]] — solving the identified equations
- [[cellular-automata-rule-inference]] — discrete analog
- [[control-systems]] — identify plant dynamics for controller design
