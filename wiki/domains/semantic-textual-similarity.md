---
domain: Semantic Textual Similarity
category: nlp
verification_type: exact_match
dataset_scale: 8.6K-10K pairs (STS-B, SICK)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Semantic Textual Similarity

## Overview

Semantic textual similarity (STS) tasks require scoring the degree of meaning overlap between two sentences on a continuous scale (typically 0-5, where 0 = completely unrelated and 5 = semantically equivalent). This is a fundamental NLP capability that underpins paraphrase detection, duplicate question detection, information retrieval, and textual entailment.

For RLVR, verification is straightforward: the model's predicted score is compared against human-annotated gold scores using correlation metrics. The continuous nature of the output means exact match is impractical, but Pearson and Spearman correlation with gold scores provides a robust training signal.

## Verification Mechanism

```python
import numpy as np
from scipy.stats import pearsonr, spearmanr

def verify_sts_single(predicted_score: float, gold_score: float,
                       tolerance: float = 0.5) -> float:
    """Verify a single STS prediction."""
    error = abs(predicted_score - gold_score)
    if error <= tolerance:
        return 1.0
    elif error <= 1.0:
        return 1.0 - (error - tolerance) / (1.0 - tolerance)
    else:
        return 0.0

def verify_sts_batch(predicted_scores: list, gold_scores: list) -> dict:
    """Batch evaluation using correlation metrics."""
    pred = np.array(predicted_scores)
    gold = np.array(gold_scores)

    pearson_r, _ = pearsonr(pred, gold)
    spearman_r, _ = spearmanr(pred, gold)

    # Mean absolute error for interpretability
    mae = np.mean(np.abs(pred - gold))

    return {
        "pearson": pearson_r,
        "spearman": spearman_r,
        "mae": mae,
        "reward": (pearson_r + spearman_r) / 2.0  # average correlation
    }
```

## Dataset Sources

- **STS Benchmark (STS-B)**: 8,628 sentence pairs from image captions, news headlines, and user forums. Scores 0-5 from 5 annotators. Part of GLUE/SuperGLUE. The standard STS evaluation set.
- **SICK (Sentences Involving Compositional Knowledge)**: 9,840 sentence pairs derived from image descriptions. Relatedness scores 1-5. Specifically designed to test compositional semantics.
- **SemEval STS tasks (2012-2017)**: Annual shared task datasets totaling ~15K pairs across diverse genres (news, glosses, Q&A forums, captions). Different years have different domains.
- **MRPC (Microsoft Research Paraphrase Corpus)**: 5,801 sentence pairs with binary paraphrase labels. Less granular but useful for thresholded STS.
- **QQP (Quora Question Pairs)**: 400K question pairs with binary duplicate labels. Large scale but binary, not graded.
- **PAWS (Paraphrase Adversaries from Word Scrambling)**: 108K pairs specifically designed to have high lexical overlap but different meanings (adversarial).
- **Multilingual STS**: STS-B has been translated to 10+ languages (STSb-multi). Cross-lingual STS datasets exist for 30+ language pairs.

## Task Format

- **Input**: Two sentences and instruction to rate similarity.
```
Rate the semantic similarity of these two sentences on a scale from 0 (completely
unrelated) to 5 (identical in meaning):

Sentence 1: "A man is playing a guitar."
Sentence 2: "Someone is playing a musical instrument."
```
- **Output**: A numerical score.
```
3.8
```

## Difficulty Curriculum

- Level 1: Identical or near-identical sentences (score ~5) vs. completely unrelated (score ~0)
- Level 2: Clear paraphrases vs. clearly different topics
- Level 3: Sentences sharing words but different meanings ("bank of a river" vs. "bank account")
- Level 4: Subtle meaning differences ("all cats are animals" vs. "all animals are cats")
- Level 5: High lexical overlap but different semantics (PAWS-style adversarial)
- Level 6: Low lexical overlap but similar meaning (abstractive paraphrases)
- Level 7: Graded similarity in the 2-3 range (partially related, hardest to score precisely)
- Level 8: Cross-domain pairs (headline vs. forum post about the same topic)
- Level 9: Cross-lingual STS (sentences in different languages)

## Limitations & Risks

- **Annotation subjectivity**: Human STS scores have inter-annotator agreement of ~0.83 Pearson correlation. The gold standard itself has noise, which puts a ceiling on learnable performance.
- **Scale anchoring**: Different annotators interpret the 0-5 scale differently. Some use the full range; others cluster around 2-4. Averaging helps but does not eliminate this.
- **Continuous vs. discrete**: Models must output a continuous score, but LLMs naturally produce text. Converting model outputs to precise numerical scores introduces quantization effects.
- **Symmetry assumption**: STS is defined as symmetric (sim(A,B) = sim(B,A)), but models may not respect this. Directional similarity (entailment) is a different task.
- **Dataset size**: STS-B is relatively small (8.6K). Overfitting is a real risk if used as the primary RLVR training signal. QQP is much larger but only binary.
- **Lexical bias**: Simple baselines (word overlap, TF-IDF cosine) achieve surprisingly high correlation on some STS datasets, suggesting the task may not always require deep semantic understanding.

## Connections

- [[natural-language-inference]] — NLI (entailment/contradiction/neutral) is a categorical cousin of graded STS
- [[reading-comprehension]] — Both require understanding sentence-level semantics
- [[information-retrieval]] — STS is the core capability behind semantic search
- [[paraphrase-detection]] — Binary version of STS
