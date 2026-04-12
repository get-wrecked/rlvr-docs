---
domain: DNA/RNA Sequence Design
category: synthetic-biology
verification_type: execution
dataset_scale: 1M+ (from sequence databases)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# DNA/RNA Sequence Design

## Overview
Design DNA or RNA sequences that satisfy biophysical constraints: melting temperature, GC content, secondary structure avoidance, codon optimization, restriction site avoidance, minimal off-target binding. Verification via computational biology tools.

## Verification Mechanism
```python
from Bio.SeqUtils import MeltingTemp, GC
from Bio.Seq import Seq
import nupack

def verify(constraints: dict, sequence: str) -> float:
    seq = Seq(sequence)
    score = 0.0
    checks = 0
    
    # Validity (only ACGT/ACGU)
    checks += 1
    valid_chars = set("ACGT") if constraints.get("type") == "dna" else set("ACGU")
    if set(sequence.upper()).issubset(valid_chars):
        score += 1
    
    # Length constraint
    if "length" in constraints:
        checks += 1
        if len(sequence) == constraints["length"]:
            score += 1
    
    # GC content
    if "gc_range" in constraints:
        checks += 1
        gc = GC(sequence) / 100
        if constraints["gc_range"][0] <= gc <= constraints["gc_range"][1]:
            score += 1
    
    # Melting temperature
    if "tm_range" in constraints:
        checks += 1
        tm = MeltingTemp.Tm_NN(seq)
        if constraints["tm_range"][0] <= tm <= constraints["tm_range"][1]:
            score += 1
    
    # Secondary structure avoidance (for primers)
    if "max_hairpin_tm" in constraints:
        checks += 1
        hairpin_tm = compute_hairpin_tm(sequence)
        if hairpin_tm <= constraints["max_hairpin_tm"]:
            score += 1
    
    # Restriction site avoidance
    if "forbidden_sites" in constraints:
        for site in constraints["forbidden_sites"]:
            checks += 1
            if site not in sequence and reverse_complement(site) not in sequence:
                score += 1
    
    # Codon optimization (for protein-coding sequences)
    if "target_organism" in constraints and "protein" in constraints:
        checks += 1
        translated = seq.translate()
        if str(translated).rstrip("*") == constraints["protein"]:
            score += 0.5  # Correct protein
            cai = compute_cai(sequence, constraints["target_organism"])
            score += 0.5 * cai  # Codon adaptation index
    
    return score / checks if checks > 0 else 0.0
```

## Dataset Sources
- **NCBI GenBank**: 250M+ sequences.
- **AddGene**: 100K+ plasmid sequences.
- **iGEM Parts Registry**: Standardized biological parts.
- **Eterna (RNA design game)**: Crowdsourced RNA design solutions.
- **Codon usage databases**: Organism-specific codon tables.
- **Primer design datasets**: PCR primer databases.

## Task Format
- **Input**: "Design a 20-mer DNA primer with Tm between 58-62°C, GC content 40-60%, no self-complementary regions, no EcoRI site"
- **Output**: "ATCGATCGAAGTCCTACCGA"

## Difficulty Curriculum
- Level 1: Design primer with basic Tm/GC constraints
- Level 3: Design primer pair for specific PCR product
- Level 5: Codon-optimize a gene for E. coli expression
- Level 7: Design guide RNA for CRISPR with minimal off-targets
- Level 9: Design synthetic gene circuits with specific regulatory properties

## Limitations & Risks
- Computational predictions (Tm, secondary structure) are approximations. Well-established tools (NUPACK, Primer3) give good estimates.
- Off-target analysis requires whole-genome search — computationally expensive.

## Connections
- [[biology-sequence]] — sequence analysis
- [[protein-design]] — protein-level design
- [[molecular-generation]] — molecular design
- [[combinatorics-optimization]] — sequence design as constrained optimization
