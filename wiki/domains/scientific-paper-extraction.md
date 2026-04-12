---
domain: Scientific Paper Information Extraction
category: science-nlp
verification_type: exact_match
dataset_scale: 10M+ (from Semantic Scholar, PubMed)
difficulty_range: easy/medium/hard
modality: multimodal
status: verified
---

# Scientific Paper Information Extraction

## Overview
Extract structured information from scientific papers: method names, datasets used, results (precision/recall/F1), experimental conditions, chemical compounds, gene names, relationships between entities. Verification: compare extracted data against human-annotated gold standards or structured databases.

## Verification Mechanism
```python
def verify(paper_text: str, extraction_task: str, prediction: dict, gold: dict) -> float:
    if extraction_task == "results_table":
        # Compare extracted results to known values
        score = 0
        total = 0
        for metric, value in gold["metrics"].items():
            total += 1
            if metric in prediction["metrics"]:
                pred_val = prediction["metrics"][metric]
                if abs(pred_val - value) < 0.001:
                    score += 1
        return score / total if total > 0 else 0.0
    
    elif extraction_task == "entities":
        # NER-style evaluation
        pred_entities = set(prediction["entities"])
        gold_entities = set(gold["entities"])
        precision = len(pred_entities & gold_entities) / max(len(pred_entities), 1)
        recall = len(pred_entities & gold_entities) / max(len(gold_entities), 1)
        f1 = 2 * precision * recall / max(precision + recall, 1e-10)
        return f1
    
    elif extraction_task == "relations":
        # Relation extraction evaluation
        pred_rels = set(tuple(r) for r in prediction["relations"])
        gold_rels = set(tuple(r) for r in gold["relations"])
        return set_f1(pred_rels, gold_rels)
```

## Dataset Sources
- **SciERC**: 500 scientific abstracts with entity and relation annotations.
- **SciREX**: Document-level relation extraction from ML papers.
- **ChemProt**: Chemical-protein interaction extraction.
- **GENIA**: Biomedical NER corpus (36K sentences).
- **PubMedQA**: Question answering from PubMed abstracts.
- **S2ORC (Semantic Scholar)**: 81M papers with structured metadata.
- **Papers With Code**: Structured results data linked to papers.
- **ACL Anthology**: Computational linguistics papers with metadata.
- **BioASQ**: Biomedical QA and extraction benchmark.

## Task Format
- **Input**: Paper text (or PDF) + "Extract all dataset names and their associated performance metrics from this paper"
- **Output**: Structured table: [{dataset: "MNIST", metric: "accuracy", value: 99.2}, ...]

## Difficulty Curriculum
- Level 1: Extract title, authors, abstract from paper
- Level 3: Extract entities (methods, datasets, metrics) from text
- Level 5: Extract relations and results tables
- Level 7: Cross-paper comparison (extract same info from multiple papers)
- Level 9: Synthesis across corpus (meta-analysis style extraction)

## Limitations & Risks
- Gold annotations may be incomplete — papers often report results in inconsistent ways.
- PDF parsing adds noise. Use text versions (LaTeX source, HTML) when available.
- Entity normalization is important (BERT/BERT-base/bert → same model).

## Connections
- [[information-extraction]] — general IE
- [[document-parsing]] — document structure extraction
- [[table-understanding]] — table extraction from papers
- [[fact-verification]] — verifying scientific claims
