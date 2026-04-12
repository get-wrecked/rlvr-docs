---
domain: Physics Simulation
category: Science
verification_type: Numerical simulation with tolerance comparison
dataset_scale: Medium (10K-100K problems feasible)
difficulty_range: High-school mechanics to graduate-level quantum/statistical mechanics
modality: Text-in, text-out (numerical predictions); optionally structured JSON
status: Promising — verification is strong for classical mechanics, weaker for complex systems
---

## Overview

Physics simulation tasks ask the model to predict quantitative outcomes of physical scenarios: trajectories, forces, energies, field values, thermodynamic quantities, and more. The key RLVR insight is that physics has ground truth computable by simulation engines. If a model predicts "the ball lands at x=3.2m after 1.4s," we can run a physics engine and check.

This domain spans classical mechanics (projectile motion, collisions, rigid body dynamics), electromagnetism (field calculations, circuit behavior), thermodynamics (heat transfer, phase transitions), and portions of quantum mechanics (energy levels, tunneling probabilities). The unifying property is that each problem has a numerically computable answer.

## Verification Mechanism

**Primary approach:** Run a physics engine (PyBullet, MuJoCo, or custom numerical solvers) with the problem's initial conditions and compare the model's prediction to the simulation output.

- **Tolerance-based comparison:** For continuous quantities (position, velocity, energy), verification checks whether the model's answer falls within an acceptable tolerance (e.g., relative error < 1%, or absolute error < 0.01 units). The tolerance must be chosen carefully per problem type.
- **Exact verification for discrete answers:** Some physics problems have integer or categorical answers (number of bounces, direction of force, which object arrives first). These can be verified exactly.
- **Conservation law checks:** Even without full simulation, verify that the model's answer conserves energy, momentum, charge, etc. This is a necessary but not sufficient check.

**Verification reliability: HIGH for classical mechanics.** Newtonian physics engines are deterministic and well-validated. For simple scenarios (projectiles, springs, collisions), this is rock-solid verification.

**Verification reliability: MODERATE for complex systems.** Chaotic systems, turbulence, many-body quantum mechanics — simulation itself becomes approximate. The "ground truth" from simulation may carry numerical error, and small perturbations in initial conditions can produce wildly different outcomes.

**Verification reliability: LOW for problems requiring idealized assumptions.** Many textbook physics problems assume frictionless surfaces, point masses, uniform fields. If the verification engine uses realistic physics, there is a mismatch. The engine must be configured to match the problem's assumptions.

## Dataset Sources

- **Physics textbook problems:** Halliday/Resnick, Griffiths (E&M), Kleppner/Kolenkow. Thousands of problems with known solutions. Require digitization and formalization.
- **PhysicsQA / SciQ:** Existing QA datasets with physics questions, though many are conceptual rather than computational.
- **MMLU physics subsets:** Multiple-choice physics problems, limited in depth.
- **Synthetic generation:** Parameterize problem templates (e.g., "A block of mass M slides down a ramp of angle theta...") and generate thousands of instances with different parameter values. This is the most scalable approach.
- **Competition problems:** Physics Olympiad problems (IPhO, USAPhO) provide challenging, well-defined problems with exact solutions.

**Realistic scale:** 10K-50K well-formed problems with simulation-verified answers is achievable. Synthetic generation can push this to 100K+.

## Task Format

**Input:** Natural language problem statement with all necessary physical parameters specified numerically.

Example:
```
A ball is launched from ground level at 25 m/s at an angle of 40 degrees
above the horizontal. Assuming no air resistance and g = 9.81 m/s^2,
what is the maximum height reached by the ball (in meters)?
```

**Output:** Numerical answer, optionally with units and intermediate steps.

```
12.47 m
```

**Verification:** Run kinematic equations or physics engine with given parameters. Compare to model output within tolerance (e.g., +/- 0.1m or 1% relative error).

## Difficulty Curriculum

1. **Level 1 — Single-concept problems:** Direct application of one formula (F=ma, v=d/t, KE=0.5mv^2). Single-step computation.
2. **Level 2 — Multi-step problems:** Combine 2-3 concepts (projectile motion requires decomposing velocity + kinematics in two axes). Require setting up and solving a system.
3. **Level 3 — Constrained systems:** Pulleys, inclined planes with friction, coupled oscillators. Require free-body diagrams and constraint equations.
4. **Level 4 — Advanced topics:** Rotational dynamics, electromagnetic induction, thermodynamic cycles. Require deeper domain knowledge and multi-step reasoning.
5. **Level 5 — Research-adjacent:** Chaotic systems, relativistic corrections, quantum tunneling probabilities. Verification itself becomes approximate.

## Limitations & Risks

- **Tolerance tuning is non-trivial.** Too tight and correct-but-rounded answers get rejected. Too loose and wrong answers pass. This must be calibrated per problem type and per difficulty level.
- **Floating-point representation:** Models may output "12.5" or "12.47" or "12.473." The verification must handle varying precision gracefully.
- **Idealization mismatch:** Textbook problems assume ideal conditions. If the simulation engine includes realistic physics (air resistance, finite-size objects), answers will differ. The engine must be configured to match problem assumptions exactly.
- **Chaotic sensitivity:** For problems involving chaotic dynamics, tiny differences in the model's intermediate computations can produce large differences in final answers. These problems should be flagged or excluded.
- **Reward hacking risk:** The model could learn to pattern-match common physics problem structures without genuine physical reasoning. Curriculum diversity is essential to mitigate this.
- **Unit handling:** The model may give correct numbers with wrong units, or convert units incorrectly. Verification should check units where possible.

## Connections

- Closely related to **engineering-optimization.md** (physics underlies structural optimization)
- Overlaps with **fluid-dynamics.md** for mechanics-of-fluids problems
- **control-systems.md** relies on physics simulation for plant models
- Shares verification philosophy with **chemistry-computation.md** (both use deterministic computation as ground truth)
- Synthetic dataset generation approach applicable across all science domains
