---
domain: Educational Content Generation with Assessment
category: education
verification_type: constraint
dataset_scale: 1M+ (from educational platforms)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Educational Content Generation with Assessment

## Overview
Generate educational content (quiz questions, explanations, worked examples) where correctness is verifiable: the generated questions have correct answers, the worked examples are mathematically sound, and the difficulty matches the specified level.

## Verification Mechanism
```python
def verify(task_type: str, content: dict) -> float:
    if task_type == "generate_quiz":
        score = 0
        checks = 0
        
        for question in content["questions"]:
            # Verify correct answer is actually correct
            checks += 1
            if verify_answer(question["question_text"], question["correct_answer"]):
                score += 1
            
            # Verify distractors are actually wrong
            for distractor in question["distractors"]:
                checks += 1
                if not verify_answer(question["question_text"], distractor):
                    score += 1  # Good: distractor is wrong
                # If distractor is correct, that's bad (ambiguous question)
            
            # Check difficulty level
            checks += 1
            estimated_diff = estimate_difficulty(question)
            if abs(estimated_diff - content["target_difficulty"]) < 0.2:
                score += 1
        
        return score / checks
    
    elif task_type == "worked_example":
        # Verify each step in the worked example
        steps = content["steps"]
        for i, step in enumerate(steps):
            if step.get("computation"):
                if not verify_computation(step["computation"]):
                    return i / len(steps)  # Partial credit up to error
        return 1.0
```

## Dataset Sources
- **Khan Academy exercises**: Structured exercises with correct answers and difficulty levels.
- **OpenStax textbooks**: Open educational resources.
- **RACE dataset**: Reading comprehension from English exams.
- **ARC (AI2 Reasoning Challenge)**: Science questions.
- **SciQ**: Science questions with supporting evidence.
- **MathQA**: Math questions from standardized tests.
- **Exam archives**: SAT, GRE, AP, IB past papers.

## Task Format
- **Input**: "Generate 5 multiple-choice questions about quadratic equations at the high school level, each with 4 options"
- **Output**: List of questions with correct answers and distractors

## Difficulty Curriculum
- Level 1: Generate simple fact-recall questions
- Level 3: Generate application questions with worked solutions
- Level 5: Generate multi-concept questions with plausible distractors
- Level 7: Generate questions that test common misconceptions
- Level 9: Generate assessment items that discriminate between skill levels (IRT-calibrated)

## Limitations & Risks
- Quality of distractors is hard to verify fully — a distractor might be "wrong" but not plausible.
- Difficulty estimation is approximate. Calibrate against existing item banks.
- The answer verification is the reliable component; the pedagogical quality is harder to verify.

## Connections
- [[math-numerical]] — math content generation
- [[question-answering-closed]] — QA as assessment
- [[constrained-writing]] — generating text with constraints
