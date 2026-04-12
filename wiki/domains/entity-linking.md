---
domain: entity-linking
category: knowledge
verified: true
verification_type: exact
scalability: high
maturity: high
tags: [entity-disambiguation, knowledge-base, wikidata, nel]
---

# Entity Linking

## Overview

Entity linking (also called named entity disambiguation or wikification) maps mentions of entities in text to their corresponding entries in a knowledge base (typically Wikipedia or Wikidata). Given a text with identified entity mentions, the model must resolve each mention to the correct knowledge base entity, handling ambiguity (e.g., "Paris" could be the city in France, a city in Texas, or the mythological figure).

Verification is exact: compare the predicted entity ID against the gold entity ID. This domain has large-scale datasets derived from Wikipedia hyperlinks and is a foundational component of knowledge-intensive NLP.

## Verification Mechanism

**Verification type: Exact**

The verification function compares predicted entity IDs (or Wikipedia/Wikidata identifiers) against gold annotations:

```python
def verify(predicted_entity_id: str, gold_entity_id: str) -> int:
    return 1 if predicted_entity_id == gold_entity_id else 0
```

For end-to-end evaluation (mention detection + linking):
- **Mention detection**: Compare predicted mention spans against gold mention spans (exact span match)
- **Entity disambiguation**: For correctly detected mentions, compare predicted entity IDs against gold entity IDs
- **Combined score**: A mention is correctly linked only if both the span and entity ID are correct

For NIL entities (mentions that refer to entities not in the knowledge base):
- The model should predict NIL or "not in KB"
- Verification checks: `predicted == NIL and gold == NIL`

Evaluation metrics: micro/macro accuracy over mentions, precision/recall/F1 for end-to-end.

## Dataset Sources

| Dataset | Size | Knowledge Base | Domain | Notes |
|---------|------|---------------|--------|-------|
| AIDA-CoNLL | 1.4K docs, 34K mentions | Wikipedia | News (Reuters) | Standard benchmark, manually annotated |
| TAC-KBP | Varies by year | TAC KB | News | Annual evaluation campaigns (2009-2017) |
| WNED-Wiki | 40 docs, 6.8K mentions | Wikipedia | Wikipedia | Wiki-derived evaluation |
| WNED-Cweb | 320 docs, 11.2K mentions | Wikipedia | Web | ClueWeb-derived evaluation |
| MSNBC | 20 docs, 656 mentions | Wikipedia | News | Small but high quality |
| ACE2004 | 36 docs, 257 mentions | Wikipedia | News | Small benchmark |
| WikilinksNED | 3.2M mentions | Wikipedia | Web | Automatically generated from web links |
| Wikipedia hyperlinks | Millions | Wikipedia | Wikipedia | Silver standard from anchor text |
| Zeshel | 49K mentions | Specialized wikis | Domain-specific | Zero-shot entity linking |
| TACKBP-2010 | 2.2K queries | TAC KB | News | Entity search queries |

**Scalability**: Wikipedia hyperlinks provide millions of entity linking examples for free. Every hyperlink in Wikipedia is an anchor text (mention) linked to a target article (entity). This silver data covers diverse entity types and contexts. Wikidata provides additional structured knowledge for disambiguation.

## Task Format

**Input**: A text passage with identified entity mentions (or the model must also detect mentions).

**Output**: For each mention, the knowledge base entity ID (Wikipedia title or Wikidata QID).

**Prompt template example**:
```
Link each entity mention in brackets to its Wikipedia article title.

Text: [Paris] is the capital of [France]. [Napoleon] was exiled to [Elba].

Entity links:
- Paris -> ?
- France -> ?
- Napoleon -> ?
- Elba -> ?
```

For disambiguation:
```
The text mentions "Mercury." Based on the context below, which Wikipedia entity does "Mercury" refer to?

Context: {passage_mentioning_mercury}

Options:
(A) Mercury (planet)
(B) Mercury (element)
(C) Mercury (mythology)
(D) Freddie Mercury

Answer:
```

## Difficulty Curriculum

1. **Level 1 — Unambiguous entities**: "Albert Einstein" has essentially one referent. The model just needs to recognize it.
2. **Level 2 — Common ambiguous entities**: "Washington" (person, state, city, university). Context usually makes the referent clear.
3. **Level 3 — Context-dependent disambiguation**: "Apple" (fruit vs. company vs. record label). Requires careful context reading.
4. **Level 4 — Rare entities**: Mentions of obscure people, places, or organizations with limited knowledge base coverage.
5. **Level 5 — NIL detection**: Recognize when a mention refers to an entity not in the knowledge base and correctly predict NIL.
6. **Level 6 — Cross-lingual entity linking**: Link mentions in one language to a knowledge base in another language (e.g., Arabic text to English Wikipedia).
7. **Level 7 — Zero-shot entity linking**: Link to specialized domains (Zeshel) where the entity set is entirely new and the model must rely on entity descriptions alone.

## Limitations & Risks

- **Knowledge base incompleteness**: Not all entities exist in the knowledge base. New entities are constantly created. The gold annotations may mark mentions as NIL when the entity actually does exist (or vice versa).
- **Granularity mismatches**: A mention might refer to an entity at a different granularity than what the KB provides (e.g., "downtown Paris" vs. the Wikipedia article for Paris the city).
- **Coreference chains**: Entity linking on coreferent mentions (pronouns, abbreviated references) requires coreference resolution as a prerequisite.
- **Wikipedia evolution**: Wikipedia articles are merged, split, renamed, and deleted. Entity IDs can become stale.
- **Silver data noise**: Wikipedia hyperlink-based data has systematic biases (editors link familiar entities more, first mentions are linked but subsequent ones are not).
- **Context window**: For long documents, the relevant context for disambiguation may be far from the mention.

## Connections

- **[information-extraction](information-extraction.md)**: NER identifies mentions; entity linking resolves them. They are sequential steps in the IE pipeline.
- **[knowledge-graph-completion](knowledge-graph-completion.md)**: Entity linking connects text to KGs; KGC fills in missing information about linked entities.
- **[question-answering-closed](question-answering-closed.md)**: Entity linking is needed for knowledge-based QA to ground entities in the KB.
- **[fact-verification](fact-verification.md)**: Verifying claims about entities requires first linking those entities to the KB.
- **[multilingual-tasks](multilingual-tasks.md)**: Cross-lingual entity linking bridges languages through shared knowledge bases.
- **[cloze-completion](cloze-completion.md)**: Entity prediction in cloze tasks is related to entity linking.
