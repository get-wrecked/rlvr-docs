---
domain: Medical Coding
category: medical-nlp
verification_type: exact_match
dataset_scale: 50K+ notes (MIMIC-III)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Medical Coding

## Overview

Medical coding assigns standardized diagnostic and procedural codes (ICD-10-CM, ICD-10-PCS, CPT) to clinical documentation: discharge summaries, operative reports, and encounter notes. Given a clinical note describing a patient's conditions and treatments, the model must identify all applicable codes from a vocabulary of ~70,000 ICD-10 codes.

RLVR verification is exact: predicted code sets are compared against gold-standard codes assigned by certified medical coders. Micro-F1 and macro-F1 are the standard metrics, and the task is well-suited to automated evaluation. Medical coding is a high-value commercial application -- it drives hospital reimbursement and is currently performed by ~300,000 human coders in the US alone.

## Verification Mechanism

```python
def verify_medical_coding(predicted_codes: set, gold_codes: set) -> dict:
    """
    predicted_codes: {"E11.9", "I10", "Z79.4", ...} (ICD-10 codes)
    gold_codes: gold standard code set
    """
    tp = len(predicted_codes & gold_codes)
    fp = len(predicted_codes - gold_codes)
    fn = len(gold_codes - predicted_codes)

    precision = tp / (tp + fp) if (tp + fp) > 0 else 0.0
    recall = tp / (tp + fn) if (tp + fn) > 0 else 0.0
    f1 = 2 * precision * recall / (precision + recall) if (precision + recall) > 0 else 0.0

    # Exact match (all codes correct, no extras)
    exact = predicted_codes == gold_codes

    # Hierarchical partial credit: parent code match
    # E11.9 and E11.65 share parent E11 (Type 2 diabetes)
    hierarchical_score = compute_hierarchical_f1(predicted_codes, gold_codes)

    return {
        "micro_f1": f1,
        "precision": precision,
        "recall": recall,
        "exact_set_match": exact,
        "hierarchical_f1": hierarchical_score,
        "reward": f1
    }

def compute_hierarchical_f1(predicted: set, gold: set) -> float:
    """Give partial credit for codes in the correct hierarchy."""
    partial_matches = 0
    for gc in gold:
        best_match = 0
        gc_parts = gc.split(".")
        for pc in predicted:
            pc_parts = pc.split(".")
            # Match on category (first 3 chars)
            if gc_parts[0][:3] == pc_parts[0][:3]:
                # Full match = 1.0, category match = 0.5
                if gc == pc:
                    best_match = 1.0
                else:
                    best_match = max(best_match, 0.5)
        partial_matches += best_match
    return partial_matches / len(gold) if gold else 0.0
```

## Dataset Sources

- **MIMIC-III**: ~52,000 discharge summaries with ICD-9 codes (from Beth Israel Deaconess Medical Center). Freely available with PhysioNet credentialed access. The most widely used benchmark for automated coding. Average ~14 codes per admission.
- **MIMIC-IV**: Updated version with ~330,000 admissions, ICD-10 codes. More recent and larger.
- **CodiEsp (eHealth CLEF 2020)**: 1,000 Spanish clinical cases with ICD-10 codes. Multilingual medical coding.
- **MLTC (Medical Language Text Classification)**: Various medical text classification datasets.
- **CANTEMIST**: 1,301 oncology clinical records in Spanish with ICD-O codes.
- **PLM-ICD**: Processed MIMIC-III/IV benchmark with standardized splits (Huang et al., 2022).
- **ICD-10 code descriptions**: ~70,000 ICD-10-CM codes with official descriptions, used for code-description matching.

## Task Format

- **Input**: Clinical note (discharge summary or encounter note).
```
Assign all applicable ICD-10-CM codes to the following discharge summary:

DISCHARGE DIAGNOSIS:
1. Acute exacerbation of chronic systolic heart failure
2. Type 2 diabetes mellitus with diabetic chronic kidney disease, stage 3
3. Essential hypertension
4. Obesity

HOSPITAL COURSE: Patient is a 67-year-old male admitted with dyspnea
and lower extremity edema. BNP elevated at 1200. Echo showed EF 30%.
Started on IV diuresis with furosemide, transitioned to oral...
```
- **Output**: Set of ICD-10 codes.
```
I50.23 - Acute on chronic systolic (congestive) heart failure
E11.22 - Type 2 diabetes mellitus with diabetic chronic kidney disease
N18.3 - Chronic kidney disease, stage 3
I10 - Essential (primary) hypertension
E66.9 - Obesity, unspecified
```

## Difficulty Curriculum

- Level 1: Single, clearly stated diagnosis with obvious ICD-10 mapping ("hypertension" -> I10)
- Level 2: 2-3 explicit diagnoses listed in a discharge summary
- Level 3: 5-10 codes including both diagnoses and procedures
- Level 4: Combination codes (diabetes with complications maps to one combination code, not two separate)
- Level 5: Implicit conditions requiring inference from clinical details (not explicitly listed as diagnosis)
- Level 6: Sequencing rules (principal vs. secondary diagnosis ordering)
- Level 7: Excludes notes and coding conventions (certain code pairs cannot coexist)
- Level 8: 15+ codes with complex laterality, severity, and episode-of-care modifiers
- Level 9: Coding from unstructured clinical narratives without explicit diagnosis lists, rare conditions

## Limitations & Risks

- **Coding convention complexity**: ICD-10 has intricate rules about code sequencing, excludes notes (codes that cannot be used together), and "code first" / "use additional code" instructions. These rules change annually.
- **Annotation inconsistency**: Professional coders disagree on codes ~20-30% of the time for complex cases. Inter-coder reliability is moderate, which limits achievable accuracy.
- **Data access barriers**: MIMIC requires PhysioNet credentialed access and data use agreement. No medical coding datasets are truly "open" due to patient privacy.
- **ICD-9 vs. ICD-10**: MIMIC-III uses ICD-9 codes; most current practice uses ICD-10. Direct transfer requires mapping tables that are imperfect.
- **Financial gaming risk**: Medical coding directly affects hospital reimbursement. A model optimized for maximum coding accuracy might inadvertently learn to "upcode" (assign more severe codes than warranted), which is fraudulent.
- **Long document challenge**: Discharge summaries can be 2,000+ words. The model must attend to details scattered throughout.
- **Label set size**: With ~70,000 possible ICD-10 codes, this is an extreme multi-label classification problem. Most codes appear rarely, creating severe class imbalance.

## Connections

- [[ecg-biosignal]] — ECG findings are coded as ICD-10 diagnoses
- [[medical-image-segmentation]] — Radiology findings from segmentation inform diagnostic coding
- [[named-entity-recognition]] — Medical NER extracts conditions and procedures that are then coded
- [[reading-comprehension]] — Coding requires deep comprehension of clinical narratives
- [[information-extraction]] — Coding is a form of structured information extraction from text
