---
domain: Molecular Generation & Drug Design
category: science-chemistry
verification_type: execution
dataset_scale: 10M+ (PubChem, ChEMBL, ZINC)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Molecular Generation & Drug Design

## Overview
Generate molecules (as SMILES strings or molecular graphs) that satisfy specified property constraints: drug-likeness, binding affinity predictions, synthetic accessibility, solubility, toxicity thresholds. Verification via computational chemistry tools.

## Verification Mechanism
```python
from rdkit import Chem
from rdkit.Chem import Descriptors, QED, Crippen

def verify(constraints: dict, smiles: str) -> float:
    mol = Chem.MolFromSmiles(smiles)
    if mol is None:
        return 0.0  # Invalid SMILES
    
    score = 0.0
    checks = 0
    
    # Validity check (already passed if mol is not None)
    checks += 1
    score += 1.0
    
    # Drug-likeness (Lipinski's Rule of 5)
    if "lipinski" in constraints:
        checks += 1
        mw = Descriptors.MolWt(mol)
        logp = Crippen.MolLogP(mol)
        hbd = Descriptors.NumHDonors(mol)
        hba = Descriptors.NumHAcceptors(mol)
        if mw <= 500 and logp <= 5 and hbd <= 5 and hba <= 10:
            score += 1.0
    
    # QED (quantitative drug-likeness)
    if "min_qed" in constraints:
        checks += 1
        qed = QED.qed(mol)
        if qed >= constraints["min_qed"]:
            score += 1.0
    
    # Synthetic accessibility
    if "max_sa" in constraints:
        checks += 1
        sa = sascorer.calculateScore(mol)
        if sa <= constraints["max_sa"]:
            score += 1.0
    
    # Molecular weight range
    if "mw_range" in constraints:
        checks += 1
        mw = Descriptors.MolWt(mol)
        if constraints["mw_range"][0] <= mw <= constraints["mw_range"][1]:
            score += 1.0
    
    # Specific substructure required/forbidden
    if "required_substructure" in constraints:
        checks += 1
        pattern = Chem.MolFromSmarts(constraints["required_substructure"])
        if mol.HasSubstructMatch(pattern):
            score += 1.0
    
    if "forbidden_substructure" in constraints:
        checks += 1
        pattern = Chem.MolFromSmarts(constraints["forbidden_substructure"])
        if not mol.HasSubstructMatch(pattern):
            score += 1.0
    
    # Similarity to reference molecule
    if "reference_smiles" in constraints:
        checks += 1
        ref = Chem.MolFromSmiles(constraints["reference_smiles"])
        sim = DataStructs.TanimotoSimilarity(
            AllChem.GetMorganFingerprintAsBitVect(mol, 2),
            AllChem.GetMorganFingerprintAsBitVect(ref, 2)
        )
        if sim >= constraints.get("min_similarity", 0.7):
            score += 1.0
    
    return score / checks
```

## Dataset Sources
- **PubChem**: 110M+ compounds with properties.
- **ChEMBL**: 2M+ bioactive molecules with assay data.
- **ZINC**: 750M+ commercially available compounds.
- **GuacaMol benchmark**: Molecular generation benchmark with defined objectives.
- **MOSES**: Molecular sets benchmark for generative models.
- **Practical Molecular Optimization (PMO)**: 23 optimization objectives.
- **MoleculeNet**: Property prediction benchmarks.

## Task Format
- **Input**: "Generate a molecule with MW between 300-500, LogP between 1-3, at least one nitrogen-containing ring, and QED > 0.6"
- **Output**: SMILES string (e.g., "c1cc(N)c2c(c1)CC(=O)N2C")

## Difficulty Curriculum
- Level 1: Generate valid SMILES satisfying basic MW/LogP constraints
- Level 3: Multi-property optimization (QED + SA + target properties)
- Level 5: Scaffold-constrained generation (keep core, modify periphery)
- Level 7: Multi-objective optimization with conflicting constraints
- Level 9: Design molecules for specific protein targets (requires docking simulation)

## Limitations & Risks
- Property prediction (especially binding affinity) is approximate — computational predictions don't always match experimental results.
- For simple property constraints (validity, MW, LogP, Lipinski), verification is exact.
- For docking-based affinity prediction, verification is approximate but standard in the field (AutoDock Vina, etc.).
- Novelty is hard to verify — how do you know a molecule is truly novel vs. trivially modified?

## Connections
- [[chemistry-computation]] — chemical property computation
- [[chemistry-retrosynthesis]] — how to actually make the designed molecule
- [[combinatorics-optimization]] — molecular design as optimization
