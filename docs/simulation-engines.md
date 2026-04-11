# Simulation & Verification Engines

A comprehensive catalog of every physics, chemistry, biology, engineering, and mathematical simulation engine available for programmatic verification of AI model outputs. Each entry includes the URL, license/cost, what it excels at, and pros/cons for use as an RLVR verifier.

---

## Table of Contents

1. [Physics & Robotics Simulation](#1-physics--robotics-simulation)
2. [Fluid Dynamics / CFD](#2-fluid-dynamics--cfd)
3. [Finite Element Analysis](#3-finite-element-analysis)
4. [Chemistry & Molecular Simulation](#4-chemistry--molecular-simulation)
5. [Quantum Chemistry / DFT](#5-quantum-chemistry--dft)
6. [Molecular Dynamics](#6-molecular-dynamics)
7. [Drug Design & Docking](#7-drug-design--docking)
8. [Retrosynthesis & Reaction Prediction](#8-retrosynthesis--reaction-prediction)
9. [Protein Structure & Design](#9-protein-structure--design)
10. [DNA/RNA Tools](#10-dnarna-tools)
11. [Genomics & Bioinformatics](#11-genomics--bioinformatics)
12. [Circuit Simulation (SPICE)](#12-circuit-simulation-spice)
13. [Chip Design / RTL / HDL](#13-chip-design--rtl--hdl)
14. [Control Systems & Signal Processing](#14-control-systems--signal-processing)
15. [Quantum Computing Simulators](#15-quantum-computing-simulators)
16. [Formal Proof & Theorem Proving](#16-formal-proof--theorem-proving)
17. [SMT & SAT Solvers](#17-smt--sat-solvers)
18. [Computer Algebra Systems](#18-computer-algebra-systems)
19. [Constraint & Optimization Solvers](#19-constraint--optimization-solvers)
20. [Graph & Network Analysis](#20-graph--network-analysis)
21. [Climate, Weather & Geospatial](#21-climate-weather--geospatial)
22. [Game Engines & RL Environments](#22-game-engines--rl-environments)
23. [Materials Science](#23-materials-science)
24. [Epidemiology & Systems Biology](#24-epidemiology--systems-biology)
25. [Power Systems & Electrical](#25-power-systems--electrical)
26. [Recommendation Matrix](#26-recommendation-matrix)

---

## 1. Physics & Robotics Simulation

### MuJoCo (Multi-Joint dynamics with Contact)

| | |
|---|---|
| **URL** | https://github.com/google-deepmind/mujoco |
| **License** | Apache 2.0 (fully open source since 2022, acquired by DeepMind) |
| **Cost** | **Free** |
| **Language** | C core, first-class Python bindings (`mujoco` pip package) |
| **Install** | `pip install mujoco` |

**What it's great at**: The gold standard for contact-rich robotics simulation. Fast, accurate, and deterministic. Used by virtually every RL robotics paper. Supports articulated bodies, tendons, actuators, sensors, and complex contact dynamics. MuJoCo 3.x added GPU-accelerated ray tracing and improved soft-body support.

**MJX (MuJoCo XLA)**: JAX-accelerated version running MuJoCo physics entirely on GPU/TPU. Enables massively parallel simulation (thousands of environments simultaneously). Part of the main `mujoco` package.

| Pros | Cons |
|------|------|
| Fastest CPU rigid-body simulator | Limited soft-body/fluid support |
| MJX enables massive GPU parallelism | MJCF model format has learning curve |
| Deterministic (same seed = same result) | Not designed for high-fidelity rendering |
| Excellent Python API | Smaller community than some alternatives |
| Huge ecosystem (Gymnasium, dm_control) | |
| Active DeepMind development | |

**RLVR verifier quality**: **Excellent**. Deterministic, fast, scriptable. Can verify: robot trajectories, control policies, physical reasoning about forces/torques, locomotion tasks.

---

### NVIDIA Isaac Sim / Isaac Lab

| | |
|---|---|
| **URL** | https://developer.nvidia.com/isaac-sim |
| **License** | Proprietary (free for individual/research via NVIDIA Omniverse) |
| **Cost** | **Free** for individuals and researchers. Enterprise licensing for production. Requires NVIDIA RTX GPU. |
| **Language** | Python (built on Omniverse/USD) |

**What it's great at**: Photorealistic robotics simulation with GPU-accelerated physics (PhysX 5). Domain randomization, synthetic data generation, digital twin workflows. Isaac Lab (formerly Isaac Gym / Orbit) provides the RL training interface.

| Pros | Cons |
|------|------|
| Photorealistic rendering (ray tracing) | Requires NVIDIA RTX GPU (expensive) |
| GPU-accelerated parallel simulation | Heavy install (~30GB) |
| Rich sensor simulation (LIDAR, cameras) | Proprietary, vendor lock-in |
| USD/Omniverse ecosystem | Steeper learning curve |
| Best-in-class for sim-to-real transfer | Linux only (for full features) |

**RLVR verifier quality**: **Very Good** for vision-based robotics. Overkill for pure physics verification — use MuJoCo instead unless you need rendering.

---

### NVIDIA Isaac Lab (successor to Isaac Gym)

| | |
|---|---|
| **URL** | https://github.com/isaac-sim/IsaacLab |
| **License** | BSD 3-Clause (open source) |
| **Cost** | **Free** (requires Isaac Sim + NVIDIA GPU) |
| **Language** | Python (PyTorch tensor interface) |

**What it's great at**: Massively parallel RL training environments built on Isaac Sim. Direct GPU tensor API — no CPU-GPU transfer bottleneck. Thousands of environments on one GPU. Locomotion, manipulation, dexterous grasping.

| Pros | Cons |
|------|------|
| Extreme throughput for parallel verification | Requires Isaac Sim (heavy) |
| Direct PyTorch tensor access | NVIDIA GPU only |
| Gymnasium-compatible wrappers | Isaac Gym standalone is deprecated |
| Growing library of tasks | |

**RLVR verifier quality**: **Excellent** for parallel robotics verification at scale.

---

### NVIDIA Warp

| | |
|---|---|
| **URL** | https://github.com/NVIDIA/warp |
| **License** | Apache 2.0 (open source) |
| **Cost** | **Free** |
| **Language** | Python (JIT-compiled to CUDA) |
| **Install** | `pip install warp-lang` |

**What it's great at**: A Python framework for writing high-performance GPU simulation code. Write physics kernels in Python, they get JIT-compiled to CUDA. Differentiable by default. Great for custom physics, cloth, particles, FEM, and any simulation you want to write from scratch.

| Pros | Cons |
|------|------|
| Write GPU kernels in Python | Requires NVIDIA GPU |
| Auto-differentiable | Smaller community than MuJoCo |
| Very flexible (write any physics) | You build the simulator yourself |
| Fast compilation | Less "batteries included" than MuJoCo |
| Good documentation | |

**RLVR verifier quality**: **Good** for custom physics verification tasks. Best when you need a specific simulation that doesn't exist in MuJoCo.

---

### Brax (Google DeepMind)

| | |
|---|---|
| **URL** | https://github.com/google/brax |
| **License** | Apache 2.0 |
| **Cost** | **Free** |
| **Language** | Python/JAX |
| **Install** | `pip install brax` |

**What it's great at**: JAX-based physics engine designed specifically for RL. Massively parallel (tens of thousands of environments on a single TPU/GPU). Multiple backends: `spring`, `positional`, `generalized`, and `mjx` (MuJoCo).

| Pros | Cons |
|------|------|
| Extreme parallelism via JAX | Less physically accurate than MuJoCo |
| Hardware-accelerated (GPU/TPU) | Limited to rigid-body dynamics |
| Differentiable physics | Fewer environment models available |
| Tight integration with JAX RL ecosystem | Spring backend has known quirks |
| MJX backend bridges to MuJoCo accuracy | |

**RLVR verifier quality**: **Good**. Best for fast batch verification of control/locomotion tasks. Use MJX backend for accuracy.

---

### PyBullet / Bullet3

| | |
|---|---|
| **URL** | https://github.com/bulletphysics/bullet3 |
| **License** | zlib (very permissive, open source) |
| **Cost** | **Free** |
| **Language** | C++, Python bindings |
| **Install** | `pip install pybullet` |

**What it's great at**: General-purpose physics with good robotics support. URDF/SDF model loading, rigid body dynamics, soft body, deformable objects. Widely used in older RL work. Built-in OpenGL renderer.

| Pros | Cons |
|------|------|
| Very permissive license | Slower than MuJoCo |
| Good URDF support | Less actively developed (mature) |
| Soft body support | Python API is less ergonomic |
| Built-in renderer | Contact model less accurate than MuJoCo |
| Large model library | Being superseded by MuJoCo in RL |

**RLVR verifier quality**: **Good**. Reliable for basic physics verification. MuJoCo is generally preferred for new work.

---

### Drake (MIT Robot Locomotion Group)

| | |
|---|---|
| **URL** | https://github.com/RobotLocomotion/drake |
| **License** | BSD 3-Clause |
| **Cost** | **Free** |
| **Language** | C++, Python bindings (`pydrake`) |
| **Install** | `pip install drake` |

**What it's great at**: Multi-body dynamics with a focus on **mathematical rigor**. Contact modeling via hydroelastic contact. Includes optimization solvers (SNOPT, IPOPT, Mosek interfaces), trajectory optimization, systems theory. Used for formal control verification.

| Pros | Cons |
|------|------|
| Mathematically rigorous contact model | Heavier than MuJoCo |
| Built-in optimization/planning | Steeper learning curve |
| Excellent for control systems | Fewer RL integrations |
| Systems framework (block diagrams) | Slower for simple tasks |
| Hydroelastic contact is state-of-the-art | |

**RLVR verifier quality**: **Excellent** for control theory verification, trajectory optimization, and tasks requiring mathematical guarantees.

---

### Project Chrono

| | |
|---|---|
| **URL** | https://github.com/projectchrono/chrono |
| **License** | BSD 3-Clause |
| **Cost** | **Free** |
| **Language** | C++, Python bindings (PyChrono) |

**What it's great at**: Multi-physics simulation: rigid body, FEM, granular, fluid (SPH), vehicle dynamics. Strong for vehicle/terrain interaction, tracked vehicles, autonomous driving simulation.

| Pros | Cons |
|------|------|
| Multi-physics in one engine | Complex build system |
| Vehicle dynamics module | Smaller community |
| Granular/terrain simulation | Python bindings less mature |
| GPU acceleration (Chrono::GPU) | Documentation can be sparse |

**RLVR verifier quality**: **Good** for vehicle dynamics, terrain interaction, and multi-physics problems.

---

### Taichi Lang

| | |
|---|---|
| **URL** | https://github.com/taichi-dev/taichi |
| **License** | Apache 2.0 |
| **Cost** | **Free** |
| **Language** | Python (JIT to CPU/CUDA/Vulkan/Metal) |
| **Install** | `pip install taichi` |

**What it's great at**: High-performance parallel computing in Python. Differentiable programming. Great for MPM (Material Point Method), SPH, FEM, and visual effects physics. Cross-platform GPU support (not NVIDIA-only).

| Pros | Cons |
|------|------|
| Cross-platform GPU (CUDA, Vulkan, Metal) | Company had layoffs; slower development |
| MPM/particle simulation is excellent | Not a "ready-made" physics engine |
| Auto-differentiation | You write the physics yourself |
| Great for visual/creative physics | API has changed across versions |

**RLVR verifier quality**: **Good** for MPM, particles, deformable body verification. Excellent for custom physics.

---

## 2. Fluid Dynamics / CFD

### OpenFOAM

| | |
|---|---|
| **URL** | https://www.openfoam.com / https://openfoam.org |
| **License** | GPL v3 (fully open source) |
| **Cost** | **Free** |
| **Language** | C++, scriptable via Python (PyFoam, Ofpp) |

**What it's great at**: The industry-standard open-source CFD toolkit. Incompressible/compressible flows, turbulence modeling (RANS, LES, DNS), multiphase, combustion, heat transfer. Used by Boeing, VW, NASA.

| Pros | Cons |
|------|------|
| Industry standard, battle-tested | Very steep learning curve |
| Enormous solver library | Slow for simple problems |
| Massive community | C++ codebase is complex |
| Validated against experiments | Mesh generation is non-trivial |
| Free forever (GPL) | Runs slow per simulation |

**RLVR verifier quality**: **Good** for verifying CFD solutions, but slow. Best for offline verification of mesh setups, boundary conditions, and solver configurations rather than real-time RL.

---

### PhiFlow

| | |
|---|---|
| **URL** | https://github.com/tum-pbs/PhiFlow |
| **License** | MIT |
| **Cost** | **Free** |
| **Language** | Python (JAX, PyTorch, TensorFlow backends) |
| **Install** | `pip install phiflow` |

**What it's great at**: Differentiable fluid simulation for ML research. Supports incompressible fluids, smoke, and basic physics. Designed to be used inside ML training loops. From TU Munich's physics-based simulation group.

| Pros | Cons |
|------|------|
| Differentiable (JAX/PyTorch) | Not production CFD |
| Easy to use in ML pipelines | Limited physics (no turbulence models) |
| Fast for simple flows | 2D/3D only for basic cases |
| Good for learning/research | Not validated against engineering data |

**RLVR verifier quality**: **Very Good** for ML-oriented fluid verification tasks. Fast enough for RL loops.

---

### SU2

| | |
|---|---|
| **URL** | https://github.com/su2code/SU2 |
| **License** | LGPL v2.1 |
| **Cost** | **Free** |
| **Language** | C++, Python wrapper (SU2 Python) |

**What it's great at**: Aerodynamic shape optimization. RANS CFD with adjoint-based optimization. Used for aerospace design (airfoil optimization, wing design).

| Pros | Cons |
|------|------|
| Built-in adjoint solver | Narrower scope than OpenFOAM |
| Good for shape optimization | Less general-purpose |
| Active academic development | Smaller community |
| Well-documented | |

**RLVR verifier quality**: **Good** for aerospace/aerodynamic design verification tasks.

---

### Mantaflow

| | |
|---|---|
| **URL** | https://github.com/thunil/mantaflow |
| **License** | Apache 2.0 |
| **Cost** | **Free** |
| **Language** | C++/Python |

**What it's great at**: Fast fluid simulation for visual effects and ML. Smoke, liquids, FLIP solvers. Integrated into Blender as the fluid simulation engine.

| Pros | Cons |
|------|------|
| Fast for visual-quality fluids | Not engineering-accurate |
| Python scriptable | Less maintained since Blender integration |
| Good for ML data generation | Limited solver types |

**RLVR verifier quality**: **Moderate**. Useful for visual fluid verification, not engineering accuracy.

---

## 3. Finite Element Analysis

### FEniCSx

| | |
|---|---|
| **URL** | https://fenicsproject.org |
| **License** | LGPL v3 / MIT (mixed) |
| **Cost** | **Free** |
| **Language** | Python (DOLFINx), C++ backend |
| **Install** | `pip install fenics-dolfinx` (or conda) |

**What it's great at**: Solving PDEs using the finite element method. Structural mechanics, heat transfer, electromagnetics, fluid-structure interaction. Dominant in academic FEM research. Express weak forms in near-mathematical notation using UFL (Unified Form Language).

| Pros | Cons |
|------|------|
| Express PDEs in mathematical notation | Steep learning curve |
| Automatic code generation from weak forms | Install can be tricky |
| Massively parallel (MPI) | Not for beginners |
| Huge academic community | FEniCSx vs legacy FEniCS confusion |
| Highly extensible | |

**RLVR verifier quality**: **Excellent** for PDE solving verification. Can verify mesh quality, boundary conditions, and solutions against analytical results.

---

### deal.II

| | |
|---|---|
| **URL** | https://www.dealii.org |
| **License** | LGPL v2.1 |
| **Cost** | **Free** |
| **Language** | C++ (Python bindings via PyDealII) |

**What it's great at**: Adaptive mesh refinement (AMR) for FEM. Highly scalable (tens of thousands of cores). Strong for research requiring hp-adaptivity.

| Pros | Cons |
|------|------|
| Best-in-class adaptive mesh refinement | C++ only (limited Python) |
| Massively parallel | Steep learning curve |
| 600+ tutorial programs | Smaller community than FEniCS |

**RLVR verifier quality**: **Good** for advanced FEM tasks requiring adaptive refinement.

---

### CalculiX

| | |
|---|---|
| **URL** | http://www.calculix.de |
| **License** | GPL v2 |
| **Cost** | **Free** |
| **Language** | Fortran/C, command-line |

**What it's great at**: Structural FEM (stress, strain, modal analysis, heat transfer). ABAQUS-compatible input format. Good for mechanical engineering verification.

| Pros | Cons |
|------|------|
| ABAQUS-compatible format | No Python API |
| Proven for structural analysis | Old-school Fortran codebase |
| Free alternative to commercial FEM | Limited documentation |

**RLVR verifier quality**: **Moderate**. Usable via subprocess but lacks Python API.

---

### Elmer

| | |
|---|---|
| **URL** | https://github.com/ElmerCSC/elmerfem |
| **License** | LGPL |
| **Cost** | **Free** |
| **Language** | Fortran, Python interface (ElmerPy) |

**What it's great at**: Multi-physics FEM: heat, fluid, structural, electromagnetics all in one solver. Developed by CSC Finland.

| Pros | Cons |
|------|------|
| Multi-physics in one package | Smaller community |
| Good electromagnetics solver | Fortran codebase |
| ElmerGUI for mesh setup | Documentation gaps |

---

## 4. Chemistry & Molecular Simulation

### RDKit

| | |
|---|---|
| **URL** | https://github.com/rdkit/rdkit |
| **License** | BSD 3-Clause |
| **Cost** | **Free** |
| **Language** | C++, excellent Python bindings |
| **Install** | `pip install rdkit` or `conda install -c conda-forge rdkit` |

**What it's great at**: The industry standard for cheminformatics. Molecular representation (SMILES/SMARTS/InChI), substructure search, molecular descriptors, fingerprints, conformer generation, reaction handling, chemical drawing. Used by every pharma company and ML chemistry paper.

| Pros | Cons |
|------|------|
| Industry standard, enormous community | Not for quantum chemistry |
| Extremely mature and reliable | No MD simulation |
| Comprehensive chemical toolkit | Steep learning curve for advanced features |
| Fast (C++ core) | |
| Handles reactions, scaffolds, fingerprints | |

**RLVR verifier quality**: **Excellent**. Can verify: valid SMILES, molecular properties (MW, LogP, TPSA), drug-likeness (Lipinski), synthetic accessibility, reaction validity, substructure constraints.

---

### Open Babel

| | |
|---|---|
| **URL** | https://github.com/openbabel/openbabel |
| **License** | GPL v2 |
| **Cost** | **Free** |
| **Language** | C++, Python (`openbabel` or `pybel`) |
| **Install** | `conda install -c conda-forge openbabel` |

**What it's great at**: Chemical file format conversion (100+ formats), force field energy calculations (MMFF94, UFF, GAFF), 3D coordinate generation, conformer search.

| Pros | Cons |
|------|------|
| Unmatched format support | GPL license (copyleft) |
| Basic force field calculations | Less feature-rich than RDKit for descriptors |
| Good 3D generation | Slower development pace |

**RLVR verifier quality**: **Good** for format conversion verification, basic energy calculations.

---

## 5. Quantum Chemistry / DFT

### xTB / GFN2-xTB (Grimme Group)

| | |
|---|---|
| **URL** | https://github.com/grimme-lab/xtb |
| **License** | LGPL v3 |
| **Cost** | **Free** |
| **Language** | Fortran, Python bindings (`xtb-python`) |
| **Install** | `conda install -c conda-forge xtb-python` |

**What it's great at**: Ultra-fast semi-empirical quantum chemistry. GFN2-xTB gives DFT-quality geometries and energies in seconds (vs. hours for DFT). Covers the entire periodic table. The sweet spot between force fields and full DFT.

| Pros | Cons |
|------|------|
| 1000x faster than DFT | Less accurate than full DFT |
| Covers entire periodic table | Semi-empirical (approximations) |
| Good geometries and frequencies | Limited for reaction barriers |
| Easy to run | |

**RLVR verifier quality**: **Excellent** for fast molecular verification. Can check geometry optimization, conformer energies, and basic thermochemistry in seconds.

---

### PySCF

| | |
|---|---|
| **URL** | https://github.com/pyscf/pyscf |
| **License** | Apache 2.0 |
| **Cost** | **Free** |
| **Language** | Python (with C extensions) |
| **Install** | `pip install pyscf` |

**What it's great at**: Ab initio quantum chemistry entirely in Python. HF, DFT, MP2, CCSD(T), CASSCF, TDDFT. The most hackable quantum chemistry code. Excellent for method development and ML integration.

| Pros | Cons |
|------|------|
| Pure Python (easy to modify) | Slower than compiled codes for big systems |
| Comprehensive methods (HF through CCSD(T)) | Steeper learning curve for non-experts |
| Great for ML integration | |
| Active development | |
| Apache 2.0 license | |

**RLVR verifier quality**: **Very Good**. Can verify electronic structure calculations, orbital energies, molecular properties. Moderate speed.

---

### Psi4

| | |
|---|---|
| **URL** | https://github.com/psi4/psi4 |
| **License** | LGPL v3 |
| **Cost** | **Free** |
| **Language** | C++/Python |
| **Install** | `conda install -c psi4 psi4` |

**What it's great at**: Production-quality quantum chemistry with Python driver. HF, DFT, MP2, CCSD(T), symmetry-adapted perturbation theory (SAPT). Excellent for intermolecular interactions.

| Pros | Cons |
|------|------|
| Production quality | Larger install than PySCF |
| Excellent SAPT implementation | Less hackable than PySCF |
| Good performance | |
| Python-driven | |

---

### ORCA

| | |
|---|---|
| **URL** | https://orcaforum.kofo.mpg.de |
| **License** | **Free for academic use**. Commercial license required for industry. |
| **Cost** | Free (academic), paid (commercial) |
| **Language** | Binary (Fortran/C++), command-line driven, no Python API |

**What it's great at**: The workhorse of computational chemistry. DFT, TDDFT, MP2, CCSD(T), multireference methods (CASSCF, NEVPT2). Extremely fast DFT (RIJCOSX approximation). Excellent spectroscopy predictions (EPR, NMR, UV-Vis). ORCA 6.0 added major performance improvements.

| Pros | Cons |
|------|------|
| Extremely fast DFT | No Python API (CLI only) |
| Best spectroscopy predictions | Closed source |
| Excellent documentation | Registration required |
| Free for academics | Commercial license needed for industry |
| Very actively developed | |

**RLVR verifier quality**: **Good** for reference calculations but requires CLI wrapping. No programmatic API — must parse output files.

---

### Gaussian

| | |
|---|---|
| **URL** | https://gaussian.com |
| **License** | Commercial |
| **Cost** | **$4,500-$28,800** per academic group license. Individual ~$150-600. |
| **Language** | Fortran, CLI only |

**What it's great at**: The most cited quantum chemistry software. Comprehensive methods, reliable, extremely well-tested. The "gold standard" reference for many calculations.

| Pros | Cons |
|------|------|
| Most cited, most validated | Expensive |
| Comprehensive methods | Closed source, no modification |
| Very reliable | No Python API |
| | Restrictive license |

**RLVR verifier quality**: **Poor** for automated verification. Too expensive, no API, restrictive license. Use PySCF or Psi4 instead.

---

### NWChem

| | |
|---|---|
| **URL** | https://github.com/nwchemgit/nwchem |
| **License** | ECL 2.0 (open source) |
| **Cost** | **Free** |
| **Language** | Fortran/C, Python interface (NWChem-Python) |

**What it's great at**: Massively parallel quantum chemistry. Good for large systems on HPC. Plane-wave DFT (NWPW module) for periodic systems.

| Pros | Cons |
|------|------|
| Excellent parallelism | Complex build |
| Plane-wave DFT | Less user-friendly |
| Free and open | Slower single-node than ORCA |

---

## 6. Molecular Dynamics

### OpenMM

| | |
|---|---|
| **URL** | https://github.com/openmm/openmm |
| **License** | MIT |
| **Cost** | **Free** |
| **Language** | C++, excellent Python API |
| **Install** | `conda install -c conda-forge openmm` |

**What it's great at**: GPU-accelerated molecular dynamics with a beautiful Python API. Biomolecular simulation (proteins, DNA, membranes). Custom force fields, enhanced sampling (metadynamics, replica exchange). The best MD code for ML integration.

| Pros | Cons |
|------|------|
| Excellent Python API | Primarily for biomolecular |
| GPU-accelerated (CUDA, OpenCL) | Less suited for materials |
| Custom force support | |
| Differentiable (OpenMM-Torch) | |
| ML force field integration | |

**RLVR verifier quality**: **Excellent** for biomolecular verification. Can verify: protein stability (energy minimization), molecular dynamics trajectories, force field correctness.

---

### GROMACS

| | |
|---|---|
| **URL** | https://www.gromacs.org |
| **License** | LGPL v2.1 |
| **Cost** | **Free** |
| **Language** | C/C++, Python (gmxapi) |

**What it's great at**: The fastest MD code for biomolecular simulation on CPUs. Extremely optimized SIMD kernels. Free energy calculations, enhanced sampling. The workhorse of computational biology.

| Pros | Cons |
|------|------|
| Fastest CPU MD code | Python API less mature than OpenMM |
| Extremely well optimized | Complex input file system |
| Free energy perturbation | Steeper learning curve |
| Massive user base | |

**RLVR verifier quality**: **Good** but less scriptable than OpenMM. Best used via subprocess.

---

### LAMMPS

| | |
|---|---|
| **URL** | https://github.com/lammps/lammps |
| **License** | GPL v2 |
| **Cost** | **Free** |
| **Language** | C++, Python interface |
| **Install** | `conda install -c conda-forge lammps` |

**What it's great at**: General-purpose MD for materials science. Metals, polymers, granular systems, coarse-grained models. Extremely flexible with hundreds of pair styles and fix commands.

| Pros | Cons |
|------|------|
| Most flexible MD code | Complex input script language |
| Materials science strength | Less suited for proteins |
| GPU acceleration | Large codebase |
| Huge user community | |

**RLVR verifier quality**: **Good** for materials science MD verification.

---

## 7. Drug Design & Docking

### AutoDock Vina

| | |
|---|---|
| **URL** | https://github.com/ccsb-scripps/AutoDock-Vina |
| **License** | Apache 2.0 |
| **Cost** | **Free** |
| **Language** | C++, Python bindings (`vina`) |
| **Install** | `pip install vina` |

**What it's great at**: Molecular docking (predicting how a small molecule binds to a protein). Fast, reliable, widely cited. The standard baseline for docking benchmarks.

| Pros | Cons |
|------|------|
| Fast and reliable | Rigid receptor (no flexibility) |
| Python API | Scoring function is approximate |
| Widely validated | Less accurate than physics-based |
| Easy to use | |

**RLVR verifier quality**: **Good**. Can verify docking poses, binding affinity predictions.

---

### GNINA

| | |
|---|---|
| **URL** | https://github.com/gnina/gnina |
| **License** | Apache 2.0 |
| **Cost** | **Free** |
| **Language** | C++, Python bindings |

**What it's great at**: CNN-scored molecular docking. Uses a convolutional neural network trained on crystal structures to rescore Vina-generated poses. Generally more accurate than Vina alone.

| Pros | Cons |
|------|------|
| More accurate than Vina | Requires GPU |
| ML-enhanced scoring | Larger install |
| Same interface as Vina | |

---

### DiffDock

| | |
|---|---|
| **URL** | https://github.com/gcorso/DiffDock |
| **License** | MIT |
| **Cost** | **Free** |
| **Language** | Python (PyTorch) |

**What it's great at**: Diffusion-based molecular docking. State-of-the-art on PoseBusters benchmark. Generates diverse binding poses. Blind docking — no predefined binding pocket needed.

| Pros | Cons |
|------|------|
| State-of-the-art accuracy (2024-2025) | Requires GPU |
| Blind docking (no box definition needed) | Less validated than Vina historically |
| Generates diverse poses with confidence scores | ML model may have blind spots |
| Active development | |

**RLVR verifier quality**: **Good** for modern docking verification.

---

### Uni-Dock (DP Technology)

| | |
|---|---|
| **URL** | https://github.com/dptech-corp/Uni-Dock |
| **License** | Apache 2.0 |
| **Cost** | **Free** |
| **Language** | C++/CUDA, Python wrapper |

**What it's great at**: GPU-accelerated molecular docking. ~1000x faster than Vina by running on GPU. Designed for ultra-large-scale virtual screening. Vina-compatible interface.

| Pros | Cons |
|------|------|
| Extreme throughput (1000x Vina) | Requires NVIDIA GPU |
| Vina-compatible interface | Relatively new, less battle-tested |
| Open source | Same scoring limitations as Vina |

**RLVR verifier quality**: **Very Good** for large-scale batch docking verification.

---

### OpenFE (Open Free Energy)

| | |
|---|---|
| **URL** | https://github.com/OpenFreeEnergy/openfe |
| **License** | MIT |
| **Cost** | **Free** |
| **Language** | Python |

**What it's great at**: Relative binding free energy calculations. More accurate than docking for lead optimization. Automates complex alchemical free energy workflows.

| Pros | Cons |
|------|------|
| Most accurate binding affinity method | Very computationally expensive |
| Automated workflows | Hours per calculation |
| Industry adoption (Merck, etc.) | Complex setup |

---

## 8. Retrosynthesis & Reaction Prediction

### AiZynthFinder (AstraZeneca)

| | |
|---|---|
| **URL** | https://github.com/MolecularAI/aizynthfinder |
| **License** | MIT |
| **Cost** | **Free** |
| **Language** | Python |
| **Install** | `pip install aizynthfinder` |

**What it's great at**: ML-powered retrosynthetic route planning. Uses Monte Carlo Tree Search with neural network expansion policy. Can find multi-step synthesis routes to target molecules.

| Pros | Cons |
|------|------|
| State-of-the-art retrosynthesis | Requires trained model weights |
| MCTS search is thorough | Routes need expert validation |
| Easy Python API | Limited reaction database |
| From AstraZeneca (production use) | |

**RLVR verifier quality**: **Good** for verifying synthesis route feasibility.

---

### RDChiral

| | |
|---|---|
| **URL** | https://github.com/connorcoley/rdchiral |
| **License** | MIT |
| **Cost** | **Free** |
| **Language** | Python (built on RDKit) |

**What it's great at**: Stereochemistry-aware reaction template application. Used as the core reaction engine in many retrosynthesis tools.

| Pros | Cons |
|------|------|
| Handles stereochemistry correctly | Template-based only |
| Fast | Requires reaction templates |
| Reliable | |

---

### RXNMapper

| | |
|---|---|
| **URL** | https://github.com/rxn4chemistry/rxnmapper |
| **License** | MIT |
| **Cost** | **Free** |
| **Language** | Python |

**What it's great at**: Attention-based atom-to-atom mapping of chemical reactions. Identifies which atoms in reactants become which atoms in products.

---

## 9. Protein Structure & Design

### AlphaFold 3 (Google DeepMind)

| | |
|---|---|
| **URL** | https://github.com/google-deepmind/alphafold3 |
| **License** | Model weights: restricted (non-commercial research only). Code: open. |
| **Cost** | **Free for academic research**. Commercial use requires license from Google. Also available via AlphaFold Server (free, web-based, limited). |
| **Language** | Python (JAX) |

**What it's great at**: State-of-the-art protein structure prediction. Also predicts protein-DNA, protein-RNA, protein-ligand, and protein-protein complexes. Diffusion-based architecture. The gold standard for biomolecular structure prediction.

| Pros | Cons |
|------|------|
| Best accuracy for most targets | Non-commercial license for weights |
| Handles complexes (protein+ligand+DNA) | Very GPU-hungry |
| Confidence scores (pLDDT, PAE) | Slow inference |
| | Can't do MD/dynamics |

**RLVR verifier quality**: **Excellent** for protein design verification. Can score designed sequences by predicted structure quality (pLDDT).

---

### ESMFold (Meta AI)

| | |
|---|---|
| **URL** | https://github.com/facebookresearch/esm |
| **License** | MIT |
| **Cost** | **Free** |
| **Language** | Python (PyTorch) |
| **Install** | `pip install fair-esm` |

**What it's great at**: Single-sequence protein structure prediction (no MSA needed). Much faster than AlphaFold (~60x). Good for rapid screening. Based on the ESM-2 language model.

| Pros | Cons |
|------|------|
| Very fast (seconds per structure) | Less accurate than AlphaFold 3 |
| No MSA needed | Single chain only |
| MIT license | Less good for complexes |
| ESM-2 embeddings useful for many tasks | |

**RLVR verifier quality**: **Very Good**. Fast enough for RL loops. MIT licensed.

---

### Boltz-1

| | |
|---|---|
| **URL** | https://github.com/jwohlwend/boltz |
| **License** | MIT |
| **Cost** | **Free** |
| **Language** | Python (PyTorch) |

**What it's great at**: Open-source biomolecular structure prediction comparable to AlphaFold 3. Handles proteins, RNA, DNA, ligands, covalent modifications. Fully open weights with MIT license. Competitive accuracy.

| Pros | Cons |
|------|------|
| Fully open (MIT license + open weights) | Newer, less validated |
| Handles complexes like AF3 | |
| Active development | |
| Commercial use allowed | |

**RLVR verifier quality**: **Excellent**. Best open alternative to AlphaFold 3. Fully permissive license.

---

### Chai-1

| | |
|---|---|
| **URL** | https://github.com/chaidiscovery/chai-lab |
| **License** | Apache 2.0 (code), model weights have separate terms |
| **Cost** | **Free for non-commercial**. Commercial requires agreement. |
| **Language** | Python (PyTorch) |

**What it's great at**: Another AlphaFold 3 alternative. Multi-chain structure prediction. Competitive on CASP benchmarks.

| Pros | Cons |
|------|------|
| Competitive accuracy | Weight license restrictions |
| Multi-chain support | Newer |

---

### RFdiffusion (Baker Lab)

| | |
|---|---|
| **URL** | https://github.com/RosettaCommons/RFdiffusion |
| **License** | BSD (code), model weights under Rosetta license |
| **Cost** | **Free for academic use** |
| **Language** | Python (PyTorch) |

**What it's great at**: The state-of-the-art for de novo protein design. Diffusion model that generates novel protein backbones conditioned on functional constraints (binding targets, symmetry, motif scaffolding). Designs have been experimentally validated.

| Pros | Cons |
|------|------|
| State-of-the-art protein design | Rosetta license for weights |
| Experimentally validated designs | Requires GPU |
| Conditional generation (binder design) | Complex setup |
| Active development (Baker Lab) | |

**RLVR verifier quality**: **Excellent** for protein design tasks. Generate backbone → score with AlphaFold/ESMFold.

---

### ProteinMPNN (Baker Lab)

| | |
|---|---|
| **URL** | https://github.com/dauparas/ProteinMPNN |
| **License** | MIT |
| **Cost** | **Free** |
| **Language** | Python (PyTorch) |

**What it's great at**: Sequence design for a given protein backbone. Given a 3D structure, designs amino acid sequences that will fold into that structure. The standard tool in the protein design pipeline.

| Pros | Cons |
|------|------|
| Fast and accurate | Only does sequence design (not backbone) |
| MIT license | |
| Well-validated experimentally | |
| Easy to use | |

**RLVR verifier quality**: **Excellent**. Design backbone (RFdiffusion) → design sequence (ProteinMPNN) → verify folding (AlphaFold/ESMFold). Complete pipeline.

---

### PyRosetta (Rosetta)

| | |
|---|---|
| **URL** | https://www.pyrosetta.org |
| **License** | Academic: **free** (requires license agreement). Commercial: paid. |
| **Cost** | Free for academic, ~$5K-50K+ for commercial |
| **Language** | Python |

**What it's great at**: The Swiss army knife of computational protein science. Energy functions, protein folding, design, docking, loop modeling, enzyme design. 25+ years of development. Still widely used despite ML methods.

| Pros | Cons |
|------|------|
| Most comprehensive protein toolkit | Slow compared to ML methods |
| Physics-based energy function (REF2015) | License required |
| Protein-protein docking | Steep learning curve |
| Enzyme design (Rosetta enzyme design) | Energy functions can be noisy |
| Decades of validation | |

**RLVR verifier quality**: **Good**. Energy function useful for scoring designs but slower than ML methods for structure prediction.

---

### ESM-IF (Inverse Folding)

| | |
|---|---|
| **URL** | Part of https://github.com/facebookresearch/esm |
| **License** | MIT |
| **Cost** | **Free** |

**What it's great at**: Inverse folding (structure → sequence) using the ESM language model. Alternative to ProteinMPNN.

---

## 10. DNA/RNA Tools

### NUPACK

| | |
|---|---|
| **URL** | https://www.nupack.org |
| **License** | Academic: **free**. Commercial: paid license. |
| **Cost** | Free for academic |
| **Language** | Python (`nupack` package), C++ core |

**What it's great at**: Nucleic acid sequence design and thermodynamic analysis. Designs DNA/RNA sequences that fold into target structures. Computes partition functions, MFE structures, and base-pairing probabilities. The gold standard for DNA nanotechnology.

| Pros | Cons |
|------|------|
| Gold standard for nucleic acid design | Academic-only free license |
| Multi-strand complexes | Not open source |
| Thermodynamically rigorous | |
| Python API | |

**RLVR verifier quality**: **Excellent** for DNA/RNA design verification. Can verify whether designed sequences fold correctly.

---

### ViennaRNA

| | |
|---|---|
| **URL** | https://github.com/ViennaRNA/ViennaRNA |
| **License** | Custom academic license (free for academic) |
| **Cost** | **Free** |
| **Language** | C, Python (`RNA` module) |

**What it's great at**: RNA secondary structure prediction and thermodynamics. Minimum free energy (MFE) structure, partition function, base-pairing probabilities. The most widely cited RNA folding tool.

| Pros | Cons |
|------|------|
| Most cited RNA folding tool | Less capable than NUPACK for multi-strand |
| Fast | RNA only (not DNA complexes) |
| Python bindings | |
| Free | |

**RLVR verifier quality**: **Excellent** for RNA structure verification.

---

### Biopython

| | |
|---|---|
| **URL** | https://github.com/biopython/biopython |
| **License** | Biopython License (BSD-like) |
| **Cost** | **Free** |
| **Language** | Python |
| **Install** | `pip install biopython` |

**What it's great at**: General-purpose bioinformatics library. Sequence alignment (pairwise, BLAST), PDB structure parsing, phylogenetics, GenBank/FASTA I/O. The "requests" of bioinformatics.

| Pros | Cons |
|------|------|
| Comprehensive bioinformatics toolkit | Not for simulation |
| Excellent I/O (PDB, FASTA, GenBank) | Some modules are thin wrappers |
| Large community | |
| Easy to use | |

**RLVR verifier quality**: **Good** for sequence analysis, format verification, basic bioinformatics tasks.

---

## 11. Genomics & Bioinformatics

### BLAST+

| | |
|---|---|
| **URL** | https://blast.ncbi.nlm.nih.gov |
| **License** | Public domain |
| **Cost** | **Free** |
| **Language** | C++, command-line, Biopython wrapper |

**What it's great at**: Sequence similarity search against databases. The most fundamental bioinformatics tool.

---

### IQ-TREE 2

| | |
|---|---|
| **URL** | https://github.com/iqtree/iqtree2 |
| **License** | GPL v2 |
| **Cost** | **Free** |
| **Language** | C++, command-line |

**What it's great at**: Maximum likelihood phylogenetic inference. State-of-the-art for tree building. ModelFinder for model selection. Ultrafast bootstrap.

**RLVR verifier quality**: **Good** for phylogenetics verification.

---

## 12. Circuit Simulation (SPICE)

### ngspice

| | |
|---|---|
| **URL** | https://ngspice.sourceforge.io |
| **License** | BSD (mixed with public domain) |
| **Cost** | **Free** |
| **Language** | C, scriptable via Python (PySpice wrapper) |

**What it's great at**: The standard open-source SPICE simulator. DC, AC, transient analysis. BSIM transistor models. Handles analog and mixed-signal circuits.

| Pros | Cons |
|------|------|
| Industry-standard SPICE engine | Less accurate models than commercial |
| Free forever | Slower than commercial SPICE |
| PySpice provides Python API | |
| Mature and reliable | |

**RLVR verifier quality**: **Excellent** for analog circuit verification. Can verify: DC operating points, frequency response, transient behavior, power dissipation.

---

### PySpice

| | |
|---|---|
| **URL** | https://github.com/PySpice-org/PySpice |
| **License** | GPL v3 |
| **Cost** | **Free** |
| **Language** | Python |
| **Install** | `pip install PySpice` |

**What it's great at**: Python wrapper around ngspice. Build and simulate circuits entirely in Python. Ideal for programmatic circuit verification.

| Pros | Cons |
|------|------|
| Build circuits in Python | Depends on ngspice |
| Programmatic access to all SPICE features | GPL license |
| Great for automated testing | |

**RLVR verifier quality**: **Excellent**. The ideal SPICE interface for RLVR.

---

### Xyce (Sandia National Labs)

| | |
|---|---|
| **URL** | https://github.com/Xyce/Xyce |
| **License** | GPL v3 |
| **Cost** | **Free** |
| **Language** | C++, command-line |

**What it's great at**: Massively parallel SPICE simulation. Scales to millions of devices. Developed for extreme-scale circuit analysis at Sandia.

| Pros | Cons |
|------|------|
| Scales to millions of devices | No Python API |
| Massively parallel | Overkill for small circuits |
| Advanced device models | |

---

### LTspice

| | |
|---|---|
| **URL** | https://www.analog.com/ltspice |
| **License** | Freeware (proprietary but free) |
| **Cost** | **Free** |
| **Language** | GUI-based, command-line batch mode |

**What it's great at**: Analog Devices' free SPICE simulator. Fast, reliable, excellent component library (especially ADI parts). The most popular free SPICE for EE education.

| Pros | Cons |
|------|------|
| Very fast simulation | Windows/macOS only (Wine on Linux) |
| Excellent component library | No Python API |
| Free | Proprietary |

**RLVR verifier quality**: **Poor** for automation. No programmatic API. Use ngspice/PySpice instead.

---

## 13. Chip Design / RTL / HDL

### Verilator

| | |
|---|---|
| **URL** | https://github.com/verilator/verilator |
| **License** | LGPL v3 (or Artistic License 2.0) |
| **Cost** | **Free** |
| **Language** | C++, generates C++ from Verilog |

**What it's great at**: The fastest open-source Verilog/SystemVerilog simulator. Compiles Verilog to optimized C++. 2-phase simulation. Used by major chip companies for RTL verification. Verilator 5.x added significant SystemVerilog support.

| Pros | Cons |
|------|------|
| Fastest open-source Verilog sim | Cycle-based only (no timing) |
| Used in industry | 2-state only (no X/Z) |
| Excellent performance | SystemVerilog subset |
| Cocotb integration | |

**RLVR verifier quality**: **Excellent** for RTL verification. Compile design → run testbench → check outputs.

---

### Cocotb

| | |
|---|---|
| **URL** | https://github.com/cocotb/cocotb |
| **License** | BSD 2-Clause |
| **Cost** | **Free** |
| **Language** | Python |
| **Install** | `pip install cocotb` |

**What it's great at**: Python testbenches for HDL simulation. Write verification in Python instead of SystemVerilog/UVM. Works with Verilator, Icarus Verilog, GHDL, and commercial simulators.

| Pros | Cons |
|------|------|
| Write testbenches in Python! | Slightly slower than native SV |
| Works with all major simulators | Less feature-rich than UVM |
| Easy to integrate with ML | |
| Active community | |

**RLVR verifier quality**: **Excellent**. The ideal interface for AI-driven RTL verification. Python testbenches can programmatically check design correctness.

---

### Icarus Verilog

| | |
|---|---|
| **URL** | https://github.com/steveicarus/iverilog |
| **License** | GPL v2 |
| **Cost** | **Free** |
| **Language** | C, command-line |

**What it's great at**: Event-driven Verilog simulator. Supports 4-state simulation (X/Z values). Simpler than Verilator, handles timing.

| Pros | Cons |
|------|------|
| 4-state simulation | Slower than Verilator |
| Event-driven (timing) | Less SystemVerilog support |
| Simple to use | |

---

### GHDL

| | |
|---|---|
| **URL** | https://github.com/ghdl/ghdl |
| **License** | GPL v2 |
| **Cost** | **Free** |
| **Language** | Ada, command-line |

**What it's great at**: Open-source VHDL simulator. Full VHDL-2008 support. The only serious open-source VHDL option.

---

### Yosys + SymbiYosys

| | |
|---|---|
| **URL** | https://github.com/YosysHQ/yosys (synthesis) / https://github.com/YosysHQ/sby (formal) |
| **License** | ISC |
| **Cost** | **Free** |
| **Language** | C++, TCL scripting, Python (pyosys) |

**What it's great at**: Open-source synthesis tool. Converts RTL (Verilog) to gate-level netlists. **SymbiYosys** is the formal verification frontend — takes SystemVerilog Assertions (SVA) and proves/disproves them via bounded model checking, k-induction, etc. The backbone of the open-source FPGA toolchain (with nextpnr).

| Pros | Cons |
|------|------|
| Industry-quality synthesis | Verilog only (no VHDL natively) |
| SymbiYosys proves properties without testbenches | SystemVerilog subset (improving) |
| Equivalence checking between designs | Formal verification expensive for large designs |
| JSON netlist output for programmatic analysis | |
| Open-source FPGA flow (with nextpnr) | |

**RLVR verifier quality**: **Excellent** for synthesis and formal verification tasks. Synthesis success/failure + gate count + formal property proof provide strong verification signals.

---

### Amaranth HDL

| | |
|---|---|
| **URL** | https://github.com/amaranth-lang/amaranth |
| **License** | BSD 2-Clause |
| **Cost** | **Free** |
| **Language** | Python |
| **Install** | `pip install amaranth` |

**What it's great at**: Hardware description in Python. Generate Verilog from Python code. Great for programmatic hardware design and verification.

---

## 14. Control Systems & Signal Processing

### python-control

| | |
|---|---|
| **URL** | https://github.com/python-control/python-control |
| **License** | BSD 3-Clause |
| **Cost** | **Free** |
| **Language** | Python |
| **Install** | `pip install control` |

**What it's great at**: MATLAB-like control systems library. Transfer functions, state space, Bode plots, root locus, LQR, H-infinity, robustness analysis.

| Pros | Cons |
|------|------|
| MATLAB-like API | Not as feature-rich as MATLAB |
| Free and open | Slower for some operations |
| Good for education and verification | |

**RLVR verifier quality**: **Excellent** for control theory verification. Check stability margins, step response, frequency response, controller design.

---

### scipy.signal

| | |
|---|---|
| **URL** | https://docs.scipy.org/doc/scipy/reference/signal.html |
| **License** | BSD |
| **Cost** | **Free** |
| **Install** | `pip install scipy` |

**What it's great at**: Signal processing fundamentals. Filter design (Butterworth, Chebyshev, FIR), FFT, spectral analysis, convolution, windowing.

**RLVR verifier quality**: **Excellent** for DSP verification.

---

## 15. Quantum Computing Simulators

### Qiskit Aer (IBM)

| | |
|---|---|
| **URL** | https://github.com/Qiskit/qiskit-aer |
| **License** | Apache 2.0 |
| **Cost** | **Free** (also connects to real IBM quantum hardware for free tier) |
| **Language** | Python |
| **Install** | `pip install qiskit-aer` |

**What it's great at**: State vector, density matrix, and stabilizer simulation of quantum circuits. Noise modeling. The most popular quantum computing framework.

| Pros | Cons |
|------|------|
| Largest ecosystem | IBM-centric |
| Real hardware access | API reorganization in 2024 |
| Noise simulation | |
| Excellent documentation | |

**RLVR verifier quality**: **Excellent** for quantum circuit verification. Simulate circuit → compare to expected state/measurement.

---

### Cirq (Google)

| | |
|---|---|
| **URL** | https://github.com/quantumlib/Cirq |
| **License** | Apache 2.0 |
| **Cost** | **Free** |
| **Language** | Python |
| **Install** | `pip install cirq` |

**What it's great at**: NISQ-era quantum circuit design. Close to hardware (qubit connectivity). Google's quantum computing framework.

---

### PennyLane (Xanadu)

| | |
|---|---|
| **URL** | https://github.com/PennyLaneAI/pennylane |
| **License** | Apache 2.0 |
| **Cost** | **Free** |
| **Language** | Python |
| **Install** | `pip install pennylane` |

**What it's great at**: Differentiable quantum computing. Quantum machine learning. Connects to multiple backends (Qiskit, Cirq, Lightning, etc.). Auto-differentiation of quantum circuits.

| Pros | Cons |
|------|------|
| Differentiable quantum circuits | QML focus (narrower scope) |
| Multi-backend | |
| Great for quantum ML | |

---

### Stim (Google)

| | |
|---|---|
| **URL** | https://github.com/quantumlib/Stim |
| **License** | Apache 2.0 |
| **Cost** | **Free** |
| **Language** | C++, Python |
| **Install** | `pip install stim` |

**What it's great at**: Ultra-fast stabilizer circuit simulation for quantum error correction. Can simulate millions of qubits for Clifford circuits. The standard tool for QEC research.

| Pros | Cons |
|------|------|
| Billions of operations per second | Clifford circuits only |
| Millions of qubits | Not universal quantum sim |
| Decoder integration (pymatching) | |

**RLVR verifier quality**: **Excellent** for quantum error correction tasks.

---

### QuTiP

| | |
|---|---|
| **URL** | https://github.com/qutip/qutip |
| **License** | BSD 3-Clause |
| **Cost** | **Free** |
| **Language** | Python |
| **Install** | `pip install qutip` |

**What it's great at**: Quantum mechanics simulation. Open quantum systems, Lindblad master equation, quantum optics. More physics-oriented than circuit-focused tools.

---

## 16. Formal Proof & Theorem Proving

### Lean 4

| | |
|---|---|
| **URL** | https://github.com/leanprover/lean4 |
| **License** | Apache 2.0 |
| **Cost** | **Free** |
| **Language** | Lean (self-hosted), with programmatic API |

**What it's great at**: The current standard for interactive theorem proving. Mathlib4 has 150K+ theorems. Used by the Fields Medal community. Lean 4 is a full programming language (not just a proof assistant). LeanDojo provides Python API for AI integration.

| Pros | Cons |
|------|------|
| Most active ITP community (Mathlib) | Learning curve |
| Full programming language | Mathlib compilation is slow |
| LeanDojo for AI integration | Ecosystem still growing |
| Active development | |
| Apache 2.0 license | |

**RLVR verifier quality**: **Excellent**. The gold standard. Proof is either accepted by the type checker or not — zero false positives.

---

### Coq

| | |
|---|---|
| **URL** | https://github.com/coq/coq |
| **License** | LGPL v2.1 |
| **Cost** | **Free** |
| **Language** | OCaml, Coq tactic language |

**What it's great at**: Mature interactive theorem prover. CompCert (verified C compiler), Fiat Cryptography (verified crypto). Strong dependent type theory (Calculus of Inductive Constructions).

| Pros | Cons |
|------|------|
| Most mature ITP | Less active than Lean 4 for math |
| Industrial applications (CompCert) | Tactic language is complex |
| Strong extraction to OCaml | |

**RLVR verifier quality**: **Excellent**. Same guarantee as Lean — proof checks or doesn't.

---

### Isabelle/HOL

| | |
|---|---|
| **URL** | https://isabelle.in.tum.de |
| **License** | BSD |
| **Cost** | **Free** |
| **Language** | ML, Isar proof language |

**What it's great at**: Interactive theorem prover with excellent automation (Sledgehammer). Large Archive of Formal Proofs (AFP). Strong in both math and computer science verification.

| Pros | Cons |
|------|------|
| Best automation (Sledgehammer) | Heavier than Lean |
| Large AFP library | Less trendy than Lean 4 |
| Mature and reliable | |

---

## 17. SMT & SAT Solvers

### Z3 (Microsoft)

| | |
|---|---|
| **URL** | https://github.com/Z3Prover/z3 |
| **License** | MIT |
| **Cost** | **Free** |
| **Language** | C++, Python (`z3-solver`), many bindings |
| **Install** | `pip install z3-solver` |

**What it's great at**: The most widely used SMT solver. Supports: integer/real arithmetic, bitvectors, arrays, strings, floating-point, regular expressions, quantifiers. Used in software verification, symbolic execution, constraint solving.

| Pros | Cons |
|------|------|
| Most popular SMT solver | Can be slow on quantifiers |
| Excellent Python API | |
| Handles many theories | |
| MIT license | |

**RLVR verifier quality**: **Excellent**. Can verify satisfiability, find models, prove unsatisfiability. Essential for formal methods domains.

---

### CVC5

| | |
|---|---|
| **URL** | https://github.com/cvc5/cvc5 |
| **License** | BSD 3-Clause |
| **Cost** | **Free** |
| **Language** | C++, Python bindings |
| **Install** | `pip install cvc5` |

**What it's great at**: Strong complement to Z3. Better on some theory combinations. Winning solver in SMT-COMP in many divisions. Excellent string/regex/sequence reasoning.

---

### Vampire (ATP)

| | |
|---|---|
| **URL** | https://github.com/vprover/vampire |
| **License** | BSD (for non-commercial) |
| **Cost** | **Free** |
| **Language** | C++ |

**What it's great at**: The strongest first-order automated theorem prover. Wins CASC (the ATP world championship) consistently. Takes TPTP format input.

---

## 18. Computer Algebra Systems

### SageMath

| | |
|---|---|
| **URL** | https://www.sagemath.org |
| **License** | GPL v2+ |
| **Cost** | **Free** |
| **Language** | Python (wraps many math libraries) |

**What it's great at**: Comprehensive open-source mathematics system. Algebra, calculus, number theory, combinatorics, graph theory, group theory, cryptography. Wraps GAP, PARI/GP, Singular, Maxima, R, and many more.

| Pros | Cons |
|------|------|
| Most comprehensive open math system | Large install |
| Python-based | Slower than specialized tools |
| Wraps best-in-class tools | |
| Free alternative to Mathematica | |

**RLVR verifier quality**: **Excellent** for mathematical verification across all domains.

---

### SymPy

| | |
|---|---|
| **URL** | https://github.com/sympy/sympy |
| **License** | BSD 3-Clause |
| **Cost** | **Free** |
| **Language** | Pure Python |
| **Install** | `pip install sympy` |

**What it's great at**: Symbolic mathematics in pure Python. Algebra, calculus, combinatorics, matrices, equation solving. Lighter than SageMath. Perfect for embedding in verification pipelines.

| Pros | Cons |
|------|------|
| Pure Python (no dependencies) | Slower than compiled CAS |
| Easy to embed | Less comprehensive than SageMath |
| Good enough for most verification | |
| BSD license | |

**RLVR verifier quality**: **Excellent** for symbolic math verification. Already used in our math_equivalence verifier.

---

### Wolfram Engine

| | |
|---|---|
| **URL** | https://www.wolfram.com/engine/ |
| **License** | **Free for development/pre-production**. Full Mathematica is commercial (~$160/yr student, ~$375/yr individual). |
| **Cost** | Free (dev), paid (production/commercial) |
| **Language** | Wolfram Language, Python via `wolframclient` (`pip install wolframclient`) |

**What it's great at**: The most powerful CAS in existence. Symbolic math, numerical computation, knowledge base, NLP, image processing. `FullSimplify`, `Reduce`, `FindInstance` are extremely strong decision procedures. Largest curated mathematical knowledge base available.

| Pros | Cons |
|------|------|
| Most powerful simplification/equivalence checking | "Free for dev" license is ambiguous at scale |
| Huge built-in dataset (identities, sequences, constants) | Proprietary, not open source |
| `wolframclient` Python bridge | Wolfram Language learning curve |
| Wolfram Engine is genuinely free for dev | Startup time (seconds per kernel) |

**RLVR verifier quality**: **Excellent** for mathematical verification where SymPy falls short. The free Engine tier makes it viable for RLVR, but license terms should be reviewed for production training at scale.

---

### GAP (Groups, Algorithms, Programming)

| | |
|---|---|
| **URL** | https://www.gap-system.org |
| **License** | GPL v2 |
| **Cost** | **Free** |
| **Language** | GAP language, Python via `gappy` or SageMath's GAP interface |

**What it's great at**: Computational group theory. The gold standard for: group construction, character tables, representation theory, permutation groups, finitely presented groups. 150+ packages. If GAP says two groups are isomorphic, they are.

**RLVR verifier quality**: **Excellent** for abstract algebra verification.

---

### PARI/GP

| | |
|---|---|
| **URL** | https://pari.math.u-bordeaux.fr |
| **License** | GPL v2 |
| **Cost** | **Free** |

**What it's great at**: Number theory. Fastest for: primality testing, factoring, algebraic number fields, elliptic curves, modular forms. Used by LMFDB.

---

## 19. Constraint & Optimization Solvers

### Google OR-Tools

| | |
|---|---|
| **URL** | https://github.com/google/or-tools |
| **License** | Apache 2.0 |
| **Cost** | **Free** |
| **Language** | C++, Python, Java, C# |
| **Install** | `pip install ortools` |

**What it's great at**: Combinatorial optimization. Vehicle routing, scheduling, bin packing, assignment, network flows. CP-SAT solver is world-class for constraint programming.

| Pros | Cons |
|------|------|
| World-class CP-SAT solver | Less for continuous optimization |
| Vehicle routing, scheduling | |
| Free and well-maintained | |
| Excellent documentation | |

**RLVR verifier quality**: **Excellent** for combinatorial optimization, scheduling, and constraint satisfaction verification.

---

### HiGHS

| | |
|---|---|
| **URL** | https://github.com/ERGO-Code/HiGHS |
| **License** | MIT |
| **Cost** | **Free** |
| **Language** | C++, Python |
| **Install** | `pip install highspy` |

**What it's great at**: The best open-source LP/MIP solver. Replaced GLPK as the go-to free solver. Competitive with commercial solvers (CPLEX, Gurobi) on many problems. Default solver in SciPy's `linprog`.

| Pros | Cons |
|------|------|
| Best free LP/MIP solver | Not as fast as Gurobi on hard MIPs |
| MIT license | Newer (less battle-tested) |
| Default in SciPy | |
| Active development | |

**RLVR verifier quality**: **Excellent** for linear/integer programming verification.

---

### SCIP

| | |
|---|---|
| **URL** | https://github.com/scipopt/scip |
| **License** | Apache 2.0 (since v8.0) |
| **Cost** | **Free** |
| **Language** | C, Python (PySCIPOpt) |
| **Install** | `pip install pyscipopt` |

**What it's great at**: Mixed-integer programming, constraint integer programming. One of the fastest non-commercial MIP solvers. Also handles nonlinear optimization.

---

### Gurobi

| | |
|---|---|
| **URL** | https://www.gurobi.com |
| **License** | Commercial. **Free academic license.** |
| **Cost** | Free for academic. $12K+/year commercial. |
| **Language** | Python (gurobipy), C++, Java, .NET |

**What it's great at**: The fastest commercial MIP solver. Industry standard for large-scale optimization.

| Pros | Cons |
|------|------|
| Fastest MIP solver | Expensive for commercial |
| Excellent Python API | Proprietary |
| Free for academic | |

---

### MiniZinc

| | |
|---|---|
| **URL** | https://www.minizinc.org |
| **License** | MPL v2 |
| **Cost** | **Free** |
| **Language** | MiniZinc modeling language, Python bindings |

**What it's great at**: Constraint modeling language. Write constraints declaratively, solve with any backend (Gecode, OR-Tools, Chuffed). Great for constraint satisfaction verification.

---

## 20. Graph & Network Analysis

### NetworkX

| | |
|---|---|
| **URL** | https://github.com/networkx/networkx |
| **License** | BSD 3-Clause |
| **Cost** | **Free** |
| **Install** | `pip install networkx` |

**What it's great at**: Pure Python graph library. Every graph algorithm: shortest paths, centrality, communities, flows, matching, coloring, planarity. The standard for Python graph analysis.

**RLVR verifier quality**: **Excellent**. Already used in our graph_properties verifier.

---

### igraph

| | |
|---|---|
| **URL** | https://github.com/igraph/python-igraph |
| **License** | GPL v2 |
| **Cost** | **Free** |
| **Install** | `pip install igraph` |

**What it's great at**: Fast graph analysis (C core). 10-100x faster than NetworkX for large graphs. Community detection (Louvain, Leiden), layout algorithms.

---

### graph-tool

| | |
|---|---|
| **URL** | https://graph-tool.skewed.de |
| **License** | LGPL v3 |
| **Cost** | **Free** |

**What it's great at**: Fastest Python graph library (C++/Boost core). Stochastic block models, MCMC graph analysis. Best for large-scale network analysis.

---

## 21. Climate, Weather & Geospatial

### ERA5 (ECMWF Reanalysis)

| | |
|---|---|
| **URL** | https://cds.climate.copernicus.eu |
| **License** | Copernicus license (free, open, requires attribution) |
| **Cost** | **Free** |
| **Language** | Python via CDS API (`pip install cdsapi`) |

**What it's great at**: The gold standard climate reanalysis dataset. Hourly data from 1940-present. Global coverage, ~30km resolution. Temperature, pressure, wind, humidity, precipitation, radiation — hundreds of variables. Petabytes of data. Also available on Google BigQuery, AWS, and Azure.

**RLVR verifier quality**: **Excellent** for verifying any historical weather/climate claim against authoritative data.

---

### xarray + cfgrib

| | |
|---|---|
| **URL** | https://github.com/pydata/xarray |
| **License** | Apache 2.0 |
| **Cost** | **Free** |
| **Install** | `pip install xarray cfgrib` |

**What it's great at**: Multi-dimensional array analysis for climate/weather data (NetCDF, GRIB). The standard for working with ERA5, CMIP6, and other climate datasets.

---

### GeoPandas

| | |
|---|---|
| **URL** | https://github.com/geopandas/geopandas |
| **License** | BSD 3-Clause |
| **Cost** | **Free** |
| **Install** | `pip install geopandas` |

**What it's great at**: Geospatial data analysis in Python. Spatial joins, distance calculations, projections. Built on Shapely, Fiona, and pyproj.

---

### Shapely

| | |
|---|---|
| **URL** | https://shapely.readthedocs.io |
| **License** | BSD 3-Clause |
| **Cost** | **Free** |
| **Install** | `pip install shapely` |

**What it's great at**: Computational geometry in Python. Point, line, polygon operations. Set-theoretic operations (union, intersection, difference), buffering, convex hull, Voronoi, triangulation. Shapely 2.0 is vectorized and much faster than 1.x (backed by GEOS C library).

**RLVR verifier quality**: **Excellent** for computational geometry verification — area, intersection, containment, distance, validity.

---

### Rasterio

| | |
|---|---|
| **URL** | https://rasterio.readthedocs.io |
| **License** | BSD 3-Clause |
| **Cost** | **Free** |
| **Install** | `pip install rasterio` |

**What it's great at**: Pythonic raster data access. Reading/writing GeoTIFFs and other raster formats. Windowed reading, reprojection, masking, raster algebra. Clean API over GDAL's raster capabilities with NumPy integration.

**RLVR verifier quality**: **Good** for raster statistics, spatial extent, projection, and pixel value verification.

---

## 22. Game Engines & RL Environments

### Gymnasium (Farama Foundation)

| | |
|---|---|
| **URL** | https://github.com/Farama-Foundation/Gymnasium |
| **License** | MIT |
| **Cost** | **Free** |
| **Install** | `pip install gymnasium` |

**What it's great at**: The standard RL environment API. Successor to OpenAI Gym. CartPole, Atari, MuJoCo, Box2D environments.

**RLVR verifier quality**: **Excellent**. Outcome-based verification (reward/score).

---

### PettingZoo (Farama)

| | |
|---|---|
| **URL** | https://github.com/Farama-Foundation/PettingZoo |
| **License** | MIT |
| **Cost** | **Free** |
| **Install** | `pip install pettingzoo` |

**What it's great at**: Multi-agent RL environments. Chess, Go, Atari multi-player, MPE, SISL.

---

## 23. Materials Science

### ASE (Atomic Simulation Environment)

| | |
|---|---|
| **URL** | https://wiki.fysik.dtu.dk/ase/ |
| **License** | LGPL v2.1 |
| **Cost** | **Free** |
| **Language** | Python |
| **Install** | `pip install ase` |

**What it's great at**: The Python glue for atomistic simulation. Connects to 40+ simulation codes (VASP, Quantum ESPRESSO, GPAW, etc.). Structure manipulation, optimization, molecular dynamics, thermodynamics.

**RLVR verifier quality**: **Excellent** for materials science verification.

---

### pymatgen (Materials Project)

| | |
|---|---|
| **URL** | https://github.com/materialsproject/pymatgen |
| **License** | MIT |
| **Cost** | **Free** |
| **Install** | `pip install pymatgen` |

**What it's great at**: Materials analysis library from the Materials Project. Crystal structure analysis, phase diagrams, electronic structure, defect calculations. Interfaces with VASP, ABINIT, and Materials Project API.

---

### Quantum ESPRESSO

| | |
|---|---|
| **URL** | https://www.quantum-espresso.org |
| **License** | GPL v2 |
| **Cost** | **Free** |

**What it's great at**: Plane-wave DFT for periodic systems (crystals, surfaces). The standard for solid-state physics calculations. Band structures, phonons, molecular dynamics.

---

## 24. Epidemiology & Systems Biology

### COPASI

| | |
|---|---|
| **URL** | https://copasi.org |
| **License** | Artistic License 2.0 |
| **Cost** | **Free** |
| **Language** | C++, Python (BasiCO) |

**What it's great at**: Biochemical network simulation. ODE/stochastic simulation, parameter fitting, metabolic control analysis. Standard for systems biology.

---

### Mesa (Agent-Based Modeling)

| | |
|---|---|
| **URL** | https://github.com/projectmesa/mesa |
| **License** | Apache 2.0 |
| **Cost** | **Free** |
| **Language** | Python |
| **Install** | `pip install mesa` |

**What it's great at**: Agent-based modeling in Python. Epidemiology (SIR/SEIR), economics, social simulation.

---

### Tellurium

| | |
|---|---|
| **URL** | https://tellurium.analogmachine.org |
| **License** | Apache 2.0 |
| **Cost** | **Free** |
| **Install** | `pip install tellurium` |

**What it's great at**: Systems biology simulation. SBML models, ODE simulation, steady-state analysis, bifurcation analysis.

---

## 25. Power Systems & Electrical

### pandapower

| | |
|---|---|
| **URL** | https://github.com/e2nIEE/pandapower |
| **License** | BSD 3-Clause |
| **Cost** | **Free** |
| **Install** | `pip install pandapower` |

**What it's great at**: Power system analysis. Load flow, optimal power flow, short circuit calculation. Python API built on pandas.

---

### PyPSA

| | |
|---|---|
| **URL** | https://github.com/PyPSA/PyPSA |
| **License** | MIT |
| **Cost** | **Free** |
| **Install** | `pip install pypsa` |

**What it's great at**: Power system analysis and optimization. Energy system modeling, capacity expansion, optimal dispatch.

---

### OpenDSS (EPRI)

| | |
|---|---|
| **URL** | https://www.epri.com/pages/sa/opendss |
| **License** | BSD |
| **Cost** | **Free** |
| **Language** | Python via OpenDSSDirect.py or dss-python |

**What it's great at**: The industry standard for utility distribution system analysis. Time-series power flow (8760 hours), harmonics, fault analysis. Designed for distribution grids with DERs (solar, storage, EVs).

**RLVR verifier quality**: **Good** for distribution system and time-series power analysis tasks.

---

## 26. Recommendation Matrix

### For each RLVR domain, the recommended engine:

| RLVR Domain | Primary Engine | Backup | Install |
|-------------|---------------|--------|---------|
| **Physics simulation** | MuJoCo | Brax (GPU) | `pip install mujoco` |
| **Robotics control** | MuJoCo + Drake | Isaac Lab (visual) | `pip install mujoco drake` |
| **Fluid dynamics** | PhiFlow (ML) / OpenFOAM (eng.) | SU2 (aero) | `pip install phiflow` |
| **FEM / structural** | FEniCSx | CalculiX | `pip install fenics-dolfinx` |
| **Cheminformatics** | RDKit | Open Babel | `pip install rdkit` |
| **Quantum chemistry** | xTB (fast) / PySCF (accurate) | Psi4 | `pip install xtb-python pyscf` |
| **Molecular dynamics** | OpenMM (bio) / LAMMPS (materials) | GROMACS | `conda install openmm` |
| **Protein structure** | Boltz-1 (open) / AlphaFold 3 | ESMFold (fast) | `pip install boltz` |
| **Protein design** | RFdiffusion + ProteinMPNN | — | See GitHub |
| **Molecular docking** | AutoDock Vina | GNINA, DiffDock, Uni-Dock (GPU) | `pip install vina` |
| **Retrosynthesis** | AiZynthFinder | RDChiral | `pip install aizynthfinder` |
| **DNA/RNA design** | NUPACK (complex) / ViennaRNA (simple) | — | See URLs |
| **Bioinformatics** | Biopython + BLAST+ | — | `pip install biopython` |
| **Analog circuits** | PySpice (ngspice) | Xyce | `pip install PySpice` |
| **Digital RTL** | Verilator + Cocotb | Icarus Verilog | `pip install cocotb` |
| **FPGA synthesis** | Yosys + nextpnr | — | `apt install yosys` |
| **Control systems** | python-control | scipy.signal | `pip install control` |
| **Quantum circuits** | Qiskit Aer | Cirq, PennyLane | `pip install qiskit-aer` |
| **Quantum error correction** | Stim | — | `pip install stim` |
| **Theorem proving** | Lean 4 (math) / Coq (CS) | Isabelle | See URLs |
| **SMT solving** | Z3 | CVC5 | `pip install z3-solver` |
| **First-order ATP** | Vampire | E prover | See URLs |
| **Symbolic math** | SymPy (light) / SageMath (full) | Wolfram Engine (free dev) | `pip install sympy` |
| **Group theory** | GAP | SageMath | See URLs |
| **Number theory** | PARI/GP | SageMath | See URLs |
| **LP/MIP optimization** | HiGHS (free) / Gurobi (academic) | SCIP | `pip install highspy` |
| **Constraint satisfaction** | OR-Tools CP-SAT | MiniZinc | `pip install ortools` |
| **Graph algorithms** | NetworkX | igraph (large graphs) | `pip install networkx` |
| **Climate/weather data** | xarray + cfgrib | — | `pip install xarray` |
| **Geospatial** | GeoPandas + Shapely (vector) | Rasterio (raster) | `pip install geopandas` |
| **Materials science** | ASE + pymatgen | Quantum ESPRESSO | `pip install ase pymatgen` |
| **Systems biology** | Tellurium / COPASI | — | `pip install tellurium` |
| **Epidemiology** | Mesa | EpiModel | `pip install mesa` |
| **Power systems** | pandapower / PyPSA | OpenDSS (distribution) | `pip install pandapower` |
| **RL environments** | Gymnasium | PettingZoo (multi-agent) | `pip install gymnasium` |
| **Vehicle dynamics** | Project Chrono | — | See URLs |

### Cost Summary

| Category | Engine | Cost |
|----------|--------|------|
| **Fully Free (Open Source)** | MuJoCo, Brax, Warp, RDKit, PySCF, OpenMM, LAMMPS, Verilator, Cocotb, Z3, Lean 4, HiGHS, OR-Tools, NetworkX, SymPy, xTB | $0 |
| **Free for Academic** | ORCA, PyRosetta, NUPACK, Gurobi, AlphaFold 3, Chai-1 | $0 (academic) |
| **Free but GPU Required** | Isaac Sim, DiffDock, ESMFold, Boltz-1, RFdiffusion | GPU cost only |
| **Commercial** | Gaussian ($4.5K-28K), Gurobi ($12K+/yr), MATLAB ($2K+/yr) | Varies |
| **Avoid for RLVR** | Gaussian (expensive, no API), LTspice (no API), MATLAB (expensive) | — |

### Total stack cost for comprehensive RLVR verification: **$0** using only open-source tools.
