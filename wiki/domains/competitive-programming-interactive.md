---
domain: competitive-programming-interactive
category: code/competitive
verification_type: execution
dataset_scale: >100K problems; unlimited test cases
difficulty_range: medium/hard/superhuman
modality: text
status: verified
---

# Competitive Programming (Interactive / Online Judge)

## Overview

Competitive programming tasks require writing code to solve well-specified algorithmic problems, verified by an online judge system that tests the code against hidden test cases. This is a gold-standard RLVR domain: verification is fully automated, perfectly reliable (the code either passes all tests or it does not), and scales to superhuman difficulty. The domain includes both standard (batch) problems and interactive problems where the program must communicate with a judge through queries. Online judges like Codeforces, AtCoder, LeetCode, and USACO provide enormous problem archives with official test suites. This is arguably the highest-quality RLVR signal available for reasoning-heavy tasks.

## Verification Mechanism

1. **Online judge execution** (execution-based): Submit code to the judge. Judge compiles the code, runs it on all test cases (typically 10-100+), and verifies output matches expected output. Returns: Accepted (AC), Wrong Answer (WA), Time Limit Exceeded (TLE), Memory Limit Exceeded (MLE), Runtime Error (RE), Compilation Error (CE). Binary reward: AC = 1, anything else = 0.
2. **Partial scoring** (execution-based): Some contests (IOI-style, Google Code Jam) award partial credit for passing a subset of test cases. Reward = fraction of test cases passed, or points based on subtask completion.
3. **Custom checkers / special judges** (execution-based): For problems with multiple valid outputs (e.g., "find any valid coloring"), a custom checker program validates the output against problem constraints rather than exact matching. Used extensively in Codeforces problems marked with "special judge."
4. **Interactive verification** (execution-based): For interactive problems, the judge provides a program (interactor) that communicates with the contestant's program through stdin/stdout. The agent's program asks queries and the interactor responds. Verification: agent finds the correct answer within the query limit.
5. **Stress testing** (execution-based): Generate random test cases, run both the agent's solution and a known-correct brute-force solution, compare outputs. Effective for finding edge case failures.

## Dataset Sources

- **Codeforces**: https://codeforces.com/ — >9,000 problems across difficulty ratings 800-3500. Full problem statements, editorial solutions, test cases (for past contests). Excellent tagging system (dp, graphs, greedy, math, interactive, etc.). API available for problem metadata.
- **AtCoder**: https://atcoder.jp/ — >5,000 problems. Known for high-quality problem statements and clean test cases. ABC (beginner), ARC (regular), AGC (grand) contests provide natural difficulty tiers.
- **LeetCode**: https://leetcode.com/ — >3,000 problems. More focused on interview prep. Easy/Medium/Hard difficulty ratings. API available.
- **USACO**: http://usaco.org/ — USA Computing Olympiad. Problems from monthly contests and camp. Four divisions: Bronze, Silver, Gold, Platinum. Very high quality.
- **Project Euler**: https://projecteuler.net/ — >900 math-heavy programming problems. Answer is a single number. Verification: exact match.
- **DMOJ**: https://dmoj.ca/ — Open-source judge with >4,000 problems. Test data downloadable.
- **Kattis**: https://open.kattis.com/ — >3,000 problems from ICPC and other contests.
- **APPS**: https://github.com/hendrycks/apps — 10,000 coding problems collected from open judge platforms. Introductory, interview, and competition levels.
- **CodeContests (DeepMind)**: https://github.com/google-deepmind/code_contests — Curated Codeforces/AtCoder problems with test cases, solutions, and metadata. Used for AlphaCode training.
- **TACO**: Competitive programming benchmark with test case generation.
- **LiveCodeBench**: https://livecodebench.github.io/ — Continuously updated benchmark using problems published after LLM training cutoffs.

## Task Format

**Standard problem solving**:
- Input: Problem statement (natural language description with constraints, examples, and I/O format) + programming language specification.
- Output: Complete program that reads from stdin, solves the problem, writes to stdout.
- Verification: Judge runs program on all test cases. All must pass.

**Interactive problem solving**:
- Input: Interactive problem statement (describes the hidden data, allowed queries, and query limit).
- Output: Program that alternates between printing queries and reading responses, eventually outputting the answer.
- Verification: Interactor validates queries are legal, responses are correct, and the answer is found within the query limit.

**Function completion**:
- Input: Function signature + docstring/problem description + test examples. LeetCode style.
- Output: Function implementation.
- Verification: Run test suite. All tests pass.

**Optimization problems**:
- Input: Problem with scoring function (e.g., "minimize the total cost of...").
- Output: Valid solution.
- Verification: Solution is valid + score is computed. Reward proportional to score quality.

## Difficulty Curriculum

1. **LeetCode Easy / Codeforces 800** (easy): Basic loops, conditionals, simple string/array manipulation.
2. **LeetCode Medium / CF 1000-1200** (easy-medium): Two pointers, basic BFS/DFS, simple DP.
3. **CF 1200-1600 / AtCoder ABC D-E** (medium): Standard algorithms: sorting, binary search, basic graph algorithms, moderate DP.
4. **CF 1600-1900 / USACO Silver** (medium-hard): Non-trivial DP, segment trees, shortest paths. Requires problem reduction skills.
5. **CF 1900-2200 / USACO Gold** (hard): Advanced DP, flow algorithms, complex graph problems. Requires creative insight.
6. **CF 2200-2500 / USACO Platinum** (very hard): Combines multiple advanced techniques. Mathematical insight often required.
7. **CF 2500-3000 / AtCoder AGC** (extremely hard): Problems requiring novel mathematical observations or obscure algorithmic techniques.
8. **CF 3000+ / IOI/ICPC World Finals** (superhuman): Top competitive programmers spend 2-5 hours on these. Multiple algorithmic insights chained together.
9. **Interactive problems** (variable, often hard+): Add communication complexity and query optimization to standard algorithmic challenges.

## Limitations & Risks

- **Test case coverage**: Even well-designed test suites can miss edge cases. A solution might pass all tests but be technically incorrect on an untested input. This is rare for well-maintained judges but not impossible.
- **Time/space optimization vs correctness**: The binary AC/TLE distinction means a correct but slow algorithm gets 0 reward, same as a completely wrong answer. This conflates algorithmic efficiency with correctness.
- **Code generation, not reasoning**: RLVR reward for passing tests does not directly reward the reasoning process. A model might generate code through pattern matching rather than understanding the algorithm. It could also overfit to common contest patterns.
- **Language-specific issues**: Compilation errors, language-specific edge cases (integer overflow in C++ but not Python), and runtime performance differences add noise unrelated to algorithmic understanding.
- **Problem statement understanding**: Competitive programming problems often have dense, precise specifications. Misreading a constraint can lead to completely wrong approaches. This tests reading comprehension as much as algorithms.
- **Memorization risk**: Many problems and their solutions are publicly available. LLMs may have seen solutions during pretraining. LiveCodeBench addresses this by using only post-cutoff problems.
- **Interactive problem simulation**: Testing interactive solutions locally requires implementing the interactor, which adds engineering complexity.

## Connections

- [[puzzle-games]] — Both require search and algorithmic thinking to find solutions
- [[chess]] — Both involve adversarial reasoning (interactive problems) and combinatorial search
- [[spreadsheet-tasks]] — Both require precise computational implementations
- [[file-system-tasks]] — Both are code-execution verified domains
