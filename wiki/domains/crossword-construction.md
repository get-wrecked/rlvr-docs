---
domain: crossword-construction
category: format-constrained
verification_type: constraint
dataset_scale: ~medium (dictionaries + existing puzzles)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Crossword Construction

## Overview

Crossword construction requires building a valid crossword puzzle grid: placing words into an interlocking grid such that all horizontal and vertical entries are valid dictionary words, the grid satisfies symmetry and connectivity constraints, and optionally the entries relate to a theme. This is a constraint satisfaction problem with a rich combinatorial structure.

The domain is interesting for RLVR because it requires search, planning, backtracking (conceptually), vocabulary breadth, and spatial reasoning — all exercised simultaneously.

## Verification Mechanism

**Type: Constraint satisfaction (deterministic)**

Verification checks:

1. **Grid validity:** All cells contain exactly one letter or are black squares.
2. **Word validity:** Every contiguous horizontal and vertical sequence of 2+ letters is a valid dictionary word. Check against a standard word list (e.g., TWL06, SOWPODS, or a curated crossword dictionary).
3. **Grid constraints:**
   - No 1-letter words.
   - No unchecked letters (every letter appears in both an across and a down word) — standard for American-style crosswords.
   - Rotational symmetry (180-degree) — standard for published crosswords.
   - All white squares connected (no isolated regions).
   - Minimum word length (typically 3).
4. **Theme constraints (optional):** If theme entries are specified, verify they appear in the grid at specified positions.

```python
def verify_crossword(grid: list[list[str]], dictionary: set[str]) -> dict:
    words = extract_all_words(grid)  # horizontal + vertical
    all_valid = all(w.lower() in dictionary for w in words)
    has_symmetry = check_rotational_symmetry(grid)
    all_connected = check_connectivity(grid)
    no_short_words = all(len(w) >= 3 for w in words)
    all_checked = check_all_letters_checked(grid)
    
    score = sum([all_valid, has_symmetry, all_connected, no_short_words, all_checked]) / 5
    return {"reward": score, "invalid_words": [w for w in words if w.lower() not in dictionary]}
```

**Verification confidence: HIGH.** All checks are deterministic given a word list. The only ambiguity is which word list to use (different lists yield different valid word sets).

## Dataset Sources

- **Crossword puzzle archives:** NYT crossword archive (paid), LA Times, Washington Post, The Guardian. Decades of puzzles with solutions.
- **Crossword Compiler word lists:** Professional crossword dictionaries (TWL, SOWPODS) available commercially.
- **Open word lists:** Peter Broda's word list, spread the word list, and various open crossword databases.
- **XWordInfo.com:** Metadata and analysis of NYT crosswords.
- **Crossword puzzle datasets on GitHub:** Several repositories with parsed crossword data.
- **Synthetic generation:** Define a grid pattern (black square layout), then task the model with filling it. Grid patterns can be extracted from existing puzzles or generated randomly with symmetry constraints.

## Task Format

**Input (fill task):**
```
Fill the following 5x5 crossword grid. Black squares are marked with '#'.
Grid pattern:
. . . . .
. # . # .
. . . . .
. # . # .
. . . . .

All horizontal and vertical words must be valid English words (minimum 3 letters).
```

**Output:**
```
A B O D E
R # I # A
E M I R S
A # E # T
FEAST
```

(Represented as a filled grid)

**Input (full construction):**
```
Construct a 15x15 American-style crossword puzzle with rotational symmetry.
Theme: words containing "LIGHT" (at least 3 theme entries spanning the grid).
Provide the completed grid and a numbered clue list.
```

**Verification output:**
```json
{
  "total_words": 78,
  "valid_words": 78,
  "invalid_words": [],
  "symmetry": true,
  "connectivity": true,
  "min_word_length_met": true,
  "theme_entries_present": 3,
  "reward": 1.0
}
```

## Difficulty Curriculum

1. **Medium:** Fill a small (5x5) grid with given black square pattern. Fewer interlocking constraints.
2. **Hard:** Fill a standard 15x15 grid with given black square pattern and theme entries pre-placed.
3. **Very hard:** Construct a full 15x15 grid from scratch — choose black square placement, fill all entries, ensure all words are valid.
4. **Superhuman:** Construct a themed crossword with creative, coherent clues, pangram constraint (all 26 letters used), low word count (fewer, longer entries = harder).

## Limitations & Risks

- **Word list dependency:** "Valid word" depends entirely on which dictionary is used. Proper nouns, abbreviations, and partial words are contentious. Must align the verification dictionary with what the model considers valid.
- **No quality verification:** A grid can be technically valid but filled with obscure abbreviations and crosswordese. Puzzle quality (fresh entries, good theme, clever clues) is not verifiable.
- **Clue quality:** If clues are required, verifying clue correctness is much harder than verifying the grid. Clue verification is essentially question-answering verification.
- **Output format complexity:** The model must output a structured grid, which is error-prone in text format. A standardized grid notation is essential.
- **Computational difficulty:** Full crossword construction is NP-hard in the general case. Expecting an LLM to solve this by generation (no backtracking) means it will fail on hard instances.

## Connections

- [[sudoku-generation]] — Both are grid-based constraint satisfaction problems.
- [[constrained-writing]] — Crosswords are constraint satisfaction applied to words.
- [[regex-crossword]] — A variant combining crossword structure with regex constraints.
