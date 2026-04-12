---
domain: Chemistry Retrosynthesis
category: Science
verification_type: Reaction database lookup, forward synthesis simulation, chemical feasibility checks
dataset_scale: Medium (50K-200K reactions in USPTO; larger in proprietary databases)
difficulty_range: Single-step retrosynthesis to multi-step route planning
modality: Text-in, text-out (SMILES reaction sequences)
status: Partially verifiable — single-step verification is solid; multi-step route quality is harder to assess
---

## Overview

Retrosynthesis tasks ask the model to plan synthetic routes to target molecules: given a target compound, propose a sequence of reactions working backward from target to available starting materials. This is one of the most practically valuable chemistry AI tasks — retrosynthetic planning saves chemists weeks of manual route design. The RLVR angle is that each proposed reaction step can be checked against databases of known reactions, and the overall route can be verified for chemical feasibility.

This domain covers single-step retrosynthesis (propose one disconnection), multi-step retrosynthesis (plan a full route), reaction condition prediction (specify reagents, solvents, temperature), and starting material availability checking. The ultimate goal is to generate complete, executable synthesis plans.

## Verification Mechanism

**Primary approach:** Verify that each proposed reaction step is chemically valid by checking against reaction databases and chemical feasibility rules.

- **Reaction template matching:** Check whether each proposed reaction matches a known reaction template in the database (USPTO, Reaxys). A template encodes the transformation rule (e.g., "ester hydrolysis: RCOOR' + H2O -> RCOOH + R'OH").
- **Forward reaction prediction:** Run a forward synthesis predictor (e.g., trained ML model or rule-based system) on the proposed reactants and conditions. Check that the predicted product matches the expected intermediate.
- **Atom mapping verification:** Verify that all atoms in products can be traced back to atoms in reactants. Conservation of atoms is a hard constraint.
- **Starting material availability:** Check that the proposed starting materials are commercially available (e.g., appear in catalogs like Sigma-Aldrich, Enamine).
- **Route completeness:** Verify that the route forms a connected DAG from available starting materials to the target with no missing steps.
- **Chemical feasibility rules:** Check for obvious infeasibilities: impossible oxidation states, forbidden functional group combinations under proposed conditions, stereochemical inconsistencies.

**Verification reliability: HIGH for single-step retrosynthesis** against known reaction templates. If the exact transformation appears in USPTO, it is valid. Template matching is deterministic.

**Verification reliability: MODERATE for multi-step routes.** Each individual step might be valid, but the overall route could be impractical (incompatible protecting groups, reagent incompatibilities across steps, poor yield accumulation). Assessing route quality holistically is harder.

**Verification reliability: LOW-to-MODERATE for novel reactions.** If the model proposes a reaction not in the database, that does not mean it is wrong — it might be a genuinely novel transformation. But we cannot verify it programmatically. This is a fundamental limitation: absence of evidence is not evidence of absence.

## Dataset Sources

- **USPTO (United States Patent and Trademark Office):** ~1M+ extracted reactions from patents. Freely available, well-studied. Noisy (patent reactions are not always reproducible).
- **Reaxys (Elsevier):** ~50M reactions with conditions. Commercial database; highest quality but requires license.
- **Pistachio (NextMove Software):** Cleaned and classified version of USPTO. ~3M reactions.
- **Retrosynthesis benchmarks:** Lin et al. (2020), Schwaller et al. (2019) provide standardized test sets for retrosynthesis evaluation.
- **ASKCOS (MIT):** Open-source retrosynthesis planning tool with its own curated dataset and evaluation framework.

**Realistic scale:** 50K-100K single-step retrosynthesis problems with database-verified answers. Multi-step routes are harder — maybe 5K-10K fully verified multi-step routes.

## Task Format

**Input:** Target molecule (as SMILES) and optionally constraints (max steps, available starting materials, prohibited reagents).

Example 1 (single-step):
```
Propose a single-step retrosynthetic disconnection for ibuprofen
(SMILES: CC(C)Cc1ccc(cc1)C(C)C(=O)O).
Give the reactants as SMILES and the reaction type.
```

Example 2 (multi-step):
```
Plan a complete retrosynthetic route for oseltamivir (Tamiflu) starting
from commercially available materials. Provide each step as a SMILES
reaction (reactants >> products) with reaction conditions.
Maximum 8 steps.
```

**Output:** Reaction SMILES for each step, reaction types, and optionally conditions.

## Difficulty Curriculum

1. **Level 1 — Functional group transformations:** Simple one-step reactions (ester hydrolysis, alcohol oxidation, Grignard addition). Directly matched to database templates.
2. **Level 2 — Standard disconnections:** Two to three-step routes using well-known reactions (Suzuki coupling, Wittig, Diels-Alder). Each step is common.
3. **Level 3 — Protecting group strategies:** Routes requiring strategic use of protecting groups to prevent unwanted side reactions. Tests planning ability.
4. **Level 4 — Complex natural products:** Multi-step (5-10 steps) routes to moderately complex molecules. Require creative disconnection strategies.
5. **Level 5 — Novel targets:** Design routes to molecules not in the training data. This pushes beyond database lookup into genuine chemical reasoning. Verification is weakest here.

## Limitations & Risks

- **SMILES validity:** Models frequently generate syntactically invalid or chemically nonsensical SMILES. A preprocessing validator is essential.
- **Database coverage bias:** USPTO is biased toward patent chemistry (pharmaceuticals, agrochemicals). Reactions common in academic labs but rare in patents may be incorrectly rejected by the verifier.
- **Novelty penalty:** The verification penalizes novel chemistry because unknown reactions cannot be confirmed. This could train the model to be conservative rather than creative — the opposite of what chemists want.
- **Yield and selectivity:** A route may be "valid" in that each reaction works, but impractical due to low yields (say, 5% at one step). Yield data is sparse and noisy. Most RLVR setups will ignore yield, which limits practical value.
- **Condition prediction:** Specifying exact reaction conditions (temperature, solvent, catalyst loading) is much harder than proposing a disconnection. Condition prediction verification requires matching to database conditions, which may not be unique.
- **Stereochemistry:** Many reactions create stereocenters. Verifying that the model predicts the correct stereochemical outcome requires careful SMILES/InChI comparison with stereochemistry preserved. This is technically feasible but adds complexity.

## Connections

- Builds on **chemistry-computation.md** for fundamental chemistry knowledge (balancing, molecular properties)
- **materials-science.md** uses synthesis planning for materials discovery
- Route optimization is a planning problem, connecting to **robotics-planning.md** (different domain, similar algorithmic structure)
- Reaction databases are the chemistry analogue of protein databases in **biology-sequence.md** — curated knowledge bases that enable verification
- **engineering-optimization.md** shares the multi-objective nature (optimize yield, cost, step count simultaneously)
