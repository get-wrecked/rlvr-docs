---
domain: Medical Diagnosis & Clinical Reasoning
category: domain-expert
verification_type: exact_match
dataset_scale: 100K+ (from medical QA benchmarks)
difficulty_range: medium/hard/superhuman
modality: multimodal
status: verified
---

# Medical Diagnosis & Clinical Reasoning

## Overview
Answer medical questions, diagnose conditions from symptoms/labs/imaging, and recommend treatments — with verifiable correct answers from medical board exams and curated clinical datasets. Verification: match against expert-validated gold standard diagnoses and treatments.

## Verification Mechanism
```python
def verify(task_type: str, question: str, answer: str, gold: str) -> float:
    if task_type == "multiple_choice":
        # Medical board exam format
        return 1.0 if answer.strip().upper() == gold.strip().upper() else 0.0
    
    elif task_type == "diagnosis":
        # Check if diagnosis matches (may accept multiple valid diagnoses)
        pred_codes = normalize_icd_codes(answer)
        gold_codes = normalize_icd_codes(gold)
        return 1.0 if pred_codes & gold_codes else 0.0
    
    elif task_type == "lab_interpretation":
        # Interpret lab values (high/normal/low + clinical significance)
        pred_interpretation = parse_interpretation(answer)
        gold_interpretation = parse_interpretation(gold)
        return interpretation_match(pred_interpretation, gold_interpretation)
    
    elif task_type == "imaging":
        # Identify findings in medical images
        pred_findings = set(answer["findings"])
        gold_findings = set(gold["findings"])
        return set_f1(pred_findings, gold_findings)
```

## Dataset Sources
- **USMLE Questions**: Thousands of board exam questions with explained answers.
- **MedQA**: 61K medical QA from board exams (US, China, Taiwan).
- **PubMedQA**: 270K QA from biomedical literature.
- **MedMCQA**: 194K MCQs from Indian medical entrance exams.
- **MMLU (medical subset)**: Professional medicine, clinical knowledge.
- **CheXpert**: 224K chest X-rays with findings labels.
- **MIMIC-III/IV**: Critical care records (with de-identification).
- **PathVQA**: 32K pathology image QA.
- **PMC-VQA**: 227K medical image QA from PubMed Central.
- **DDxPlus**: Differential diagnosis dataset.
- **JAMA Clinical Challenge**: Clinical reasoning cases.

## Task Format
- **Input**: "A 55-year-old male presents with crushing chest pain, diaphoresis, and shortness of breath. ECG shows ST elevation in leads II, III, and aVF. What is the most likely diagnosis?"
- **Output**: "Inferior ST-Elevation Myocardial Infarction (STEMI)"

## Difficulty Curriculum
- Level 1: Common conditions, textbook presentations
- Level 3: Atypical presentations, differential diagnosis
- Level 5: Complex multi-system cases, lab interpretation
- Level 7: Rare diseases, drug interactions, treatment algorithms
- Level 9: Board exam edge cases, novel clinical reasoning

## Limitations & Risks
- Medical questions have gold standard answers from expert consensus. However, medicine is not always clear-cut — accept multiple valid diagnoses when appropriate.
- This is NOT for clinical use — it's for developing general reasoning ability on medical knowledge.
- Imaging tasks require vision capability.
- Board exam format (MCQ) is well-verified. Open-ended diagnosis is harder.

## Connections
- [[question-answering-closed]] — QA format
- [[bayesian-network-reasoning]] — diagnostic reasoning
- [[scientific-paper-extraction]] — medical literature extraction
- [[visual-question-answering]] — medical image QA
