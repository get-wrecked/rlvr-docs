---
domain: Differential Equations (ODE & PDE)
category: math-analysis
verification_type: execution
dataset_scale: unlimited (procedurally generated)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Differential Equations (ODE & PDE)

## Overview
Solve ordinary and partial differential equations: find closed-form solutions, verify solutions, determine stability, compute numerical solutions. Verification: substitute the proposed solution back into the equation, or compare numerical solutions.

## Verification Mechanism
```python
import sympy
from scipy.integrate import odeint, solve_ivp

def verify(task_type: str, equation: str, solution: str, conditions: dict = None) -> float:
    if task_type == "closed_form_ode":
        # Substitute solution into ODE, check it satisfies the equation
        y_sol = sympy.sympify(solution)
        t = sympy.Symbol('t')
        y = sympy.Function('y')
        
        # Parse ODE: y'' + 3y' + 2y = 0
        ode = sympy.sympify(equation)
        
        # Substitute y(t) = solution
        substituted = ode.subs(y(t), y_sol)
        substituted = substituted.doit()
        result = sympy.simplify(substituted)
        
        if result == 0:
            # Check initial/boundary conditions
            if conditions:
                for cond_var, cond_val in conditions.items():
                    if not check_condition(y_sol, cond_var, cond_val):
                        return 0.5  # Satisfies ODE but not conditions
            return 1.0
        return 0.0
    
    elif task_type == "numerical":
        # Compare numerical solution to reference
        reference = solve_ivp(equation_rhs, conditions["t_span"], 
                             conditions["y0"], dense_output=True)
        proposed = parse_numerical_solution(solution)
        
        # Compare at sample points
        t_eval = np.linspace(*conditions["t_span"], 100)
        ref_vals = reference.sol(t_eval)
        prop_vals = proposed(t_eval)
        
        rel_error = np.mean(np.abs(ref_vals - prop_vals) / (np.abs(ref_vals) + 1e-10))
        return max(0, 1 - rel_error)
    
    elif task_type == "stability":
        # Check stability classification (stable, unstable, center, spiral, etc.)
        correct = classify_equilibrium(equation, conditions["equilibrium"])
        return 1.0 if solution.lower() == correct.lower() else 0.0
    
    elif task_type == "laplace_transform":
        # Verify Laplace transform solution
        F_s = sympy.sympify(solution)
        f_t = sympy.inverse_laplace_transform(F_s, sympy.Symbol('s'), sympy.Symbol('t'))
        # Check original equation
        return 1.0 if verify_ode_solution(equation, f_t, conditions) else 0.0
```

## Dataset Sources
- **MIT OCW 18.03**: ODE course problem sets with solutions.
- **Wolfram Alpha**: ODE/PDE solver with step-by-step solutions.
- **SymPy test suite**: ODE/PDE test cases with known solutions.
- **AMPS dataset**: Math problem solutions including differential equations.
- **Procedural generation**: Generate ODEs with known solutions (work backwards from solutions).
- **Physics/engineering textbooks**: PDEs with known analytical solutions.
- **Navier-Stokes benchmark solutions**: Exact solutions for simplified geometries.

## Task Format
- **Input**: "Solve y'' + 4y' + 4y = 0 with y(0) = 1, y'(0) = 0"
- **Output**: "y(t) = (1 + 2t)e^{-2t}"

## Difficulty Curriculum
- Level 1: First-order separable/linear ODEs
- Level 3: Second-order constant coefficient ODEs
- Level 5: Systems of ODEs, Laplace transforms
- Level 7: Partial differential equations (heat, wave, Laplace)
- Level 9: Nonlinear PDEs, existence/uniqueness proofs

## Limitations & Risks
- CAS (SymPy) can verify most closed-form solutions by substitution — very reliable.
- For numerical solutions, tolerance must be calibrated.
- Some PDEs don't have closed-form solutions — use numerical verification.

## Connections
- [[math-symbolic]] — symbolic manipulation
- [[physics-simulation]] — physics equations
- [[mathematical-modeling]] — modeling with DEs
- [[control-systems]] — control theory uses ODEs
