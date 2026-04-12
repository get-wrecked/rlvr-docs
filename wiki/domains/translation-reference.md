---
domain: translation-reference
category: language
verified: true
verification_type: approximate
scalability: very-high
maturity: very-high
tags: [translation, bleu, chrf, wmt, multilingual]
---

# Machine Translation with Reference Translations

## Overview

Machine translation (MT) with reference-based evaluation is a foundational NLP task where the model translates text from a source language to a target language, and the output is scored against one or more human-produced reference translations.

This domain has enormous scale (billions of parallel sentences available), well-studied evaluation metrics, and decades of shared task infrastructure. However, it has a fundamental limitation for RLVR: **all automated metrics are approximate**. A translation can be correct and fluent but score poorly because it uses different word choices than the reference, and conversely, a translation can score well by copying reference patterns without genuine understanding.

> **Honesty flag**: This domain uses approximate verification. BLEU, chrF, and similar metrics are imperfect proxies for translation quality. They correlate with human judgment at the corpus level but are unreliable for individual sentences. This means the reward signal in RLVR is noisy.

## Verification Mechanism

**Verification type: APPROXIMATE**

No automated metric perfectly captures translation quality. The most common metrics used as reward signals:

### BLEU (Bilingual Evaluation Understudy)
- Computes n-gram precision (typically 1-4 grams) between candidate and reference
- Applies a brevity penalty for short translations
- Scores range from 0 to 100
- For RLVR: can threshold (BLEU > X = reward 1) or use as continuous reward

### chrF / chrF++
- Character n-gram F-score between candidate and reference
- More robust than BLEU for morphologically rich languages
- Less sensitive to tokenization choices
- chrF++ adds word n-grams to character n-grams

### COMET
- Neural metric trained on human quality judgments
- Scores correlate better with human evaluation than BLEU/chrF
- Requires a model checkpoint (adds dependency)
- **Caution**: Using a neural metric as reward introduces an LLM-as-judge element, which departs from the strict RLVR paradigm

### BERTScore
- Contextual embedding similarity between candidate and reference tokens
- Better at handling paraphrases than lexical metrics
- Same LLM-as-judge concern as COMET

**For strict RLVR, BLEU and chrF are the most defensible choices**: they are fully programmatic, deterministic, and require no model. They are imperfect but transparent.

```python
import sacrebleu

def verify(candidate: str, references: list[str]) -> float:
    score = sacrebleu.sentence_bleu(candidate, references).score
    return score / 100.0  # normalize to [0, 1]
```

## Dataset Sources

| Dataset | Size | Languages | Notes |
|---------|------|-----------|-------|
| WMT (yearly) | Millions of segment pairs | 10+ language pairs | Annual shared task since 2006; gold standard |
| OPUS | Billions of segments | 500+ languages | Aggregation of parallel corpora from web, subtitles, documents |
| Europarl | 60M+ sentences | 21 EU languages | European Parliament proceedings |
| ParaCrawl | 4.6B+ sentence pairs | 42 languages | Web-crawled parallel data |
| CCAligned | 8.4B+ sentence pairs | 138 languages | Common Crawl aligned data |
| FLORES-200 | 3K sentences x 200 langs | 200 languages | High-quality evaluation benchmark for low-resource languages |
| TED Talks | 4.5K TED transcripts | 100+ languages | Multi-way parallel |
| UN Parallel Corpus | 11.3M sentences | 6 UN languages | United Nations documents |

**Scalability**: This is one of the most data-rich RLVR domains. OPUS alone provides billions of parallel sentences. Web mining continuously produces new parallel data. The bottleneck is not data but evaluation quality.

## Task Format

**Input**: A sentence or paragraph in the source language, with a specified target language.

**Output**: The translation in the target language.

**Prompt template example**:
```
Translate the following sentence from English to German.

English: {source_sentence}

German:
```

## Difficulty Curriculum

1. **Level 1 — Short, simple sentences**: Common phrases and simple declarative sentences. High overlap with training data.
2. **Level 2 — Medium-length informational text**: News articles, Wikipedia passages. Standard vocabulary, clear structure.
3. **Level 3 — Idiomatic expressions**: Phrases that cannot be translated word-by-word. Requires cultural knowledge.
4. **Level 4 — Technical and domain-specific text**: Medical, legal, or scientific text with specialized terminology.
5. **Level 5 — Literary and creative text**: Poetry, prose, humor. Style and nuance matter; reference metrics are least reliable here.
6. **Level 6 — Low-resource language pairs**: Languages with limited parallel data (e.g., Swahili-Icelandic). Models must generalize from related languages.
7. **Level 7 — Document-level translation**: Full documents where cross-sentence coherence, pronoun resolution, and discourse structure matter.

## Limitations & Risks

- **Approximate verification is the core limitation**: BLEU/chrF penalize valid paraphrases and reward superficial n-gram overlap. A model optimized purely for BLEU may produce "translationese" — stilted output that scores well but reads unnaturally. This is a known failure mode of RL with BLEU reward.
- **Reference bias**: Metrics are computed against specific references. Multiple equally valid translations exist, but only the reference phrasing is rewarded. This systematically underestimates quality.
- **Metric gaming**: RL agents are known to exploit BLEU's weaknesses (e.g., repeating common n-grams, producing safe/generic translations). Reward shaping and metric combination can mitigate this.
- **Single-reference limitation**: Most datasets provide only 1-2 references per source. Multi-reference evaluation is more reliable but much more expensive.
- **Language coverage inequality**: High-resource pairs (EN-DE, EN-FR, EN-ZH) have excellent data; low-resource pairs may have noisy or insufficient parallel data.
- **Cultural nuance**: Automated metrics cannot evaluate cultural appropriateness, register, or pragmatic adequacy.

## Connections

- **[multilingual-tasks](multilingual-tasks.md)**: Translation is the foundation of multilingual capabilities; many cross-lingual tasks implicitly rely on translation ability.
- **[summarization-extractive](summarization-extractive.md)**: Both translation and summarization use approximate automated metrics (BLEU/ROUGE) as verification. Both face the same fundamental limitation.
- **[text-to-code-nl](text-to-code-nl.md)**: Translating between natural language and code is structurally similar to MT.
- **[semantic-parsing](semantic-parsing.md)**: Semantic parsing is "translation" from natural language to formal language.
- **[spelling-grammar](spelling-grammar.md)**: Post-editing machine translation output overlaps with grammar correction.
