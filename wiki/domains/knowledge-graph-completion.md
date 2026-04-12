---
domain: knowledge-graph-completion
category: knowledge
verified: true
verification_type: exact
scalability: high
maturity: high
tags: [knowledge-graph, link-prediction, triples, wikidata, freebase]
---

# Knowledge Graph Completion

## Overview

Knowledge graph completion (KGC) tasks require the model to predict missing links (triples) in a knowledge graph. Given a head entity and relation, predict the tail entity — or given a tail entity and relation, predict the head entity. The model must reason over the structure of the knowledge graph and its factual content to infer plausible missing facts.

Verification is exact: the predicted entity is compared against held-out ground truth triples. Knowledge graphs provide massive, clean, structured data, making this domain highly scalable.

## Verification Mechanism

**Verification type: Exact**

The verification function checks whether the model's predicted entity matches a known correct entity from the held-out test set.

```python
def verify(prediction: str, gold_entities: set[str]) -> int:
    return 1 if normalize(prediction) in {normalize(g) for g in gold_entities} else 0
```

**Standard KGC evaluation details**:
- For each test triple `(h, r, t)`, the model is asked to predict `t` given `(h, r, ?)` and `h` given `(?, r, t)`
- The predicted entity must exactly match a held-out ground truth entity
- Multiple correct answers may exist (e.g., "born_in" may have multiple valid locations for disambiguation)
- Standard metrics: Hits@1, Hits@3, Hits@10, Mean Reciprocal Rank (MRR)
- For RLVR binary reward: Hits@1 (exact match on top prediction)

**Filtering protocol**: During evaluation, triples that appear in the training set are filtered out so the model is not penalized for predicting known-correct triples that happen to not be the test answer.

## Dataset Sources

| Dataset | Entities | Relations | Triples | Source | Notes |
|---------|----------|-----------|---------|--------|-------|
| FB15k-237 | 14.5K | 237 | 310K | Freebase | Filtered version of FB15k (no inverse leakage) |
| WN18RR | 40.9K | 11 | 93K | WordNet | Filtered version of WN18 |
| Wikidata5M | 4.6M | 822 | 21M | Wikidata | Large-scale, transductive and inductive splits |
| YAGO3-10 | 123K | 37 | 1.1M | YAGO | Facts with at least 10 relations |
| CoDEx | 77K | 69 | 206K | Wikidata + Wikipedia | Hard negatives included |
| NELL-995 | 75K | 200 | 154K | NELL | Web-extracted knowledge |
| ConceptNet | 2.1M | 36 | 7.4M | ConceptNet | Commonsense knowledge |
| ATOMIC | 304K | 9 | 877K | Crowdsourced | Commonsense about events/causes/effects |

**Scalability**: Wikidata contains over 100 million entities and billions of statements. Any held-out subset becomes test data. The challenge is not data volume but constructing meaningful evaluation splits.

## Task Format

**Input**: A partial triple with one element missing.

**Output**: The missing entity (or relation).

**Prompt template example**:
```
Complete the following knowledge graph triple.

Head entity: Marie Curie
Relation: field of work
Tail entity: ?

Answer:
```

Alternative (text-based reasoning):
```
Given what you know, what is the missing entity in this relationship?

(Albert Einstein, won_award, ?)

Answer the entity name exactly:
```

## Difficulty Curriculum

1. **Level 1 — Common entities, frequent relations**: Predict well-known facts like (Paris, capital_of, France). High coverage in training data.
2. **Level 2 — One-hop inference**: The answer can be inferred from a single other triple in the graph.
3. **Level 3 — Multi-hop inference**: Requires chaining 2-3 triples to reach the answer (e.g., "What country is the birthplace of the inventor of X in?").
4. **Level 4 — Rare entities**: Predict facts about entities with few known triples. Requires generalization from entity types and relation patterns.
5. **Level 5 — Commonsense KGC**: Complete commonsense knowledge triples (ATOMIC, ConceptNet). Less factual, more inferential.
6. **Level 6 — Inductive KGC**: Predict triples for entities not seen during training (Wikidata5M inductive split). Requires reasoning from entity descriptions.
7. **Level 7 — Temporal KGC**: Predict facts that are true during specific time periods. Requires temporal reasoning over the validity of triples.

## Limitations & Risks

- **Incomplete ground truth**: Knowledge graphs are inherently incomplete. A predicted triple may be correct but absent from the ground truth, resulting in a false negative reward. This is a significant issue — the "closed world assumption" penalizes genuine knowledge.
- **Entity ambiguity**: Entity names can be ambiguous ("Paris" = city in France or city in Texas). KGC datasets use entity IDs to disambiguate, but text-based evaluation with LLMs must handle this.
- **Leakage and inverse relations**: Older datasets (FB15k, WN18) contained inverse relation pairs that allowed trivial memorization. Use filtered versions (FB15k-237, WN18RR).
- **Static evaluation**: KGC benchmarks represent a snapshot. The real world's knowledge changes; evaluation does not.
- **Text vs. structure**: Traditional KGC models operate on graph structure (embeddings). LLM-based KGC operates on text. The two modalities test different capabilities.

## Connections

- **[fact-verification](fact-verification.md)**: KG triples serve as evidence for fact verification. Verified claims can populate KGs.
- **[information-extraction](information-extraction.md)**: IE populates knowledge graphs. KGC fills gaps left by IE.
- **[question-answering-closed](question-answering-closed.md)**: KG-based QA (WebQuestions, SimpleQuestions) is closely related to KGC.
- **[entity-linking](entity-linking.md)**: Entity linking maps text mentions to KG entities, a prerequisite for KG-based reasoning.
- **[temporal-reasoning](temporal-reasoning.md)**: Temporal KGC requires reasoning about when facts are valid.
- **[semantic-parsing](semantic-parsing.md)**: KBQA (knowledge base question answering) combines semantic parsing with KGC.
