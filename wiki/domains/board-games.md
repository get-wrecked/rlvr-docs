---
domain: board-games
category: games/board-games
verification_type: rule | outcome | simulation
dataset_scale: >10M games (varies by game)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Board Games (Other)

## Overview

Beyond chess and Go, many classic board games offer clean RLVR environments with well-defined rules and deterministic verification. This page covers Othello (Reversi), Connect Four, Checkers (Draughts), Nine Men's Morris, Hex, Gomoku, and similar two-player combinatorial games. Their key RLVR advantage: many of these games are either fully solved (Connect Four, Checkers) or solvable for small board sizes, providing perfect ground-truth verification. They also vary in complexity, creating a natural curriculum from trivial (Tic-Tac-Toe) to highly non-trivial (Hex on large boards).

## Verification Mechanism

1. **Legal move checking** (rule-based): Each game has simple, formally specifiable rules. Verify proposed moves in microseconds. Libraries exist for most popular games or can be trivially implemented.
2. **Game outcome** (outcome-based): Play to completion, winner determined by game rules. Deterministic and unambiguous for all games listed.
3. **Optimal play verification** (exact match): For solved games, the minimax-optimal move is known.
   - **Connect Four**: Solved by Victor Allis (1988). First player wins with perfect play. Full solution database available (~4.5 trillion positions for standard 7x6).
   - **Checkers**: Weakly solved by Chinook (Schaeffer et al., 2007). Optimal play from the start is a draw. Endgame databases available for all positions with <=10 pieces.
   - **Tic-Tac-Toe**: Trivially solved. All lines of play are known.
   - **Nine Men's Morris**: Solved (Gasser 1996). Draw with perfect play.
4. **Engine evaluation** (simulation-based): For unsolved games (Othello 8x8, Hex 11x11+), strong engines provide near-optimal evaluation. Edax for Othello (perfect endgame solver for ~20 empty squares), specialized Hex solvers for boards up to 9x9.
5. **Endgame databases** (exact match): Many games have complete endgame tablebases for small piece counts, enabling exact verification of late-game positions.

## Dataset Sources

- **Connect Four solver**: http://connect4.gamesolver.org/ — Complete solution, lookup any position.
- **Chinook (Checkers)**: https://webdocs.cs.ualberta.ca/~chinook/ — Endgame database, opening book, game records.
- **Othello/Reversi**: 
  - WTHOR database: http://www.ffothello.org/informatique/la-base-wthor/ — ~100K high-quality tournament games.
  - Edax: https://github.com/abulmo/edax-reversi — Engine with perfect endgame solving.
- **Hex**: MoHex (U of Alberta), various solver implementations. Limited human game databases but strong engine self-play generation.
- **Gomoku/Renju**: Online server records (PlayOK, LittleGolem). Yixin engine for strong play.
- **General game-playing datasets**: OpenSpiel (DeepMind) provides implementations for dozens of board games: https://github.com/google-deepmind/open_spiel
- **LittleGolem**: https://www.littlegolem.net/ — Multi-game server with archives for Hex, Havannah, Gomoku, and others.

## Task Format

**Move selection**:
- Input: Board state as text grid or compact notation + whose turn it is. Example for Connect Four: column heights or full grid. For Othello: 8x8 grid with B/W/empty.
- Output: Move (column number for Connect4, coordinate for Othello/Hex, square notation for Checkers).

**Optimal play**:
- Input: Game position + "What is the optimal move?"
- Output: The minimax-optimal move (for solved games/positions). Verification: exact match with solution database.

**Game outcome prediction**:
- Input: Game position + "Who wins with perfect play?"
- Output: "First player" / "Second player" / "Draw". Verification: exact match with solved result.

**Full game play**:
- Input: Game state + history
- Output: Next move. Verification: game outcome after full play.

## Difficulty Curriculum

1. **Tic-Tac-Toe** (trivial): Solved, tiny state space. Warm-up.
2. **Connect Four (small board)** (easy): 4x4 or 5x4 boards. Few positions.
3. **Connect Four (standard 7x6)** (medium): Solved but requires lookahead. First player wins.
4. **Othello 6x6** (medium): Smaller than standard but still strategic. Can be solved.
5. **Checkers (endgame)** (medium-hard): Endgame positions from the Chinook database.
6. **Othello 8x8** (hard): Standard board. Very strong engines exist but not fully solved.
7. **Hex (small boards)** (hard): Hex on 7x7-9x9 is partially solved. Deep strategic game.
8. **Hex (large boards)** (very hard): 11x11+ standard tournament size. First-player advantage proven but no practical solution.
9. **Multi-game generalization** (superhuman): Train a single agent that plays all these games well — tests generalization of strategic reasoning.

## Limitations & Risks

- **Small community, less data**: Most of these games have far less recorded human play than chess or Go. Self-play or engine generation may be needed to scale.
- **Text representation challenges**: Spatial board states in text format lose visual structure. Grid formatting helps but is imperfect.
- **Solved games can be "cheated"**: For fully solved games, the agent could in principle memorize the solution table rather than learning to reason. Use unseen positions and verify generalization.
- **Varying rule variants**: Many games have multiple rule variants (e.g., international vs English checkers, Othello vs Reversi edge cases, Gomoku vs Renju forbidden-move rules). Must be consistent.
- **Limited practical utility**: These games are primarily useful as reasoning benchmarks rather than for real-world agent tasks. Their value is in providing clean, verifiable environments for developing search and planning capabilities.

## Connections

- [[chess]] — The most data-rich board game RLVR domain
- [[go-game]] — The most strategically complex board game domain
- [[puzzle-games]] — Shares the combinatorial reasoning aspect
- [[game-playing-atari]] — Another family of game-based RLVR environments
