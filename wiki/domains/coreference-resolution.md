---
domain: Coreference Resolution
category: nlp
verification_type: exact_match
dataset_scale: 1.7M+ words (OntoNotes)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Coreference Resolution

## Overview

Coreference resolution links mentions in text that refer to the same real-world entity: pronouns to their antecedents, nominal mentions to named entities, and event mentions to each other. The task is central to text understanding and information extraction. Verification is well-established through cluster-based metrics (B3, MUC, CEAF) that compare predicted coreference clusters against gold-annotated clusters.

The RLVR value is strong: coreference is a structured prediction task where the gold annotation provides unambiguous ground truth for most cases. Edge cases exist (near-identity, generic mentions), but the core task is highly verifiable.

## Verification Mechanism

```python
from collections import Counter

def b3_precision_recall(predicted_clusters, gold_clusters):
    """B-cubed metric: per-mention precision/recall averaged over all mentions."""
    mention_to_pred = {}
    for i, cluster in enumerate(predicted_clusters):
        for mention in cluster:
            mention_to_pred[mention] = i
    mention_to_gold = {}
    for i, cluster in enumerate(gold_clusters):
        for mention in cluster:
            mention_to_gold[mention] = i

    precisions, recalls = [], []
    all_mentions = set(mention_to_pred) | set(mention_to_gold)

    for m in all_mentions:
        if m in mention_to_pred and m in mention_to_gold:
            pred_cluster = set(predicted_clusters[mention_to_pred[m]])
            gold_cluster = set(gold_clusters[mention_to_gold[m]])
            overlap = pred_cluster & gold_cluster
            precisions.append(len(overlap) / len(pred_cluster))
            recalls.append(len(overlap) / len(gold_cluster))
        else:
            precisions.append(0.0)
            recalls.append(0.0)

    p = sum(precisions) / len(precisions)
    r = sum(recalls) / len(recalls)
    f1 = 2 * p * r / (p + r) if (p + r) > 0 else 0.0
    return f1

def verify_coreference(predicted_clusters, gold_clusters) -> float:
    """CoNLL average: mean of MUC, B3, CEAFe F1 scores."""
    b3 = b3_precision_recall(predicted_clusters, gold_clusters)
    muc = muc_f1(predicted_clusters, gold_clusters)       # link-based
    ceafe = ceafe_f1(predicted_clusters, gold_clusters)    # entity-based alignment
    return (b3 + muc + ceafe) / 3.0
```

## Dataset Sources

- **OntoNotes 5.0**: 1.7M words across newswire, broadcast, web, telephone conversations. Gold coreference annotations. The standard benchmark (CoNLL-2012 shared task split).
- **CoNLL-2012 Shared Task**: Train/dev/test splits of OntoNotes for coreference. ~2,800 documents, ~1.6M tokens.
- **GAP (Google)**: 8,908 pronoun resolution examples specifically targeting gender-balanced evaluation. Ambiguous pronoun pairs in Wikipedia text.
- **Winograd Schema Challenge (WSC)**: 273 carefully crafted pronoun resolution problems requiring commonsense reasoning. Small but diagnostically valuable.
- **WinoBias**: 3,160 examples probing gender bias in coreference systems.
- **PreCo**: 38K documents with coreference annotations, larger but lower annotation quality than OntoNotes.
- **LitBank**: Coreference in literary texts (100 novels), useful for domain transfer evaluation.

## Task Format

- **Input**: Text passage + instruction to identify coreferent mentions.
```
In the following text, identify all coreference clusters (groups of mentions
referring to the same entity):

"John Smith went to the store. He bought milk because his wife asked him to.
She had called earlier that morning."
```
- **Output**: Coreference clusters as lists of mention spans.
```
Cluster 1: ["John Smith", "He", "his", "him"]
Cluster 2: ["his wife", "She"]
Cluster 3: ["the store"]
Cluster 4: ["milk"]
```

## Difficulty Curriculum

- Level 1: Simple pronominal anaphora with one entity ("The dog ate. It slept.")
- Level 2: Two entities with unambiguous pronouns differentiated by gender/number
- Level 3: Multiple entities, nominal coreference ("the president" = "Obama")
- Level 4: Complex sentences with nested clauses, cataphora (forward reference)
- Level 5: Ambiguous pronouns requiring world knowledge (Winograd-style)
- Level 6: Event coreference ("the attack" = "the bombing"), abstract entities
- Level 7: Cross-sentence coreference across long passages (10+ sentences)
- Level 8: Conversational coreference with disfluencies, speaker switches
- Level 9: Literary/figurative coreference, metonymy ("Washington" = US government)

## Limitations & Risks

- **Annotation disagreement**: Even expert annotators disagree on ~5-10% of coreference decisions, especially for near-identity, generic/specific, and bridging cases. The gold standard is imperfect.
- **Singleton mentions**: OntoNotes does not annotate singleton mentions (entities mentioned only once), which means the system is not penalized for missing them but also cannot learn to identify them.
- **Genre sensitivity**: Models trained on newswire perform poorly on dialogue and literary text. The difficulty curriculum must sample across genres.
- **Gender bias**: Systems historically linked pronouns to gender-stereotypical roles. WinoBias and GAP specifically test for this. RLVR training could reinforce or mitigate bias depending on training distribution.
- **Long documents**: Most benchmarks use relatively short documents. Coreference across very long documents (books, legal briefs) is understudied and harder to verify.

## Connections

- [[semantic-role-labeling]] — SRL identifies who-did-what; coreference links those "who" mentions across sentences
- [[dialogue-state-tracking]] — Coreference in dialogue is critical for tracking entity references across turns
- [[named-entity-recognition]] — NER identifies entity mentions that coreference then clusters
- [[reading-comprehension]] — Many reading comprehension questions implicitly require coreference resolution
