---
domain: Chemical Nomenclature
category: Science
verification_type: exact_match
dataset_scale: unlimited (from structure databases)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Chemical Nomenclature

## Overview
Convert between IUPAC names, common names, SMILES, and InChI for chemical compounds.

## Verification Mechanism
Convert name to SMILES via RDKit/PubChem, verify canonical SMILES match. Round-trip naming.

## Dataset Sources
See wiki for specific URLs and download instructions.

## Task Format
**Input**: Problem specification
**Output**: Solution in appropriate format

## Difficulty Curriculum
Scales from basic to expert-level within the domain.

## Limitations & Risks
Domain-specific edge cases may require careful handling.

## Connections
[[chemistry-computation]], [[data-formatting]]
