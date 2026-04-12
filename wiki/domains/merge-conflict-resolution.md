---
domain: Merge Conflict Resolution
category: code
verification_type: diff
dataset_scale: millions of resolutions (extractable from GitHub)
difficulty_range: easy/medium/hard
modality: code
status: strong_hypothesis
---

# Merge Conflict Resolution

## Overview

Merge conflict resolution takes a git-style merge conflict (showing the base version, "ours", and "theirs" for conflicting code regions) and produces a correctly resolved version. The model must understand both code semantics and developer intent to produce a resolution that correctly integrates changes from both branches.

RLVR verification compares the model's resolution against the human-resolved version from the actual merge commit. Exact match (after whitespace normalization) is the strictest metric; diff-based similarity and test-suite passing provide softer signals. The training data is essentially unlimited: every merge commit on GitHub with conflicts provides a (conflict, resolution) pair.

## Verification Mechanism

```python
import difflib

def verify_merge_resolution(predicted: str, gold_resolution: str,
                             test_suite_fn=None) -> dict:
    """
    predicted: the resolved code
    gold_resolution: the human-written resolution from the actual merge commit
    """
    # Normalize whitespace for comparison
    pred_lines = [l.rstrip() for l in predicted.strip().splitlines()]
    gold_lines = [l.rstrip() for l in gold_resolution.strip().splitlines()]

    # Exact match
    exact = pred_lines == gold_lines

    # Line-level similarity (SequenceMatcher)
    sm = difflib.SequenceMatcher(None, pred_lines, gold_lines)
    similarity = sm.ratio()

    # Character-level edit distance for fine-grained comparison
    pred_str = '\n'.join(pred_lines)
    gold_str = '\n'.join(gold_lines)
    char_sm = difflib.SequenceMatcher(None, pred_str, gold_str)
    char_similarity = char_sm.ratio()

    result = {
        "exact_match": exact,
        "line_similarity": similarity,
        "char_similarity": char_similarity,
        "reward": 1.0 if exact else similarity
    }

    # Optional: run test suite
    if test_suite_fn is not None:
        tests_pass = test_suite_fn(predicted)
        result["tests_pass"] = tests_pass
        # Tests passing is more important than exact match
        if tests_pass and not exact:
            result["reward"] = max(result["reward"], 0.8)

    return result

def extract_conflict_regions(conflict_text: str) -> dict:
    """Parse git conflict markers."""
    ours_lines, theirs_lines, base_lines = [], [], []
    section = None
    for line in conflict_text.splitlines():
        if line.startswith('<<<<<<<'):
            section = 'ours'
        elif line.startswith('|||||||'):
            section = 'base'
        elif line.startswith('======='):
            section = 'theirs'
        elif line.startswith('>>>>>>>'):
            section = None
        elif section == 'ours':
            ours_lines.append(line)
        elif section == 'base':
            base_lines.append(line)
        elif section == 'theirs':
            theirs_lines.append(line)
    return {"ours": ours_lines, "theirs": theirs_lines, "base": base_lines}
```

## Dataset Sources

- **GitHub merge commits**: Millions of merge commits with conflicts can be extracted from public GitHub repositories. For each merge commit, reconstruct the conflict by replaying the merge with `git merge --no-commit`, then compare against the actual resolution. DeepMerge (Svyatkovskiy et al., 2022) provides a methodology.
- **MergeBot dataset**: ~10K merge conflicts from 40 popular Java repositories with human resolutions.
- **MergeBERT training data**: Curated merge conflicts from Python and JavaScript repositories.
- **IntelliMerge**: Merge conflict dataset from large-scale Java projects (Zhang et al., 2022).
- **Synthetic conflicts**: Generate controlled conflicts by:
  1. Take a file at version A
  2. Apply two independent edits creating versions B and C
  3. The merge conflict is B vs. C with base A
  4. The resolution is the manually integrated version
- **Open-source project histories**: Projects like Linux kernel, Chromium, and VS Code have extensive merge histories with complex conflicts.

## Task Format

- **Input**: A merge conflict with base, ours, and theirs versions.
```
Resolve this merge conflict:

<<<<<<< HEAD
def calculate_total(items, tax_rate=0.08):
    subtotal = sum(item.price for item in items)
    tax = subtotal * tax_rate
    return subtotal + tax
||||||| base
def calculate_total(items):
    subtotal = sum(item.price for item in items)
    return subtotal
=======
def calculate_total(items):
    subtotal = sum(item.price * item.quantity for item in items)
    discount = apply_discount(subtotal)
    return subtotal - discount
>>>>>>> feature-branch
```
- **Output**: Resolved code integrating both changes.
```
def calculate_total(items, tax_rate=0.08):
    subtotal = sum(item.price * item.quantity for item in items)
    discount = apply_discount(subtotal)
    tax = (subtotal - discount) * tax_rate
    return subtotal - discount + tax
```

## Difficulty Curriculum

- Level 1: Non-overlapping changes (one side adds a line, the other modifies a different line)
- Level 2: Simple overlapping changes to the same line (variable rename on both sides)
- Level 3: One side adds code, the other modifies adjacent code
- Level 4: Both sides modify the same function with different features
- Level 5: Structural changes (one side refactors, the other adds features to the old structure)
- Level 6: Import/dependency conflicts (both sides add different imports or dependencies)
- Level 7: Multi-hunk conflicts within a single file (multiple conflict regions that interact)
- Level 8: Cross-file conflicts where resolution of one file depends on changes in another
- Level 9: Semantic conflicts (no textual conflict, but combined changes break behavior -- requires understanding program semantics)

## Limitations & Risks

- **Multiple valid resolutions**: There are often many correct ways to resolve a conflict. Exact match against one human resolution penalizes equally valid alternatives. Test-suite verification helps but requires a test suite.
- **Semantic understanding required**: Truly correct resolution requires understanding what both changes intend to achieve and how they interact. Text-level merging without semantic understanding produces plausible but buggy code.
- **Test coverage**: Verifying resolution correctness via tests requires the project's test suite, which is not always available, may not cover the changed code, or may have its own conflicts.
- **Context window**: Complex conflicts require understanding the broader codebase (types, interfaces, other files). A single conflict region may be insufficient context.
- **Language diversity**: Conflict resolution strategies differ by programming language (Python whitespace sensitivity, C++ template complexity, etc.). A single model must handle all languages.
- **Trivial vs. interesting conflicts**: Most real-world merge conflicts are trivial (non-overlapping changes, formatting differences). The interesting cases (semantic conflicts) are rare and hard to verify.

## Connections

- [[code-generation]] — Both require generating correct code; conflict resolution adds the constraint of integrating two existing versions
- [[code-debugging]] — Incorrect merge resolutions introduce bugs; debugging is the downstream task
- [[api-usage]] — API changes frequently cause merge conflicts
- [[build-configuration]] — Build file conflicts (package.json, pom.xml) are a common special case
