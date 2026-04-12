---
domain: Engineering Optimization
category: Engineering
verification_type: FEA simulation confirms constraints; objective function evaluation
dataset_scale: Small-to-Medium (1K-20K benchmark problems)
difficulty_range: Single-variable sizing to multi-objective topology optimization
modality: Text-in, structured-out (design variables, dimensions, material selections)
status: Promising — verification is solid but computationally expensive
---

## Overview

Engineering optimization tasks ask the model to find designs that optimize an objective (minimize weight, minimize cost, maximize stiffness) while satisfying engineering constraints (stress limits, deflection limits, buckling, manufacturability). The RLVR value is that constraints can be verified by simulation (FEA for structural, CFD for thermal/fluid), and the objective function is numerically computable. There is no subjective judgment — either the design meets all constraints and achieves a good objective, or it does not.

This domain covers structural optimization (truss sizing, beam design, plate thickness), mechanical design (gear trains, linkages, springs), thermal design (heat exchangers, fin optimization), and multi-disciplinary optimization. The common thread is a well-defined objective function and verifiable constraints.

## Verification Mechanism

**Primary approach:** Evaluate the model's design through engineering simulation and check both constraint satisfaction and objective value.

- **FEA-based constraint verification:** Mesh the design, apply loads and boundary conditions, solve. Check that maximum stress < allowable stress, maximum deflection < limit, natural frequency > minimum, etc.
- **Objective function evaluation:** Compute the objective (total mass, total cost, maximum displacement) from the design variables. This is typically a direct computation.
- **Feasibility check:** Verify that design variables are within allowable ranges (positive thicknesses, available material grades, standard sizes).
- **Optimality assessment:** Compare the model's objective value to known optimal solutions (for benchmark problems) or to a population of solutions. Award higher reward for better objectives among feasible designs.
- **Multi-objective verification:** For Pareto-optimal problems, check whether the design is Pareto-dominated by known solutions. Non-dominated designs receive positive reward.

**Verification reliability: HIGH for constraint checking.** FEA is a mature, validated technology for structural and thermal analysis. Whether a design exceeds a stress limit is objectively determinable.

**Verification reliability: HIGH for objective function evaluation.** Mass, cost, and other objectives are direct functions of design variables.

**Verification reliability: MODERATE for optimality assessment.** Knowing whether a feasible design is "good" vs. "optimal" requires comparison to known solutions. For well-studied benchmarks, optima are known. For novel problems, we can only verify feasibility, not optimality.

## Dataset Sources

- **Structural optimization benchmarks:** 10-bar truss, 25-bar truss, 72-bar truss, and other classic problems from the optimization literature. Well-studied with known optimal solutions.
- **CEC competition problems:** Congress on Evolutionary Computation benchmark suites for constrained optimization.
- **Engineering textbook problems:** Arora, Haftka/Gurdal provide standard optimization problems from structural, mechanical, and aerospace engineering.
- **AIAA/ASME benchmark problems:** Published benchmark problems from engineering societies.
- **Synthetic generation:** Parameterize structural configurations (loads, supports, material properties, allowable stresses) and generate instances. Each instance requires an FEA solve for verification.

**Realistic scale:** 5K-20K problems. Each verification requires a FEA solve (seconds to minutes per problem). This computational cost limits scale compared to domains with instant verification.

## Task Format

**Input:** Design problem with objective, constraints, and parameters.

Example 1 (truss sizing):
```
A 10-bar truss structure supports loads at specific nodes. Each bar
has cross-sectional area as a design variable (range: 0.1 to 35 in^2).
Material: aluminum (E = 10,000 ksi, density = 0.1 lb/in^3, stress limit = 25 ksi).
Minimize total weight while ensuring:
- Stress in every bar < 25 ksi
- Displacement at every node < 2.0 inches
[Load and geometry specification follows]
Give the 10 cross-sectional areas.
```

Example 2 (beam design):
```
Design a simply supported steel beam of length 10 m carrying a uniform
load of 20 kN/m. Choose a standard W-shape from the AISC database that
minimizes weight while keeping:
- Maximum bending stress < 250 MPa
- Maximum deflection < L/360
```

**Output:** Design variable values, selected components, or optimal objective value.

## Difficulty Curriculum

1. **Level 1 — Single-variable optimization:** Find optimal thickness of a pressure vessel, optimal diameter of a shaft. One design variable, one or two constraints.
2. **Level 2 — Small discrete/continuous problems:** 3-5 design variables with multiple constraints. Truss sizing, spring design.
3. **Level 3 — Medium-scale problems:** 10-25 design variables (e.g., 25-bar truss). Multiple load cases and constraint types.
4. **Level 4 — Multi-objective and discrete problems:** Pareto-optimal design with conflicting objectives. Standard size selection from catalogs.
5. **Level 5 — Topology optimization and MDO:** Determine both topology and sizing. Multi-disciplinary problems coupling structural, thermal, and fluid domains.

## Limitations & Risks

- **Computational cost of verification:** Each candidate design requires a FEA solve. This is orders of magnitude slower than checking a math equation. Batch verification and efficient FEA solvers are essential.
- **FEA setup complexity:** Different problems require different element types, mesh densities, boundary conditions, and material models. A general-purpose verification system is hard to build — each problem may need a custom FEA setup.
- **Discrete variables:** Many real engineering problems involve discrete choices (standard bolt sizes, material grades, integer number of components). These create combinatorial search spaces that are harder for both the model and the verifier.
- **Non-unique optima:** Many optimization problems have multiple local optima (especially for topology). The model may find a different local optimum than the reference. Verification should assess feasibility and objective quality, not specific design match.
- **Approximation in FEA:** FEA is approximate (mesh-dependent). Coarser meshes give faster verification but less accurate constraint checking. The mesh density used for verification must be defined as part of the problem.
- **Problem encoding:** Communicating a structural optimization problem (geometry, loads, BCs, material, constraints) entirely in text is verbose and error-prone. Standardized formats help.

## Connections

- **physics-simulation.md** provides the underlying mechanics that FEA solves
- **materials-science.md** provides material property data for design
- **fluid-dynamics.md** connects for thermal-fluid optimization problems
- **control-systems.md** shares optimization concepts (LQR is control optimization)
- **circuit-design.md** has an analogous structure: design-then-simulate-to-verify
