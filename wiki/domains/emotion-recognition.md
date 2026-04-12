---
domain: Emotion Recognition
category: multimodal
verification_type: exact_match
dataset_scale: 35K-58K instances (GoEmotions, FER-2013)
difficulty_range: easy/medium/hard
modality: multimodal
status: strong_hypothesis
---

# Emotion Recognition

## Overview

Emotion recognition classifies emotional states from text, audio (speech prosody), facial expressions, or multimodal combinations. Output taxonomies range from basic emotions (Ekman's 6: anger, disgust, fear, happiness, sadness, surprise) to fine-grained models (Plutchik's 8, GoEmotions' 27 labels, or dimensional models like valence-arousal).

RLVR verification matches predicted labels against human-annotated gold labels. For categorical systems, this is exact match or F1. For dimensional models (valence-arousal), this is correlation or mean absolute error. The main challenge is that emotion perception is inherently subjective -- annotator agreement is moderate at best.

## Verification Mechanism

```python
from sklearn.metrics import f1_score, classification_report

def verify_emotion_single(predicted: str, gold: str) -> dict:
    """Single-instance emotion classification verification."""
    match = predicted.strip().lower() == gold.strip().lower()
    return {"correct": match, "reward": 1.0 if match else 0.0}

def verify_emotion_multilabel(predicted_labels: set, gold_labels: set) -> dict:
    """Multi-label emotion verification (text can express multiple emotions)."""
    tp = len(predicted_labels & gold_labels)
    precision = tp / len(predicted_labels) if predicted_labels else 0.0
    recall = tp / len(gold_labels) if gold_labels else 0.0
    f1 = 2 * precision * recall / (precision + recall) if (precision + recall) > 0 else 0.0
    return {"f1": f1, "exact_match": predicted_labels == gold_labels, "reward": f1}

def verify_valence_arousal(predicted_v: float, predicted_a: float,
                            gold_v: float, gold_a: float) -> dict:
    """Dimensional emotion model verification."""
    import numpy as np
    v_error = abs(predicted_v - gold_v)
    a_error = abs(predicted_a - gold_a)
    euclidean = np.sqrt(v_error**2 + a_error**2)

    # Reward: inverse distance, normalized (VA typically in [-1, 1])
    max_dist = np.sqrt(2) * 2  # max possible distance in [-1,1]x[-1,1]
    reward = max(0, 1.0 - euclidean / max_dist)

    return {
        "valence_error": v_error,
        "arousal_error": a_error,
        "euclidean_distance": euclidean,
        "reward": reward
    }

def verify_emotion_batch(predictions: list, golds: list) -> dict:
    """Batch evaluation with macro and weighted F1."""
    macro_f1 = f1_score(golds, predictions, average='macro', zero_division=0)
    weighted_f1 = f1_score(golds, predictions, average='weighted', zero_division=0)
    return {
        "macro_f1": macro_f1,
        "weighted_f1": weighted_f1,
        "reward": macro_f1
    }
```

## Dataset Sources

- **GoEmotions (Google)**: 58,009 Reddit comments annotated with 27 emotion labels + neutral. Multi-label (a comment can have multiple emotions). The largest fine-grained text emotion dataset.
- **IEMOCAP**: 12 hours of dyadic conversations (10 speakers) with categorical and dimensional emotion labels. Audio + video + text transcripts. The standard multimodal emotion benchmark.
- **FER-2013**: 35,887 48x48 grayscale facial images classified into 7 emotions (angry, disgust, fear, happy, sad, surprise, neutral). From the Kaggle facial expression challenge.
- **AffectNet**: 440,000 facial images with categorical (8 emotions) and dimensional (valence-arousal) labels. The largest facial emotion dataset.
- **SemEval-2018 Task 1 (Affect in Tweets)**: 10K+ tweets with emotion intensity scores across 4 emotions.
- **MELD (Multimodal EmotionLines Dataset)**: 13K utterances from Friends TV show with emotion labels. Multimodal (audio + video + text).
- **EmoBank**: 10K sentences with valence-arousal-dominance annotations.
- **RAVDESS**: 7,356 audio-visual clips of 24 actors expressing 8 emotions. Acted speech/song.
- **CMU-MOSEI**: 23,453 annotated video segments from YouTube with sentiment and emotion labels.

## Task Format

- **Input (text)**: Text with emotion classification instruction.
```
Classify the emotion(s) expressed in this text:

"I can't believe they canceled the concert. I've been looking forward to
this for months and now it's all ruined."

Labels: {admiration, amusement, anger, annoyance, approval, caring, confusion,
curiosity, desire, disappointment, disapproval, disgust, embarrassment,
excitement, fear, gratitude, grief, joy, love, nervousness, optimism,
pride, realization, relief, remorse, sadness, surprise, neutral}
```
- **Output**: Emotion label(s).
```
Primary: disappointment
Secondary: anger, sadness
```

- **Input (facial)**: Face image + classification task.
- **Input (audio)**: Speech clip + emotion classification.
- **Input (multimodal)**: Video clip with audio + transcript.

## Difficulty Curriculum

- Level 1: Strong basic emotions with clear lexical cues ("I'm so happy!", "This is terrifying")
- Level 2: Basic emotions without explicit emotion words (inferred from context)
- Level 3: Mixed emotions in a single text ("bittersweet", "angry but proud")
- Level 4: Sarcasm and irony ("Oh great, another Monday" = annoyance, not excitement)
- Level 5: Fine-grained distinctions (anger vs. annoyance vs. frustration vs. contempt)
- Level 6: Cross-modal: emotion from audio prosody alone (no text content)
- Level 7: Facial expression recognition in natural (non-posed) settings
- Level 8: Multimodal fusion: text says one thing, tone says another
- Level 9: Cultural context-dependent emotions, subtle emotional nuance in literature, detecting suppressed or masked emotions

## Limitations & Risks

- **Annotator disagreement**: Emotion is subjective. Inter-annotator agreement is typically 60-75% for fine-grained labels. The gold standard is a majority vote, not an objective truth.
- **Cultural bias**: Emotion expression and perception vary significantly across cultures. Ekman's universal emotions are disputed. Most datasets are English-centric and Western-biased.
- **Label taxonomy mismatch**: Different datasets use different emotion taxonomies (6, 7, 8, 27 categories, or continuous dimensions). Cross-dataset evaluation requires mapping that loses information.
- **Sarcasm and context**: Text emotion recognition fails badly on sarcastic or ironic text without context. A single sentence is often insufficient to determine emotion.
- **Modality limitations**: Text-only models cannot process facial expressions or audio prosody. The multimodal promise of this domain is only accessible to multimodal models.
- **Ethics of emotion detection**: Emotion recognition technology raises concerns about surveillance, manipulation, and privacy. Models should not be used for covert emotional profiling.
- **Acted vs. natural data**: RAVDESS and similar acted datasets do not represent natural emotional expression. Models trained on acted data perform poorly on real emotions.

## Connections

- [[face-detection-recognition]] — Facial emotion recognition builds on face detection
- [[audio-speech-recognition]] — Emotion from speech leverages prosodic features extracted during ASR
- [[semantic-textual-similarity]] — Text emotion requires understanding semantic content
- [[recommender-systems]] — Sentiment/emotion from reviews informs recommendation
- [[ecg-biosignal]] — Physiological signals (heart rate, GSR) are emotion indicators
