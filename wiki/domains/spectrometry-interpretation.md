---
domain: Spectrometry Interpretation
category: science-chemistry
verification_type: exact_match
dataset_scale: 35K-100K+ spectra (SDBS, MassBank)
difficulty_range: easy/medium/hard
modality: multimodal
status: strong_hypothesis
---

# Spectrometry Interpretation

## Overview

Spectrometry interpretation identifies chemical compounds from spectroscopic data: NMR (nuclear magnetic resonance), mass spectrometry (MS), infrared spectroscopy (IR), and UV-Vis spectra. Given a spectrum (peaks, patterns, mass-to-charge ratios), the model must determine the molecular formula, functional groups, or full molecular structure (SMILES).

RLVR verification works at multiple levels: molecular formula match is exact, functional group identification can be checked against known databases, and full SMILES prediction can be verified via canonicalization. The challenge is that spectra-to-structure mapping can be ambiguous (multiple structures produce similar spectra), but database-backed verification covers the common cases.

## Verification Mechanism

```python
from rdkit import Chem

def verify_molecular_formula(predicted: str, gold: str) -> dict:
    """Exact match on molecular formula."""
    pred_clean = normalize_formula(predicted.strip())
    gold_clean = normalize_formula(gold.strip())
    match = pred_clean == gold_clean
    return {"exact_match": match, "reward": 1.0 if match else 0.0}

def normalize_formula(formula: str) -> str:
    """Normalize to Hill system: C first, H second, then alphabetical."""
    from collections import OrderedDict
    import re
    elements = re.findall(r'([A-Z][a-z]?)(\d*)', formula)
    counts = {}
    for elem, count in elements:
        if elem:
            counts[elem] = counts.get(elem, 0) + int(count if count else 1)
    # Hill system ordering
    result = ""
    for elem in ['C', 'H']:
        if elem in counts:
            result += elem + (str(counts[elem]) if counts[elem] > 1 else "")
            del counts[elem]
    for elem in sorted(counts):
        result += elem + (str(counts[elem]) if counts[elem] > 1 else "")
    return result

def verify_structure_from_spectrum(predicted_smiles: str, gold_smiles: str) -> dict:
    """Full structure verification via canonical SMILES."""
    pred_mol = Chem.MolFromSmiles(predicted_smiles)
    gold_mol = Chem.MolFromSmiles(gold_smiles)

    if pred_mol is None:
        return {"valid": False, "exact_match": False, "reward": 0.0}

    pred_canon = Chem.MolToSmiles(pred_mol, canonical=True)
    gold_canon = Chem.MolToSmiles(gold_mol, canonical=True)

    exact = pred_canon == gold_canon

    # Partial credit: correct molecular formula even if structure is wrong
    pred_formula = Chem.rdMolDescriptors.CalcMolFormula(pred_mol)
    gold_formula = Chem.rdMolDescriptors.CalcMolFormula(gold_mol)
    formula_match = pred_formula == gold_formula

    reward = 1.0 if exact else (0.3 if formula_match else 0.0)

    return {
        "valid": True,
        "exact_match": exact,
        "formula_match": formula_match,
        "reward": reward
    }
```

## Dataset Sources

- **SDBS (Spectral Database for Organic Compounds)**: ~35,000 compounds with NMR (1H, 13C), IR, MS, and Raman spectra. Maintained by AIST (Japan). Freely accessible online. The most comprehensive multi-technique spectral reference.
- **MassBank**: 100,000+ mass spectra of metabolites, environmental chemicals, and drugs. Community-curated, open access. Multiple ionization modes.
- **HMDB (Human Metabolome Database)**: 220,000+ metabolites with associated MS and NMR spectra. Focus on human metabolomics.
- **NIST WebBook / NIST MS Library**: ~350,000 mass spectra (commercial for full library). The gold standard for MS identification.
- **NMRShiftDB**: ~50,000 organic structures with experimental NMR shifts. Open access.
- **MoNA (MassBank of North America)**: 800,000+ mass spectra aggregated from multiple sources.
- **METLIN**: 1M+ MS/MS spectra, primarily for metabolomics (commercial for full access).
- **Synthetic generation**: For NMR, chemical shift prediction software (MestReNova, ACD/Labs) can generate predicted spectra for any SMILES, enabling unlimited training data generation.

## Task Format

- **Input**: Spectral data (peak lists, spectrum image, or mass-to-charge ratios) with context.
```
Identify the compound from this 1H NMR spectrum (CDCl3, 400 MHz):
Peaks: delta 7.26 (5H, m), 3.65 (2H, s), 2.20 (3H, s)
Molecular formula from MS: C9H10O
IR: strong absorption at 1715 cm-1 (C=O stretch)

What is the structure? Provide SMILES.
```
- **Output**: Molecular identification.
```
CC(=O)Cc1ccccc1
(phenylacetone / 1-phenyl-2-propanone)
Explanation: The 5H multiplet at 7.26 is a monosubstituted benzene ring,
the 2H singlet at 3.65 is a benzylic CH2, and the 3H singlet at 2.20
is a methyl ketone. The IR carbonyl stretch at 1715 confirms a ketone.
```

## Difficulty Curriculum

- Level 1: Identify molecular formula from molecular ion peak in mass spectrum
- Level 2: Identify functional groups from IR peak positions (OH, C=O, NH, etc.)
- Level 3: Identify simple compounds (< 8 heavy atoms) from 1H NMR peak patterns
- Level 4: Determine molecular structure from combined NMR + MS data for small molecules
- Level 5: Interpret 13C NMR and DEPT to assign carbon types
- Level 6: Complex structure determination using 2D NMR (COSY, HSQC, HMBC)
- Level 7: Distinguish isomers with similar spectra (regioisomers, stereoisomers)
- Level 8: Natural product structure elucidation from spectral data
- Level 9: Novel compound identification not in spectral databases, degraded/noisy spectra

## Limitations & Risks

- **Spectral ambiguity**: Multiple compounds can have very similar spectra, especially in mass spectrometry. Without additional data (retention time, exact mass, fragmentation), identification can be genuinely ambiguous.
- **Database coverage**: SDBS covers ~35K compounds, but there are millions of known organic compounds. Verification is only possible for compounds in reference databases.
- **Data format**: Spectra can be represented as peak lists (text-friendly), numerical arrays, or images. Text-based models can process peak lists but not raw spectral images.
- **Instrument variation**: Spectra vary with instrument type, conditions (solvent, temperature, ionization mode). Models must generalize across measurement conditions.
- **Stereochemistry from spectra**: Distinguishing enantiomers requires chiral techniques (circular dichroism, chiral HPLC). Standard NMR/MS cannot distinguish enantiomers, making stereochemical assignment unverifiable from standard spectra alone.
- **Partial identification**: Sometimes only the molecular formula or functional groups can be determined, not the full structure. The reward function should give partial credit for partial identification.

## Connections

- [[forward-reaction-prediction]] — Product prediction and product identification are complementary: predict what you will make, then verify by spectrum
- [[chemical-equation-balancing]] — Molecular formula determination is a prerequisite for balancing
- [[protein-ligand-docking]] — Spectroscopy is used to experimentally validate binding interactions
- [[chemistry-retrosynthesis]] — Retrosynthetic planning combined with spectral verification forms a complete chemistry workflow
