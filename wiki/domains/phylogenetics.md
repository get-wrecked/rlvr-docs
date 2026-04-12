---
domain: Phylogenetics & Evolutionary Biology
category: science-biology
verification_type: execution
dataset_scale: 100K+ (from genetic databases)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Phylogenetics & Evolutionary Biology

## Overview
Reconstruct evolutionary trees from sequence data, compute evolutionary distances, identify conserved regions, perform ancestral sequence reconstruction. Verification: compare trees using Robinson-Foulds distance, verify distance computations.

## Verification Mechanism
```python
from Bio import Phylo
from ete3 import Tree

def verify(task_type: str, sequences: dict, answer: Any) -> float:
    if task_type == "tree_reconstruction":
        # Compare predicted tree to reference using Robinson-Foulds
        pred_tree = Tree(answer["newick"])
        ref_tree = Tree(ground_truth["newick"])
        rf, max_rf, _, _, _, _, _ = pred_tree.robinson_foulds(ref_tree)
        normalized_rf = rf / max_rf if max_rf > 0 else 0
        return max(0, 1 - normalized_rf)
    
    elif task_type == "distance_matrix":
        # Verify evolutionary distance computation
        pred_matrix = np.array(answer["distances"])
        ref_matrix = compute_distance_matrix(sequences, model="JC69")
        rel_error = np.mean(np.abs(pred_matrix - ref_matrix) / (ref_matrix + 1e-10))
        return max(0, 1 - rel_error)
    
    elif task_type == "alignment":
        # Verify sequence alignment score
        from Bio import pairwise2
        ref_score = pairwise2.align.globalxx(sequences["seq1"], sequences["seq2"], score_only=True)
        return 1.0 if abs(answer["score"] - ref_score) < 1 else 0.0
```

## Dataset Sources
- **TreeBASE**: 13K+ phylogenetic trees.
- **NCBI Taxonomy**: Complete taxonomic classification.
- **Pfam**: 19K+ protein families with alignments.
- **OrthoMCL**: Ortholog groups.
- **BAliBASE**: Alignment benchmark.
- **SimPhy/INDELible**: Simulated phylogenetic datasets with known true trees.

## Task Format
- **Input**: Set of DNA/protein sequences + "Reconstruct the phylogenetic tree using neighbor-joining"
- **Output**: Newick-format tree string

## Difficulty Curriculum
- Level 1: Simple distance-based tree (UPGMA, 5 sequences)
- Level 3: Neighbor-joining, larger trees (20 sequences)
- Level 5: Maximum likelihood tree estimation
- Level 7: Bayesian phylogenetics, molecular clock estimation
- Level 9: Phylogenomics, incomplete lineage sorting, horizontal gene transfer

## Connections
- [[biology-sequence]] — sequence analysis
- [[graph-theory]] — tree graph operations
- [[probability-statistics]] — statistical phylogenetics
