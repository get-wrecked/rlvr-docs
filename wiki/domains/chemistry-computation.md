---
domain: Chemistry Computation
category: Science
verification_type: Cheminformatics computation (RDKit, OpenBabel) and conservation law checks
dataset_scale: Large (100K+ from MoleculeNet/PubChem)
difficulty_range: High-school stoichiometry to graduate-level molecular property prediction
modality: Text-in, text-out (SMILES, numerical values, balanced equations)
status: Strong — verification is reliable for well-defined chemical computations
---

## Overview

Chemistry computation tasks ask the model to solve chemical problems that have objectively computable answers: balance equations, predict molecular properties from structure, compute stoichiometric quantities, determine molecular formulas, and classify chemical species. The key advantage for RLVR is that chemistry has a rich ecosystem of validated computational tools (RDKit, OpenBabel, PySCF) that can serve as verifiers.

This domain covers stoichiometry (mass/mole calculations, limiting reagents), molecular properties (molecular weight, LogP, topological polar surface area), equation balancing, oxidation state determination, and molecular structure queries (functional group identification, stereochemistry).

## Verification Mechanism

**Primary approach:** Use established cheminformatics libraries to compute the correct answer from the problem's chemical inputs, then compare to the model's output.

- **SMILES/InChI validation:** For problems that produce molecular structures, verify the output is a valid SMILES string that matches the expected molecule (canonical SMILES comparison or InChI matching).
- **Numerical property computation:** For molecular properties (MW, LogP, TPSA, number of rotatable bonds), compute using RDKit from the input molecule and compare to model output within tolerance.
- **Equation balancing verification:** Check that all elements have equal counts on both sides, and that coefficients are in lowest integer terms. This is exact verification.
- **Stoichiometry verification:** Given input quantities and a balanced equation, compute expected products/yields. Compare to model output.
- **Conservation law checks:** Verify conservation of mass, charge, and atom counts across reactions.

**Verification reliability: HIGH for stoichiometry and equation balancing.** These are fully deterministic computations with exact answers.

**Verification reliability: HIGH for molecular descriptor computation.** RDKit's molecular descriptors are deterministic given a molecular structure.

**Verification reliability: MODERATE for property prediction against experimental values.** Experimental measurements have uncertainty. Computed properties (especially from DFT or force fields) are approximations. However, for RLVR purposes we can use the computational answer as ground truth and accept this approximation.

## Dataset Sources

- **MoleculeNet:** Standard benchmark collection for molecular ML. Includes datasets for solubility, toxicity, lipophilicity, and more. ~700K+ data points across all tasks.
- **PubChem:** Massive chemical database (100M+ compounds) with computed and experimental properties. Rich source for molecular property tasks.
- **QM7, QM8, QM9:** Quantum chemistry datasets with DFT-computed properties for small molecules.
- **Textbook problems:** General chemistry and organic chemistry textbooks provide stoichiometry and equation balancing problems. Require digitization.
- **Synthetic generation:** Parameterize reaction templates (e.g., "Given X grams of A reacting with Y grams of B in reaction A + 2B -> C, find mass of C produced"). Highly scalable.

**Realistic scale:** 100K+ problems achievable, especially with synthetic generation from reaction templates and molecular databases.

## Task Format

**Input:** Natural language chemistry problem or structured chemical query.

Example 1 (stoichiometry):
```
How many grams of water are produced when 10.0 g of hydrogen gas
reacts completely with excess oxygen gas?
2H2 + O2 -> 2H2O
```

Example 2 (molecular property):
```
What is the molecular weight of aspirin (SMILES: CC(=O)Oc1ccccc1C(=O)O)?
```

Example 3 (equation balancing):
```
Balance the following chemical equation:
Fe + O2 -> Fe2O3
```

**Output:** Numerical answer with units, balanced equation, or molecular property value.

## Difficulty Curriculum

1. **Level 1 — Direct computation:** Molecular weight from formula, simple mole conversions, counting atoms in a formula.
2. **Level 2 — Single-step problems:** Equation balancing, mole-to-mass conversions with a given reaction, functional group identification.
3. **Level 3 — Multi-step stoichiometry:** Limiting reagent calculations, percent yield, solution dilution chains.
4. **Level 4 — Molecular property reasoning:** Predict LogP, pKa, solubility from structure. Requires understanding structure-property relationships.
5. **Level 5 — Complex molecular tasks:** Retrosynthesis fragments (see chemistry-retrosynthesis.md), stereochemistry determination, reaction mechanism prediction.

## Limitations & Risks

- **SMILES parsing fragility:** Models often produce invalid SMILES strings. The verification must handle this gracefully (invalid SMILES = automatic failure, not a crash).
- **Canonical form ambiguity:** The same molecule can be represented by many valid SMILES strings. Verification must canonicalize before comparing (RDKit handles this).
- **Significant figures:** Chemistry problems often care about significant figures. The model may give 89.47 when the expected answer is 89.5. Tolerance and sig-fig handling must be explicit.
- **Implicit assumptions:** Many textbook problems assume STP, ideal gas behavior, 100% yield unless stated otherwise. The verification must share these assumptions.
- **Property prediction vs. computation:** Some "chemistry" tasks (e.g., predicting toxicity) don't have computable ground truth — they rely on experimental data. These are less suitable for RLVR unless framed as lookup/interpolation tasks.
- **Reward hacking:** The model could memorize common molecules' properties rather than learning to reason about structure. Diverse molecular scaffolds in training data help mitigate this.

## Connections

- Directly related to **chemistry-retrosynthesis.md** (retrosynthesis is a higher-order chemistry task)
- Shares computational verification philosophy with **physics-simulation.md**
- **materials-science.md** overlaps for solid-state chemistry and crystal property prediction
- Molecular property tasks connect to **biology-sequence.md** (drug-target interactions)
- Equation balancing is formally a linear algebra / constraint satisfaction problem, connecting to mathematical reasoning domains
