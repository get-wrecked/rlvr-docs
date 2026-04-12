---
domain: Static Analysis Rule Writing
category: Code & Software
verification_type: execution
dataset_scale: ~2K+ Semgrep rules with test cases + CodeQL suites
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Static Analysis Rule Writing

## Overview
Write static analysis rules (Semgrep rules, CodeQL queries, ESLint rules) that detect specific code patterns. Verification via precision/recall on labeled code corpora.

## Verification Mechanism
1. Rule parses without errors
2. Run against positive examples (should match) — measure recall
3. Run against negative examples (should not match) — measure precision
4. Combined F1 score as reward

## Dataset Sources
- Semgrep rule registry (2K+ community rules with test cases)
- CodeQL query suites
- ESLint rule test fixtures
- PMD rule tests

## Task Format
**Input**: Description of code pattern to detect + positive/negative examples
**Output**: Semgrep YAML rule, CodeQL query, or ESLint rule

## Difficulty Curriculum
1. Simple pattern matching (function name, import)
2. Taint tracking (user input to dangerous sink)
3. Control flow analysis (null check before dereference)
4. Data flow across function boundaries
5. Complex inter-procedural analysis

## Connections
[[code-vulnerability-detection]], [[regex-synthesis]], [[code-review-automation]]
