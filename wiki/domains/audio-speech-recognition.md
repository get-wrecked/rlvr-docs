---
domain: Speech Recognition & Audio Understanding
category: audio
verification_type: exact_match
dataset_scale: 100K+ hours (LibriSpeech, Common Voice)
difficulty_range: easy/medium/hard
modality: multimodal
status: verified
---

# Speech Recognition & Audio Understanding

## Overview
Transcribe speech to text, identify speakers, classify audio events, answer questions about audio content. Verification: word error rate against reference transcripts, exact match for classification tasks.

## Verification Mechanism
```python
import jiwer

def verify(task_type: str, audio: np.array, prediction: str, reference: str) -> float:
    if task_type == "transcription":
        wer = jiwer.wer(reference, prediction)
        return max(0, 1 - wer)
    
    elif task_type == "speaker_id":
        return 1.0 if prediction == reference else 0.0
    
    elif task_type == "audio_classification":
        return 1.0 if prediction.lower() == reference.lower() else 0.0
    
    elif task_type == "spoken_qa":
        # Answer questions about audio content
        return 1.0 if normalize(prediction) == normalize(reference) else 0.0
```

## Dataset Sources
- **LibriSpeech**: 960 hours of read English speech.
- **Common Voice**: 20K+ hours in 100+ languages (Mozilla).
- **VoxCeleb**: 2K+ speakers for speaker recognition.
- **AudioSet**: 2M+ 10-second clips classified into 527 categories.
- **SUPERB**: Speech processing universal benchmark.
- **Spoken SQuAD**: SQuAD questions in spoken form.
- **ESC-50**: Environmental Sound Classification (2K clips, 50 classes).
- **UrbanSound8K**: 8K urban sound clips.
- **FLEURS**: 102 languages for multilingual ASR.
- **GigaSpeech**: 10K hours of English audio.

## Task Format
- **Input**: Audio clip + "Transcribe this speech"
- **Output**: "The quick brown fox jumps over the lazy dog"

## Difficulty Curriculum
- Level 1: Clean read speech, common words
- Level 3: Conversational speech, background noise
- Level 5: Multiple speakers, accented speech
- Level 7: Technical vocabulary, low-resource languages
- Level 9: Noisy environments, far-field audio, code-switching

## Limitations & Risks
- WER is well-established but penalizes minor errors (articles, plurals).
- Multiple valid transcriptions may exist (numbers, abbreviations).
- Requires audio processing capability — the agent needs to handle audio modality.

## Connections
- [[music-audio-processing]] — music-specific audio
- [[optical-character-recognition]] — text recognition in another modality
- [[video-understanding]] — audio component of video
