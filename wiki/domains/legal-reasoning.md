---
domain: Legal Reasoning & Statutory Interpretation
category: domain-expert
verification_type: exact_match
dataset_scale: 100K+ (from bar exams + legal benchmarks)
difficulty_range: medium/hard
modality: text
status: verified
---

# Legal Reasoning & Statutory Interpretation

## Overview
Answer legal questions: apply rules to fact patterns, interpret statutes, identify relevant precedents, analyze contracts. Verified against bar exam gold standards and expert-annotated legal benchmarks.

## Verification Mechanism
```python
def verify(task_type: str, question: str, answer: str, gold: str) -> float:
    if task_type == "bar_exam_mcq":
        return 1.0 if answer.strip().upper() == gold.strip().upper() else 0.0
    
    elif task_type == "contract_analysis":
        # Extract specific clause answers
        pred = parse_legal_answers(answer)
        gold_parsed = parse_legal_answers(gold)
        return answer_f1(pred, gold_parsed)
    
    elif task_type == "statutory_interpretation":
        return 1.0 if normalize_legal(answer) == normalize_legal(gold) else 0.0
    
    elif task_type == "case_outcome_prediction":
        pred_outcome = answer.strip().lower()
        gold_outcome = gold.strip().lower()
        return 1.0 if pred_outcome == gold_outcome else 0.0
```

## Dataset Sources
- **LegalBench**: 162 legal reasoning tasks with gold labels.
- **Bar Exam (MBE/MEE)**: Thousands of Multistate Bar Exam questions.
- **LSAT**: Analytical reasoning + logical reasoning sections.
- **CaseHOLD**: 53K legal holdings from case law.
- **CUAD**: 13K contract understanding annotations (510 contracts).
- **EUR-Lex**: European legal documents with classifications.
- **LexGLUE**: Legal NLU benchmark (7 datasets).
- **SARA**: Statutory reasoning benchmark (tax code).
- **Contract NLI**: 10K contract-hypothesis pairs.
- **Overruling**: Case overruling detection.

## Task Format
- **Input**: "A landowner conveys property to 'B and his heirs, but if alcohol is ever consumed on the premises, then to C.' What estate does B have?"
- **Output**: "Fee simple subject to executory limitation"

## Difficulty Curriculum
- Level 1: Basic rule application (criminal intent, contract formation)
- Level 3: Multi-factor tests, balancing (negligence, First Amendment)
- Level 5: Statutory interpretation with ambiguity
- Level 7: Novel fact patterns requiring analogical reasoning
- Level 9: Constitutional interpretation, conflicting precedents

## Limitations & Risks
- Legal answers can be contested — bar exam has "best answer" but real law is often ambiguous.
- Common law varies by jurisdiction. Focus on general principles or specify jurisdiction.
- MCQ format has strong verification. Open-ended legal analysis is harder to verify.

## Connections
- [[legal-logic]] — formal legal reasoning
- [[natural-language-inference]] — legal entailment
- [[reading-comprehension]] — legal document comprehension
- [[fact-verification]] — legal fact-checking
