---
domain: text-classification
category: language
verified: true
verification_type: exact
scalability: very-high
maturity: very-high
tags: [classification, sentiment, topic, nli]
---

# Text Classification

## Overview

Text classification is the task of assigning a predefined label (or labels) to a piece of text. This includes sentiment analysis, topic classification, intent detection, natural language inference, toxicity detection, and more. It is among the simplest and most scalable RLVR domains: the verification function is just label comparison.

Despite its simplicity, text classification is foundational. Many complex NLP capabilities (reasoning, safety, instruction following) can be decomposed into classification subtasks. The clean reward signal and massive data availability make it an excellent starting point for RLVR.

## Verification Mechanism

**Verification type: Exact**

```python
def verify(prediction: str, gold_label: str) -> int:
    return 1 if normalize(prediction) == normalize(gold_label) else 0
```

Verification is a simple string match after normalization. For multi-label classification, verification checks set equality or computes subset accuracy.

The normalization step maps model output to the label space:
- Lowercase and strip whitespace
- Map synonyms to canonical labels (e.g., "positive" / "pos" / "1" all map to "positive")
- For multi-label: parse comma-separated or JSON list output

This is fully deterministic, binary, and trivially fast. No approximation.

## Dataset Sources

| Dataset | Task | Size | Labels | Source |
|---------|------|------|--------|--------|
| SST-2 | Sentiment analysis | 67K | Positive / Negative | Movie reviews |
| SST-5 | Fine-grained sentiment | 11K | 5-way sentiment | Movie reviews |
| AG News | Topic classification | 127K | World / Sports / Business / Sci-Tech | News articles |
| DBpedia | Ontology classification | 630K | 14 topics | DBpedia abstracts |
| 20 Newsgroups | Topic classification | 20K | 20 topics | Usenet posts |
| IMDB | Sentiment analysis | 50K | Positive / Negative | Movie reviews |
| Yelp Reviews | Sentiment analysis | 700K | 1-5 stars | Business reviews |
| Amazon Reviews | Sentiment analysis | 3.6M | 1-5 stars | Product reviews |
| SNLI | Natural language inference | 570K | Entailment / Contradiction / Neutral | Image captions |
| MultiNLI | Natural language inference | 433K | Entailment / Contradiction / Neutral | 10 genres |
| TREC | Question type classification | 6K | 6 coarse / 50 fine labels | TREC questions |
| Jigsaw Toxicity | Toxicity detection | 1.8M | Toxic / Not toxic | Online comments |
| GoEmotions | Emotion detection | 58K | 27 emotions | Reddit comments |
| Banking77 | Intent detection | 13K | 77 intents | Banking queries |

**Scalability**: Text classification data is virtually unlimited. Any labeled corpus works. User reviews (Amazon, Yelp) provide millions of sentiment-labeled examples. News articles come pre-categorized. Wikipedia categories provide topic labels. Web-crawled data with metadata provides further scale.

## Task Format

**Input**: A text (sentence, paragraph, or document) and optionally a label set.

**Output**: A label from the predefined set.

**Prompt template example**:
```
Classify the sentiment of the following movie review as "positive" or "negative".

Review: {text}

Sentiment:
```

For NLI:
```
Given the premise and hypothesis, determine the relationship: "entailment", "contradiction", or "neutral".

Premise: {premise}
Hypothesis: {hypothesis}

Relationship:
```

## Difficulty Curriculum

1. **Level 1 — Binary sentiment, clear cases**: Strongly positive or negative reviews with obvious sentiment words. "This movie was amazing!" vs "Terrible waste of time."
2. **Level 2 — Topic classification**: Assign articles to broad categories (Sports, Politics, Science). Usually clear from content.
3. **Level 3 — Fine-grained sentiment**: 5-way sentiment (SST-5). Distinguishing "somewhat positive" from "very positive" is harder.
4. **Level 4 — Natural language inference**: Determining entailment/contradiction/neutral between premise-hypothesis pairs. Requires logical reasoning.
5. **Level 5 — Intent detection with many classes**: Banking77-style with 77 fine-grained intents. Requires discriminating between similar categories.
6. **Level 6 — Irony and sarcasm detection**: "Oh great, another meeting" — detecting sentiment that contradicts surface-level word choice.
7. **Level 7 — Multi-label classification with long documents**: Assigning multiple relevant tags to a long document. Requires comprehensive reading and multi-label output.

## Limitations & Risks

- **Subjectivity**: Sentiment and emotion labels often have low inter-annotator agreement. What one person calls "neutral," another calls "slightly positive." This introduces inherent noise in the reward signal.
- **Label granularity**: Coarse labels (positive/negative) are reliable but uninformative; fine-grained labels are more useful but noisier.
- **Domain shift**: A model trained on movie review sentiment may not transfer to product reviews or financial text. Domain-specific data is needed.
- **Annotation artifacts**: Models can learn to classify based on spurious correlations (e.g., negation words for SNLI contradiction) rather than genuine understanding.
- **Simplicity ceiling**: For strong LLMs, binary text classification is largely solved. The RLVR reward becomes uninformative when accuracy saturates near 100%.
- **Label space leakage**: If the model has seen the dataset during pretraining, it may memorize labels rather than learn classification ability.

## Connections

- **[fact-verification](fact-verification.md)**: Fact verification is a specialized classification task (SUPPORTS/REFUTES/NEI).
- **[text-classification](text-classification.md)**: NLI (a classification task) underpins many other domains including fact verification and citation verification.
- **[question-answering-closed](question-answering-closed.md)**: Multiple-choice QA is classification over answer options.
- **[spelling-grammar](spelling-grammar.md)**: Error detection (is this sentence grammatical?) is a binary classification task.
- **[multilingual-tasks](multilingual-tasks.md)**: XNLI extends NLI classification to multiple languages.
- **[summarization-extractive](summarization-extractive.md)**: Sentence-level extractive summarization can be framed as binary classification (include/exclude each sentence).
