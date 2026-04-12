---
domain: Protein Design & Engineering
category: science-biology
verification_type: simulation
dataset_scale: 500K+ (PDB, UniProt)
difficulty_range: hard/superhuman
modality: text
status: strong_hypothesis
---

# Protein Design & Engineering

## Overview
Design amino acid sequences that fold into target structures or exhibit desired functions. Verification via structure prediction (AlphaFold/ESMFold) and energy-based scoring functions. This is one of the most impactful scientific applications of RLVR.

## Verification Mechanism
```python
def verify(target_structure: Structure, designed_sequence: str) -> float:
    # Predict structure of designed sequence
    predicted = run_esmfold(designed_sequence)  # or AlphaFold
    
    score = 0.0
    checks = 0
    
    # Structural similarity (TM-score)
    checks += 1
    tm = tm_score(predicted, target_structure)
    score += tm  # 0.0 to 1.0, >0.5 = same fold
    
    # Backbone RMSD
    checks += 1
    rmsd = compute_rmsd(predicted.backbone, target_structure.backbone)
    score += max(0, 1 - rmsd / 5.0)  # 0 Å = perfect, 5 Å = bad
    
    # Confidence (pLDDT from structure prediction)
    checks += 1
    confidence = predicted.mean_plddt / 100.0
    score += confidence
    
    # Rosetta energy score (physics-based)
    checks += 1
    energy = rosetta_score(designed_sequence, target_structure)
    score += sigmoid(-energy / 100)  # lower energy = better
    
    # Sequence recovery (if redesigning known protein)
    if hasattr(target_structure, "native_sequence"):
        checks += 1
        recovery = sequence_identity(designed_sequence, target_structure.native_sequence)
        score += recovery
    
    return score / checks
```

## Dataset Sources
- **PDB (Protein Data Bank)**: 200K+ experimentally determined structures.
- **AlphaFold DB**: 200M+ predicted structures.
- **UniProt**: 250M+ protein sequences with annotations.
- **CATH/SCOP**: Protein structure classification databases.
- **ProteinMPNN training data**: Inverse folding benchmark.
- **RFdiffusion benchmarks**: De novo design benchmarks.

## Task Format
- **Input**: Target backbone structure (PDB format or backbone coordinates) + design constraints (fixed residues, symmetry, binding interface)
- **Output**: Amino acid sequence (one-letter code)

## Difficulty Curriculum
- Level 1: Redesign known protein (inverse folding — sequence recovery)
- Level 3: Design sequence for given fold topology
- Level 5: Design with functional constraints (active site, binding pocket)
- Level 7: De novo design of novel folds
- Level 9: Design functional enzymes, multi-chain complexes
- Level 10: Design proteins with novel functions not seen in nature

## Limitations & Risks
- Structure prediction (AlphaFold/ESMFold) is good but not perfect — designed sequences may not actually fold as predicted experimentally.
- Rosetta energy function is an approximation of physical reality.
- True verification requires wet-lab experiments (not automatable). Computational verification is the best available proxy.
- This is acceptable for RLVR: the computational verification is well-correlated with experimental success, and the field uses these same tools for real drug design.

## Connections
- [[biology-sequence]] — sequence-level tasks
- [[molecular-generation]] — small molecule design
- [[engineering-optimization]] — protein design as optimization
