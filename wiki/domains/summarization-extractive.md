---
domain: summarization-extractive
category: language
verified: true
verification_type: approximate
scalability: very-high
maturity: high
tags: [summarization, rouge, extractive, cnn-dailymail]
---

# Extractive Summarization

## Overview

Extractive summarization selects a subset of sentences from a source document to form a summary. Unlike abstractive summarization (which generates new text), extractive summarization identifies the most important sentences and presents them as the summary.

Verification uses ROUGE (Recall-Oriented Understudy for Gisting Evaluation) to compare the selected sentences against reference summaries. Like BLEU for translation, ROUGE is an automated metric that correlates with human judgment at the corpus level but is imperfect for individual examples.

> **Honesty flag**: This domain uses approximate verification. ROUGE measures n-gram overlap between the system summary and reference summary, but a good summary can use different sentences than the reference while capturing the same key information. The reward signal is noisy.

## Verification Mechanism

**Verification type: APPROXIMATE**

### ROUGE Metrics

- **ROUGE-1**: Unigram overlap between system and reference summary. Measures content coverage.
- **ROUGE-2**: Bigram overlap. Measures fluency and content together.
- **ROUGE-L**: Longest common subsequence. Measures sentence-level structural similarity.

```python
from rouge_score import rouge_scorer

def verify(summary: str, reference: str, threshold: float = 0.4) -> int:
    scorer = rouge_scorer.RougeScorer(['rouge1', 'rouge2', 'rougeL'])
    scores = scorer.score(reference, summary)
    # Use ROUGE-L F1 as primary metric
    return 1 if scores['rougeL'].fmeasure >= threshold else 0
```

**Why ROUGE is approximate**:
- Multiple valid summaries exist for any document; ROUGE only measures overlap with specific references
- A summary can have high ROUGE by copying lead sentences (which often overlap with references in news) without genuine content selection
- ROUGE rewards lexical overlap, not semantic equivalence
- ROUGE-2 and ROUGE-L are more robust than ROUGE-1 but still imperfect

**For RLVR**: ROUGE can serve as a continuous reward signal (0-1 score) or be thresholded for binary reward. Continuous ROUGE is more informative for training. The approximation is a known limitation that should be monitored for reward hacking (e.g., the model learning to produce summaries that maximize ROUGE without being genuinely informative).

### Alternative verification for strict extractive
If the task is restricted to sentence selection (choosing sentence indices), verification can be made more exact:
```python
def verify_sentence_selection(selected_indices: list[int], gold_indices: list[int]) -> int:
    return 1 if set(selected_indices) == set(gold_indices) else 0
```
This is exact but requires sentence-level gold annotations, which are rare.

## Dataset Sources

| Dataset | Size | Source | Reference Type | Notes |
|---------|------|--------|---------------|-------|
| CNN/DailyMail | 312K articles | CNN, DailyMail | Highlight bullets | Most widely used; reference summaries are article highlights |
| XSum | 227K articles | BBC | Single sentence | Extreme summarization; often more abstractive |
| NY Times | 650K articles | NY Times archive | Article summaries | Professionally written summaries |
| Multi-News | 56K clusters | News articles | Multi-document | Summarize multiple articles |
| arXiv/PubMed | 346K papers | Scientific papers | Abstracts | Long document summarization |
| Reddit TIFU | 120K posts | Reddit | TL;DR | Informal summarization |
| BookSum | 12K chapters | Books | Chapter summaries | Very long input documents |
| WikiHow | 230K articles | WikiHow | Article descriptions | How-to domain |

**Scalability**: News articles with associated summaries are produced daily in massive volume. Any article with a summary, abstract, or highlights section provides a training example. Scientific papers have abstracts. Wikipedia articles have lead sections. The data pipeline is effectively unlimited.

## Task Format

**Input**: A document (article, paper, web page).

**Output**: A summary composed of sentences selected from the document.

**Prompt template example**:
```
Select the 3 most important sentences from the following article to form a summary. Output only the selected sentences.

Article:
{article_text}

Summary:
```

For sentence index selection:
```
The following article has numbered sentences. Select the indices of the 3 most important sentences for a summary.

[1] {sentence_1}
[2] {sentence_2}
...

Selected sentence indices:
```

## Difficulty Curriculum

1. **Level 1 — Lead bias exploitation**: Short news articles where the first 2-3 sentences are the summary. Models learn the lead bias.
2. **Level 2 — Key sentence identification**: Articles where important information is distributed throughout. Requires content ranking.
3. **Level 3 — Redundancy elimination**: Multiple sentences convey similar information. The model must select non-redundant sentences.
4. **Level 4 — Long documents**: Scientific papers or legal documents with 5000+ tokens. Requires processing long contexts.
5. **Level 5 — Multi-document summarization**: Summarize a cluster of related articles. Requires cross-document reasoning and deduplication.
6. **Level 6 — Query-focused summarization**: Summarize a document with respect to a specific query or question. Requires relevance filtering.
7. **Level 7 — Coherent summary construction**: Selected sentences must form a coherent narrative, not just a bag of important sentences. Ordering and transitions matter.

## Limitations & Risks

- **ROUGE is the fundamental limitation**: Optimizing for ROUGE can lead to pathological summaries — sentences that share words with the reference but fail to capture the document's key points. This is a well-documented failure mode of RL with ROUGE reward.
- **Lead bias**: In news summarization, the first few sentences often score highest. Models can learn to always select the lead without genuine reasoning. This inflates ROUGE scores without useful capability development.
- **Extractive constraint**: Forcing extractive selection means the summary may lack coherence (sentences ripped from context) or miss information that requires synthesis.
- **Reference quality**: Reference summaries (especially crowdsourced ones) vary in quality. Noisy references mean noisy ROUGE rewards.
- **Length sensitivity**: ROUGE scores depend on summary length. Without length constraints, models may produce overly long summaries to increase recall.

## Connections

- **[translation-reference](translation-reference.md)**: Both domains use approximate automated metrics (ROUGE/BLEU) as verification. Both face the same fundamental limitation of imperfect reward signals.
- **[text-classification](text-classification.md)**: Sentence-level extractive summarization can be framed as binary classification (include/exclude) for each sentence.
- **[reading-comprehension](reading-comprehension.md)**: Summarization requires comprehension of the source document.
- **[citation-verification](citation-verification.md)**: Both involve identifying the most relevant passages in a source document.
- **[question-answering-extractive](question-answering-extractive.md)**: Extractive QA and extractive summarization both select spans from a source text.
- **[document-parsing](document-parsing.md)**: Understanding document structure helps identify summary-worthy content.
