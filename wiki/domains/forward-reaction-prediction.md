---
domain: Forward Reaction Prediction
category: science-chemistry
verification_type: exact_match
dataset_scale: 1.8M reactions (USPTO)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Forward Reaction Prediction

## Overview

Forward reaction prediction takes reactants and reagents as input and predicts the products of the chemical reaction. Given "what do I start with?", the model must answer "what do I get?". This is the forward complement to retrosynthesis (which works backward from products to reactants).

RLVR verification is strong: the predicted product SMILES is compared against the known product. Canonicalized SMILES comparison provides exact match verification, and molecular fingerprint similarity provides a continuous backup signal. The USPTO dataset provides 1.8M+ reactions with known products, making this one of the largest verifiable chemistry domains.

## Verification Mechanism

```python
from rdkit import Chem
from rdkit.Chem import AllChem, DataStructs

def canonicalize_smiles(smi: str) -> str:
    """Convert SMILES to canonical form for comparison."""
    mol = Chem.MolFromSmiles(smi)
    if mol is None:
        return None
    return Chem.MolToSmiles(mol, canonical=True)

def verify_reaction_prediction(predicted_smiles: str, gold_smiles: str) -> dict:
    """Compare predicted product SMILES against gold."""
    pred_canon = canonicalize_smiles(predicted_smiles)
    gold_canon = canonicalize_smiles(gold_smiles)

    if pred_canon is None:
        return {"valid_smiles": False, "exact_match": False,
                "tanimoto": 0.0, "reward": 0.0}

    exact = (pred_canon == gold_canon)

    # Tanimoto similarity on Morgan fingerprints for partial credit
    pred_mol = Chem.MolFromSmiles(pred_canon)
    gold_mol = Chem.MolFromSmiles(gold_canon)
    pred_fp = AllChem.GetMorganFingerprintAsBitVect(pred_mol, 2, nBits=2048)
    gold_fp = AllChem.GetMorganFingerprintAsBitVect(gold_mol, 2, nBits=2048)
    tanimoto = DataStructs.TanimotoSimilarity(pred_fp, gold_fp)

    return {
        "valid_smiles": True,
        "exact_match": exact,
        "tanimoto": tanimoto,
        "reward": 1.0 if exact else tanimoto * 0.5
    }

def verify_with_atom_conservation(reactant_smiles: str, predicted_product: str) -> bool:
    """Check that atoms are conserved (no atoms created/destroyed)."""
    from collections import Counter
    reactant_mol = Chem.MolFromSmiles(reactant_smiles)
    product_mol = Chem.MolFromSmiles(predicted_product)
    if reactant_mol is None or product_mol is None:
        return False
    r_atoms = Counter(atom.GetSymbol() for atom in reactant_mol.GetAtoms())
    p_atoms = Counter(atom.GetSymbol() for atom in product_mol.GetAtoms())
    # Note: reagents/solvents may contribute atoms; full reaction SMILES needed
    return r_atoms == p_atoms
```

## Dataset Sources

- **USPTO-MIT (Jin et al.)**: ~480K reactions extracted from US patents, cleaned and split. The most widely used academic benchmark for forward prediction.
- **USPTO-Full**: ~1.8M reactions from US patent literature (Lowe's extraction). Noisier than USPTO-MIT but much larger.
- **USPTO-50K**: 50K reactions classified into 10 reaction types. Smaller, cleaner, commonly used for comparison.
- **ORD (Open Reaction Database)**: Growing collection of machine-readable reaction data contributed by the chemistry community. Includes conditions, yields, and metadata.
- **Pistachio (NextMove Software)**: ~3M classified reactions from USPTO. Commercial but higher quality than raw USPTO.
- **CReM**: Reaction enumeration tool that can generate synthetic reaction pairs for augmentation.
- **Buchwald-Hartwig reactions**: Curated dataset of ~4,000 Pd-catalyzed C-N coupling reactions with yields (Ahneman et al., 2018).

## Task Format

- **Input**: Reactants and reagents as SMILES, separated by ">" symbols.
```
Predict the major product of this reaction:

Reactants: CC(=O)Cl.c1ccc(N)cc1
Reagents: CCN(CC)CC (triethylamine)
Reaction SMILES: CC(=O)Cl.c1ccc(N)cc1>CCN(CC)CC>?
```
- **Output**: Product SMILES.
```
CC(=O)Nc1ccccc1
(acetanilide: acyl chloride + aniline -> amide)
```

## Difficulty Curriculum

- Level 1: Simple functional group reactions (acid-base, ester hydrolysis, alcohol oxidation)
- Level 2: Named reactions with one clear product (Diels-Alder, Grignard, Wittig)
- Level 3: Reactions with regiochemistry (Markovnikov/anti-Markovnikov addition)
- Level 4: Reactions with stereochemistry (cis/trans, R/S selectivity)
- Level 5: Multi-step reaction cascades (domino/tandem reactions)
- Level 6: Transition-metal catalyzed reactions (Suzuki, Heck, Buchwald-Hartwig)
- Level 7: Reactions with multiple possible products (predict the major product)
- Level 8: Uncommon reaction types not heavily represented in USPTO
- Level 9: Novel substrates with complex functional group compatibility, predicting side products and yields

## Limitations & Risks

- **SMILES validity**: Models frequently generate syntactically invalid SMILES. A validity check is essential before comparison. ~5-15% of predictions may be invalid.
- **Canonical form sensitivity**: "CCO" and "OCC" represent the same molecule. All comparisons must use canonicalized SMILES. RDKit canonicalization is the standard.
- **Major vs. minor products**: Reactions often produce multiple products. USPTO typically records only the major product. The model may predict a valid minor product but receive zero reward.
- **Stereochemistry**: Many reactions create stereocenters. SMILES comparison must handle stereochemistry correctly (@ vs @@). Some benchmarks strip stereochemistry, which loses information.
- **Reagent/condition ambiguity**: The same reactants with different conditions can yield different products. If conditions are underspecified, the prediction is ambiguous.
- **USPTO noise**: Patent reactions are not always reproducible. Some reactions in USPTO have incorrect products, wrong atom mappings, or missing reagents. ~5% error rate in raw data.
- **Molecular size limits**: Very large molecules (polymers, proteins) cannot be represented as simple SMILES and are outside scope.

## Connections

- [[chemistry-retrosynthesis]] — Retrosynthesis is the inverse task; forward prediction can verify retrosynthetic proposals
- [[chemical-equation-balancing]] — Balancing is a prerequisite; forward prediction goes further to predict actual products
- [[protein-ligand-docking]] — Both predict molecular interactions; reaction prediction focuses on covalent bond changes
- [[spectrometry-interpretation]] — Product identification from spectra complements product prediction from reactants
