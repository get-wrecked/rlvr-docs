---
domain: sudoku-generation
category: format-constrained
verification_type: constraint
dataset_scale: ~infinite (algorithmic generation)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Sudoku Generation

## Overview

Sudoku generation tasks require producing valid sudoku puzzles: a 9x9 grid where each row, column, and 3x3 box contains the digits 1-9 exactly once. The task may involve solving a given puzzle, completing a partial grid, or generating a puzzle with a unique solution from scratch. Variants include different grid sizes (4x4, 16x16) and constraint modifications (diagonal sudoku, killer sudoku).

This is a clean RLVR domain because verification is exact, fast, and well-understood. Sudoku is NP-complete in the general case but practical instances are efficiently verifiable.

## Verification Mechanism

**Type: Constraint satisfaction (exact)**

**For solved grids:**
```python
def verify_sudoku(grid: list[list[int]]) -> bool:
    for i in range(9):
        if sorted(grid[i]) != list(range(1, 10)):  # row check
            return False
        if sorted(grid[j][i] for j in range(9)) != list(range(1, 10)):  # column check
            return False
    for box_r in range(3):
        for box_c in range(3):
            box = [grid[box_r*3+r][box_c*3+c] for r in range(3) for c in range(3)]
            if sorted(box) != list(range(1, 10)):
                return False
    return True
```

**For puzzle generation (must have unique solution):**
1. Verify the given clues appear in the solution.
2. Run a constraint solver (e.g., backtracking with constraint propagation) to confirm exactly one solution exists.

Libraries: `py-sudoku`, constraint solvers, or custom backtracking implementation. Verification is O(1) for solution checking, O(exponential worst case but fast in practice) for uniqueness.

**Verification confidence: VERY HIGH.** Sudoku verification is perfectly deterministic. No ambiguity whatsoever.

## Dataset Sources

- **Algorithmic generation:** Sudoku puzzles can be generated in unlimited quantities by: (1) generating a random valid completed grid, (2) removing clues while maintaining unique solution.
- **Gordon Royle's minimum sudoku collection:** 49,000+ 17-clue sudoku puzzles (minimum known clue count for unique solution).
- **Project Euler:** Problem 96 contains 50 sudoku puzzles.
- **Kaggle sudoku datasets:** Several million-puzzle datasets (e.g., "1 Million Sudoku Games").
- **QQWing:** Open-source sudoku generator/solver that can produce rated puzzles.
- **Sudoku variant databases:** KillerSudokuOnline, LogicMasters for variant puzzles.

## Task Format

**Input (solve):**
```
Solve this sudoku puzzle (0 = empty cell):
5 3 0 | 0 7 0 | 0 0 0
6 0 0 | 1 9 5 | 0 0 0
0 9 8 | 0 0 0 | 0 6 0
------+-------+------
8 0 0 | 0 6 0 | 0 0 3
4 0 0 | 8 0 3 | 0 0 1
7 0 0 | 0 2 0 | 0 0 6
------+-------+------
0 6 0 | 0 0 0 | 2 8 0
0 0 0 | 4 1 9 | 0 0 5
0 0 0 | 0 8 0 | 0 7 9
```

**Output:** The completed 9x9 grid.

**Input (generate):**
```
Generate a valid sudoku puzzle with exactly 28 given clues that has a unique solution. Provide both the puzzle and its solution.
```

**Output:** Puzzle grid (with 28 filled cells and 53 zeros) + solution grid.

## Difficulty Curriculum

1. **Easy:** Solve a puzzle with many givens (35-45 clues). Naked singles suffice.
2. **Medium:** Solve a puzzle with 25-34 clues. Requires hidden singles, pointing pairs.
3. **Hard:** Solve a puzzle with 20-24 clues. Requires X-wing, swordfish, advanced techniques.
4. **Very hard:** Solve a 17-clue puzzle (minimum possible). Generate a valid puzzle with a unique solution.
5. **Variants:** Solve diagonal sudoku, killer sudoku (cage sums), or 16x16 grids.

For generation tasks, difficulty = fewer clues (harder to ensure uniqueness) and specific constraints on puzzle difficulty rating.

## Limitations & Risks

- **Limited reasoning transfer:** Sudoku solving is a narrow skill. It exercises constraint propagation and logical deduction but may not transfer broadly to other reasoning tasks.
- **Brute force vs. reasoning:** An LLM cannot efficiently backtrack. If the puzzle requires deep search, the model may fail even on "easy" rated puzzles that happen to require trial-and-error at some point.
- **Output parsing:** Grid formatting in text is error-prone. A strict output format (e.g., 81 comma-separated digits) reduces parsing errors.
- **Generation uniqueness:** Verifying unique solution requires running a solver, which is computationally cheap but adds infrastructure.

## Connections

- [[crossword-construction]] — Both are grid-based constraint satisfaction.
- [[constraint-satisfaction]] — Sudoku is the canonical example of CSP.
- [[math-competition]] — Logical deduction skills overlap.
