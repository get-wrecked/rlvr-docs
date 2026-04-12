---
domain: Data-to-Text Generation
category: nlp-generation
verification_type: constraint
dataset_scale: 25K-120K examples (WebNLG, E2E, ToTTo)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Data-to-Text Generation

## Overview

Data-to-text generation produces fluent natural language descriptions from structured inputs: RDF triples, table cells, database records, or key-value pairs. For example, given the triple (Ristorante Roma, food, Italian) and (Ristorante Roma, priceRange, moderate), produce "Ristorante Roma is a moderately priced Italian restaurant."

The RLVR angle is that verification can check factual fidelity: every fact in the generated text must be supported by the input data, and no facts should be hallucinated. Slot error rate (counting missing, wrong, or hallucinated slots) provides a concrete reward signal. Fluency is harder to verify automatically but less critical for RLVR than factual accuracy.

## Verification Mechanism

```python
def verify_data_to_text(generated: str, input_data: dict, 
                         slot_values: dict) -> dict:
    """
    slot_values: {"name": "Ristorante Roma", "food": "Italian", "priceRange": "moderate"}
    """
    generated_lower = generated.lower()

    # Check slot coverage: are all input facts mentioned?
    mentioned = 0
    missing = []
    for slot, value in slot_values.items():
        # Allow value synonyms (e.g., "moderate" matches "moderately priced")
        value_variants = get_value_variants(value)
        if any(v.lower() in generated_lower for v in value_variants):
            mentioned += 1
        else:
            missing.append(slot)

    coverage = mentioned / len(slot_values) if slot_values else 1.0

    # Check for hallucinated slots: facts not in input
    hallucinated = detect_hallucinated_facts(generated, slot_values)
    hallucination_penalty = len(hallucinated) * 0.2

    # Slot error rate
    n_slots = len(slot_values)
    slot_error_rate = (len(missing) + len(hallucinated)) / n_slots if n_slots else 0.0

    reward = max(0, coverage - hallucination_penalty)

    return {
        "slot_coverage": coverage,
        "missing_slots": missing,
        "hallucinated_facts": hallucinated,
        "slot_error_rate": slot_error_rate,
        "reward": reward
    }

def get_value_variants(value: str) -> list:
    """Generate acceptable surface form variants of a slot value."""
    variants = [value]
    # e.g., "moderate" -> ["moderate", "moderately priced", "mid-range"]
    synonym_map = {
        "moderate": ["moderate", "moderately priced", "mid-range"],
        "high": ["high", "expensive", "high-end"],
        "low": ["low", "cheap", "budget"],
    }
    if value.lower() in synonym_map:
        variants.extend(synonym_map[value.lower()])
    return variants

def detect_hallucinated_facts(text: str, known_facts: dict) -> list:
    """Heuristic: detect entity names/values in text not present in input."""
    # This is a simplified version; robust hallucination detection is an open problem
    hallucinated = []
    # Check for competitor names, wrong prices, etc.
    return hallucinated
```

## Dataset Sources

- **WebNLG**: 25,298 text-triple pairs across 15 DBpedia categories. Input: 1-7 RDF triples. Output: 1-3 sentence descriptions. Multiple reference texts per input. Standard benchmark for triple-to-text.
- **E2E (End-to-End NLG Challenge)**: 50,602 instances in the restaurant domain. Input: meaning representations with 3-8 slots (name, food, area, priceRange, etc.). Multiple references per input.
- **ToTTo (Table-to-Text)**: 120,761 examples. Input: Wikipedia table + highlighted cells. Output: one-sentence description of the highlighted cells. Google.
- **DART**: 82,191 examples across open-domain tables and triples. Unified format from WebNLG, E2E, and other sources.
- **RotoWire**: 4,853 NBA game summaries from box score statistics. Long-form generation (300+ words).
- **WikiBio**: 728K Wikipedia biography first sentences from infobox data. Large-scale but noisy.
- **GenWiki**: Wikipedia-scale table-to-text with 1.3M instances.

## Task Format

- **Input**: Structured data (triples, table rows, or key-value pairs).
```
Generate a restaurant description from these attributes:
name: The Golden Dragon
food: Chinese
area: riverside
priceRange: high
familyFriendly: no
near: The Bakers
```
- **Output**: Natural language text.
```
The Golden Dragon is an upscale Chinese restaurant located in the riverside
area near The Bakers. It is not family friendly.
```

## Difficulty Curriculum

- Level 1: Single triple/slot -> one sentence ("name: X, food: Y" -> "X serves Y food")
- Level 2: 3-4 slots, all easily expressible in one sentence
- Level 3: 5-7 slots requiring multi-sentence output, proper aggregation
- Level 4: Conflicting or complex attributes requiring careful phrasing ("familyFriendly: no")
- Level 5: Multiple related triples requiring coherent ordering and discourse planning
- Level 6: Table-to-text (ToTTo style) requiring selecting relevant cells and interpreting table structure
- Level 7: Statistical data (RotoWire) requiring numerical comparison and trend description
- Level 8: Multi-table/multi-source data requiring information fusion
- Level 9: Open-domain structured data with unseen schemas, requiring generalization to new attribute types

## Limitations & Risks

- **Hallucination detection is imperfect**: Checking that all input facts are mentioned is feasible, but detecting hallucinated facts (things the model says that are not in the input) is much harder. Simple string matching misses subtle hallucinations.
- **Fluency-fidelity tradeoff**: Maximizing slot coverage can produce disfluent text ("X is an Italian restaurant that is moderately priced and is in the city centre and is family friendly and..."). Fluency requires a separate evaluation axis.
- **Value normalization**: Checking whether "moderately priced" matches "priceRange: moderate" requires a normalization layer. This mapping is domain-specific and incomplete.
- **Multiple valid outputs**: There are many correct ways to express the same data. Exact match against a single reference is inappropriate. Slot error rate is better but still imperfect.
- **Domain narrowness**: E2E is only restaurants. WebNLG covers 15 DBpedia categories. Real-world data-to-text spans arbitrary domains.
- **Ordering is unverified**: The order in which facts are presented affects readability but is not captured by slot error rate.

## Connections

- [[text-normalization]] — Both involve rendering structured information as natural language
- [[semantic-textual-similarity]] — Evaluating generated text quality against references
- [[chart-understanding]] — Reverse task: extracting structured data from text/charts
- [[dialogue-state-tracking]] — DST extracts structured state from text; data-to-text generates text from structured state
- [[html-css-generation]] — Both generate formatted output from specifications
