---
domain: Biology Sequence Analysis
category: Science
verification_type: Structural scoring functions (RMSD, TM-score), alignment algorithms, known annotations
dataset_scale: Large (millions of sequences in UniProt/PDB, but labeled/verifiable subset is smaller)
difficulty_range: Sequence lookup to protein structure prediction
modality: Text-in, text-out (sequences, scores, annotations)
status: Mixed — some tasks have strong verification, others are fundamentally approximate
---

## Overview

Biology sequence tasks ask the model to work with protein, DNA, and RNA sequences: predict properties, align sequences, identify functional regions, score structural predictions, and classify organisms or genes. The RLVR opportunity here is that many biological sequence tasks have well-defined computational or database-backed ground truth.

This domain spans sequence alignment (pairwise and multiple), protein secondary/tertiary structure prediction, gene finding, motif identification, phylogenetic classification, and protein function annotation. The unifying thread is that inputs are biological sequences (strings over nucleotide or amino acid alphabets) and outputs are verifiable against known algorithms or curated databases.

## Verification Mechanism

**Primary approach:** Compare model outputs to results from established bioinformatics algorithms or curated database annotations.

- **Sequence alignment verification:** Run the standard algorithm (Needleman-Wunsch for global, Smith-Waterman for local, BLAST for heuristic) and compare. Alignment scores are deterministic given a scoring matrix. Exact match of optimal alignment or score within tolerance.
- **Protein structure scoring:** Given a predicted 3D structure (as coordinates), compute RMSD against known crystal structure, or TM-score for global structural similarity. TM-score > 0.5 generally indicates correct fold.
- **Gene prediction verification:** Compare predicted gene boundaries to annotated gene models from RefSeq/Ensembl. Sensitivity and specificity can be computed per-gene.
- **Motif identification:** Check whether identified motifs match PROSITE patterns or Pfam domain annotations for the input sequence.
- **Phylogenetic classification:** Compare to NCBI taxonomy assignments for known sequences.

**Verification reliability: HIGH for sequence alignment scores.** Given a fixed scoring matrix and gap penalties, optimal alignment scores are exactly computable. The alignment itself may have multiple optima, but the score is unique.

**Verification reliability: HIGH for structure comparison metrics.** RMSD and TM-score are well-defined mathematical functions of coordinate sets. However, the "ground truth" structure must exist (limits coverage).

**Verification reliability: MODERATE for gene prediction and annotation.** Database annotations are curated but not infallible. Some genes are annotated computationally (not experimentally validated), introducing noise in the ground truth.

**Verification reliability: LOW for de novo structure prediction quality.** If we are asking the model to predict a structure from sequence alone, the verification depends on having an experimental structure for comparison — which limits us to the ~200K structures in PDB.

## Dataset Sources

- **UniProt:** ~250M protein sequences with functional annotations. Rich but annotations vary in quality (Swiss-Prot = curated, TrEMBL = automated).
- **PDB (Protein Data Bank):** ~200K experimentally determined 3D structures. Gold standard for structure verification.
- **NCBI RefSeq:** Curated reference sequences for genomes, transcripts, proteins.
- **Pfam / InterPro:** Protein family and domain databases with known motifs and domain architectures.
- **CASP (Critical Assessment of Structure Prediction):** Biennial competition targets with blind structure predictions. Excellent for difficulty calibration.
- **Synthetic problems:** Generate alignment problems by mutating known sequences at controlled rates. Generate gene-finding problems from annotated genomes.

**Realistic scale:** 50K-200K well-verified problems. Limited by the availability of experimentally validated ground truth rather than by sequence data itself.

## Task Format

**Input:** Biological sequence(s) and a specific question.

Example 1 (alignment):
```
What is the optimal global alignment score between these two protein sequences
using BLOSUM62 matrix with gap open penalty -10 and gap extend penalty -1?
Seq1: MVLSPADKTNVKAAWGKVGAHAGEYGAEALERMFLSFPTTKTYFPHFDLSH
Seq2: MVLSGEDKSNIKAAWGKIGGHGAEYGAEALERMFASFPTTKTYFPHFDVSH
```

Example 2 (structure):
```
Given the protein sequence MKFLIALF..., predict the secondary structure
(H=helix, E=sheet, C=coil) for each residue.
```

Example 3 (gene finding):
```
In the following DNA sequence, identify the start and end positions of
protein-coding genes (give coordinates):
ATGCGTACG...
```

**Output:** Numerical score, sequence annotation string, or coordinate list.

## Difficulty Curriculum

1. **Level 1 — Sequence properties:** Compute GC content, amino acid composition, molecular weight of a peptide. Direct computation from sequence.
2. **Level 2 — Pairwise alignment:** Align two short sequences, compute alignment score. Requires understanding scoring matrices and dynamic programming.
3. **Level 3 — Motif and domain identification:** Identify known motifs (start codons, splice sites, signal peptides) in sequences. Requires pattern recognition.
4. **Level 4 — Structure prediction:** Predict secondary structure elements, solvent accessibility, or disorder regions. Requires learned biological knowledge.
5. **Level 5 — Complex prediction:** Protein fold classification, function prediction from sequence, multi-sequence phylogenetic inference. Verification becomes increasingly approximate.

## Limitations & Risks

- **Ground truth quality varies enormously.** Swiss-Prot annotations are highly reliable; TrEMBL and automated gene predictions much less so. The verification is only as good as the reference data.
- **Multiple valid answers:** Sequence alignment often has multiple co-optimal alignments (same score, different traceback). Verification must check scores, not specific alignments.
- **Structure prediction coverage:** Only ~200K experimental structures exist. Many protein families have no experimental structure, making verification impossible for those targets.
- **Sequence memorization risk:** The model may memorize known sequences and their annotations from training data rather than learning generalizable biological reasoning. Held-out sequences from novel organisms help, but this is a real concern.
- **Biological complexity:** Biology is messy. Alternative splicing, post-translational modifications, and context-dependent function mean that "correct" annotations are often context-dependent. RLVR works best when answers are unambiguous.
- **Scoring function choice:** For structure comparison, RMSD vs. TM-score vs. GDT-TS give different rankings. The choice of metric affects what the model optimizes for.

## Connections

- Directly extends to **genomics-bioinformatics.md** (which covers broader bioinformatics tasks)
- **chemistry-computation.md** connects via molecular property prediction for biomolecules
- Protein structure relates to **materials-science.md** (protein materials, biomimetics)
- Sequence alignment algorithms are a special case of dynamic programming, connecting to algorithmic reasoning domains
- CASP competition provides a model for how to evaluate and benchmark these tasks
