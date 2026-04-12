---
domain: Regex Synthesis
category: Code & Software
verification_type: execution-based (match/reject against positive and negative examples)
dataset_scale: 1K–50K problems
difficulty_range: simple character classes → complex lookaheads with backreferences
modality: text/examples-to-regex
status: emerging
---

## Overview

Regex synthesis tasks the model with generating a regular expression that matches a specified set of positive examples and rejects a set of negative examples (and optionally satisfies a natural-language description). Verification is fully automatic: compile the regex, test it against all examples, and check that every positive matches and every negative does not.

This domain is a clean RLVR target: the verifier is trivial to implement, execution is near-instant, and the search space is large enough that RL can meaningfully explore. The practical value is also clear — developers frequently struggle with regex and turn to StackOverflow or AI assistants for help.

## Verification Mechanism

```
def verify_regex(generated_regex, positive_examples, negative_examples,
                 timeout=5):
    # 1. Compile the regex
    try:
        pattern = re.compile(generated_regex, re.DOTALL)
    except re.error:
        return 0.0  # invalid regex syntax

    # 2. Test against positive examples (must all match)
    for pos in positive_examples:
        try:
            match = run_with_timeout(
                lambda: pattern.fullmatch(pos),  # or .search() depending on task
                timeout=timeout
            )
        except TimeoutError:
            return 0.0  # catastrophic backtracking
        if match is None:
            return 0.0  # false negative

    # 3. Test against negative examples (must all reject)
    for neg in negative_examples:
        try:
            match = run_with_timeout(
                lambda: pattern.fullmatch(neg),
                timeout=timeout
            )
        except TimeoutError:
            return 0.0
        if match is not None:
            return 0.0  # false positive

    return 1.0
```

For tasks with a natural-language description and no explicit examples, a supplementary check uses held-out test strings:

```
def verify_regex_with_held_out(generated_regex, description,
                               held_out_positives, held_out_negatives):
    """
    The model sees only the description; held-out examples
    are used solely for verification.
    """
    # Same matching logic as above, but against held-out sets
    return verify_regex(generated_regex, held_out_positives,
                        held_out_negatives)
```

Key considerations:

- **Catastrophic backtracking**: Malicious or poorly constructed regexes can cause exponential runtime on certain inputs. A strict timeout (1–5 seconds per test string) prevents this from stalling verification.
- **fullmatch vs. search**: Task definition must specify whether the regex should match the entire string or find a substring. Most synthesis tasks use fullmatch.
- **Regex flavor**: Python `re`, PCRE, JavaScript, and POSIX regexes differ in feature support. Task should specify the target engine.
- **Overfitting to examples**: A regex that enumerates all positive strings literally (e.g., `(foo|bar|baz)`) is technically correct but useless. Penalizing regex length or testing against held-out examples mitigates this.

## Dataset Sources

| Dataset | Size | Source | Notes |
|---------|------|--------|-------|
| **RegexLib** | ~5,000 annotated regexes | RegexLib.com | Community-contributed patterns with descriptions |
| **StackOverflow Regex Q&A** | ~50K posts | StackOverflow | Mined regex tag; question = description, answer = regex, test strings in post |
| **KB13** | 824 regex pairs | NLP regex corpus | Natural-language descriptions paired with regexes |
| **NL-RX** | 10,000 pairs | Synthetic | Template-generated NL descriptions and regexes |
| **Structured Regex** | 3,000+ | Academic | From regex synthesis competitions and PBE benchmarks |
| **Regex Golf** | ~100 problems | regex golf challenges | Match one set of strings, reject another; competitive |
| **AlphaRegex** | 2,800 problems | Academic | Benchmark for regex synthesis from examples |
| **SO-Regex** | 8,898 pairs | StackOverflow mined | Question-regex pairs with test cases extracted |

**Synthetic data generation**: Generate random regexes, produce positive examples by sampling matches and negative examples by sampling near-misses (edit distance 1–2 from positives). This can scale to millions of training instances.

## Task Format

**Variant A: Examples-based (Programming by Example)**
```
Write a regex that matches all positive examples and rejects
all negative examples.

Positive: ["2024-01-15", "1999-12-31", "2000-06-01"]
Negative: ["24-01-15", "2024/01/15", "2024-13-01", "hello"]
```

**Variant B: Description-based**
```
Write a regex that matches valid ISO 8601 dates in YYYY-MM-DD
format where month is 01-12 and day is 01-31.
```

**Variant C: Regex Golf**
```
Write the shortest regex that matches all strings in set A
but none in set B.

A: ["foo", "foobar", "foobaz"]
B: ["bar", "baz", "fo", "fob"]
```

**Expected output**: A single regex string, e.g., `\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])`

## Difficulty Curriculum

| Level | Features | Example Pattern |
|-------|----------|----------------|
| 1 | Literals, character classes | `[aeiou]+` |
| 2 | Quantifiers, alternation | `(cat|dog)\s+\d+` |
| 3 | Anchors, groups, backreferences | `^(\w+)\s+\1$` |
| 4 | Lookahead/lookbehind | `(?<=@)\w+\.\w+` |
| 5 | Complex nested groups | Email/URL validation patterns |
| 6 | Performance-aware patterns | Avoid catastrophic backtracking on adversarial inputs |

For RL curriculum, start with character-class-level problems and progress through features. The reward is always binary (all examples match/reject correctly), so curriculum controls difficulty via regex feature complexity and number of examples.

## Limitations & Risks

- **Overfitting**: A regex like `(exact_string_1|exact_string_2|...)` passes verification but isn't useful. Mitigations: held-out test strings, regex length penalty, requiring the regex to be shorter than the enumeration of examples.
- **Ambiguous specifications**: Multiple regexes can satisfy the same set of examples. The "intended" pattern may differ from what the model produces. More examples reduce ambiguity.
- **Regex readability**: RL may produce correct but unreadable regexes. If readability matters, a supplementary metric (e.g., regex complexity score) can be added.
- **Limited dataset scale**: Compared to code generation, regex datasets are smaller. Synthetic generation helps but may not capture the distribution of real-world regex needs.
- **Flavor divergence**: A regex valid in Python may not work in JavaScript or POSIX. The task must specify and the verifier must use the correct engine.

## Connections

- Shares the "programming by example" paradigm with **Data Wrangling** (transform data based on examples).
- **Shell Commands** often involve regex for text processing (grep, sed, awk), creating a natural interface between the two domains.
- The examples-based specification is similar to **Test Generation** in reverse: here the tests (examples) are given and the code (regex) is generated.
- **SQL Generation** shares the "formal language from natural language" structure.
- Regex can be a subtask within **Web Scraping** (extracting patterns from HTML text).
