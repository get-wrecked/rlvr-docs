---
domain: instruction-following
category: miscellaneous
verification_type: constraint
dataset_scale: ~large (IFEval + synthetic generation)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Instruction Following

## Overview

Instruction following tasks require generating text that satisfies a set of precise, verifiable instructions simultaneously. Unlike open-ended instruction following (which requires human judgment), RLVR-compatible instruction following restricts to instructions that can be checked programmatically: word count, sentence count, format requirements (bullet points, numbered lists), content inclusion/exclusion, structural constraints (number of paragraphs, sections), language constraints (reading level, no jargon), and format constraints (JSON output, specific delimiter usage).

**This is arguably the single most important RLVR domain for practical LLM alignment.** An LLM that reliably follows precise instructions is dramatically more useful than one that doesn't. IFEval (Google, 2023) demonstrated that even frontier models fail to follow simple combinations of verifiable instructions 20-40% of the time.

## Verification Mechanism

**Type: Constraint satisfaction (deterministic per instruction)**

Each instruction type has a dedicated verification function:

```python
VERIFIERS = {
    'word_count': lambda text, n: len(text.split()) == n,
    'word_count_range': lambda text, lo, hi: lo <= len(text.split()) <= hi,
    'sentence_count': lambda text, n: len(sent_tokenize(text)) == n,
    'paragraph_count': lambda text, n: len(text.strip().split('\n\n')) == n,
    'must_include_word': lambda text, w: w.lower() in text.lower(),
    'must_exclude_word': lambda text, w: w.lower() not in text.lower(),
    'must_include_phrase': lambda text, p: p.lower() in text.lower(),
    'starts_with': lambda text, s: text.strip().startswith(s),
    'ends_with': lambda text, s: text.strip().endswith(s),
    'all_uppercase': lambda text: text == text.upper(),
    'all_lowercase': lambda text: text == text.lower(),
    'no_commas': lambda text: ',' not in text,
    'contains_bullet_points': lambda text: any(l.strip().startswith(('- ', '* ', '• ')) for l in text.split('\n')),
    'bullet_point_count': lambda text, n: sum(1 for l in text.split('\n') if l.strip().startswith(('- ', '* ', '• '))) == n,
    'contains_numbered_list': lambda text: any(re.match(r'^\d+\.', l.strip()) for l in text.split('\n')),
    'has_title': lambda text: text.strip().split('\n')[0].startswith('#'),
    'json_format': lambda text: is_valid_json(text),
    'min_sentences': lambda text, n: len(sent_tokenize(text)) >= n,
    'max_sentences': lambda text, n: len(sent_tokenize(text)) <= n,
    'contains_section_headers': lambda text, n: sum(1 for l in text.split('\n') if l.startswith('#')) >= n,
    'language': lambda text, lang: detect_language(text) == lang,
    'no_repetition': lambda text: len(set(text.split())) / len(text.split()) > 0.7,
}

def verify_instruction_following(text: str, instructions: list[dict]) -> float:
    results = []
    for inst in instructions:
        verifier = VERIFIERS[inst['type']]
        args = inst.get('args', {})
        results.append(verifier(text, **args))
    
    # Partial credit for partial satisfaction
    fraction_met = sum(results) / len(results)
    # Bonus for meeting ALL constraints
    all_met_bonus = 0.2 if all(results) else 0.0
    
    return min(1.0, fraction_met * 0.8 + all_met_bonus)
```

**Verification confidence: HIGH.** Each individual instruction verifier is deterministic. The only soft spots are sentence boundary detection (use a standard tokenizer like NLTK punkt) and language detection (use fasttext or langdetect). Everything else is exact string operations.

## Dataset Sources

- **IFEval (Google, 2023):** ~500 prompts with 25 types of verifiable instructions. The foundational benchmark. Each prompt has 1-4 instructions, each with a programmatic verifier.
- **Template generation (primary for scale):** Define instruction templates and sample random combinations:
  ```
  "Write a {topic} that: (1) has exactly {N} words, (2) includes the word '{W}', 
   (3) is formatted as {K} bullet points, (4) does not use the letter '{L}'"
  ```
  Unlimited scale.
- **IFEval extensions:** Expand the 25 IFEval instruction types to 50+ with new verifiable constraints.
- **Alpaca/ShareGPT filtered:** Filter existing instruction datasets for prompts with verifiable requirements.
- **Writing prompts with constraints:** Combine creative prompts with mechanical constraints.
- **Technical writing with format requirements:** "Write a bug report with these sections: ...", "Summarize in exactly 3 bullet points."

## Task Format

**Input:**
```
Write a product description for a wireless Bluetooth speaker that:
1. Contains between 80 and 100 words
2. Has exactly 3 paragraphs
3. Includes the words "bass" and "portable"
4. Does not use the word "amazing"
5. Ends with a question
6. Uses at least 2 bullet points in the second paragraph
```

**Output:**
```
Introducing the SoundWave Pro, a portable Bluetooth speaker designed for music 
lovers who demand quality on the go. With deep bass response and crystal-clear 
highs, this speaker transforms any space into your personal concert hall.

Key features:
- 12-hour battery life for all-day listening
- IPX7 waterproof rating for outdoor adventures
- Dual drivers with enhanced bass technology

Whether you are at the beach, in the park, or relaxing at home, the SoundWave 
Pro delivers. Ready to upgrade your portable audio experience?
```

**Verification:** Word count (87, in range), paragraph count (3), "bass" present, "portable" present, "amazing" absent, ends with "?", 2 bullet points in P2. All pass.

## Difficulty Curriculum

1. **Easy:** Single instruction (word count, include word, format as list).
2. **Medium:** 2-3 compatible instructions (word count + required words + paragraph structure).
3. **Hard:** 4-6 instructions including some in tension (tight word count + many required words + structural constraints).
4. **Very hard:** 7+ instructions, some requiring careful planning (acrostic + word count + exclusions + format + language constraints). Conflicting constraints that require creative solutions.

Additional difficulty axes:
- Constraint tightness (exact word count = 50 is harder than range 40-60)
- Constraint interaction (required words that are hard to use naturally with topic constraints)
- Output length (longer text with maintained constraints is harder)

## Limitations & Risks

- **Coherence is not verified:** The model can satisfy all constraints with incoherent text. This is the fundamental limitation shared with [[constrained-writing]].
- **Instruction type coverage:** The set of verifiable instructions is finite. Open-ended instructions ("make it persuasive", "be creative") are outside RLVR scope.
- **Sentence boundary detection:** Different tokenizers may disagree on sentence count. Standardize on one.
- **Overfitting to instruction types:** Model may learn to game specific verifiers rather than develop general instruction-following ability. Mitigation: large variety of instruction types and random combinations.
- **Reward hacking:** Model may learn templates that satisfy common constraint combinations mechanically. Mitigation: vary topics, constraint parameters, and combinations.

## Connections

- [[constrained-writing]] — Instruction following subsumes constrained writing; constrained writing is the creative-focused subset.
- [[poetry-formal]] — Formal poetry constraints are a specific instruction type.
- [[data-formatting]] — Format instructions ("output as JSON", "use CSV") are one instruction type.
- [[legal-logic]] — Applying rules to produce correct output is a form of instruction following.
- [[accessibility-compliance]] — WCAG rules are instructions for HTML generation.
