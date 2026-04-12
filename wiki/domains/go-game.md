---
domain: go-game
category: games/board-games
verification_type: rule | simulation | outcome
dataset_scale: >100M games
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Go

## Overview

Go is a two-player perfect-information board game with simple rules but extraordinary strategic depth. Its branching factor (~250 legal moves per position vs ~35 in chess) makes brute-force search infeasible, making it a premier testbed for learned evaluation and policy. AlphaGo/AlphaZero demonstrated superhuman Go via self-play RL with game-outcome rewards, establishing Go as a canonical RLVR success story. For LLM-based agents, Go tasks include move selection, life-and-death problems (tsumego), territory counting, and game analysis.

## Verification Mechanism

1. **Legal move checking** (rule-based): Verify a proposed move is legal under the rules of Go (not suicide, not ko violation, intersection is empty). Libraries: `gnugo`, `katago`, `gomill`, or any SGF library. Cost: microseconds.
2. **Game outcome** (outcome-based): Play the game to completion, then score by Chinese or Japanese rules. The winner is deterministic given the final board state. Libraries: GnuGo scoring, KataGo.
3. **Engine evaluation** (simulation-based): Compare agent's move to KataGo's top choice. KataGo provides win-rate estimates and score estimates with high accuracy. Reward = 1 if agent's move is within KataGo's top-K by win-rate, scaled reward otherwise.
4. **Tsumego verification** (rule-based): Life-and-death problems have a unique correct answer sequence. Verify by checking that the proposed sequence leads to the group living or dying regardless of opponent response. Can be done with confined-area search.
5. **Territory counting** (exact match): Given a finished or near-finished game, count territory. Verification = exact match with correct count under specified ruleset.

## Dataset Sources

- **KGS Go Server archives**: https://www.u-go.net/gamerecords/ — Millions of games from KGS, various skill levels (30 kyu to 9 dan). SGF format.
- **OGS (Online Go Server)**: https://online-go.com/ — Large collection of games, API available for bulk download.
- **GoGoD (Games of Go on Disc)**: ~100K professional games with detailed metadata. Commercial but widely used in Go AI research.
- **Sensei's Library / GoProblems.com**: Tsumego collections with solutions. Thousands of curated life-and-death problems at various difficulty levels.
- **KataGo self-play data**: https://katagotraining.org/ — Billions of positions from KataGo training runs with high-quality evaluations.
- **Tygem/WBaduk**: Korean/Chinese server archives. Millions of games, skewing toward strong amateur and professional.
- **Go4Go**: https://www.go4go.net/ — Professional game database with >100K games.
- **SmartGo**: Professional game database, overlaps with GoGoD.

## Task Format

**Move selection**:
- Input: Board state as SGF/GTP coordinates or 19x19 text grid + move history. Instruction: "Black to play. Choose the best move."
- Output: Move as coordinate (e.g., "Q16", "D4"), or pass.

**Tsumego (life-and-death)**:
- Input: Partial board showing a local position. Instruction: "Black to play and live" or "White to kill."
- Output: Move sequence. Verification: all responses lead to correct outcome.

**Territory estimation**:
- Input: Board state in mid/endgame.
- Output: Score estimate (e.g., "Black +7.5"). Verification: compare to KataGo's evaluation or final game result.

**Game play**:
- Input: Full game state + history in SGF
- Output: Next move

## Difficulty Curriculum

1. **Legal move identification** (trivial): Is this intersection a legal move? Binary.
2. **Capture problems** (easy): Simple capturing races. 1-3 moves.
3. **Basic tsumego** (easy-medium): Life-and-death problems rated 25-15 kyu.
4. **Joseki selection** (medium): Choose the correct local sequence from standard patterns.
5. **Intermediate tsumego** (medium-hard): Problems rated 15-5 kyu. Require reading 5-10 moves deep.
6. **Whole-board evaluation** (hard): Assess which side is ahead and by how much.
7. **Advanced tsumego** (hard): Problems rated 5 kyu to 5 dan. Deep reading with multiple branches.
8. **Professional-level move selection** (very hard): Match professional choices in complex middle-game positions.
9. **Superhuman play** (superhuman): Beat KataGo at increasing strength. Board sizes 9x9 -> 13x13 -> 19x19 provide a natural scale axis.

## Limitations & Risks

- **Board representation for LLMs**: A 19x19 Go board is difficult to represent in text. Coordinate systems (A1-T19) lose spatial locality. This is a fundamental challenge for text-only models; vision models with board images may fare better.
- **Scoring rule ambiguity**: Chinese vs Japanese rules can disagree on score in edge cases. Must specify ruleset consistently.
- **Engine-as-oracle limitations**: KataGo is superhuman but not perfect. In rare positions (complex ko fights, unusual rulesets) it can err.
- **Data bias**: Available game records skew toward KGS/OGS demographics. Professional game databases are smaller and potentially behind paywalls.
- **Tsumego scope**: Life-and-death problems are local, but Go mastery requires whole-board thinking. Over-training on tsumego might not transfer to full-game play.
- **Ko and superko**: Ko situations create long cycles that complicate verification of "correct" play. Positional vs situational superko rules differ.

## Connections

- [[chess]] — Fellow perfect-information board game with engine-based verification, but Go's search space is vastly larger
- [[board-games]] — Go is a member of the broader board game family
- [[puzzle-games]] — Tsumego problems share the puzzle-solving paradigm
- [[game-playing-atari]] — Historical RL benchmark; Go was the "next frontier" after Atari
