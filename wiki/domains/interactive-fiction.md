---
domain: interactive-fiction
category: games/text-games
verification_type: outcome | rule
dataset_scale: >100 games (Jericho); unlimited (TextWorld)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Interactive Fiction

## Overview

Interactive fiction (IF) — text adventure games — requires an agent to read textual descriptions of a game world, issue text commands to interact with it, and achieve goals specified by the game. Classic games like Zork, Hitchhiker's Guide, and Anchorhead present rich natural language understanding challenges: parsing room descriptions, tracking inventory, solving puzzles, and navigating complex worlds. For RLVR, interactive fiction is uniquely valuable because it is a pure-text environment where verification comes from the game engine itself (score, win condition, game state). This makes it a natural fit for training language models to act as agents in text-based worlds.

## Verification Mechanism

1. **Game score** (outcome-based): Most IF games have a numeric score that increases as the player achieves milestones (finding treasures, solving puzzles, progressing in the story). The interpreter reports the score after each action. Standard metric: normalized score = agent_score / max_possible_score.
2. **Win/completion detection** (outcome-based): The game engine signals when the game is won or lost. Binary reward for completion.
3. **Game state inspection** (rule-based): The Jericho framework exposes structured game state: current room, inventory, world object states. These can be queried programmatically. Verification: check that specific objects are in specific locations, doors are open/closed, etc.
4. **Command validity** (rule-based): Jericho's valid action detection identifies which commands the game engine will accept in the current state. The set of "valid actions" is a well-defined concept.
5. **TextWorld goal checking** (constraint-based): In Microsoft TextWorld, games are generated with explicit goal specifications (e.g., "put the red ball on the table in the kitchen"). The engine checks goal satisfaction directly.
6. **Step count / efficiency** (constraint-based): Many games can be won in a minimum number of moves (known as "speed runs"). Verification: game won AND step_count <= threshold.

## Dataset Sources

- **Jericho**: https://github.com/microsoft/jericho — Framework for playing Z-Machine interactive fiction games. Supports 50+ classic games (Zork I-III, Hitchhiker's Guide, Anchorhead, etc.) with score extraction, valid action detection, and game state inspection.
- **TextWorld**: https://github.com/microsoft/TextWorld — Microsoft's procedural text game generator. Can create unlimited games with specified complexity: number of rooms, objects, puzzles, recipe length. Full control over difficulty.
- **IFDB (Interactive Fiction Database)**: https://ifdb.org/ — Catalog of thousands of IF games. Many are Z-Machine (.z5/.z8) format compatible with Jericho.
- **IF Archive**: https://www.ifarchive.org/ — Massive collection of free interactive fiction games, source code, and tools.
- **ScienceWorld**: https://scienceworld.github.io/ — Text-based science simulation environment. 30 tasks testing scientific reasoning (e.g., boil water, grow a plant, conduct experiments). Scored 0-100 based on task completion.
- **ALFWorld**: https://alfworld.github.io/ — Text-based version of ALFRED household tasks. 134 task types in 6 categories. Verification: goal object in goal location.
- **LIGHT**: https://parl.ai/projects/light/ — Facebook's text adventure dialogue platform set in a fantasy world.
- **ClubFloyd transcripts**: https://ifarchive.org/indexes/if-archive/rec/clubfloyd/ — Human playthroughs of IF games with annotations.

## Task Format

**Standard game play**:
- Input: Text observation from the game engine (room description, inventory changes, NPC dialogue, score updates) + task/game description.
- Output: Text command (e.g., "go north", "pick up the brass lantern", "unlock door with key", "examine painting").
- Verification: Game score after episode, or binary win/loss.

**Goal-directed play (TextWorld)**:
- Input: Initial room description + explicit goal (e.g., "Prepare a meal: dice the red potato, fry the diced red potato, eat the fried diced red potato").
- Output: Sequence of text commands.
- Verification: Goal state achieved (checked by TextWorld engine).

**ScienceWorld tasks**:
- Input: Environment description + science task (e.g., "Determine the boiling point of water").
- Output: Action sequence (navigate rooms, pick up items, use equipment).
- Verification: Numeric score (0-100) based on partial task completion and final answer.

## Difficulty Curriculum

1. **TextWorld coin collector** (trivial): Navigate rooms to find a coin. 1-5 rooms. Only navigation commands needed.
2. **TextWorld simple recipes** (easy): Find ingredients, process them, eat them. 1-3 step recipes.
3. **ALFWorld tasks** (easy-medium): Pick up and place objects in household settings. Clear instructions.
4. **TextWorld complex recipes** (medium): Multi-step recipes requiring navigation across many rooms. 5-10 rooms, 3-5 step recipes.
5. **ScienceWorld tasks** (medium-hard): Require scientific reasoning + multi-step interaction.
6. **Classic IF: Zork I** (hard): Large world, diverse puzzles, cryptic hints, minimal instruction. 350-point game.
7. **Classic IF: Hitchhiker's Guide** (very hard): Notoriously unfair puzzles, timed events, instant-death traps.
8. **Classic IF: Anchorhead** (very hard): Horror text adventure with complex multi-day puzzles and item management.
9. **Randomly generated hard TextWorld** (variable): 20+ rooms, 10+ objects, long dependency chains.

## Limitations & Risks

- **Parser limitations**: Classic IF games have limited text parsers. The agent must learn the parser's vocabulary, which is game-specific and often frustrating. Commands like "put lantern in bag" work but "place lantern inside bag" may not.
- **Valid action space is unknown**: Unlike board games with enumerable moves, IF requires generating free-form text commands. The set of useful commands is unknown a priori (though Jericho provides valid-action detection for some games).
- **Reward sparsity**: Many classic IF games have sparse scoring milestones separated by long sequences of unrewarded exploration. This makes RL training very challenging.
- **Game-specific knowledge**: Each classic IF game has its own world, rules, and puzzles. Transfer learning between games is limited.
- **Verification depth**: Score-based verification only measures progress, not quality of play. An agent might stumble into points through random exploration rather than reasoned play.
- **Dead ends**: Many classic IF games have unwinnable states (e.g., used a required item in the wrong way). The agent may not realize it is stuck, wasting many actions.
- **Scale ceiling**: There are only ~50 games in the Jericho suite. TextWorld can generate unlimited games but they tend to be formulaic. The diversity of real IF is hard to replicate procedurally.

## Connections

- [[game-playing-atari]] — Both are classic RL game environments; IF is the text counterpart to Atari's visual games
- [[web-navigation]] — Both involve navigating through an interactive environment by issuing commands
- [[card-games]] — Both involve partial observability and reasoning under uncertainty
- [[puzzle-games]] — Classic IF puzzles share the combinatorial reasoning aspect
