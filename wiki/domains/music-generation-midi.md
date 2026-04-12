---
domain: MIDI Music Generation with Theory Constraints
category: creative-constrained
verification_type: rule
dataset_scale: 1M+ (MIDI archives)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# MIDI Music Generation with Theory Constraints

## Overview
Generate MIDI music that satisfies specified music theory constraints (key, time signature, chord progression, voice leading rules, counterpoint rules, instrumentation). Unlike subjective "make good music," the constraint-based approach is verifiable.

## Verification Mechanism
```python
from music21 import *

def verify(constraints: dict, midi_data: bytes) -> float:
    score = converter.parse(midi_data)
    checks = 0
    passed = 0
    
    # Key signature
    if "key" in constraints:
        checks += 1
        analyzed_key = score.analyze('key')
        if str(analyzed_key) == constraints["key"]:
            passed += 1
    
    # Time signature
    if "time_signature" in constraints:
        checks += 1
        ts = score.getTimeSignatures()[0]
        if f"{ts.numerator}/{ts.denominator}" == constraints["time_signature"]:
            passed += 1
    
    # Chord progression
    if "chord_progression" in constraints:
        checks += 1
        chords = score.chordify()
        progression = extract_roman_numerals(chords, constraints["key"])
        if progression == constraints["chord_progression"]:
            passed += 1
    
    # Voice leading rules (no parallel fifths/octaves)
    if "voice_leading" in constraints:
        checks += 1
        violations = check_voice_leading(score)
        if len(violations) == 0:
            passed += 1
    
    # Range constraints (each voice within standard range)
    if "voice_ranges" in constraints:
        for voice_name, (low, high) in constraints["voice_ranges"].items():
            checks += 1
            voice = score.parts[voice_name]
            all_in_range = all(low <= n.pitch.midi <= high for n in voice.flatten().notes)
            if all_in_range:
                passed += 1
    
    # Note count / duration
    if "measures" in constraints:
        checks += 1
        if len(score.measures(0, None)) == constraints["measures"]:
            passed += 1
    
    # Species counterpoint rules
    if "counterpoint_species" in constraints:
        checks += 1
        cp_violations = check_species_counterpoint(score, constraints["counterpoint_species"])
        if len(cp_violations) == 0:
            passed += 1
    
    return passed / checks if checks > 0 else 0.0
```

## Dataset Sources
- **Lakh MIDI Dataset**: 170K+ MIDI files with metadata.
- **MAESTRO**: 200+ hours of piano MIDI with audio.
- **MuseScore dataset**: 500K+ public domain scores.
- **Bach chorales** (music21 corpus): 371 chorales — classic counterpoint training data.
- **IMSLP**: 600K+ public domain scores (need MIDI conversion).
- **MusicNet**: 330 classical music recordings with MIDI annotations.
- **Textbook exercises**: Harmony and counterpoint textbook exercises with solutions.

## Task Format
- **Input**: "Compose a 4-measure chorale in C major, 4/4 time, with the chord progression I-IV-V-I, using 4 voices (SATB) following standard voice leading rules."
- **Output**: MIDI data or music21 notation

## Difficulty Curriculum
- Level 1: Single-voice melody in given key and meter
- Level 3: Harmonize a given melody with chords
- Level 5: 4-part chorale with voice leading rules
- Level 7: Species counterpoint (strict rules)
- Level 9: Fugue composition (subject, answer, episodes)

## Limitations & Risks
- Music theory rules are well-defined and checkable, but "good music" has subjective aspects beyond theory.
- This verifies theory compliance, not aesthetic quality. Acceptable for RLVR — we're training reasoning about rules, not taste.
- Some rules are debatable (what counts as a "hidden fifth"?). Use standard textbook definitions.
- music21 library handles most checks natively.

## Connections
- [[music-theory]] — the theory that constrains generation
- [[constrained-writing]] — constrained generation in another medium
- [[poetry-formal]] — formal structure constraints
