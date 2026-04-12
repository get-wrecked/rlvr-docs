---
domain: Genomics and Bioinformatics
category: Science
verification_type: Known ground truth from validated databases, algorithmic comparison, curated annotations
dataset_scale: Large (millions of sequences; verified subset varies by task)
difficulty_range: Sequence statistics to variant calling and phylogenetic inference
modality: Text-in, text-out (sequences, alignments, variant calls, trees)
status: Mixed — strong for algorithmic tasks, weaker for biological interpretation
---

## Overview

Genomics and bioinformatics tasks extend beyond protein/DNA sequence analysis (covered in **biology-sequence.md**) to the full computational biology pipeline: variant calling (identifying mutations), genome assembly fragments, phylogenetic tree construction, gene expression analysis, pathway enrichment, and population genetics. The RLVR opportunity is that many of these tasks have well-defined computational procedures with deterministic outputs (given inputs and parameters), and extensive curated databases provide ground truth.

This domain covers variant calling (SNPs, indels, structural variants from sequencing data), multiple sequence alignment, phylogenetic tree construction, gene expression analysis (differential expression), genome annotation, and population genetics calculations (Hardy-Weinberg, allele frequencies, F-statistics).

## Verification Mechanism

**Primary approach:** Compare model outputs to established algorithm results or curated database annotations.

- **Variant calling verification:** Compare model's variant calls against gold-standard datasets (Genome in a Bottle, Platinum Genomes). Measure precision, recall, F1 for SNPs and indels.
- **Multiple sequence alignment verification:** Score alignment using sum-of-pairs or column score against a reference alignment (e.g., BAliBASE, PREFAB benchmarks). Deterministic scoring given reference.
- **Phylogenetic tree verification:** Compare tree topology to reference tree using Robinson-Foulds distance, or compare likelihood scores. Topology comparison is exact; likelihood comparison requires computing the model.
- **Gene expression verification:** For differential expression tasks, compare identified genes to known DE gene lists from validated studies. Statistical significance can be recomputed.
- **Population genetics verification:** Hardy-Weinberg equilibrium tests, allele frequency computations, and F-statistics are deterministic calculations from allele count data.
- **Genome annotation verification:** Compare predicted gene models, promoters, and regulatory elements to curated annotations in RefSeq, Ensembl, or GENCODE.

**Verification reliability: HIGH for population genetics computations.** Allele frequencies, heterozygosity, HWE tests are straightforward arithmetic from genotype data.

**Verification reliability: HIGH for variant calling against gold standards.** Genome in a Bottle provides highly curated truth sets for specific samples. Comparison is well-defined.

**Verification reliability: MODERATE for multiple sequence alignment.** There is no unique "correct" alignment for divergent sequences. Reference alignments are based on structural superposition where available, or expert curation, and different references may disagree.

**Verification reliability: MODERATE for phylogenetics.** The "true" tree is rarely known. Reference trees come from well-supported published analyses, but tree inference is an active research area with legitimate disagreements.

**Verification reliability: LOW for functional interpretation.** Assigning biological function to genes, interpreting pathway enrichment, or predicting disease relevance involves judgment that cannot be fully automated.

## Dataset Sources

- **Genome in a Bottle (GIAB):** Gold-standard variant calls for 7 human genomes. Highest quality truth set for variant calling.
- **Platinum Genomes (Illumina):** High-confidence variant calls for a pedigree. Complements GIAB.
- **BAliBASE, PREFAB, SABmark:** Reference alignment benchmarks for multiple sequence alignment.
- **TreeBASE:** Repository of published phylogenetic trees. ~13K studies.
- **NCBI/Ensembl/GENCODE:** Genome annotations for hundreds of species.
- **1000 Genomes Project:** Population-scale variant data for population genetics tasks.
- **GTEx:** Gene expression data across human tissues. Ground truth for expression-level tasks.
- **ClinVar:** Curated database of clinically significant variants. Ground truth for variant interpretation.

**Realistic scale:** 20K-100K problems depending on task type. Variant calling: limited by number of gold-standard samples. Population genetics: unlimited synthetic problems from allele frequency data. Alignment: ~5K reference alignment problems from benchmark databases.

## Task Format

**Input:** Genomic data and a specific bioinformatics question.

Example 1 (variant calling):
```
Given the following read alignments at chr1:1000000-1000100
[SAM format reads], identify any SNPs or indels. Report position,
reference allele, and alternate allele.
```

Example 2 (population genetics):
```
In a population of 500 individuals, the genotype counts for a SNP are:
AA = 200, AG = 220, GG = 80
Is this population in Hardy-Weinberg equilibrium? (chi-squared test,
alpha = 0.05). Report the chi-squared statistic and p-value.
```

Example 3 (phylogenetics):
```
Given the following aligned sequences for 5 species, construct a
neighbor-joining tree. Report the tree in Newick format.
[Multiple sequence alignment]
```

**Output:** Variant calls (VCF-like), statistical test results, phylogenetic trees (Newick), or gene lists.

## Difficulty Curriculum

1. **Level 1 — Sequence statistics:** GC content, codon usage, restriction site identification. Direct computation from sequence.
2. **Level 2 — Population genetics calculations:** Allele frequencies, HWE tests, basic F-statistics. Statistical computation from genotype data.
3. **Level 3 — Alignment and variant identification:** Pairwise/multiple alignment, simple variant calling from aligned reads. Algorithmic tasks.
4. **Level 4 — Phylogenetic inference:** Build trees from alignment, assess bootstrap support, interpret divergence times. Requires method selection and parameter choices.
5. **Level 5 — Integrative analysis:** Combine variant data with expression data, perform pathway analysis, interpret clinical significance. Verification is weakest here.

## Limitations & Risks

- **Gold-standard limitations:** Even GIAB has regions of the genome that are excluded from the truth set (repetitive regions, structural variants). Verification is incomplete in these regions.
- **Reference genome bias:** Most ground truth is defined relative to a reference genome (GRCh38). Variants in populations distant from the reference may be harder to call and verify.
- **Alignment reference ambiguity:** Multiple sequence alignment has no unique correct answer for divergent sequences. Reference alignments are approximations, introducing noise in the verification signal.
- **Phylogenetic uncertainty:** Tree topology is not uniquely determined by sequence data, especially with short alignments or rapid radiations. Multiple trees may be statistically indistinguishable.
- **Clinical interpretation danger:** If this domain extends to clinical variant interpretation, errors have real consequences. Verification for clinical significance is inherently incomplete — not all variant effects are known.
- **Data format complexity:** Bioinformatics involves complex file formats (SAM/BAM, VCF, GFF, FASTA). Parsing model outputs in these formats requires robust format handling.
- **Computational cost for variant calling:** Full variant calling pipelines are computationally intensive. For RLVR, pre-computed truth sets are essential.

## Connections

- **biology-sequence.md** covers the foundational sequence analysis skills this domain builds on
- **chemistry-computation.md** connects via molecular biology (DNA/protein chemistry)
- **climate-weather.md** shares the challenge of working with large, noisy observational datasets
- **signal-processing.md** connects via genomic signal processing (spectral methods for sequence analysis)
- Population genetics calculations are fundamentally statistical, connecting to probability and statistics domains
