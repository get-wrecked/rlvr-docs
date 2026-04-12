---
domain: Logic Puzzles
category: Logic
verification_type: constraint
dataset_scale: ~10M+ (unlimited via procedural generation)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

## Overview

Logic puzzles — Sudoku, KenKen, Nonograms, Zebra/Einstein puzzles, Kakuro, Futoshiki, Skyscrapers, logic grid puzzles, and more — are a large and diverse family of constraint satisfaction problems with perfect verifiability. Each puzzle type has a set of well-defined rules, and a candidate solution either satisfies all constraints or it doesn't. No ambiguity, no partial credit debates.

This domain has enormous dataset potential. Millions of Sudoku puzzles are freely available online, and every puzzle type can be procedurally generated at controlled difficulty. The domain tests logical deduction, constraint propagation, and search — core reasoning skills. While less explored in RLVR than math, the verification infrastructure is trivial to implement and the data is abundant.

## Verification Mechanism

**Primary method: Rule-based constraint checking.**

Each puzzle type has a specific set of constraints. Verification checks all of them.

```python
def verify_sudoku(puzzle: list[list[int]], solution: list[list[int]]) -> float:
    """
    Verify a Sudoku solution.
    
    Args:
        puzzle: 9x9 grid with 0 for empty cells
        solution: 9x9 grid with all cells filled
    
    Constraints:
        1. Every cell in [1, 9]
        2. Each row contains 1-9 exactly once
        3. Each column contains 1-9 exactly once
        4. Each 3x3 box contains 1-9 exactly once
        5. Solution agrees with puzzle on given cells
    """
    # Check consistency with puzzle
    for i in range(9):
        for j in range(9):
            if puzzle[i][j] != 0 and puzzle[i][j] != solution[i][j]:
                return 0.0
    
    # Check all values in range
    for i in range(9):
        for j in range(9):
            if solution[i][j] < 1 or solution[i][j] > 9:
                return 0.0
    
    # Check rows
    for i in range(9):
        if len(set(solution[i])) != 9:
            return 0.0
    
    # Check columns
    for j in range(9):
        col = [solution[i][j] for i in range(9)]
        if len(set(col)) != 9:
            return 0.0
    
    # Check 3x3 boxes
    for box_r in range(3):
        for box_c in range(3):
            box = []
            for i in range(3):
                for j in range(3):
                    box.append(solution[box_r*3 + i][box_c*3 + j])
            if len(set(box)) != 9:
                return 0.0
    
    return 1.0


def verify_zebra_puzzle(clues: list, solution: dict) -> float:
    """
    Verify an Einstein/Zebra puzzle solution.
    
    Args:
        clues: List of constraint objects (e.g., {"type": "same_house", "attr1": "red", "attr2": "Brit"})
        solution: Dict mapping house positions to attribute assignments
                  E.g., {1: {"color": "red", "nationality": "Brit", ...}, 2: {...}, ...}
    
    Returns:
        1.0 if all clues satisfied, 0.0 otherwise.
    """
    for clue in clues:
        if clue["type"] == "same_house":
            # Two attributes belong to the same house
            house1 = find_house_with_attr(solution, clue["attr1"])
            house2 = find_house_with_attr(solution, clue["attr2"])
            if house1 != house2:
                return 0.0
        elif clue["type"] == "next_to":
            house1 = find_house_with_attr(solution, clue["attr1"])
            house2 = find_house_with_attr(solution, clue["attr2"])
            if abs(house1 - house2) != 1:
                return 0.0
        elif clue["type"] == "left_of":
            house1 = find_house_with_attr(solution, clue["attr1"])
            house2 = find_house_with_attr(solution, clue["attr2"])
            if house1 != house2 - 1:
                return 0.0
        elif clue["type"] == "position":
            house = find_house_with_attr(solution, clue["attr"])
            if house != clue["position"]:
                return 0.0
    
    # Also check: each attribute used exactly once, all houses filled
    for category in get_categories(clues):
        values = [solution[h][category] for h in solution]
        if len(set(values)) != len(values):
            return 0.0
    
    return 1.0


def verify_nonogram(row_clues: list, col_clues: list, solution: list[list[int]]) -> float:
    """
    Verify a Nonogram (Picross) solution.
    
    Args:
        row_clues: For each row, the sequence of filled-block lengths. E.g., [[3,1], [2], [5]]
        col_clues: For each col, the sequence of filled-block lengths.
        solution: 2D grid of 0 (empty) and 1 (filled).
    """
    n_rows = len(row_clues)
    n_cols = len(col_clues)
    
    # Check row clues
    for i in range(n_rows):
        runs = get_runs(solution[i])
        if runs != row_clues[i]:
            return 0.0
    
    # Check column clues
    for j in range(n_cols):
        col = [solution[i][j] for i in range(n_rows)]
        runs = get_runs(col)
        if runs != col_clues[j]:
            return 0.0
    
    return 1.0

def get_runs(line: list[int]) -> list[int]:
    """Extract consecutive runs of 1s from a binary line."""
    runs = []
    count = 0
    for cell in line:
        if cell == 1:
            count += 1
        else:
            if count > 0:
                runs.append(count)
            count = 0
    if count > 0:
        runs.append(count)
    return runs if runs else [0]


def verify_kenken(size: int, cages: list, solution: list[list[int]]) -> float:
    """
    Verify a KenKen solution.
    
    Args:
        size: Grid size (e.g., 6 for 6x6)
        cages: List of (cells, target, operation) tuples.
               E.g., ([(0,0),(0,1)], 11, '+') means cells (0,0)+(0,1) sum to 11
        solution: size x size grid
    """
    # Check Latin square property
    for i in range(size):
        if len(set(solution[i])) != size:
            return 0.0
        if any(v < 1 or v > size for v in solution[i]):
            return 0.0
    for j in range(size):
        col = [solution[i][j] for i in range(size)]
        if len(set(col)) != size:
            return 0.0
    
    # Check cage constraints
    for cells, target, op in cages:
        values = [solution[r][c] for r, c in cells]
        if op == '+':
            if sum(values) != target:
                return 0.0
        elif op == '*':
            product = 1
            for v in values:
                product *= v
            if product != target:
                return 0.0
        elif op == '-':
            if abs(values[0] - values[1]) != target:
                return 0.0
        elif op == '/':
            if max(values) / min(values) != target:
                return 0.0
        elif op == '=':  # single cell
            if values[0] != target:
                return 0.0
    
    return 1.0
```

## Dataset Sources

| Dataset | Size | Source | Notes |
|---------|------|--------|-------|
| **Sudoku (various)** | >1M | [kaggle.com/datasets (multiple)](https://www.kaggle.com/datasets), [github.com/t-dillon/tdoku](https://github.com/t-dillon/tdoku) | 1M+ puzzles at all difficulty levels |
| **Sudoku 17-clue minimum** | ~49,000 | [staffhome.ecm.uwa.edu.au/~00013890/sudokumin.php](http://staffhome.ecm.uwa.edu.au/~00013890/sudokumin.php) | Hardest Sudoku (fewest given digits) |
| **Project Euler Sudoku** | 50 | [projecteuler.net/problem=96](https://projecteuler.net/problem=96) | Classic benchmark |
| **Zebra/Einstein puzzles** | ~5K | Various web scrapers, puzzle books | Classic "who owns the fish" type |
| **Nonogram puzzles** | ~50K | [webpbn.com](https://webpbn.com/), [nonograms.org](https://www.nonograms.org/) | Various sizes 5x5 to 50x50+ |
| **KenKen** | ~10K | [kenken.com](https://www.kenken.com/), procedural | Sizes 3x3 to 9x9 |
| **Kakuro** | ~10K | Various puzzle sites | Cross-sum puzzles |
| **Logic grid puzzles** | ~2K | [logic-puzzles.org](https://www.logic-puzzles.org/), [brainzilla.com](https://www.brainzilla.com/) | "Clue"-style deduction puzzles |
| **Futoshiki** | ~5K | Puzzle sites, procedural | Inequality constraints + Latin square |
| **Procedural generation** | Unlimited | Custom generators | All puzzle types can be generated |

### Procedural Generation

```python
def generate_sudoku(difficulty: str = "medium"):
    """
    Generate a Sudoku puzzle with guaranteed unique solution.
    
    Strategy:
    1. Generate a valid complete grid (backtracking or permutation method)
    2. Remove cells one at a time, checking uniqueness after each removal
    3. Stop when desired difficulty is reached
    """
    # Step 1: Create valid complete grid
    grid = generate_complete_sudoku_grid()
    
    # Step 2: Remove cells while maintaining unique solution
    cells = [(i, j) for i in range(9) for j in range(9)]
    random.shuffle(cells)
    
    puzzle = [row[:] for row in grid]
    n_removed = 0
    target_clues = {"easy": 40, "medium": 30, "hard": 25, "expert": 20}[difficulty]
    
    for r, c in cells:
        if 81 - n_removed <= target_clues:
            break
        backup = puzzle[r][c]
        puzzle[r][c] = 0
        if count_solutions(puzzle) == 1:
            n_removed += 1
        else:
            puzzle[r][c] = backup  # Restore — removal creates ambiguity
    
    return {"puzzle": puzzle, "solution": grid}

def generate_zebra_puzzle(n_houses=5, n_categories=5):
    """
    Generate an Einstein/Zebra puzzle.
    
    Strategy:
    1. Create a random valid assignment (ground truth)
    2. Generate clues that uniquely determine the assignment
    3. Minimize the clue set (remove redundant clues)
    """
    categories = ["color", "nationality", "drink", "pet", "cigarette"][:n_categories]
    values = {cat: random.sample(CATEGORY_VALUES[cat], n_houses) for cat in categories}
    
    # Ground truth: values[cat][i] = value of category cat in house i
    truth = {i: {cat: values[cat][i] for cat in categories} for i in range(n_houses)}
    
    # Generate clues from truth
    clues = generate_clues_from_truth(truth, categories)
    
    # Minimize: remove clues that are redundant
    clues = minimize_clue_set(clues, truth)
    
    return {"clues": clues, "solution": truth, "n_houses": n_houses}
```

## Task Format

**Sudoku Input**:
```
Solve this Sudoku puzzle (0 = empty cell):

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

**Zebra Puzzle Input**:
```
There are 5 houses in a row. Each house has a different color, and each homeowner 
has a different nationality, drink, pet, and hobby. Given these clues:

1. The Brit lives in the red house.
2. The Swede keeps dogs.
3. The Dane drinks tea.
...

Question: Who owns the fish?
```

**Output**: Complete grid (Sudoku) or full assignment table (Zebra).

## Difficulty Curriculum

| Level | Puzzle Type | Specifics | Scale |
|-------|-------------|-----------|-------|
| Easy | 4x4 Sudoku, 2-house Zebra | Trivial constraint propagation | Unlimited |
| Medium | 9x9 Sudoku (30+ clues), 3-house Zebra, small Nonogram | Some search needed | Unlimited |
| Hard | 9x9 Sudoku (<25 clues), 5-house Zebra, 15x15 Nonogram | Significant backtracking | Unlimited |
| Very Hard | 17-clue Sudoku, large Nonograms, multi-category Zebra | Expert-level deduction | ~50K |

## Limitations & Risks

1. **Formatting challenges**: Parsing a 9x9 grid from model output is error-prone. Whitespace, alignment, and delimiter variations cause extraction failures. Structured output formats (JSON) help.
2. **Memorization risk**: Popular Sudoku puzzles may be in training data. Procedural generation mitigates this but must ensure puzzle uniqueness.
3. **Narrow reasoning**: Sudoku-solving skill may not transfer to broader logical reasoning. The constraint types are very specific to each puzzle.
4. **Solver competition**: Like SAT, dedicated constraint solvers can solve any Sudoku instantly. The LLM's value must be in generalization, natural language understanding, or transfer.
5. **Grid size limits**: LLMs struggle with large spatial grids. Even 9x9 is challenging for current models to track mentally. Larger puzzles (16x16 Sudoku, large Nonograms) may be impractical.

## Connections

- **logic-propositional.md**: Every constraint puzzle can be encoded as a SAT instance. Sudoku is a classic SAT encoding exercise.
- **logic-first-order.md**: Puzzle rules can be expressed in FOL.
- **combinatorics-optimization.md**: Some puzzles (e.g., Kakuro) have optimization variants.
- **automated-reasoning.md**: Puzzle solving is a concrete, accessible form of automated reasoning.
