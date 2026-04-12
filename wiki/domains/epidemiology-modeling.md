---
domain: Epidemiological Modeling
category: science-biology
verification_type: execution
dataset_scale: 10K+ (from epidemic data + simulation)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Epidemiological Modeling

## Overview
Build epidemiological models (SIR, SEIR, agent-based), fit parameters to outbreak data, forecast disease spread, and evaluate intervention strategies. Verification: compare model predictions to held-out real epidemic data, or verify mathematical properties of the model (conservation, steady states).

## Verification Mechanism
```python
from scipy.integrate import odeint
import numpy as np

def verify(task_type: str, problem: dict, solution: dict) -> float:
    if task_type == "parameter_estimation":
        # Run SIR/SEIR model with proposed parameters
        params = solution["parameters"]
        trajectory = simulate_model(problem["model_type"], params, problem["t_span"])
        
        # Compare to observed data
        observed = problem["observed_data"]
        mse = np.mean((trajectory - observed) ** 2)
        baseline_mse = np.var(observed)
        r_squared = 1 - mse / baseline_mse
        return max(0, r_squared)
    
    elif task_type == "basic_reproduction_number":
        # R0 calculation — verifiable analytically for standard models
        R0 = solution["R0"]
        correct_R0 = compute_R0(problem["model_type"], problem["parameters"])
        return 1.0 if abs(R0 - correct_R0) / correct_R0 < 0.01 else 0.0
    
    elif task_type == "intervention_analysis":
        # Simulate with and without intervention
        baseline = simulate_model(problem["model_type"], problem["params"], problem["t_span"])
        with_intervention = simulate_model(
            problem["model_type"], 
            apply_intervention(problem["params"], solution["intervention"]),
            problem["t_span"]
        )
        
        # Check claimed reduction
        claimed_reduction = solution["reduction_percentage"]
        actual_reduction = 1 - with_intervention.total_infected / baseline.total_infected
        return 1.0 if abs(claimed_reduction - actual_reduction) < 0.05 else 0.0
    
    elif task_type == "steady_state":
        # Verify endemic equilibrium
        equilibrium = solution["equilibrium"]
        derivatives = compute_derivatives(problem["model_type"], problem["params"], equilibrium)
        return 1.0 if np.allclose(derivatives, 0, atol=1e-6) else 0.0
```

## Dataset Sources
- **Johns Hopkins COVID-19 data**: Global time series data.
- **WHO disease outbreak database**: Historical epidemic data.
- **CDC FluView**: US influenza surveillance data.
- **GLEAM model outputs**: Global epidemic simulation.
- **MIDAS Network data**: Modeling Infectious Disease Agent Study.
- **Epidemic benchmark models**: Standard SIR/SEIR implementations.
- **Project Tycho**: Historical disease data for the US.

## Task Format
- **Input**: "Given this COVID-19 case data for a city [data], estimate the basic reproduction number R0 and predict the peak date using an SIR model."
- **Output**: Model parameters + R0 estimate + peak date prediction

## Difficulty Curriculum
- Level 1: Simple SIR model, compute R0
- Level 3: SEIR with vaccination, parameter fitting
- Level 5: Age-structured models, spatial spread
- Level 7: Agent-based models with heterogeneous behavior
- Level 9: Real-time epidemic forecasting with multiple data streams

## Limitations & Risks
- Epidemic models are inherently uncertain — real outbreaks depend on behavior changes.
- Verification against held-out data is the strongest test.
- Mathematical properties (R0, steady states) are exactly verifiable.

## Connections
- [[simulation-modeling]] — simulation methods
- [[mathematical-modeling]] — mathematical model construction
- [[probability-statistics]] — statistical fitting
- [[data-science-eda]] — epidemiological data analysis
