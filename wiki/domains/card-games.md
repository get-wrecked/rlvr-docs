---
domain: card-games
category: games/card-games
verification_type: outcome | rule
dataset_scale: >100M hands (poker); >1M deals (bridge)
difficulty_range: medium/hard/superhuman
modality: text
status: strong_hypothesis
---

# Card Games

## Overview

Card games — poker, bridge, hearts, spades, skat, and others — introduce imperfect information into the RLVR game-playing paradigm. Unlike chess or Go, players cannot see opponents' hands, forcing reasoning under uncertainty: belief tracking, bluffing, signaling, and Bayesian inference. Poker (particularly No-Limit Texas Hold'em) and contract bridge are the two most researched card games in AI, both having produced superhuman systems (Pluribus for poker, NukkAI for bridge). RLVR verification is straightforward for the mechanical aspects (legal play, score computation) but fundamentally limited for evaluating strategic quality in single hands due to variance.

## Verification Mechanism

1. **Legal play checking** (rule-based): Verify that a played card or action is legal given the game rules and current state. Trivial to implement for all standard card games. E.g., in bridge, must follow suit if possible; in poker, bet must meet minimum raise rules.
2. **Game outcome / scoring** (outcome-based): At the end of a hand/deal, compute the score or chip change deterministically from the play record. Poker: pot goes to best hand (or last remaining player). Bridge: tricks taken vs contract bid, scored per duplicate bridge rules. Hearts: penalty points from hearts and queen of spades.
3. **Aggregate performance over many hands** (outcome-based): Because individual hand outcomes are high-variance, meaningful evaluation requires playing thousands of hands and measuring expected value (bb/100 in poker, IMPs in bridge). This is the most reliable verification signal but expensive.
4. **Duplicate scoring** (outcome-based): In bridge (and applicable to other games), the same deal is played at multiple tables. Performance is measured relative to other players on the same cards, eliminating card-luck variance.
5. **Optimal play for solved subgames** (exact match): In certain simplified poker variants (e.g., Kuhn poker, Leduc poker), Nash equilibrium strategies are known. Verification = compare to GTO (game theory optimal) play.
6. **Convention adherence in bridge** (rule-based): Verify that bids follow a specified bidding system (e.g., Standard American, 2/1). Can be checked against convention card rules.

## Dataset Sources

- **PokerStars hand histories**: Millions of hand histories available through data sharing communities. Contains all actions, stack sizes, and results.
- **IRC Poker Database**: https://poker.cs.ualberta.ca/irc_poker_online.html — Historic dataset from University of Alberta.
- **Annual Computer Poker Competition (ACPC)**: http://www.computerpokercompetition.org/ — Standardized poker AI evaluation. Includes Kuhn, Leduc, and full NLHE.
- **Bridge Base Online (BBO)**: https://www.bridgebase.com/ — Largest online bridge platform. Hand records from major tournaments are publicly available. Millions of deals with bidding and play records.
- **ACBL hand records**: American Contract Bridge League tournament records.
- **WBF Vugraph archives**: World Bridge Federation broadcasts of major championships with expert commentary.
- **OpenSpiel**: https://github.com/google-deepmind/open_spiel — Implementations of many card games including Kuhn poker, Leduc poker, bridge, hearts, and others.
- **PokerRL / DeepStack / Pluribus papers**: Published datasets and evaluation protocols for poker AI.

## Task Format

**Single action decision**:
- Input: Game state (hand, community cards/table state, action history, pot/score). E.g., for poker: "You hold Ah Kd. Board: Qs Jc 2h. Pot: $150. Opponent bets $75. Your action?"
- Output: Action (fold/call/raise amount for poker, card play or bid for bridge).

**Full hand play**:
- Input: Starting hand + game rules + opponent model (or self-play).
- Output: Sequence of actions through the hand. Verification: outcome at end.

**Bidding (bridge)**:
- Input: Hand + bidding sequence so far + system agreement.
- Output: Next bid. Verification: convention adherence (rule-based) + hand outcome (outcome-based).

**Card play (bridge)**:
- Input: Dummy + hand + tricks played so far + contract.
- Output: Next card. Verification: double-dummy analysis gives optimal number of tricks; compare agent's play.

## Difficulty Curriculum

1. **Kuhn poker** (easy): 3-card poker game. Nash equilibrium is known analytically. Tiny state space.
2. **Leduc poker** (easy-medium): 6-card poker game. Solved by CFR. Good testbed.
3. **Simple bridge bidding** (medium): Follow standard conventions for common hand patterns.
4. **Limit Texas Hold'em** (medium-hard): Essentially solved by Cepheus (Bowling et al., 2015).
5. **Bridge card play** (hard): Double-dummy analysis provides verification. Requires planning over 13 tricks.
6. **No-Limit Texas Hold'em, heads-up** (hard): Superhuman systems exist (Libratus, Pluribus) but the game is not solved.
7. **No-Limit Hold'em, 6-player** (very hard): Multiplayer poker with complex dynamics.
8. **Full bridge** (superhuman): Bidding + play + defense. Requires partnership coordination, deception detection, and inference under uncertainty.

## Limitations & Risks

- **HIGH VARIANCE IS A FUNDAMENTAL PROBLEM**: In poker, even optimal play loses many individual hands. A single hand's outcome is a very noisy reward signal. Thousands or millions of hands are needed for statistically meaningful evaluation. This makes RL training expensive.
- **Imperfect information makes single-decision verification impossible**: There is no way to verify that a poker fold was "correct" without knowing the opponent's cards — and even then, the decision depends on the opponent's strategy. The best we can do is measure aggregate performance.
- **Opponent modeling confound**: Optimal play depends on the opponent. A strategy that exploits weak opponents may be terrible against strong ones. GTO play is a safe baseline but not maximally exploitative.
- **Data privacy/legality**: Poker hand history distribution exists in a legal gray area. Some sites prohibit sharing.
- **Bridge partnership complexity**: Bridge is inherently a partnership game. An agent's bidding quality depends on its partner's system understanding, creating a chicken-and-egg problem.
- **Partial verifiability only**: We can verify that play is legal, compute outcomes, and compare to GTO/solved baselines in simple variants. But for complex real-world poker and bridge, verification of individual decisions remains fundamentally limited.

## Connections

- [[chess]] — Both are classic game AI domains, but chess has perfect information making verification far cleaner
- [[board-games]] — Card games extend the game-playing paradigm to imperfect information
- [[interactive-fiction]] — Both involve reasoning under partial observability
- [[calendar-scheduling]] — Both involve constraint satisfaction under uncertainty
