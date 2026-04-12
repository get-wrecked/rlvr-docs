---
domain: Game Level / Puzzle Generation
category: creative-constrained
verification_type: constraint
dataset_scale: unlimited (procedurally generated + internet archives)
difficulty_range: easy/medium/hard
modality: multimodal
status: strong_hypothesis
---

# Game Level / Puzzle Generation

## Overview
Generate valid, solvable game levels/puzzles: Sokoban levels, Zelda-style dungeons, platformer layouts, maze designs. Verification: (1) level satisfies structural constraints, (2) level is solvable (path from start to goal exists), (3) level has desired difficulty properties.

## Verification Mechanism
```python
def verify(game_type: str, level: GameLevel, constraints: dict) -> float:
    score = 0.0
    checks = 0
    
    # Structural validity
    checks += 1
    if level.is_structurally_valid():
        score += 1
    
    # Solvability (critical!)
    checks += 1
    solution = solve(game_type, level, timeout=60)
    if solution is not None:
        score += 1
    else:
        return score / checks  # Unsolvable = immediate partial failure
    
    # Unique solution (if required)
    if constraints.get("unique_solution"):
        checks += 1
        alt_solutions = find_all_solutions(game_type, level, max_solutions=2, timeout=120)
        if len(alt_solutions) == 1:
            score += 1
    
    # Difficulty (solution length as proxy)
    if "min_solution_length" in constraints:
        checks += 1
        if len(solution) >= constraints["min_solution_length"]:
            score += 1
    
    if "max_solution_length" in constraints:
        checks += 1
        if len(solution) <= constraints["max_solution_length"]:
            score += 1
    
    # Size constraints
    if "dimensions" in constraints:
        checks += 1
        if level.width == constraints["dimensions"][0] and \
           level.height == constraints["dimensions"][1]:
            score += 1
    
    # Aesthetic constraints (symmetry, open space ratio, etc.)
    if "min_open_ratio" in constraints:
        checks += 1
        open_ratio = level.count_open() / level.total_cells()
        if open_ratio >= constraints["min_open_ratio"]:
            score += 1
    
    return score / checks

def solve(game_type, level, timeout):
    """Use BFS/A*/domain-specific solver to find solution."""
    if game_type == "sokoban":
        return sokoban_solver(level, timeout=timeout)
    elif game_type == "maze":
        return bfs(level.start, level.goal, level.neighbors)
    elif game_type == "platformer":
        return physics_based_solver(level, timeout=timeout)
    # etc.
```

## Dataset Sources
- **Sokoban**: Standard Microban level sets (300+ levels). Random level generation is well-studied.
- **NetHack / MiniHack**: Procedurally generated dungeon levels.
- **Mario AI benchmarks**: Super Mario Bros level generation competition.
- **Zelda-like dungeons**: GVGAI framework, procedural Zelda level generation.
- **Maze generation**: Trivially generated at any scale/difficulty.
- **PCGRL (Procedural Content Generation via RL)**: Published benchmarks.
- **Game Maker's Toolkit puzzle design examples**: Educational content (manual extraction).

## Task Format
- **Input**: "Create a 7x7 Sokoban level with 3 boxes that requires at least 15 moves to solve"
- **Output**: Grid-based level representation

## Difficulty Curriculum
- Level 1: Simple mazes (guaranteed solvable via DFS)
- Level 3: Sokoban with 2 boxes, small grid
- Level 5: Sokoban with 4+ boxes, larger grid
- Level 7: Multi-room dungeons with keys and locks
- Level 9: Levels with emergent difficulty properties (backtracking required, etc.)

## Limitations & Risks
- Solvability checking can be expensive (Sokoban solving is PSPACE-complete). Limit grid sizes.
- "Interesting" and "fun" are subjective. Focus on structural/solvability constraints, not aesthetics.
- Solution length is a rough proxy for difficulty — doesn't capture cognitive difficulty.

## Connections
- [[puzzle-games]] — playing vs. creating puzzles
- [[planning-classical]] — solvability is a planning problem
- [[combinatorics-optimization]] — level design as constrained optimization
