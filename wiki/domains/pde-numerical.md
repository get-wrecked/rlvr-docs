---
domain: Numerical PDE Solving
category: scientific-computing
verification_type: execution
dataset_scale: 10K+ (from computational science benchmarks)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Numerical PDE Solving

## Overview
Solve PDEs numerically: implement finite difference/element/volume methods, choose appropriate discretizations, set boundary conditions, achieve convergence. Verification: compare numerical solution to analytical solutions (where available) or verify convergence properties.

## Verification Mechanism
```python
import numpy as np

def verify(pde: dict, code: str, analytical_solution: Callable = None) -> float:
    # Execute the numerical solver
    result = execute_in_sandbox(code, pde, timeout=120)
    if result is None:
        return 0.0
    
    numerical_solution = result["solution"]
    grid = result["grid"]
    
    if analytical_solution:
        # Compare to analytical solution
        exact = analytical_solution(grid)
        l2_error = np.sqrt(np.mean((numerical_solution - exact)**2))
        max_error = np.max(np.abs(numerical_solution - exact))
        
        # Error should be within expected convergence rate
        h = grid[1] - grid[0]
        expected_order = pde.get("expected_convergence_order", 2)
        expected_error = h ** expected_order
        
        return 1.0 if max_error < 10 * expected_error else max(0, 1 - max_error)
    
    else:
        # Verify convergence: solve at two resolutions
        coarse = solve_at_resolution(code, pde, n=50)
        fine = solve_at_resolution(code, pde, n=100)
        
        # Richardson extrapolation: check convergence rate
        rate = convergence_rate(coarse, fine)
        return 1.0 if rate > 1.5 else max(0, rate / 2)  # At least 1.5 order convergence
```

## Dataset Sources
- **FEniCS tutorials**: Finite element solver with benchmark problems.
- **Dealii tutorials**: Finite element library benchmarks.
- **CFD benchmarks**: Lid-driven cavity, flow past cylinder, etc.
- **NIST PDE benchmarks**: Standardized verification problems.
- **Textbook exercises**: LeVeque, Strang & Fix.
- **Method of Manufactured Solutions**: Generate PDEs with known analytical solutions.

## Task Format
- **Input**: "Solve the 2D heat equation ∂u/∂t = α∇²u on a unit square with Dirichlet BCs using finite differences"
- **Output**: Python code implementing the solver

## Difficulty Curriculum
- Level 1: 1D heat equation, explicit Euler
- Level 3: 2D Poisson equation, finite differences
- Level 5: Advection-diffusion, implicit methods, stability analysis
- Level 7: Navier-Stokes (simplified), adaptive mesh refinement
- Level 9: Coupled multi-physics, turbulence modeling

## Connections
- [[differential-equations]] — analytical ODE/PDE
- [[fluid-dynamics]] — CFD applications
- [[physics-simulation]] — physics-based PDEs
- [[engineering-optimization]] — shape optimization
