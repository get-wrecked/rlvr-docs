---
domain: Competitive Programming (Interactive & Advanced)
category: code
verification_type: execution
dataset_scale: 500K+ (Codeforces, AtCoder, USACO archives)
difficulty_range: medium/hard/superhuman
modality: text
status: verified
---

# Competitive Programming (Interactive & Advanced)

## Overview
Beyond standard competitive programming (covered in code-generation), this covers: interactive problems (where the program must query an oracle), multi-step constructive problems, optimization problems with scoring, and marathon-style competitive programming. These require planning, strategy, and adaptive reasoning — not just algorithm implementation.

## Verification Mechanism
```python
def verify(problem: CompetitiveProblem, code: str) -> float:
    if problem.type == "interactive":
        # Run code as subprocess, mediate interaction with judge
        process = run_sandboxed(code)
        judge = InteractiveJudge(problem)
        while not judge.done:
            query = process.read_output()
            response = judge.handle_query(query)
            process.write_input(response)
        return 1.0 if judge.accepted else 0.0
    
    elif problem.type == "optimization":
        # Run code, compute score
        output = execute_in_sandbox(code, problem.input, timeout=problem.time_limit)
        if not problem.validate(output):
            return 0.0
        score = problem.compute_score(output)
        # Normalize against known best score
        return min(1.0, score / problem.best_known_score)
    
    elif problem.type == "standard":
        # Standard: run on all test cases
        passed = 0
        for test_input, test_output in problem.test_cases:
            actual = execute_in_sandbox(code, test_input, timeout=problem.time_limit)
            if actual.strip() == test_output.strip():
                passed += 1
        return passed / len(problem.test_cases)
```

## Dataset Sources
- **Codeforces**: 10K+ problems with editorial solutions, test cases, and judges. API available.
- **AtCoder**: 5K+ problems, particularly strong for interactive/constructive problems.
- **USACO**: Training pages + competition archives. Excellent difficulty progression.
- **Google Code Jam / Kick Start / Hash Code**: Archive of competition problems.
- **Facebook Hacker Cup**: Archive available.
- **ICPC problem archive**: International Collegiate Programming Contest problems.
- **LeetCode**: 3K+ problems with test cases.
- **Project Euler**: 800+ math-algorithmic problems.
- **DMOJ / Kattis**: Online judges with 1K+ problems each.
- **CodeContests** (DeepMind): Curated dataset of competitive programming problems with tests.
- **LiveCodeBench**: Temporal benchmark to avoid contamination.

## Task Format
- **Input**: Problem statement (natural language + examples + constraints)
- **Output**: Complete solution code (C++/Python/Java)

## Difficulty Curriculum
- Level 1: Implementation problems (Codeforces A/B, ~800 rating)
- Level 3: Standard algorithms (sorting, BFS/DFS, binary search) (~1200 rating)
- Level 5: Dynamic programming, graph algorithms, data structures (~1600 rating)
- Level 7: Advanced algorithms (flows, FFT, suffix arrays) (~2000 rating)
- Level 9: Research-level algorithmic problems (~2400+ rating)
- Level 10: Unsolved competitive programming problems

## Limitations & Risks
- Test cases may not cover all edge cases. Codeforces system tests are thorough but not exhaustive.
- Interactive problems require special infrastructure (process communication, judge simulation).
- Optimization problems have subjective scoring (what fraction of optimal is "good enough"?).
- Time limits are language-dependent (C++ gets 2s, Python gets 6s, etc.) — affects which solutions are "correct."

## Connections
- [[code-generation]] — basic code generation
- [[code-optimization]] — performance matters in competitive programming
- [[combinatorics-optimization]] — many problems involve combinatorial optimization
- [[graph-theory]] — graph algorithms are a major component
