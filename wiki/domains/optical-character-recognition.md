---
domain: Optical Character Recognition (Advanced)
category: vision-document
verification_type: exact_match
dataset_scale: 10M+ (document datasets)
difficulty_range: easy/medium/hard
modality: vision
status: verified
---

# Optical Character Recognition (Advanced)

## Overview
Extract text from images of documents, handwriting, scene text, historical manuscripts. Goes beyond basic OCR to include: handwritten math recognition, scene text in the wild, degraded historical documents, multilingual text. Verification: character/word error rate against ground truth.

## Verification Mechanism
```python
def verify(image: np.array, predicted_text: str, ground_truth: str) -> float:
    # Character Error Rate
    cer = edit_distance(predicted_text, ground_truth) / max(len(ground_truth), 1)
    
    # Word Error Rate
    pred_words = predicted_text.split()
    gt_words = ground_truth.split()
    wer = edit_distance(pred_words, gt_words) / max(len(gt_words), 1)
    
    # Return accuracy (1 - error rate)
    return max(0, 1 - cer)  # Or use WER: max(0, 1 - wer)
```

## Dataset Sources
- **IAM Handwriting Database**: 1500+ pages of handwritten English.
- **RIMES**: French handwriting dataset.
- **CSER**: Chinese handwriting dataset.
- **ICDAR competitions**: Regular text recognition competitions.
- **SVT (Street View Text)**: Scene text in natural images.
- **IIIT-5K**: 5000 word images from the internet.
- **CROHME**: Competition on Recognition of Handwritten Math Expressions.
- **HisDoc**: Historical document dataset.
- **TextOCR**: 900K annotated text in natural images.
- **Total-Text**: Curved text in the wild.
- **Google's Newspaper dataset**: Digitized newspaper archives.
- **HTR datasets**: Historical Text Recognition from national archives.

## Task Format
- **Input**: Image of text (handwritten note, sign, receipt, historical document)
- **Output**: Transcribed text string

## Difficulty Curriculum
- Level 1: Printed text, clean background, English
- Level 3: Handwritten block letters
- Level 5: Cursive handwriting, noisy backgrounds
- Level 7: Historical manuscripts, degraded documents
- Level 9: Scene text with perspective distortion, multilingual mixed text

## Limitations & Risks
- Ground truth may have errors (especially for historical documents).
- CER/WER are standard, well-accepted metrics.
- Multiple valid transcriptions may exist for ambiguous handwriting — accept alternatives.

## Connections
- [[document-ocr-extraction]] — structured extraction from OCR'd text
- [[document-parsing]] — document layout analysis
- [[data-entry-correction]] — correcting OCR errors
- [[latex-typesetting]] — math recognition → LaTeX
