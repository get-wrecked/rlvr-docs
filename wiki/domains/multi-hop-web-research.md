---
domain: Multi-Hop Web Research
category: agent
verification_type: exact_match
dataset_scale: 50K+ (from QA benchmarks + web snapshots)
difficulty_range: medium/hard
modality: multimodal
status: strong_hypothesis
---

# Multi-Hop Web Research

## Overview
Answer complex questions that require finding and synthesizing information from multiple web pages. The agent must navigate between pages, extract relevant information, and combine it to answer. Verification: match against known correct answers derived from the source pages.

## Verification Mechanism
```python
def verify(question: str, agent_answer: str, gold_answers: list[str]) -> float:
    # Normalize and compare
    norm_answer = normalize(agent_answer)
    for gold in gold_answers:
        if exact_match(norm_answer, normalize(gold)):
            return 1.0
        if f1_score(norm_answer, normalize(gold)) > 0.8:
            return 0.8  # Partial credit for close matches
    return 0.0
```

## Dataset Sources
- **HotpotQA**: 113K multi-hop questions requiring 2+ Wikipedia pages.
- **MuSiQue**: 25K multi-hop questions requiring 2-4 hops.
- **2WikiMultiHopQA**: 193K questions across Wikipedia.
- **StrategyQA**: 2.7K yes/no questions requiring implicit multi-step reasoning.
- **FEVER**: Fact verification requiring evidence retrieval.
- **WebArena**: Real web browsing tasks (harder, more agentic).
- **Bamboogle**: Multi-step questions designed to require search.
- **Cached web snapshots**: Pin specific versions for reproducibility.

## Task Format
- **Input**: "What is the capital of the country that won the most medals in the 2020 Olympics? When was that city founded?"
- **Output**: "Tokyo, Japan won the most medals (host country). Tokyo was founded in 1457."

Or in agentic form:
- **Input**: Question + browser environment
- **Output**: Agent navigates web pages, extracts info, synthesizes answer

## Difficulty Curriculum
- Level 1: 2-hop questions (entity → property → answer)
- Level 3: 3-hop questions with comparison
- Level 5: Questions requiring synthesis across 4+ sources
- Level 7: Questions requiring temporal reasoning + multi-source
- Level 9: Open-ended research questions with verifiable factual answers

## Limitations & Risks
- Web content changes over time. Must use cached/pinned snapshots.
- Some multi-hop questions have debatable answers. Use questions with unambiguous gold answers.
- F1 scoring for partial credit is imperfect.

## Connections
- [[web-navigation]] — browser-based information seeking
- [[reading-comprehension]] — multi-hop comprehension
- [[question-answering-extractive]] — single-hop QA
- [[fact-verification]] — verifying claims with evidence
