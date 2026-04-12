---
domain: Materials Science
category: Science
verification_type: DFT calculations, experimental database lookup, structure-property models
dataset_scale: Large (100K+ entries in Materials Project, AFLOW, OQMD)
difficulty_range: Crystal structure lookup to novel material property prediction
modality: Text-in, text-out (numerical properties, crystal structures, compositions)
status: Mixed — verification is strong for known materials, weak for novel predictions
---

## Overview

Materials science tasks ask the model to predict properties of materials from their composition and structure: band gap, elastic modulus, formation energy, thermal conductivity, magnetic ordering, and more. The RLVR opportunity is that extensive computational databases exist (Materials Project, AFLOW, OQMD) containing DFT-calculated properties for hundreds of thousands of materials, providing a large pool of verifiable ground truth.

This domain covers crystal structure prediction, property prediction from composition, phase stability analysis, defect calculations, and materials selection/screening. The central challenge is that verification quality depends heavily on whether we are checking against computed DFT values (reproducible) or experimental measurements (noisy).

## Verification Mechanism

**Primary approach:** Compare model predictions to values in validated computational/experimental materials databases.

- **DFT database lookup:** For known materials, look up DFT-computed properties in Materials Project, AFLOW, or OQMD. Compare model prediction within tolerance. DFT values are reproducible given the same functional and parameters.
- **Formation energy verification:** Check that predicted formation energy is consistent with database values. Also verify that the predicted ground-state structure has lower energy than competing polymorphs.
- **Crystal structure validation:** Verify that a predicted crystal structure has the correct space group, Wyckoff positions, and lattice parameters (within tolerance) matching known structures.
- **Composition-based rules:** Verify charge neutrality, oxidation state feasibility, Pauling's rules for ionic compounds. These are necessary but not sufficient checks.
- **Experimental database comparison:** For properties with experimental measurements (CRC Handbook, NIST), compare against measured values. Tolerances must account for measurement uncertainty.

**Verification reliability: HIGH for DFT-computed properties of known materials.** DFT calculations are reproducible, and databases contain consistent values. But note: DFT itself is an approximation — it systematically underestimates band gaps (LDA/GGA) by ~40%, for instance.

**Verification reliability: HIGH for structural properties of known materials.** Lattice parameters, space groups, and crystal structures are well-determined.

**Verification reliability: MODERATE for comparing to experimental values.** Experimental measurements have uncertainty, and DFT values may disagree with experiment. The "ground truth" for RLVR should be clearly defined as either DFT-computed or experimental.

**Verification reliability: LOW for predicting properties of hypothetical/novel materials.** If the material is not in any database, there is no ground truth to verify against without running a new DFT calculation (which is computationally expensive and slow).

## Dataset Sources

- **Materials Project:** ~150K inorganic compounds with DFT-computed properties (energy, band gap, elastic tensor, etc.). Well-maintained API.
- **AFLOW:** ~3.5M material entries with calculated properties. Largest computational materials database.
- **OQMD (Open Quantum Materials Database):** ~1M DFT calculations focused on thermodynamic stability.
- **ICSD (Inorganic Crystal Structure Database):** ~250K experimentally determined crystal structures. Requires license.
- **JARVIS-DFT:** ~75K materials with diverse computed properties including solar efficiency, thermoelectric properties.
- **Matbench:** Benchmark suite for materials property prediction with standardized train/test splits.

**Realistic scale:** 50K-100K well-defined prediction problems with DFT-verified answers. The databases are large enough; the bottleneck is creating well-posed natural language problems around the data.

## Task Format

**Input:** Material composition/structure and property query.

Example 1 (property prediction):
```
What is the DFT-computed band gap (in eV) of cubic silicon carbide (3C-SiC)
according to GGA-PBE calculations?
```

Example 2 (stability):
```
Is the perovskite compound CsSnI3 thermodynamically stable with respect
to decomposition into CsI and SnI2, based on DFT formation energies?
```

Example 3 (structure):
```
What is the space group of rutile TiO2? What are the lattice parameters
a and c (in Angstroms)?
```

**Output:** Numerical property values, yes/no stability assessments, structural parameters.

## Difficulty Curriculum

1. **Level 1 — Database lookup:** Retrieve known properties of well-studied materials (Si, NaCl, Fe). Tests knowledge retrieval.
2. **Level 2 — Simple property reasoning:** Predict trends (e.g., band gap increases with electronegativity difference). Compute properties from given structure using simple models.
3. **Level 3 — Phase stability analysis:** Determine stability from formation energies and competing phases. Construct and analyze convex hulls.
4. **Level 4 — Structure-property relationships:** Predict properties of less common materials by reasoning about crystal structure, bonding, and electronic effects.
5. **Level 5 — Novel material prediction:** Suggest materials with desired properties. Predict properties of hypothetical compositions. Verification requires new DFT calculations or is impossible without them.

## Limitations & Risks

- **DFT accuracy limitations:** DFT is an approximation. GGA-PBE systematically underestimates band gaps. Formation energies have ~50 meV/atom error. The model might learn to reproduce DFT errors rather than physical truth.
- **Data memorization concern:** With only ~150K entries in Materials Project, the model might memorize database entries rather than learn generalizable materials science. Holdout validation is critical.
- **Composition vs. structure ambiguity:** Many compositions have multiple polymorphs with different properties. The problem must specify the exact structure or polymorph.
- **Temperature and pressure dependence:** Database values are typically for 0K and 0 pressure. Real-world properties depend on conditions. The problem must specify conditions or accept 0K/0atm values.
- **Experimental data noise:** Experimental measurements vary between labs, samples, and measurement techniques. Using DFT values as ground truth is more reproducible but introduces systematic bias.
- **Computational cost for novel verification:** Verifying predictions for novel materials requires running new DFT calculations (~hours per material), making real-time verification impractical.

## Connections

- **chemistry-computation.md** provides molecular-level understanding that underlies materials behavior
- **physics-simulation.md** connects via condensed matter physics and statistical mechanics
- **engineering-optimization.md** overlaps for materials selection and design optimization
- **biology-sequence.md** has analogous challenges (property prediction from sequence/structure)
- Computational databases like Materials Project are the materials analogue of PDB in biology
