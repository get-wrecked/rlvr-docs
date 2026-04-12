---
domain: ECG & Biosignal Analysis
category: medical-signal
verification_type: exact_match
dataset_scale: 22K-110K recordings (PTB-XL, MIT-BIH)
difficulty_range: easy/medium/hard
modality: multimodal
status: verified
---

# ECG & Biosignal Analysis

## Overview

ECG biosignal analysis classifies cardiac conditions from electrocardiogram recordings: detecting arrhythmias (atrial fibrillation, ventricular tachycardia, bundle branch blocks), myocardial infarction, and other abnormalities. The model takes a time-series ECG signal (voltage over time across multiple leads) and outputs diagnostic labels.

RLVR verification is strong: each ECG recording has expert cardiologist annotations (often with consensus from multiple experts), and classification accuracy can be measured via standard metrics. The task is unambiguous for most arrhythmia types and has large, well-annotated public datasets.

## Verification Mechanism

```python
import numpy as np
from sklearn.metrics import f1_score, roc_auc_score

def verify_ecg_classification(predicted_labels: list, gold_labels: list,
                                predicted_probs: np.ndarray = None,
                                gold_onehot: np.ndarray = None) -> dict:
    """
    Multi-label ECG classification verification.
    predicted_labels: list of predicted diagnostic codes
    gold_labels: list of gold standard diagnostic codes
    """
    # Exact set match
    pred_set = set(predicted_labels)
    gold_set = set(gold_labels)
    exact_match = pred_set == gold_set

    # Set-level F1
    tp = len(pred_set & gold_set)
    precision = tp / len(pred_set) if pred_set else 0.0
    recall = tp / len(gold_set) if gold_set else 0.0
    f1 = 2 * precision * recall / (precision + recall) if (precision + recall) > 0 else 0.0

    result = {
        "exact_match": exact_match,
        "f1": f1,
        "reward": 1.0 if exact_match else f1
    }

    # AUC-ROC if probability scores are available (multi-label)
    if predicted_probs is not None and gold_onehot is not None:
        try:
            macro_auc = roc_auc_score(gold_onehot, predicted_probs, average='macro')
            result["macro_auc"] = macro_auc
        except ValueError:
            pass  # undefined when a class has no positive samples

    return result

def verify_beat_detection(predicted_beats: list, gold_beats: list,
                           tolerance_ms: float = 75) -> dict:
    """Verify R-peak detection (beat segmentation)."""
    matched = 0
    for gb in gold_beats:
        if any(abs(pb - gb) <= tolerance_ms for pb in predicted_beats):
            matched += 1
    recall = matched / len(gold_beats) if gold_beats else 0.0
    precision = matched / len(predicted_beats) if predicted_beats else 0.0
    f1 = 2 * precision * recall / (precision + recall) if (precision + recall) > 0 else 0.0
    return {"beat_f1": f1, "reward": f1}
```

## Dataset Sources

- **MIT-BIH Arrhythmia Database**: 110,000+ beat annotations across 48 half-hour ECG recordings from 47 patients. 2-lead ECG. 15 beat types annotated. The oldest and most cited ECG dataset (1979, still relevant).
- **PTB-XL**: 21,837 12-lead ECG recordings (10 seconds each) from 18,885 patients. 71 diagnostic labels across 5 superclasses (normal, MI, conduction disturbance, hypertrophy, ST/T change). Expert-annotated with confidence scores. The modern standard.
- **PhysioNet/CinC Challenge datasets**: Annual challenges on ECG classification (2017: AF detection from single-lead, 2020/2021: multi-label 12-lead classification with 27+ classes from 6 databases, ~110K recordings total).
- **CPSC 2018 (China Physiological Signal Challenge)**: 6,877 12-lead ECGs with 9 rhythm classes.
- **INCART**: 75 12-lead recordings with beat-by-beat annotations.
- **Chapman-Shaoxing**: 10,646 12-lead ECGs with 11 rhythm classes.
- **Georgia (Emory)**: 10,344 12-lead ECGs from CinC 2020.
- **LUDB (Lobachevsky University ECG Database)**: 200 ECGs with fiducial point annotations (P, QRS, T wave boundaries).

## Task Format

- **Input**: ECG signal (time-series data or description) with lead information.
```
Classify this 12-lead ECG recording:
Duration: 10 seconds, Sample rate: 500 Hz
Lead II rhythm: [voltage values or description]
Notable features: Irregular R-R intervals, absent P waves, fibrillatory baseline

Provide diagnostic labels from: {Normal, AF, LBBB, RBBB, 1AVB, PAC, PVC, STD, STE}
```
- **Output**: Diagnostic labels.
```
Primary diagnosis: Atrial Fibrillation (AF)
Confidence: 0.94
Additional findings: ST depression (STD) in leads V4-V6
```

## Difficulty Curriculum

- Level 1: Normal sinus rhythm vs. obvious atrial fibrillation (irregularly irregular)
- Level 2: Clear bundle branch blocks (LBBB vs. RBBB from QRS morphology)
- Level 3: Premature beats (PAC, PVC) detection and counting
- Level 4: First-degree AV block, ST elevation/depression
- Level 5: Multi-label classification (co-occurring conditions)
- Level 6: Myocardial infarction localization from ST-T changes across leads
- Level 7: Subtle rhythm abnormalities (Wenckebach, multifocal atrial tachycardia)
- Level 8: Pediatric ECGs, pacemaker rhythms, drug effects (QT prolongation)
- Level 9: Ambiguous recordings where expert cardiologists disagree, noisy/artifact-contaminated signals

## Limitations & Risks

- **Clinical safety**: ECG interpretation errors can be life-threatening. RLVR-trained models must not be used for clinical diagnosis without extensive validation and regulatory approval.
- **Signal modality**: ECG is a time-series signal. Text-only models can work with textual descriptions of ECG features but cannot process raw waveforms. Full exploitation requires signal-processing capabilities.
- **Annotation subjectivity**: Cardiologist agreement varies by condition. Agreement is high for AF (~95%) but lower for subtle conditions (~70-80%). The gold standard has inherent noise.
- **Class imbalance**: Normal rhythms dominate most datasets. Rare but critical conditions (e.g., Brugada syndrome, long QT) have very few training examples.
- **Demographic bias**: Datasets are drawn from specific hospital populations. ECG morphology varies with age, sex, race, and body habitus. Models may perform poorly on underrepresented populations.
- **Lead configuration**: MIT-BIH uses 2 leads; PTB-XL uses 12 leads. Models trained on 12-lead may not work with single-lead wearable data, and vice versa.

## Connections

- [[medical-image-segmentation]] — Both are medical AI tasks requiring expert annotations
- [[medical-coding]] — ECG findings are coded in clinical documentation (ICD-10 codes)
- [[audio-source-separation]] — Signal processing techniques overlap (time-frequency analysis)
- [[emotion-recognition]] — Physiological signal analysis shares feature extraction methods
