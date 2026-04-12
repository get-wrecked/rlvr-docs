---
domain: Protein-Ligand Docking
category: science-biology
verification_type: exact_match
dataset_scale: 23K complexes (PDBbind)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Protein-Ligand Docking

## Overview

Protein-ligand docking predicts how a small molecule (ligand) binds to a protein: both the binding pose (3D orientation within the binding site) and binding affinity (how tightly it binds). This is central to drug discovery -- knowing where and how strongly a drug candidate binds to its target enables rational drug design.

RLVR verification uses structural comparison: the predicted binding pose is compared to the experimentally determined crystal structure via RMSD (Root Mean Square Deviation). For affinity prediction, the predicted binding energy is compared to measured experimental values (Ki, Kd, IC50). Both provide quantitative, deterministic metrics.

## Verification Mechanism

```python
import numpy as np

def compute_rmsd(predicted_coords: np.ndarray, gold_coords: np.ndarray) -> float:
    """
    RMSD between predicted and crystal ligand poses.
    predicted_coords: (N_atoms, 3) array of predicted atom positions
    gold_coords: (N_atoms, 3) array of crystal structure positions
    """
    assert predicted_coords.shape == gold_coords.shape
    diff = predicted_coords - gold_coords
    return np.sqrt(np.mean(np.sum(diff**2, axis=1)))

def verify_docking_pose(predicted_coords, gold_coords, threshold=2.0) -> dict:
    """
    Standard docking success: RMSD < 2.0 Angstroms from crystal pose.
    """
    rmsd = compute_rmsd(predicted_coords, gold_coords)
    success = rmsd < threshold

    # Continuous reward: exponential decay with RMSD
    reward = np.exp(-rmsd / 2.0)

    return {
        "rmsd": rmsd,
        "success_2A": success,
        "reward": reward
    }

def verify_binding_affinity(predicted_pKd: float, gold_pKd: float) -> dict:
    """
    Binding affinity prediction verification.
    pKd = -log10(Kd) where Kd is dissociation constant.
    """
    error = abs(predicted_pKd - gold_pKd)

    return {
        "absolute_error": error,
        "within_1_log": error < 1.0,
        "reward": max(0, 1.0 - error / 3.0)  # linear decay, 0 at 3 log units error
    }
```

## Dataset Sources

- **PDBbind v2020**: 23,496 protein-ligand complexes with experimental binding affinity data (Ki, Kd, or IC50). Includes 3D crystal structures. Refined set (~5,000) has higher-quality data. Core set (~300) is the curated test set.
- **CASF (Comparative Assessment of Scoring Functions)**: 285 complexes from PDBbind core set, specifically designed for benchmarking docking/scoring. Four tasks: scoring, ranking, docking, screening.
- **DUD-E (Directory of Useful Decoys - Enhanced)**: 22,886 active ligands across 102 protein targets, each with 50 property-matched decoys. Standard virtual screening benchmark.
- **DUDE-Z**: Updated DUD-E with better decoy generation.
- **Astex Diverse Set**: 85 high-quality protein-ligand complexes for docking validation.
- **PoseBusters**: Benchmark specifically checking physical validity of docked poses (no clashes, correct bond lengths).
- **CrossDocked2020**: 22.5M docked poses across 13K protein-ligand pairs for training.
- **BindingDB**: 2.7M binding measurements for 1.2M compounds against 9,000 targets. Affinity data without crystal structures.

## Task Format

- **Input**: Protein structure (PDB format or pocket residues) and ligand (SMILES or 3D coordinates).
```
Predict the binding pose and affinity of this ligand to the protein target.

Protein: [4LDE.pdb] (binding pocket residues: ASP189, SER195, HIS57, ...)
Ligand SMILES: CC(=O)Nc1ccc(O)cc1
```
- **Output**: 3D coordinates of ligand atoms in the binding site, plus predicted affinity.
```
Predicted pose: [(x1,y1,z1), (x2,y2,z2), ...] (Angstroms)
Predicted pKd: 6.8 (nanomolar binder)
RMSD from crystal: 1.3 Angstroms (successful dock)
```

## Difficulty Curriculum

- Level 1: Re-docking: given the crystal protein-ligand complex, re-predict the known pose (self-docking)
- Level 2: Cross-docking: dock a ligand into a protein structure solved with a different ligand
- Level 3: Rigid protein, flexible ligand docking (standard docking)
- Level 4: Binding affinity prediction for known binders (PDBbind refined set)
- Level 5: Virtual screening: rank 50 compounds, distinguish actives from decoys (DUD-E)
- Level 6: Flexible protein side-chain docking (induced fit)
- Level 7: Blind docking: predict binding site location without prior knowledge
- Level 8: Affinity prediction for novel scaffolds (scaffold-split evaluation)
- Level 9: Protein-protein docking, allosteric binding sites, covalent docking

## Limitations & Risks

- **3D coordinate representation**: Docking outputs are 3D coordinates, which are cumbersome to represent in text. Text-based LLMs may need to work with SMILES-based interaction descriptions or pocket residue contacts rather than full 3D poses.
- **RMSD is imperfect**: Two poses with RMSD > 2A can be functionally equivalent (e.g., symmetric ligands, flexible tails). Symmetry-corrected RMSD helps but does not cover all cases.
- **Scoring function limitations**: Predicting binding affinity with <1 log unit accuracy remains unsolved. Physics-based scoring functions (MMGBSA, etc.) and ML-based ones both have systematic errors.
- **Crystal structure artifacts**: Crystal structures represent a single conformation that may not reflect the physiological binding mode. Crystal packing effects can distort poses.
- **Data leakage**: Many protein-ligand complexes in PDBbind may be in LLM pretraining data. Strict temporal or structural splits are needed to evaluate genuine prediction ability.
- **Computational cost**: Traditional docking software (AutoDock, Glide) takes minutes per compound. ML approaches are faster but less physically grounded.
- **Protein flexibility**: Real proteins move. Docking into a single rigid crystal structure ignores conformational changes upon ligand binding (induced fit).

## Connections

- [[biology-sequence]] — Protein sequence determines structure which determines binding
- [[forward-reaction-prediction]] — Both predict molecular interactions; reaction prediction focuses on covalent changes, docking on non-covalent
- [[chemistry-retrosynthesis]] — Drug design pipeline: docking identifies targets, retrosynthesis plans how to make them
- [[spectrometry-interpretation]] — Experimental validation of binding uses spectroscopic techniques
