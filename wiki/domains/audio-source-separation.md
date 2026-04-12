---
domain: Audio Source Separation
category: audio
verification_type: exact_match
dataset_scale: 100-150 tracks (MUSDB18), augmented to thousands
difficulty_range: easy/medium/hard
modality: audio
status: verified
---

# Audio Source Separation

## Overview

Audio source separation (also called "cocktail party problem") isolates individual sound sources from a mixture: separating vocals from instruments in music, isolating a speaker from background noise, or extracting individual instruments from an ensemble recording. The output is one or more separated audio signals.

RLVR verification is strong: when ground truth isolated sources exist (as in MUSDB18, where original multitrack recordings provide the "answer"), Signal-to-Distortion Ratio (SDR) and related metrics provide a continuous, well-understood quality score. The separated audio is compared sample-by-sample against the ground truth source signal.

## Verification Mechanism

```python
import numpy as np

def compute_sdr(estimated: np.ndarray, reference: np.ndarray) -> float:
    """
    Signal-to-Distortion Ratio in dB.
    estimated: separated source signal (N samples)
    reference: ground truth source signal (N samples)
    """
    # Project estimated onto reference
    ref_energy = np.sum(reference ** 2)
    if ref_energy == 0:
        return float('-inf')

    # s_target: component of estimated in direction of reference
    alpha = np.dot(estimated, reference) / ref_energy
    s_target = alpha * reference

    # e_noise: everything else
    e_noise = estimated - s_target

    noise_energy = np.sum(e_noise ** 2)
    if noise_energy == 0:
        return float('inf')

    sdr = 10 * np.log10(np.sum(s_target ** 2) / noise_energy)
    return sdr

def verify_separation(estimated_sources: dict, reference_sources: dict) -> dict:
    """
    Verify multi-source separation.
    estimated_sources: {"vocals": np.array, "drums": np.array, ...}
    reference_sources: same format, ground truth
    """
    sdr_scores = {}
    for source_name in reference_sources:
        if source_name in estimated_sources:
            sdr = compute_sdr(estimated_sources[source_name],
                              reference_sources[source_name])
            sdr_scores[source_name] = sdr
        else:
            sdr_scores[source_name] = float('-inf')

    mean_sdr = np.mean([s for s in sdr_scores.values() if np.isfinite(s)])

    # Reward: SDR improvement over input mixture (typically 0-15 dB improvement)
    reward = max(0, min(1.0, mean_sdr / 15.0))  # normalize to [0, 1]

    return {
        "per_source_sdr": sdr_scores,
        "mean_sdr": mean_sdr,
        "reward": reward
    }
```

## Dataset Sources

- **MUSDB18**: 150 full-length music tracks (10 hours) with isolated stems: vocals, drums, bass, other. The standard music source separation benchmark. Training: 100 tracks, test: 50 tracks. HQ version at 44.1kHz stereo.
- **DSD100**: 100 tracks with 4 stems (vocals, bass, drums, accompaniment). Predecessor to MUSDB18.
- **FUSS (Free Universal Sound Separation)**: 23,000 10-second clips of mixed environmental sounds with separated sources. Derived from FSD50K. Google.
- **WSJ0-2mix / WSJ0-3mix**: Speech separation benchmark. 20,000 / 30,000 mixtures from Wall Street Journal corpus. The standard speech separation dataset.
- **LibriMix**: Speech mixtures from LibriSpeech. 290 hours of 2-speaker mixtures. Open-source alternative to WSJ0-mix.
- **DCASE Challenge datasets**: Sound event separation in environmental audio.
- **Slakh2100**: 2,100 tracks of MIDI-synthesized music with 34 instrument classes. Large-scale with perfect ground truth.
- **MUSDB18-HQ**: Higher-quality version of MUSDB18.
- **Synthesized mixtures**: Mix any clean audio sources together to create unlimited training data with known ground truth.

## Task Format

- **Input**: Mixed audio signal.
```
Separate this music recording into individual stems:
- Vocals
- Drums
- Bass
- Other (accompaniment)

Input: [mixed_audio.wav] (44.1kHz stereo, 3:45 duration)
```
- **Output**: Separated audio signals (one per source).
```
Separated stems:
- vocals.wav: SDR = 7.2 dB
- drums.wav: SDR = 6.8 dB
- bass.wav: SDR = 5.4 dB
- other.wav: SDR = 4.9 dB
Mean SDR: 6.1 dB
```

## Difficulty Curriculum

- Level 1: Two-speaker speech separation with minimal overlap (WSJ0-2mix easy)
- Level 2: Two-speaker separation with significant overlap
- Level 3: Vocals vs. accompaniment (music, 2-source)
- Level 4: 4-stem music separation (vocals, drums, bass, other)
- Level 5: Three-speaker speech separation
- Level 6: Music with closely harmonizing vocals (backup singers mixed with lead)
- Level 7: Separation of similar-timbre instruments (two guitars, two violins)
- Level 8: Noisy mixtures with ambient sound, reverberation, and recording artifacts
- Level 9: Universal sound separation (arbitrary number of arbitrary sources), live concert recordings with audience noise

## Limitations & Risks

- **Audio-only task**: Source separation requires processing audio waveforms or spectrograms. Text-only models cannot perform this task. The domain is exclusively for audio-capable models.
- **Small benchmark size**: MUSDB18 has only 150 tracks. This is a tiny test set by ML standards. Results can vary significantly with track selection.
- **SDR limitations**: SDR measures signal-level reconstruction quality but does not capture perceptual quality. A source with high SDR may sound worse than one with lower SDR due to artifact types.
- **Permutation problem**: For blind source separation, the model does not know which output should be which source. Permutation-invariant training (PIT) addresses this for speech but adds complexity.
- **Ground truth availability**: Isolated stems only exist for professionally recorded music (multitrack masters) or synthetically mixed audio. Real-world recordings rarely have ground truth separation.
- **Generalization**: Models trained on MUSDB18 (mostly pop/rock) perform poorly on genres with very different mixing (classical, jazz, electronic). Domain-specific training data helps but is scarce.
- **Evaluation bias**: SDR improvement depends on the input mixture SDR. A source that is already loud in the mixture is easier to separate than one buried in noise.

## Connections

- [[audio-speech-recognition]] — ASR performance improves with source separation as preprocessing
- [[ecg-biosignal]] — Signal processing techniques (time-frequency analysis, filtering) overlap
- [[emotion-recognition]] — Isolating speech from noise enables better emotion detection from audio
- [[music-audio-processing]] — Music-specific audio analysis builds on source separation
