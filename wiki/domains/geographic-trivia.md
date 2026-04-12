---
domain: Geographic & World Knowledge QA
category: knowledge
verification_type: exact_match
dataset_scale: 10M+ (from encyclopedias + databases)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Geographic & World Knowledge QA

## Overview
Answer verifiable factual questions about geography, history, politics, culture, sports, entertainment — any domain with objectively correct answers. This is the broadest QA category, covering everything from "What is the capital of France?" to "Which country has the longest coastline?"

## Verification Mechanism
```python
def verify(question: str, answer: str, gold_answers: list[str]) -> float:
    normalized = normalize(answer)
    for gold in gold_answers:
        if exact_match(normalized, normalize(gold)):
            return 1.0
        if f1_token_match(normalized, normalize(gold)) > 0.8:
            return 0.8
    return 0.0
```

## Dataset Sources
- **TriviaQA**: 95K trivia questions with evidence documents.
- **Natural Questions**: 300K Google search questions with Wikipedia answers.
- **WebQuestions**: 6.6K questions with Freebase answers.
- **Jeopardy! archive**: 200K+ clue-answer pairs.
- **Quiz Bowl**: 100K+ academic competition questions.
- **GeoQuiz datasets**: Geographic knowledge quizzes.
- **FreebaseQA**: 28K QA from Freebase.
- **SimpleQuestions**: 100K simple factoid questions.
- **CountryData**: Structured data for all countries.
- **Wikidata-based QA**: Unlimited from Wikidata queries.

## Task Format
- **Input**: "What is the deepest point in the ocean?"
- **Output**: "Challenger Deep" or "Mariana Trench"

## Difficulty Curriculum
- Level 1: Common knowledge (capitals, largest countries)
- Level 3: Specific factual knowledge (populations, dates, records)
- Level 5: Multi-hop factual reasoning
- Level 7: Obscure but verifiable facts
- Level 9: Questions requiring synthesis of multiple facts

## Connections
- [[question-answering-closed]] — formal QA
- [[reading-comprehension]] — extractive QA
- [[geospatial-analysis]] — geographic computation
