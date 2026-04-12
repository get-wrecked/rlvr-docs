---
domain: chess
category: games/board-games
verification_type: rule | simulation | outcome
dataset_scale: >10B positions, >4B games
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Chess

## Overview

Chess is one of the richest RLVR domains available. The game has perfect information, deterministic rules, and a massive corpus of human and engine play. RLVR tasks span the full spectrum: legal move generation, tactical puzzle solving (mate-in-N, find the best move), positional evaluation, full game play, and natural-language analysis of positions. Chess is particularly valuable because verification is cheap and exact: a position either is checkmate or it is not, a move is legal or it is not, and engine evaluation provides a continuous quality signal.

## Verification Mechanism

Multiple verification layers are available, all fully programmatic:

1. **Legal move checking** (rule-based): Given a FEN position, verify that a proposed move is legal under FIDE rules. Libraries: `python-chess`, `shakmern`, Stockfish. Cost: microseconds per check.
2. **Mate verification** (rule-based): For "mate in N" puzzles, verify that the proposed sequence forces checkmate in exactly N moves against all defenses. Implemented via minimax search or direct engine verification. Libraries: Stockfish `go mate N`, `python-chess` with full tree search for small N.
3. **Engine evaluation** (simulation-based): Compare agent's chosen move to Stockfish's evaluation. Reward = 1 if agent's move is within X centipawns of the engine's top choice, 0 otherwise. Alternatively, use a continuous reward = sigmoid(-(eval_agent_move - eval_best_move)/scale). Stockfish 16+ at depth 20+ provides near-perfect ground truth.
4. **Game outcome** (outcome-based): Play the agent against an engine or another agent. Reward = win/draw/loss. Most natural but noisiest signal.
5. **Puzzle correctness** (exact match): For Lichess puzzles, there is a single correct move sequence. Verification = exact match against the known solution.

## Dataset Sources

- **Lichess open database**: https://database.lichess.org/ — All games played on Lichess, updated monthly. ~5 billion games as of 2025. PGN format with clock data, evaluations, and player ratings.
- **Lichess puzzle database**: https://database.lichess.org/#puzzles — >4 million tactical puzzles with verified solutions, rated by difficulty (Elo 400-3000+). CSV format: FEN, move sequence, rating, themes.
- **FICS Games Database**: https://www.ficsgames.org/ — ~800M games from the Free Internet Chess Server.
- **CCRL archives**: http://ccrl.chessdom.com/ — Engine vs engine games at various time controls, useful for superhuman-level data.
- **KingBase**: http://www.kingbase-chess.net/ — ~2.5M high-quality human games (Elo 2000+).
- **ChessGPT datasets**: Curated datasets from Feng et al. (2024) combining game play, puzzles, and commentary.
- **Stockfish NNUE training data**: https://tests.stockfishchess.org/nns — Billions of positions with depth-12+ evaluations.

## Task Format

**Puzzle solving (primary)**:
- Input: FEN string + natural language instruction (e.g., "White to play. Find the best move." or "Find mate in 3.")
- Output: Move in UCI or SAN notation (e.g., "e2e4" or "Nf6+"), or a full sequence for multi-move puzzles.

**Game play**:
- Input: PGN/FEN of current position + move history
- Output: Next move in UCI/SAN

**Position analysis**:
- Input: FEN + question ("Is this position winning for White?", "What is the best plan?")
- Output: Structured analysis. Verification via engine eval agreement.

**Move legality**:
- Input: FEN + proposed move
- Output: "legal" or "illegal". Verification: exact match.

## Difficulty Curriculum

1. **Move legality** (trivial): Is this move legal? Binary classification.
2. **Mate in 1** (easy): ~500K puzzles available. Single correct move, easy to verify.
3. **Simple tactics** (easy-medium): Forks, pins, skewers. Lichess puzzles rated 800-1400.
4. **Mate in 2-3** (medium): Requires short lookahead. Puzzles rated 1400-1800.
5. **Complex tactics** (hard): Sacrifices, quiet moves, long combinations. Puzzles rated 1800-2400.
6. **Positional evaluation** (hard): "Which side is better and why?" Requires deep understanding. Verified against engine eval.
7. **Mate in 4+** (very hard): Exponentially growing search tree. Puzzles rated 2400+.
8. **Full game play vs engine** (superhuman): Playing entire games against Stockfish at increasing strength levels (Elo 1000 to 3500).
9. **Endgame tablebases** (superhuman): Positions where perfect play is known (Syzygy 7-piece tablebases). Agent must find the unique winning line.

## Limitations & Risks

- **Reward hacking in game play**: An agent playing against a weak opponent might learn degenerate strategies that exploit the opponent's weaknesses rather than learning general chess skill. Mitigate by using diverse opponents and engine evaluation as reward.
- **Engine evaluation is not ground truth**: Stockfish at finite depth can misjudge complex positions, especially in closed/fortress positions. For most practical purposes this is negligible.
- **Memorization risk**: The agent might memorize opening sequences or specific puzzle solutions rather than learning general chess reasoning. Mitigate by testing on held-out puzzles and novel positions.
- **Text format limitations**: Representing chess positions as FEN strings is lossy for spatial reasoning compared to 2D board representations. For text-only models, this is an inherent limitation.
- **Evaluation discontinuities**: Engine centipawn scores can jump dramatically between similar positions, making continuous reward shaping noisy.

## Connections

- [[board-games]] — Chess is the most data-rich member of the broader board games family
- [[go-game]] — Similar paradigm (perfect information board game, engine verification) but very different structure
- [[puzzle-games]] — Chess puzzles share the "find the solution sequence" format with combinatorial puzzles
- [[competitive-programming-interactive]] — Both involve adversarial search and verification against a judge
