---
domain: music-theory
category: creative-with-constraints
verification_type: rule
dataset_scale: ~medium (textbook + MIDI corpora)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Music Theory

## Overview

Music theory tasks require composing, analyzing, or completing music that obeys formal rules: voice leading, counterpoint, harmonization, chord progression, part writing, and form analysis. These rules have been codified over centuries and are taught in every conservatory, making them amenable to programmatic verification.

The domain is valuable for RLVR because (1) the rule systems are well-defined and have been implemented in software (music21, etc.), (2) there is a natural difficulty curriculum from simple intervals to full four-part counterpoint, and (3) it exercises planning, constraint satisfaction, and aesthetic judgment under rules — a combination rare in other domains.

## Verification Mechanism

**Type: Rule-based (deterministic)**

Music theory rules can be checked programmatically using the `music21` Python library or equivalent:

| Rule | Verification |
|---|---|
| No parallel fifths/octaves | Check successive intervals between voice pairs |
| Voice range constraints | Each voice stays within standard range (soprano: C4-G5, etc.) |
| No voice crossing | At each beat, soprano > alto > tenor > bass |
| Proper chord spacing | Adjacent upper voices within an octave |
| Resolution of tendency tones | Leading tone resolves up, chordal 7th resolves down |
| Doubling rules | Root doubled in root position, no doubled leading tone |
| Cadence identification | Final two chords match standard cadence patterns |
| Species counterpoint rules | Check cantus firmus rules per species (note-against-note, etc.) |
| Chord progression validity | Roman numeral sequence follows standard progressions |

**Implementation:** Parse model output as MusicXML, MIDI, or `**kern` notation. Load into music21. Run rule checker functions. Each violated rule deducts from reward.

**Reward structure:** Start at 1.0, subtract penalty per violation (weighted by severity). Parallel fifths = -0.15, voice crossing = -0.1, range violation = -0.2, etc. Floor at 0.0.

**Verification confidence: HIGH for rule checking, LOW for aesthetic quality.** The rules of voice leading and counterpoint are entirely formal. However, following all rules does not guarantee the music sounds good — that aspect is not verifiable without human judgment.

## Dataset Sources

- **music21 corpus:** Ships with hundreds of Bach chorales, Monteverdi, Palestrina — canonical voice-leading examples.
- **Bach Chorales dataset:** 371 four-part chorales, each a gold-standard voice-leading exercise. Multiple digital editions (Bach10, music21 built-in).
- **IMSLP (Petrucci Music Library):** Massive public domain score collection. Species counterpoint examples from Fux, Jeppesen textbooks.
- **Kostka/Payne/Almen textbook exercises:** Standard harmony textbook with graded exercises (would need to digitize).
- **Musedata, KernScores:** Large collections in machine-readable formats.
- **MusicNet:** 330+ freely licensed classical recordings with aligned MIDI.
- **Synthetic generation:** Generate partially completed exercises (given soprano, harmonize; given Roman numerals, write four parts).

## Task Format

**Input:**
```
Harmonize the following soprano melody in four-part chorale style (SATB) in C major:
Soprano: C5 D5 E5 C5 | F5 E5 D5 C5 |

Write your answer as four lines (S/A/T/B) with note names and octave numbers.
```

**Output:**
```
S: C5 D5 E5 C5 | F5 E5 D5 C5 |
A: E4 F4 G4 E4 | A4 G4 F4 E4 |
T: G3 A3 C4 G3 | C4 C4 A3 G3 |
B: C3 D3 C3 C3 | F3 C3 D3 C3 |
```

**Verification output:**
```json
{
  "parallel_fifths": 0,
  "parallel_octaves": 0,
  "voice_crossing": 0,
  "range_violations": 0,
  "spacing_violations": 0,
  "doubling_errors": 0,
  "resolution_errors": 1,
  "total_penalty": 0.1,
  "reward": 0.9
}
```

## Difficulty Curriculum

1. **Easy:** Identify intervals, name chords, simple transposition.
2. **Medium:** Two-part first-species counterpoint (note-against-note). Harmonize with root-position triads only.
3. **Hard:** Four-part chorale harmonization with inversions, seventh chords, secondary dominants. Second/third species counterpoint.
4. **Very hard:** Free counterpoint, modulating passages, four-part writing with chromatic harmony. Full 16-bar chorale harmonization.
5. **Superhuman:** Compose a fugue exposition on a given subject. Double counterpoint. Full sonata-form analysis and completion.

## Limitations & Risks

- **Notation parsing:** Model must output in a parseable format. Defining a clean notation format and training the model to use it is a prerequisite. Errors in notation (not music) should not be penalized as music errors.
- **Rules are necessary but not sufficient:** A rule-perfect chorale can sound mechanical and unmusical. Aesthetic quality is outside RLVR scope.
- **Historical style variation:** Rules differ by era (Renaissance vs. Baroque vs. Classical). Must specify which rule set applies per task.
- **Limited to Western tonal music:** The standard rule set does not apply to atonal, microtonal, or non-Western music traditions.
- **Reward hacking:** Model may learn to write extremely simple, rule-safe music (all unisons, static harmony) rather than take risks. Mitigation: require minimum harmonic variety or melodic movement.

## Connections

- [[poetry-formal]] — Both are creative domains with formal structural rules.
- [[constrained-writing]] — Music composition is constraint satisfaction in a different medium.
- [[chess]] — Both require planning ahead under rules (choosing a note now that doesn't create problems later).
