---
domain: Music/Audio Processing
category: signal-processing
verification_type: execution
dataset_scale: 100K+ (from music information retrieval datasets)
difficulty_range: easy/medium/hard
modality: multimodal
status: strong_hypothesis
---

# Music/Audio Processing

## Overview
Process audio signals to extract verifiable information: pitch detection, tempo estimation, beat tracking, chord recognition, music transcription, source separation quality. Verification: compare extracted features against ground truth annotations.

## Verification Mechanism
```python
import librosa
import mir_eval

def verify(task_type: str, audio: np.array, prediction: Any, ground_truth: Any) -> float:
    if task_type == "tempo":
        # Tempo estimation (BPM)
        return 1.0 if mir_eval.tempo.detection(
            [ground_truth], [prediction], tol=0.08
        )[0] > 0.5 else 0.0
    
    elif task_type == "beat_tracking":
        scores = mir_eval.beat.evaluate(ground_truth, prediction)
        return scores["F-measure"]
    
    elif task_type == "pitch":
        # F0 estimation
        scores = mir_eval.melody.evaluate(
            ground_truth["times"], ground_truth["freqs"],
            prediction["times"], prediction["freqs"]
        )
        return scores["Overall Accuracy"]
    
    elif task_type == "chord":
        scores = mir_eval.chord.evaluate(
            ground_truth["intervals"], ground_truth["labels"],
            prediction["intervals"], prediction["labels"]
        )
        return scores["thirds"]
    
    elif task_type == "transcription":
        # Note-level transcription (piano roll)
        scores = mir_eval.transcription.evaluate(
            ground_truth["intervals"], ground_truth["pitches"],
            prediction["intervals"], prediction["pitches"]
        )
        return scores["F-measure"]
```

## Dataset Sources
- **MIR-1K**: 1000 song clips with pitch annotations.
- **MAPS**: Piano music dataset with note-level annotations.
- **GuitarSet**: Multi-track guitar recordings with annotations.
- **DALI**: 5K+ songs with time-aligned lyrics.
- **MedleyDB**: Multi-track recordings with annotations.
- **FMA (Free Music Archive)**: 106K tracks with metadata.
- **MTG-Jamendo**: 55K tracks for music tagging.
- **Ballroom dataset**: Tempo and rhythm annotations.

## Task Format
- **Input**: Audio file (or spectrogram) + "Identify all notes played between 5.0s and 10.0s"
- **Output**: List of (onset_time, offset_time, midi_pitch) tuples

## Difficulty Curriculum
- Level 1: Simple tempo detection, single-instrument pitch tracking
- Level 3: Multi-instrument chord recognition
- Level 5: Full piano transcription
- Level 7: Multi-instrument transcription, source separation
- Level 9: Complex polyphonic transcription with dynamics

## Limitations & Risks
- Audio processing is computationally expensive. May need to work with spectrograms rather than raw audio.
- mir_eval provides standardized evaluation metrics — well-established in the community.
- Ground truth annotations have human error margins.

## Connections
- [[signal-processing]] — general DSP
- [[music-theory]] — theory underlying music
- [[music-generation-midi]] — generation counterpart
