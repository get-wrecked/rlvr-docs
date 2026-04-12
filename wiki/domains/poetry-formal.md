---
domain: poetry-formal
category: creative-with-constraints
verification_type: constraint
dataset_scale: ~large (corpus + generation)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Formal Poetry Generation

## Overview

Formal poetry requires generating text that satisfies precise literary constraints: syllable counts (haiku), rhyme schemes (sonnets, limericks), meter patterns (iambic pentameter), stanza structures, and combinations thereof. Unlike free verse, formal poetry has rules that are largely — though not perfectly — automatable for verification.

This domain sits at the intersection of constrained writing and creative generation. It exercises the model's phonological awareness, vocabulary flexibility, and ability to plan ahead structurally (e.g., choosing end words for a sonnet that will rhyme 12 lines later).

## Verification Mechanism

**Type: Constraint satisfaction (approximate)**

| Form | Constraints | Verification Tool |
|---|---|---|
| Haiku | 5-7-5 syllables, 3 lines | Syllable counter (CMU Pronouncing Dictionary / Pyphen) |
| Limerick | AABBA rhyme, anapestic meter, 5 lines | Rhyme checker + stress pattern analysis |
| Sonnet (Shakespearean) | 14 lines, ABAB CDCD EFEF GG rhyme, iambic pentameter | Line count + rhyme checker + meter scanner |
| Sonnet (Petrarchan) | 14 lines, ABBAABBA + CDECDE/CDCDCD | Same tools |
| Villanelle | 19 lines, specific repetition pattern, ABA rhyme | Line matching + rhyme checker |
| Terza rima | ABA BCB CDC... chain rhyme | Rhyme checker on end words |

**Syllable counting:** Use CMU Pronouncing Dictionary (134k words with phoneme breakdowns). For unknown words, fall back to heuristic syllable counters (e.g., Pyphen, syllable estimation from vowel clusters). Accuracy ~95-98% on common English.

**Rhyme checking:** Extract last stressed vowel + following phonemes from CMU dict. Perfect rhyme = identical from last stressed vowel onward. Slant rhyme can be allowed at reduced reward.

**Meter analysis:** Map words to stress patterns via CMU dict (0=no stress, 1=primary, 2=secondary). Check against expected pattern (e.g., 0-1-0-1-0-1-0-1-0-1 for iambic pentameter). Allow substitution feet (trochaic substitution in first foot is standard).

**Reward structure:**
- Line count correct: 0.2
- Rhyme scheme correct: 0.3
- Syllable/meter pattern correct: 0.3
- All constraints met bonus: 0.2

**Verification confidence: MEDIUM-HIGH.** Syllable counting and rhyme detection work well for common vocabulary but degrade on proper nouns, neologisms, and words with variable pronunciation. Meter analysis is the weakest link — natural English poetry frequently bends meter rules, making strict checking overly punitive.

## Dataset Sources

- **Project Gutenberg:** Thousands of formal poems in public domain. Sonnets, villanelles, limericks readily extractable.
- **Poetry Foundation / poets.org:** Large tagged collections (form-labeled).
- **Shakespeare's Sonnets:** 154 sonnets, canonical iambic pentameter training data.
- **Limerick collections:** Various public domain anthologies; also r/limericks.
- **CMU Pronouncing Dictionary:** Essential for syllable/rhyme verification. ~134k entries.
- **Haiku datasets:** Several on HuggingFace (e.g., `hjian/haiku`), plus classical Japanese haiku in translation.
- **Template generation:** Generate prompts like "Write a Shakespearean sonnet about {topic}" for arbitrary topics.

## Task Format

**Input:**
```
Write a haiku about the ocean.
```

**Output:**
```
Waves crash on the shore
Salt and foam beneath the sun
Tides pull back the sand
```

**Verification output:**
```json
{
  "line_count": {"expected": 3, "actual": 3, "pass": true},
  "syllables": {"expected": [5, 7, 5], "actual": [5, 7, 5], "pass": true},
  "reward": 1.0
}
```

**Input (harder):**
```
Write a Shakespearean sonnet about the passage of time.
```

**Output:** 14 lines of iambic pentameter with ABAB CDCD EFEF GG rhyme scheme.

**Verification output:** Line count, rhyme scheme match, per-line syllable count, meter stress pattern analysis.

## Difficulty Curriculum

1. **Easy:** Haiku (3 lines, syllable counting only). Couplets (2 rhyming lines).
2. **Medium:** Limericks (5 lines, AABBA rhyme, loose meter). Quatrains with specific rhyme schemes.
3. **Hard:** Shakespearean sonnets (14 lines, strict rhyme + meter). Villanelles (complex repetition).
4. **Very hard:** Petrarchan sonnets with volta. Sestinas (complex word-rotation pattern across 39 lines). Formal poetry in non-English languages.

Additional difficulty axes:
- Requiring specific topic adherence (verifiable via keyword presence)
- Combining form constraints with content constraints (e.g., a lipogrammatic sonnet)
- Requiring enjambment or specific rhetorical devices (harder to verify)

## Limitations & Risks

- **Syllable counting is imperfect:** CMU dict does not cover all words. Heuristic fallbacks introduce noise. Words with multiple valid pronunciations (e.g., "caramel" = 2 or 3 syllables) create ambiguity.
- **Quality is not verified:** A metrically perfect sonnet can be terrible poetry. Semantic coherence, imagery, emotional resonance — none of these are captured by formal verification.
- **Meter is fuzzy:** English meter is not perfectly regular even in canonical poetry. Overly strict meter checking penalizes natural-sounding poetry; overly lenient checking rewards doggerel.
- **Rhyme detection edge cases:** Regional pronunciation differences (cot-caught merger), archaic rhymes (love/prove in Shakespeare), and proper nouns cause false negatives/positives.
- **Reward hacking:** Model may learn to use only CMU-dict-friendly words, producing stilted vocabulary. May also learn to pad with filler words to hit syllable targets.

## Connections

- [[constrained-writing]] — Poetry forms are a specific category of writing constraints.
- [[music-theory]] — Both involve formal structural rules applied to creative output.
- [[instruction-following]] — Multi-constraint satisfaction is the core shared skill.
