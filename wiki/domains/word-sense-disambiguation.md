---
domain: Word Sense Disambiguation
category: nlp
verification_type: exact_match
dataset_scale: 200K+ annotated instances (SemEval, WordNet corpora)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Word Sense Disambiguation

## Overview

Word sense disambiguation (WSD) identifies the correct meaning of a polysemous word in context. Given a sentence with a target word, the system must select the appropriate sense from a predefined sense inventory (typically WordNet). For example, "bank" in "He sat on the river bank" maps to WordNet sense `bank.n.01` (sloping land) rather than `bank.n.02` (financial institution).

RLVR verification is clean: exact match against the gold sense key. The output space is small and discrete (typically 2-20 senses per word), and gold annotations come from expert lexicographers. The challenge is that WSD requires genuine contextual understanding, not just pattern matching.

## Verification Mechanism

```python
def verify_wsd(predicted_sense: str, gold_sense: str,
               valid_senses: list = None) -> dict:
    """
    predicted_sense: e.g., "bank%1:17:01::" or "bank.n.01"
    gold_sense: gold standard sense key
    """
    # Exact match on sense key
    exact = predicted_sense.strip() == gold_sense.strip()

    # Check if prediction is even a valid sense for this word
    valid = True
    if valid_senses is not None:
        valid = predicted_sense.strip() in valid_senses

    return {
        "exact_match": 1.0 if exact else 0.0,
        "valid_sense": valid,
        "reward": 1.0 if exact else 0.0
    }

def verify_wsd_batch(predictions: list, golds: list) -> dict:
    """Batch F1 metric as used in SemEval WSD evaluations."""
    correct = sum(1 for p, g in zip(predictions, golds) if p.strip() == g.strip())
    # In all-words WSD, precision=recall=F1 when system attempts all instances
    f1 = correct / len(golds) if golds else 0.0
    return {"f1": f1, "reward": f1}
```

## Dataset Sources

- **SemEval/Senseval WSD tasks**: Multiple evaluation campaigns spanning 2001-2021.
  - SemEval-2007 Task 17 (all-words): 465 instances
  - SemEval-2013 Task 12: 1,644 instances across 13 languages
  - SemEval-2015 Task 13: 1,261 instances (unified WSD+Entity Linking)
- **WordNet Gloss Tagged Corpus**: ~117K sense-tagged content words automatically annotated using WordNet glosses. Princeton.
- **SemCor**: ~234K sense-tagged words from the Brown Corpus. The largest manually annotated WSD dataset. Gold standard.
- **OMSTI (One Million Sense-Tagged Instances)**: ~1M automatically sense-tagged instances. Noisy but massive.
- **WSD Evaluation Framework (Raganato et al., 2017)**: Unified evaluation on 5 standard test sets (Senseval-2, Senseval-3, SemEval-2007, SemEval-2013, SemEval-2015). Includes standardized sense inventory mapping.
- **XL-WSD**: Cross-lingual WSD dataset covering 18 languages.
- **WordNet**: The sense inventory itself (117K synsets in English WordNet 3.0). Provides definitions, examples, and relations for each sense.

## Task Format

- **Input**: Sentence with target word highlighted, and available senses.
```
Sentence: "The bank approved the loan application yesterday."
Target word: "bank"

Available senses:
1. bank.n.01: sloping land beside a body of water
2. bank.n.02: a financial institution
3. bank.n.03: a long ridge or pile
4. bank.n.05: a supply or stock held in reserve
5. bank.v.01: tip laterally (an aircraft)

Which sense of "bank" is used in this sentence?
```
- **Output**: The sense key.
```
bank.n.02
```

## Difficulty Curriculum

- Level 1: Highly frequent sense vs. rare sense with strong contextual clues ("bank" with "money", "loan")
- Level 2: Two common senses with moderate context ("I went to the bank")
- Level 3: Fine-grained distinctions within a domain ("paper" as material vs. document vs. newspaper)
- Level 4: Metaphorical usage ("a bright student" -> bright.a.02 not bright.a.01)
- Level 5: Verbs with many senses ("run" has 40+ senses in WordNet)
- Level 6: Domain-specific senses requiring specialized knowledge
- Level 7: Rare senses that appear infrequently in training data
- Level 8: Cross-lingual WSD (same concept expressed differently across languages)
- Level 9: Novel/emerging senses not well-covered in WordNet ("cloud" in computing), coarse vs. fine-grained disambiguation

## Limitations & Risks

- **Sense inventory granularity**: WordNet senses are often too fine-grained. Annotators frequently disagree on closely related senses (inter-annotator agreement ~72% for fine-grained, ~85% for coarse). Training on disputed annotations adds noise.
- **Most-frequent-sense baseline**: Simply choosing the most frequent sense per word achieves ~65% F1 on standard benchmarks. The model might learn this shortcut rather than genuine disambiguation.
- **Sense inventory staleness**: WordNet was last significantly updated years ago. New word senses (e.g., "tweet" as a social media post) are missing or underrepresented.
- **Coverage gaps**: WordNet covers English comprehensively but many languages lack comparable resources. Cross-lingual evaluation is limited.
- **Evaluation by recall at 100% coverage**: Standard WSD evaluation assumes the system attempts every instance. In RLVR, abstention or low-confidence predictions would need separate handling.
- **Knowledge-based vs. supervised**: Some WSD approaches use only WordNet definitions (knowledge-based) while others use annotated corpora (supervised). RLVR should test generalization beyond memorized training examples.

## Connections

- [[coreference-resolution]] — Both require understanding what a linguistic expression refers to
- [[semantic-role-labeling]] — SRL + WSD together capture core sentence meaning
- [[morphological-inflection]] — Both are core linguistic analysis tasks; inflection handles form, WSD handles meaning
- [[reading-comprehension]] — Many comprehension questions hinge on understanding word senses in context
- [[named-entity-recognition]] — Entity disambiguation (entity linking) is a closely related task
