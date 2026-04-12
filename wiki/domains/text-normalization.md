---
domain: Text Normalization
category: nlp
verification_type: exact_match
dataset_scale: 1.1B tokens (Google Text Normalization)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Text Normalization

## Overview

Text normalization converts non-standard written forms into their standard spoken equivalents: "Feb 3rd" -> "February third", "$4.50" -> "four dollars and fifty cents", "Dr." -> "Doctor", "2:30pm" -> "two thirty p m". This task is critical for text-to-speech (TTS) systems, where the normalized form must be read aloud correctly.

RLVR verification is exact string match: the normalized output must exactly match the gold standard spoken form. The task is highly verifiable and has massive training data (Google released a 1.1B-token corpus). The main challenge is that normalization is context-dependent ("St." can be "street" or "saint") and locale-dependent ("01/02/03" means different dates in US vs. UK).

## Verification Mechanism

```python
def verify_normalization(predicted: str, gold: str) -> dict:
    """Exact match verification for text normalization."""
    pred_clean = predicted.strip().lower()
    gold_clean = gold.strip().lower()

    exact = pred_clean == gold_clean

    # Token-level accuracy for partial credit
    pred_tokens = pred_clean.split()
    gold_tokens = gold_clean.split()
    if gold_tokens:
        matching = sum(1 for p, g in zip(pred_tokens, gold_tokens) if p == g)
        token_acc = matching / max(len(pred_tokens), len(gold_tokens))
    else:
        token_acc = 1.0 if not pred_tokens else 0.0

    return {
        "exact_match": 1.0 if exact else 0.0,
        "token_accuracy": token_acc,
        "reward": 1.0 if exact else 0.0  # strict exact match for TTS
    }

def verify_normalization_safe(predicted: str, gold: str, input_text: str) -> dict:
    """Also check for catastrophic errors (wrong number, wrong name)."""
    result = verify_normalization(predicted, gold)

    # Catastrophic error detection: digits in input should map to correct words
    # "123" -> "one hundred twenty three" NOT "one hundred thirty two"
    # This is handled by exact match, but worth flagging separately
    result["is_number_token"] = any(c.isdigit() for c in input_text)
    return result
```

## Dataset Sources

- **Google Text Normalization Challenge**: 1.1 billion tokens of English and Russian text with normalization annotations. Covers 16 semiotic classes: PLAIN, PUNCT, DATE, LETTERS, CARDINAL, VERBATIM, MEASURE, ORDINAL, DECIMAL, MONEY, TIME, ELECTRONIC, DIGIT, FRACTION, TELEPHONE, ADDRESS. Released on Kaggle (2017).
- **ITN (Inverse Text Normalization) datasets**: The reverse task (spoken form -> written form) from ASR output. NVIDIA NeMo provides ITN grammars for 10+ languages.
- **Sproat & Jaitly (2017)** standard splits: The canonical train/dev/test split for the Google dataset.
- **Multilingual TN datasets**: Text normalization data for German, Spanish, French, Portuguese via TTS research groups (Google, Amazon, Microsoft).
- **Custom rule-based generators**: For specific semiotic classes (dates, currencies, measurements), unlimited training data can be procedurally generated.

## Task Format

- **Input**: A sentence or token with its semiotic class, requiring normalization.
```
Normalize the following text for spoken output:

"The meeting is on Feb 3rd, 2025 at 2:30pm. Cost: $14.99/person."
```
- **Output**: Fully normalized spoken form.
```
"The meeting is on February third twenty twenty five at two thirty p m.
Cost: fourteen dollars and ninety nine cents per person."
```

Per-token format (as in the Google dataset):
```
Input: "3rd" (ORDINAL)
Output: "third"

Input: "$4.50" (MONEY)
Output: "four dollars and fifty cents"

Input: "Dr." (context: "Dr. Smith")
Output: "doctor"

Input: "St." (context: "42 St. James St.")
Output: "saint" (first), "street" (second)
```

## Difficulty Curriculum

- Level 1: Plain text (no normalization needed), simple punctuation
- Level 2: Cardinal numbers ("42" -> "forty two"), common abbreviations ("Dr.", "Mr.")
- Level 3: Ordinals ("3rd" -> "third"), simple dates ("Jan 5" -> "January fifth")
- Level 4: Money ("$14.99"), time ("2:30pm"), measurements ("5kg")
- Level 5: Context-dependent abbreviations ("St." = "street" vs "saint", "Dr." = "doctor" vs "drive")
- Level 6: Complex numbers (fractions "3/4", decimals "3.14159"), telephone numbers
- Level 7: Electronic tokens (URLs, email addresses, hashtags)
- Level 8: Mixed-class tokens ("$3.5M" = "three point five million dollars"), nested normalization
- Level 9: Locale-dependent formats (date formats, number separators), code-switching text, rare semiotic classes

## Limitations & Risks

- **Context sensitivity**: The same token can normalize differently depending on context ("1/2" = "one half" vs. "January second" vs. "one slash two"). Context windows must be large enough.
- **Locale/dialect variation**: "1,000" means "one thousand" in the US but "one" (with decimal) in Germany. "$" could be USD, AUD, CAD, etc. The model needs locale specification.
- **Catastrophic errors in TTS**: A normalization error in a TTS system is immediately audible and can be embarrassing or dangerous (reading a phone number wrong). Exact match is the only acceptable standard.
- **Ambiguity in gold**: Some tokens have genuinely ambiguous normalizations ("read" could be present or past tense, affecting pronunciation). The Google dataset resolves most ambiguities but not all.
- **Evolving conventions**: New abbreviations, slang, and formats emerge constantly (e.g., crypto addresses, new emoji conventions). Static datasets become stale.
- **English/Russian bias**: The Google dataset only covers English and Russian. Many languages lack comparable resources.

## Connections

- [[audio-speech-recognition]] — ASR systems need inverse text normalization; TTS systems need forward normalization
- [[morphological-inflection]] — Both are string transduction tasks with exact-match verification
- [[named-entity-recognition]] — NER helps identify which tokens need normalization (dates, money, etc.)
- [[data-to-text]] — Data-to-text generation must also render numbers, dates, and measurements in natural language
