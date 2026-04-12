---
domain: information-extraction
category: language
verified: true
verification_type: exact
scalability: high
maturity: high
tags: [ner, relation-extraction, event-extraction, structured-data]
---

# Information Extraction

## Overview

Information extraction (IE) encompasses tasks that transform unstructured text into structured representations: named entity recognition (NER), relation extraction, event extraction, and slot filling. The model reads a passage and outputs structured tuples such as `(Entity, Type)`, `(Subject, Relation, Object)`, or `(Event, Trigger, Arguments)`.

This is one of the strongest RLVR domains because verification is exact and datasets are enormous. The core loop is simple: give the model text, ask it to extract structured facts, compare its output against a gold-standard annotation.

## Verification Mechanism

**Verification type: Exact (with minor caveats)**

- **NER**: Compare predicted entity spans and types against gold annotations. Compute exact match at the entity level (both span boundaries and type must match). Standard metrics are precision, recall, and F1 at the entity level. For RLVR, a binary reward can be issued per entity or per example (all entities correct = 1, else 0).
- **Relation extraction**: Compare predicted `(subject, relation, object)` triples against gold triples. Exact match on all three components. Partial credit schemes are possible (correct relation type but wrong argument).
- **Event extraction**: Compare predicted event triggers and argument roles against gold annotations. Exact match on trigger span, event type, argument span, and argument role.

The verification function is fully programmatic: parse the model's structured output, compare element-wise to gold annotations, return a score. No human judgment or LLM-as-judge is needed.

**Caveats**: Span boundary disagreements can be noisy (e.g., "the United States" vs "United States"). Normalization (lowercasing, stripping determiners) helps. Some datasets allow approximate span matching with a tolerance window.

## Dataset Sources

| Dataset | Task | Size | Source | Notes |
|---------|------|------|--------|-------|
| CoNLL-2003 | NER | 22K sentences | Reuters news | English and German; PER, LOC, ORG, MISC |
| OntoNotes 5.0 | NER | 1.7M words | Newswire, broadcast, web | 18 entity types |
| ACE 2005 | Relation + Event extraction | 599 docs | Newswire, broadcast | 7 entity types, 6 relation types, 33 event types |
| TACRED | Relation extraction | 106K examples | TAC KBP newswire | 42 relation types |
| DocRED | Document-level RE | 5K docs | Wikipedia | 96 relation types, requires cross-sentence reasoning |
| FewRel | Relation extraction | 70K instances | Wikipedia + Wikidata | Few-shot relation classification |
| WNUT-2017 | NER (emerging entities) | 5.7K tweets | Twitter | Novel/emerging entities |
| WikiAnn | Multilingual NER | 40 languages | Wikipedia | Silver-standard from anchor links |

**Massive-scale generation**: Wikipedia + Wikidata alignment provides virtually unlimited silver-standard training data. Wikidata triples can be projected onto Wikipedia sentences to create relation extraction examples. Wikipedia anchor links provide entity linking and NER data. This makes IE one of the most scalable RLVR domains.

## Task Format

**Input**: A text passage, optionally with a schema defining target entity types, relation types, or event types.

**Output**: Structured extraction in a canonical format, e.g.:

```
NER: [("Barack Obama", PERSON), ("Hawaii", LOCATION)]
RE:  [("Barack Obama", born_in, "Hawaii")]
EE:  [(Attack, trigger="bombed", agent="militants", target="embassy")]
```

For RLVR, the model should produce output in a parseable format (JSON, tagged text, or tuple notation) so the verifier can programmatically compare against gold.

**Prompt template example**:
```
Extract all person and organization entities from the following text.
Output as JSON: [{"text": "...", "type": "PERSON"|"ORG"}]

Text: {passage}
```

## Difficulty Curriculum

1. **Level 1 — Single entity type NER**: Extract only person names from clean news text. High-frequency entities, unambiguous boundaries.
2. **Level 2 — Multi-type NER**: Extract multiple entity types (PER, ORG, LOC, DATE). Requires type discrimination.
3. **Level 3 — Sentence-level relation extraction**: Extract relations from single sentences with explicitly stated relations.
4. **Level 4 — Nested/overlapping entities**: Handle entities that share spans or nest inside each other. Requires more sophisticated output representation.
5. **Level 5 — Document-level relation extraction**: Extract relations from long documents where the subject and object may be in different paragraphs (DocRED-style).
6. **Level 6 — Event extraction with multiple arguments**: Full event extraction with trigger identification and role labeling for all arguments.
7. **Level 7 — Cross-document IE**: Extract and merge information across multiple documents, resolving coreference and contradictions.

## Limitations & Risks

- **Annotation disagreement**: IE gold standards have non-trivial inter-annotator disagreement, especially for span boundaries and edge-case entity types. This introduces noise in the reward signal.
- **Schema dependence**: Models may overfit to specific ontologies (e.g., ACE's 33 event types) rather than learning general extraction ability.
- **Silver data quality**: Wikipedia/Wikidata alignment produces silver data with ~85-90% precision. Noise from distant supervision can mislead training.
- **Output format brittleness**: The verifier requires a specific output format. Models may produce correct extractions in slightly wrong formats, receiving zero reward. Robust parsing of model output is essential.
- **Long-tail entities**: Rare entity types and relations have few training examples, making reward signals sparse for the hardest cases.

## Connections

- **[entity-linking](entity-linking.md)**: IE provides the mentions that entity linking then resolves to knowledge base entries.
- **[knowledge-graph-completion](knowledge-graph-completion.md)**: Extracted triples feed into knowledge graphs; IE and KGC are complementary.
- **[structured-output-generation](structured-output-generation.md)**: IE requires generating structured output (JSON, tuples), so structured output skills transfer directly.
- **[semantic-parsing](semantic-parsing.md)**: Both tasks map natural language to formal representations.
- **[fact-verification](fact-verification.md)**: Extracted claims can be verified against knowledge bases.
- **[table-understanding](table-understanding.md)**: Extracting structured data from tables is a specialized form of IE.
