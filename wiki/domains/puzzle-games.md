---
domain: puzzle-games
category: games/puzzles
verification_type: simulation | exact_match
dataset_scale: unlimited (procedurally generated)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Puzzle Games

## Overview

Combinatorial puzzle games — Sokoban, Rush Hour, sliding tile puzzles (15-puzzle), Rubik's Cube, Tower of Hanoi, Lights Out, Nonograms, Sudoku-like constraint puzzles, and peg solitaire — form an exceptionally clean RLVR domain. Their defining property: verification is trivial (simulate the move sequence and check if the goal state is reached) while solving can be arbitrarily hard (many are NP-hard or PSPACE-complete). Crucially, puzzles can be procedurally generated in unlimited quantity at precisely controlled difficulty levels, making them ideal for curriculum learning. These environments test planning, search, spatial reasoning, and constraint satisfaction.

## Verification Mechanism

1. **Simulation verification** (simulation-based): Execute the agent's proposed move sequence in a simulator. Check if the final state matches the goal state. This is the primary verification mechanism and is perfectly reliable.
   - Sokoban: All boxes on goal positions.
   - Rush Hour: Target car at exit.
   - Sliding puzzle: Tiles in order.
   - Rubik's Cube: All faces uniform color.
   - Tower of Hanoi: All discs on target peg.
   - Lights Out: All lights off.
2. **Move legality** (rule-based): Each move in the sequence must be legal (e.g., in Sokoban, cannot push box into wall or another box). Checked incrementally during simulation.
3. **Optimality verification** (exact match): For many puzzle instances, the optimal solution length is known (via BFS/A* for small instances, or mathematical bounds). Reward can incorporate solution quality: reward = 1 for solving, bonus for optimal or near-optimal solutions.
4. **Step count** (constraint-based): Verify solution uses at most K moves (for constrained variants). Simple numeric check.

## Dataset Sources

- **Sokoban**:
  - Microban/Sasquatch collections: http://www.sourcecode.se/sokoban/levels.php — Thousands of curated human-designed levels.
  - Boxoban (DeepMind): https://github.com/google-deepmind/boxoban-levels — 900K procedurally generated Sokoban levels at three difficulty tiers.
  - Random level generators: Multiple open-source generators can produce unlimited instances with guaranteed solvability.
- **Rush Hour**:
  - ThinkFun puzzle database: 2500+ official puzzles rated beginner-expert.
  - Procedural generators: https://github.com/topics/rush-hour-puzzle — Multiple generators available.
- **Sliding puzzles (15-puzzle, 24-puzzle)**:
  - Any random permutation of tiles (50% are solvable, easily filtered). Unlimited generation.
  - Standard benchmark sets from IDA* research (Korf, 1985).
- **Rubik's Cube**:
  - God's number is 20 (Rokicki et al., 2010). Generate any scramble by random moves.
  - Kociemba's algorithm provides optimal or near-optimal solutions for verification.
  - Half-turn metric and quarter-turn metric provide two difficulty scales.
- **Tower of Hanoi**:
  - Optimal solution for N discs is 2^N - 1 moves. Generate for any N.
  - Frame-Stewart conjecture provides (conjectured) optimal solutions for 4-peg variant.
- **Nonograms / Picross**:
  - Puzzles can be generated from any binary image. Unlimited supply.
  - Web scrapers for puzzle sites (nonograms.org, etc.).
- **Lights Out**:
  - Solvability can be determined via linear algebra over GF(2). Generate solvable instances at will.

## Task Format

**Standard solving**:
- Input: Puzzle state description (grid for Sokoban/Rush Hour, permutation for sliding puzzle, face colors for Rubik's Cube) + rules reminder.
- Output: Sequence of moves (e.g., "U R D L" for directions, "F R' U F2" for Rubik's cube notation).
- Verification: simulate moves, check goal reached.

**Minimum-move solving**:
- Input: Same as above + "Solve in at most K moves."
- Output: Move sequence of length <= K.
- Verification: simulate + check length.

**State prediction**:
- Input: Puzzle state + move sequence. "What is the state after these moves?"
- Output: Resulting state. Verification: exact match with simulation.

**Solvability classification**:
- Input: Puzzle state. "Is this solvable?"
- Output: Yes/No. Verification: exact match (determined by solver).

## Difficulty Curriculum

1. **Tower of Hanoi, 3-4 discs** (trivial): 7-15 moves. Simple recursion.
2. **Lights Out, 3x3** (easy): Small grid, solvable via enumeration.
3. **Sliding 8-puzzle** (easy-medium): Optimal solutions average ~22 moves.
4. **Sokoban, Microban level 1-50** (easy-medium): Small rooms, 1-2 boxes.
5. **Rush Hour, beginner cards** (medium): Short solutions.
6. **Rubik's Cube, 2x2** (medium): 11 moves max, 3.67M positions.
7. **Sokoban, Microban level 50-155** (medium-hard): More boxes, tighter rooms.
8. **Sliding 15-puzzle** (hard): Optimal solutions average ~53 moves. Huge state space.
9. **Rubik's Cube, 3x3** (hard): ~4.3 x 10^19 positions, max 20 moves optimal.
10. **Sokoban, large instances** (very hard): 10+ boxes. PSPACE-complete in general.
11. **Rush Hour, expert+ cards** (very hard): Solutions requiring 50+ moves.
12. **24-puzzle and beyond** (superhuman): Optimal solving is extremely challenging.
13. **4x4+ Rubik's Cube** (superhuman): Dramatically increased state space and solution length.

## Limitations & Risks

- **Spatial reasoning in text**: Many puzzles are inherently spatial (Sokoban, sliding puzzles). Text representations (coordinate lists, ASCII grids) are awkward and may not support the spatial reasoning needed. Vision-based input could help significantly.
- **Solution memorization**: For fixed puzzle sets, the agent might memorize solutions rather than learn to solve. Mitigate with procedural generation of held-out test sets.
- **Reward sparsity**: Binary "solved/not-solved" is extremely sparse for hard puzzles. Consider partial rewards: number of boxes on goals (Sokoban), number of solved faces (Rubik's), or distance-to-goal heuristics. But partial rewards risk reward hacking.
- **Search vs reasoning**: Many puzzle-solving algorithms are essentially search (BFS, A*, IDA*). An LLM "solving" puzzles might not be doing the same kind of reasoning — it might learn heuristic patterns. This is fine for RLVR training but should be understood.
- **Generation bias**: Procedural generators may produce puzzles with different difficulty distributions than human-designed puzzles. Human-designed puzzles often have elegant or tricky solutions that test specific reasoning skills.

## Connections

- [[chess]] — Chess puzzles (mate-in-N) share the "find the solution sequence" structure
- [[go-game]] — Tsumego (life-and-death) problems are essentially Go puzzles
- [[board-games]] — Puzzles are single-player games, board games are two-player, but both test combinatorial reasoning
- [[spatial-reasoning]] — Puzzles heavily exercise spatial reasoning skills
