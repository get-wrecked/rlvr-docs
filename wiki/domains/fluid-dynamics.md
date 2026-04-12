---
domain: Fluid Dynamics (CFD)
category: Science/Engineering
verification_type: Numerical simulation (OpenFOAM, Fluent) against benchmark solutions
dataset_scale: Small (1K-10K; each problem is computationally expensive)
difficulty_range: Pipe flow to turbulent multi-phase flows
modality: Text-in, structured-out (flow quantities, boundary conditions, design parameters)
status: Feasible but expensive — verification requires running CFD solvers, limiting scale
---

## Overview

Fluid dynamics tasks ask the model to predict flow behavior, design fluid systems, or solve CFD-related problems: predict drag coefficients, design piping systems, determine flow regimes, estimate heat transfer coefficients, and analyze aerodynamic configurations. The RLVR opportunity is that fluid behavior is governed by the Navier-Stokes equations, which can be solved numerically to provide ground truth. The challenge is that CFD solutions are computationally expensive and require careful setup.

This domain covers internal flows (pipe flow, duct flow, channel flow), external flows (flow over bodies, boundary layers, wakes), heat transfer (convection, conjugate heat transfer), multiphase flows (free surface, droplets, bubbles), and turbulence modeling. Standard benchmarks like the lid-driven cavity, backward-facing step, and flow over a cylinder provide well-characterized reference solutions.

## Verification Mechanism

**Primary approach:** Run CFD simulation with problem parameters and compare model predictions to simulation results or established benchmark solutions.

- **Benchmark solution comparison:** For standard problems (lid-driven cavity at specific Re, flow over cylinder), high-resolution reference solutions exist in the literature. Compare model predictions to these tabulated values.
- **Analytical solution comparison:** For simple geometries (Poiseuille flow, Couette flow, Stokes flow), exact analytical solutions exist. Compare directly.
- **Correlation-based verification:** Many engineering fluid problems have well-established empirical correlations (Moody chart for friction factor, Churchill-Bernstein for Nusselt number). Verify model output against these correlations.
- **CFD simulation verification:** For novel geometries, run OpenFOAM or equivalent solver. Compare velocity/pressure fields, integral quantities (drag, lift, heat flux) to model predictions.
- **Dimensional analysis checks:** Verify that answers have correct dimensions, Reynolds number dependencies match expected scaling, and limiting cases are consistent.

**Verification reliability: VERY HIGH for analytical solutions.** Poiseuille flow, Stokes drag, potential flow — exact answers exist.

**Verification reliability: HIGH for standard benchmark problems.** Lid-driven cavity, backward-facing step, flow over cylinder have grid-converged solutions in the literature verified by many independent groups.

**Verification reliability: MODERATE for correlation-based verification.** Correlations have stated ranges of validity and typical accuracies (e.g., friction factor correlations are accurate to ~5%). Outside their validity range, they may be inaccurate.

**Verification reliability: MODERATE for general CFD.** CFD solutions depend on mesh quality, turbulence model choice, boundary conditions, and numerical schemes. Different setups can give different answers for the same physical problem, especially for turbulent flows. The "ground truth" must be defined as "the answer from a specific well-defined simulation setup."

## Dataset Sources

- **Benchmark databases:** ERCOFTAC, NASA Turbulence Modeling Resource, AGARD test cases. Provide well-documented validation cases with reference data.
- **Textbook problems:** Kundu/Cohen, White, Cengel/Cimbala provide hundreds of computational problems with known solutions. Mostly based on correlations and analytical solutions.
- **OpenFOAM tutorials:** ~50 tutorial cases covering diverse flow types. Can be parameterized for additional instances.
- **NASA TMR (Turbulence Modeling Resource):** Grid-converged solutions for standard turbulent flow cases. High quality.
- **Synthetic generation:** Parameterize pipe flow (diameter, velocity, fluid properties) or external flow (body shape, freestream conditions) problems. Scale limited by CFD cost per problem.

**Realistic scale:** 5K-10K problems for correlation/analytical verification. Only 500-2K for full CFD-verified problems (due to computational cost of ~minutes to hours per simulation).

## Task Format

**Input:** Flow problem description with geometry, fluid properties, and boundary conditions.

Example 1 (pipe flow):
```
Water (density = 998 kg/m^3, viscosity = 1.002e-3 Pa.s) flows through
a smooth horizontal pipe of diameter 0.05 m and length 10 m at a mean
velocity of 2 m/s. What is the pressure drop (in Pa)?
```

Example 2 (external flow):
```
Air at 20°C flows over a smooth sphere of diameter 0.1 m at 5 m/s.
What is the drag coefficient? (Air properties: rho = 1.204 kg/m^3,
mu = 1.825e-5 Pa.s)
```

Example 3 (benchmark):
```
For a 2D lid-driven cavity at Re = 1000, what is the x-velocity at the
geometric center of the cavity (using the top lid moving at U = 1)?
```

**Output:** Numerical values (pressure, velocity, force, dimensionless numbers).

## Difficulty Curriculum

1. **Level 1 — Analytical solutions:** Poiseuille flow, Couette flow, hydrostatic pressure. Exact formulas.
2. **Level 2 — Correlation-based problems:** Friction factor from Moody chart, heat transfer coefficients, drag on standard bodies. Look up and apply correlations.
3. **Level 3 — Multi-step flow analysis:** Pipe network analysis, boundary layer calculations, natural convection estimates. Combine multiple concepts.
4. **Level 4 — Benchmark CFD problems:** Predict flow quantities for standard benchmark geometries (lid-driven cavity, backward-facing step). Requires understanding of CFD concepts.
5. **Level 5 — Complex flows:** Turbulent flows with separation, multiphase flows, reacting flows. Verification is approximate and expensive.

## Limitations & Risks

- **Computational cost is the dominant limitation.** Running CFD for each candidate answer is expensive (minutes to hours). This limits both training scale and verification throughput. Consider pre-computing reference solutions for a fixed problem set.
- **Turbulence model dependence:** For turbulent flows, the CFD answer depends heavily on the chosen turbulence model (k-epsilon, k-omega SST, LES, DNS). Different models give different "ground truth." The problem must specify the turbulence model or target DNS/experimental data.
- **Mesh dependence:** CFD results are mesh-dependent. Coarse meshes give inaccurate results; fine meshes are expensive. Grid-convergence studies are needed to establish reliable reference values.
- **Setup sensitivity:** Small changes in boundary conditions, domain size, or numerical schemes can significantly affect results, especially for separated and turbulent flows.
- **Correlation validity ranges:** Many textbook problems are solved using correlations that have limited validity ranges (specific Re ranges, geometry constraints). The model might apply correlations outside their range without warning.
- **3D vs. 2D:** Many benchmark problems are 2D. Real flows are 3D. The simplification must be explicit.

## Connections

- **physics-simulation.md** provides the underlying continuum mechanics
- **climate-weather.md** is essentially atmospheric fluid dynamics — shares the Navier-Stokes foundation
- **engineering-optimization.md** connects for aerodynamic shape optimization
- **control-systems.md** relates for flow control problems (active drag reduction, mixing enhancement)
- **materials-science.md** connects for fluid-structure interaction and material processing flows
