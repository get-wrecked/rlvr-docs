---
domain: constrained-writing
category: format-constrained
verification_type: constraint
dataset_scale: ~infinite (template-generated)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Constrained Writing

## Overview

Constrained writing tasks require generating text that satisfies a set of formal, programmatically verifiable constraints. This is one of the purest RLVR domains because constraints are entirely mechanical: word count limits, required or forbidden words, acrostics, lipograms, pangrams, specific structural patterns (e.g., every sentence must start with successive letters of the alphabet), fixed paragraph counts, or combinations thereof.

This domain is highly valuable for RLVR because (1) verification is exact and cheap, (2) tasks can be generated in unlimited quantities from constraint templates, and (3) the skills transfer directly to instruction-following — arguably the single most important capability for practical LLM alignment.

## Verification Mechanism

**Type: Constraint satisfaction (exact)**

Each constraint maps to a deterministic check:

| Constraint | Verification |
|---|---|
| Word count = N | `len(text.split()) == N` |
| Must include word W | `W.lower() in text.lower()` (or regex for whole-word match) |
| Must exclude word W | `W.lower() not in text.lower()` |
| Acrostic spells X | First letter of each line/sentence matches X[i] |
| Lipogram (no letter L) | `L not in text.lower()` |
| Pangram | `set(string.ascii_lowercase).issubset(set(text.lower()))` |
| Sentence count = N | `len(sent_tokenize(text)) == N` |
| All sentences < K words | `all(len(s.split()) < K for s in sentences)` |
| Alliteration in each sentence | Check first-letter frequency per sentence |
| Paragraph structure | Split on `\n\n`, check count and per-paragraph constraints |

Reward can be binary (all constraints met or not) or fractional (proportion of constraints satisfied). For RL training, fractional reward with a bonus for full satisfaction is recommended.

**Verification confidence: HIGH.** All checks are deterministic string operations. Edge cases exist (sentence boundary detection, hyphenated words, contractions) but are manageable with standard NLP tokenizers.

## Dataset Sources

- **Template generation (primary):** Define a constraint grammar and sample random constraint sets. Unlimited scale. Example template: "Write a {3-7} sentence paragraph about {topic} that includes the word {W1} and {W2}, does not use the letter {L}, and has exactly {N} words."
- **IFEval benchmark:** Google's instruction-following evaluation set contains many constrained-writing tasks. ~500 examples but can be used as seed templates.
- **BIG-bench:** Several subtasks involve constrained text generation.
- **Writing prompts subreddits:** r/WritingPrompts, r/constrainedwriting for topic variety.
- **Existing constrained writing literature:** OuLiPo (Ouvroir de litterature potentielle) catalogues dozens of constraint types.

## Task Format

**Input:**
```
Write a paragraph about autumn that:
1. Contains exactly 50 words
2. Includes the words "amber" and "whisper"
3. Does not contain the letter "z"
4. Every sentence begins with a different letter
5. Has exactly 4 sentences
```

**Output:**
```
Amber leaves drift slowly through the cool morning air. Branches sway and whisper
secrets to the fading sunlight below. Crisp winds carry the scent of earth and
dried bark across the quiet meadow. Deep reds paint the hillside in brilliant,
warm color.
```

**Verification output:** `{"constraints_met": 5, "constraints_total": 5, "reward": 1.0}`

## Difficulty Curriculum

1. **Easy:** Single constraint (e.g., "write exactly 20 words about dogs").
2. **Medium:** 2-3 compatible constraints (word count + required words + sentence count).
3. **Hard:** 4-6 constraints including structural ones (acrostic + word count + forbidden words).
4. **Very hard:** Conflicting-tension constraints (lipogram on a common letter + pangram-like requirements for remaining letters + tight word count). Also: nested constraints (each paragraph has its own constraint set).

Difficulty can also be scaled by:
- Tightening tolerances (exact word count vs. range)
- Adding more forbidden/required words
- Requiring coherent, on-topic text (checked via keyword overlap with topic, though this moves toward softer verification)

## Limitations & Risks

- **Coherence is not verified:** A model can satisfy all constraints with gibberish. Partial mitigation: require specific topic keywords, but full coherence checking would require LLM-as-judge (outside RLVR scope).
- **Sentence/word boundary ambiguity:** "Word count" depends on tokenization. Hyphenated words, contractions, and abbreviations create edge cases. Must standardize the tokenizer and make it available to the model.
- **Reward hacking via degenerate text:** Model may learn to produce constraint-satisfying but meaningless text. Mitigation: combine with a language modeling objective or add soft fluency penalties.
- **Syllable counting is imperfect:** If syllable-based constraints are included, CMU Pronouncing Dictionary or similar has gaps for rare/novel words.

## Connections

- [[instruction-following]] — Constrained writing is a subset of instruction following; skills transfer directly.
- [[poetry-formal]] — Formal poetry is constrained writing with literary-specific constraints (meter, rhyme).
- [[data-formatting]] — Structural output constraints parallel format compliance.
- [[crossword-construction]] — Another constraint satisfaction domain, but over a grid rather than prose.
