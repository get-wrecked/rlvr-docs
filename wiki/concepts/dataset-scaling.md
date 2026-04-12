---
concept: Dataset Scaling
description: How to scale RLVR datasets from internet sources to unlimited procedural generation
---

# Dataset Scaling for RLVR

## Three Tiers of Data Sourcing

### Tier A: Curated Internet Datasets
Pre-existing, labeled datasets from the internet. Fixed size but high quality.

**Examples**:
- MATH dataset: 12.5K competition math problems
- HumanEval: 164 coding problems
- Spider: 10K text-to-SQL pairs
- SQuAD: 100K+ QA pairs
- Lichess puzzle database: 3M+ chess puzzles

**Total estimated**: ~500M problem-solution pairs across all domains

### Tier B: Internet Mining
Extract problem-solution pairs from raw internet data at scale.

**Sources**:
- **StackOverflow**: 23M+ questions with accepted answers. Mine for: code problems (test extraction), regex, SQL, shell commands, debugging.
- **Wikipedia + Wikidata**: Billions of factual claims → QA pairs. Entity relationships → knowledge graph tasks.
- **GitHub**: 100M+ repos. Extract: function-docstring pairs, test-function pairs, commit-diff pairs. Each is a potential RLVR task.
- **arXiv**: 2M+ papers. Extract: theorem-proof pairs, mathematical claims.
- **Project Euler / Competitive Programming Archives**: 100K+ problems with test cases.
- **Reddit (r/math, r/learnprogramming, etc.)**: Questions with verifiable answers.
- **Textbook collections**: Physics, chemistry, engineering with known solutions. OpenStax, MIT OCW.
- **Khan Academy exercise corpus**: Structured problems with exact answers.
- **Kaggle**: 50K+ datasets with competition solutions (data science tasks).
- **LeetCode/HackerRank/Codeforces**: Millions of problem-solution-test-case triples.
- **Recipe sites** (AllRecipes, etc.): Structured data for conversion tasks.
- **Legal databases** (PACER, etc.): Case law for rule application.
- **Patent databases**: USPTO full text for information extraction.
- **Scientific databases**: PDB, UniProt, ChEMBL, Materials Project — structured data for science tasks.
- **Government open data**: Census, weather, economic — structured data for analysis tasks.

**Estimated total**: 1-10B extractable problem-solution pairs

### Tier C: Procedural Generation
Algorithmically generate unlimited training instances.

**Methods**:
- **Math**: Sample random equations, compute solutions symbolically. Generate geometry problems from random configurations. Sample theorem statements and attempt automated proof.
- **Code**: Generate random programs, extract functions, use existing tests as spec. Mutate correct code to create repair tasks. Cross-compile to create translation tasks.
- **Logic**: Random SAT/CSP instance generation with controlled difficulty. Sudoku/puzzle generation with unique-solution guarantees.
- **Science**: Sample physical scenarios, simulate to get ground truth. Generate circuit netlists, simulate to get behavior.
- **Games**: Self-play generates unlimited game states. Procedural puzzle generation.
- **Constrained writing**: Sample random constraint sets from templates.
- **Format tasks**: Convert randomly sampled data between formats.
- **Web/GUI**: Procedurally generated web pages with tasks. Synthetic app environments.

**Estimated total**: Effectively unlimited

## Difficulty Control

For procedural generation, controlling difficulty is critical:

### Math
- Number of steps to solution
- Number/type of operations required
- Size of numbers involved
- Whether the answer is integer vs. rational vs. irrational

### Code
- Function complexity (LOC, cyclomatic complexity)
- Number of test cases
- Edge case coverage
- Algorithmic difficulty (brute force → dynamic programming → advanced data structures)

### Logic
- Number of variables/clauses (SAT)
- Grid size (puzzles)
- Number of constraints
- Proximity to phase transition (for randomly generated instances)

### Science
- Number of interacting bodies/components
- Dimensionality
- Nonlinearity
- Whether closed-form solution exists

### Games
- Board complexity
- Opponent strength (self-play difficulty)
- Puzzle move depth

## Data Quality Filters

Not all internet data is suitable for RLVR. Apply:

1. **Verification filter**: Only keep pairs where the verification function confirms the answer is correct.
2. **Difficulty filter**: Remove trivially easy problems (no training signal) and problems beyond reasonable difficulty.
3. **Duplication filter**: Deduplicate to prevent memorization.
4. **Contamination filter**: Remove problems that appear in standard benchmarks.
5. **Diversity filter**: Ensure broad coverage across sub-topics within each domain.

## Scaling Law Implications

More domains × more difficulty levels × procedural generation = effectively unlimited verifiable training data.

The constraint is NOT data. It is:
1. Compute for running verification functions
2. Sandbox infrastructure for execution-based verification
3. Engineering effort to build domain-specific verifiers
4. Curriculum design across domains
