---
domain: Automated Reasoning
category: Logic / General
verification_type: rule
dataset_scale: ~50K existing + unlimited procedural
difficulty_range: easy/medium/hard/superhuman
modality: text (potentially multimodal)
status: strong_hypothesis
---

## Overview

Automated reasoning encompasses general-purpose reasoning benchmarks where the answer is deterministic and verifiable by rule: pattern completion (ARC-style), abstract reasoning, analogy tasks, logical deduction chains, and structured inference. The distinguishing feature from other logic domains is breadth — this is not limited to a single formal system (SAT, FOL, etc.) but covers the general capacity for systematic, rule-governed inference.

The flagship example is the Abstraction and Reasoning Corpus (ARC), where the model must infer transformation rules from input-output grid examples and apply them to a test input. Verification is exact grid match. ARC is notoriously hard for LLMs and is considered a benchmark for "general intelligence." Other benchmarks in this category include ACRE, bAbI, BIG-Bench Hard reasoning tasks, and procedurally generated logical reasoning chains.

This domain has growing RLVR relevance. The ARC Prize (2024-2025) has drawn significant attention, and RL-based approaches to ARC are active research. The key challenge is that these tasks require learning abstract rules from few examples, not just pattern matching over large datasets.

## Verification Mechanism

**Primary method: Exact match of output grid/sequence/answer against gold standard.**

```python
import numpy as np

def verify_arc_task(test_input: list[list[int]], model_output: list[list[int]], 
                    gold_output: list[list[int]]) -> float:
    """
    Verify an ARC (Abstraction and Reasoning Corpus) task.
    
    The model sees 2-5 input-output training pairs, then must produce
    the correct output for a test input.
    
    Verification: exact grid match. Every cell must be correct.
    """
    if len(model_output) != len(gold_output):
        return 0.0
    
    for i in range(len(gold_output)):
        if len(model_output[i]) != len(gold_output[i]):
            return 0.0
        for j in range(len(gold_output[i])):
            if model_output[i][j] != gold_output[i][j]:
                return 0.0
    
    return 1.0


def verify_pattern_completion(sequence: list, model_next: list, 
                              gold_next: list) -> float:
    """
    Verify a pattern/sequence completion task.
    
    E.g., given [2, 6, 18, 54, ?], answer is 162.
    """
    if model_next == gold_next:
        return 1.0
    return 0.0


def verify_logical_deduction(premises: list[str], conclusion: str,
                              model_answer: str, gold_answer: str) -> float:
    """
    Verify a logical deduction task.
    
    Given premises and a query, the model must derive the correct answer.
    The answer is deterministic given the premises.
    
    E.g.:
    Premises: "All bloops are razzles. All razzles are lazzles."
    Query: "Are all bloops lazzles?"
    Gold: "Yes"
    """
    model_normalized = normalize_logic_answer(model_answer)
    gold_normalized = normalize_logic_answer(gold_answer)
    
    return 1.0 if model_normalized == gold_normalized else 0.0


def verify_analogy(a: str, b: str, c: str, model_d: str, gold_d: str) -> float:
    """
    Verify an analogy: A is to B as C is to D.
    
    Only works when the analogy has a single correct answer.
    """
    return 1.0 if model_d.strip().lower() == gold_d.strip().lower() else 0.0


def verify_rule_application(rule_description: str, input_data: dict,
                            model_output: dict, gold_output: dict) -> float:
    """
    Verify that a model correctly applied a stated rule.
    
    The rule is given explicitly (unlike ARC where it must be inferred).
    Verification: check output matches gold, OR independently apply the rule.
    """
    # Option 1: Compare to gold
    if model_output == gold_output:
        return 1.0
    
    # Option 2: Apply rule programmatically and compare
    try:
        rule_fn = parse_rule_to_function(rule_description)
        expected = rule_fn(input_data)
        return 1.0 if model_output == expected else 0.0
    except:
        return 0.0


def verify_babi_task(story: str, question: str, model_answer: str, 
                     gold_answer: str) -> float:
    """
    Verify a bAbI-style task (simple text reasoning).
    
    20 task types: single supporting fact, two supporting facts,
    counting, path finding, etc.
    """
    return 1.0 if model_answer.strip().lower() == gold_answer.strip().lower() else 0.0
```

### ARC-Specific Verification Details

```python
def verify_arc_with_partial_credit(model_output, gold_output) -> float:
    """
    ARC verification with optional partial credit.
    
    Partial credit options:
    1. Fraction of correct cells (not standard but useful for RL)
    2. Correct grid dimensions (small signal for very wrong answers)
    3. Correct color palette usage
    """
    # Standard: binary
    if model_output == gold_output:
        return 1.0
    
    # Partial credit option (useful for RL reward shaping):
    if len(model_output) == len(gold_output) and all(
        len(model_output[i]) == len(gold_output[i]) for i in range(len(gold_output))
    ):
        # Same dimensions — give partial credit for correct cells
        total_cells = sum(len(row) for row in gold_output)
        correct_cells = sum(
            1 for i in range(len(gold_output)) 
            for j in range(len(gold_output[i]))
            if model_output[i][j] == gold_output[i][j]
        )
        return correct_cells / total_cells * 0.5  # Max 0.5 for partial credit
    
    return 0.0
```

## Dataset Sources

| Dataset | Size | Source | Notes |
|---------|------|--------|-------|
| **ARC (Abstraction and Reasoning Corpus)** | 800 (400 train, 400 eval) | [github.com/fchollet/ARC-AGI](https://github.com/fchollet/ARC-AGI) | Chollet's benchmark. 10 colors, grids up to 30x30. |
| **ARC-AGI-2** (2025) | 800+ | [arcprize.org](https://arcprize.org/) | Updated version with harder tasks |
| **1D-ARC** | ~1,000 | Various | Simplified 1D version of ARC |
| **LARC (Language ARC)** | 400 | [github.com/samacqua/LARC](https://github.com/samacqua/LARC) | ARC tasks with natural language descriptions |
| **ACRE** (Abstract Causal Reasoning) | 36,000 | [github.com/WellyZhang/ACRE](https://github.com/WellyZhang/ACRE) | Causal reasoning on Blicket-style tasks |
| **bAbI** (Weston et al., 2016) | 20 task types, 1K-10K each | [github.com/facebookresearch/bAbI-tasks](https://github.com/facebookresearch/bAbI-tasks) | 20 categories of text reasoning |
| **BIG-Bench Hard (BBH)** | ~6,500 | [github.com/suzgunmirac/BIG-Bench-Hard](https://github.com/suzgunmirac/BIG-Bench-Hard) | 23 hard tasks from BIG-Bench |
| **RAVEN** (Zhang et al., 2019) | 70,000 | [github.com/WellyZhang/RAVEN](https://github.com/WellyZhang/RAVEN) | Progressive matrices (visual IQ test) |
| **Concept ARC** | 480 | [github.com/victorvikram/ConceptARC](https://github.com/victorvikram/ConceptARC) | ARC organized by concept |
| **PrOntoQA** (Saparov & He, 2023) | Unlimited | [github.com/asaparov/prontoqa](https://github.com/asaparov/prontoqa) | Procedurally generated logical reasoning chains |
| **BoardgameQA** | ~20K | Various | Board game state reasoning |
| **RuleTaker** (Clark et al., 2020) | ~500K | Allen AI | Synthetic reasoning over rules |
| **LogiQA** (Liu et al., 2020) | 8,678 | GitHub | Logical reasoning questions |
| **ReClor** (Yu et al., 2020) | 6,138 | [github.com/yuweihao/reclor](https://github.com/yuweihao/reclor) | LSAT/GMAT logical reasoning |

### Procedural Generation

```python
def generate_pattern_completion(difficulty="medium"):
    """Generate a deterministic pattern completion task."""
    
    if difficulty == "easy":
        # Arithmetic/geometric sequences
        seq_type = random.choice(["arithmetic", "geometric", "fibonacci_like"])
        
        if seq_type == "arithmetic":
            start = random.randint(1, 50)
            diff = random.randint(1, 10)
            seq = [start + i * diff for i in range(6)]
            return {
                "sequence": seq[:5],
                "answer": seq[5],
                "rule": f"arithmetic sequence with common difference {diff}"
            }
        elif seq_type == "geometric":
            start = random.randint(1, 5)
            ratio = random.randint(2, 5)
            seq = [start * ratio**i for i in range(6)]
            return {"sequence": seq[:5], "answer": seq[5]}
    
    elif difficulty == "medium":
        # Composite rules: alternating, nested, modular
        rule_type = random.choice(["alternating", "polynomial", "modular"])
        
        if rule_type == "alternating":
            a_start, a_diff = random.randint(1, 10), random.randint(1, 5)
            b_start, b_diff = random.randint(1, 10), random.randint(1, 5)
            seq = []
            for i in range(8):
                if i % 2 == 0:
                    seq.append(a_start + (i // 2) * a_diff)
                else:
                    seq.append(b_start + (i // 2) * b_diff)
            return {"sequence": seq[:7], "answer": seq[7]}


def generate_logical_deduction(depth=3, n_entities=5):
    """
    Generate a multi-step logical deduction problem.
    
    Creates a set of premises (facts + rules) and a query
    whose answer is deterministic.
    """
    entities = [f"entity_{i}" for i in range(n_entities)]
    properties = ["red", "blue", "green", "large", "small", "round", "square"]
    
    # Generate random facts
    facts = {}
    for e in entities:
        facts[e] = set(random.sample(properties, random.randint(1, 3)))
    
    # Generate rules: "if X has property A, then X has property B"
    rules = []
    for _ in range(depth):
        src_prop = random.choice(properties)
        tgt_prop = random.choice([p for p in properties if p != src_prop])
        rules.append((src_prop, tgt_prop))
    
    # Apply rules to derive new facts (forward chaining)
    changed = True
    while changed:
        changed = False
        for src, tgt in rules:
            for e in entities:
                if src in facts[e] and tgt not in facts[e]:
                    facts[e].add(tgt)
                    changed = True
    
    # Generate query
    query_entity = random.choice(entities)
    query_property = random.choice(properties)
    answer = query_property in facts[query_entity]
    
    return {
        "facts": {e: list(props) for e, props in facts.items()},
        "rules": rules,
        "query": f"Does {query_entity} have property {query_property}?",
        "answer": answer
    }
```

## Task Format

**ARC-style**:
```
Training examples:
Input 1:  [[0,0,1],   Output 1: [[1,0,0],
            [0,1,0],              [0,1,0],
            [1,0,0]]              [0,0,1]]

Input 2:  [[2,0],     Output 2: [[0,2],
            [0,2]]                [2,0]]

Test Input: [[0,3,0],
              [3,0,3],
              [0,3,0]]

What is the test output?
```

**Logical deduction**:
```
Facts:
- Alice is a teacher.
- All teachers are educated.
- All educated people read books.
- Bob is a doctor.
- All doctors are educated.

Query: Does Bob read books?
Answer: Yes
```

**Pattern completion**:
```
Sequence: 2, 6, 18, 54, 162, ?
Answer: 486 (geometric, ratio 3)
```

## Difficulty Curriculum

| Level | Type | Current SOTA Performance | Scale |
|-------|------|-------------------------|-------|
| Easy | bAbI tasks 1-10, simple sequences | >99% (most LLMs) | Unlimited |
| Medium | bAbI tasks 11-20, BBH tasks, multi-step deduction | 50-90% | ~10K |
| Hard | ARC easy tasks, complex logical chains | 20-50% | ~400 |
| Very Hard | ARC hard tasks, novel abstract rules | 5-20% | ~400 |
| Superhuman | ARC-AGI-2 hard tasks | <5% | ~200 |

**ARC-specific**: As of 2025, the best systems achieve ~50% on ARC evaluation set (combining program synthesis with LLMs). The ARC Prize target was 85%.

## Limitations & Risks

1. **ARC is tiny**: Only 800 tasks in the original dataset. This is far too small for standard RLVR training. Augmentation (transformations, reflections, re-coloring) helps but has limits. Procedural generation of ARC-like tasks is an active research challenge.
2. **Few-shot generalization vs. RL**: ARC is designed as a few-shot task — learn a rule from 2-5 examples. Standard RL training (many rollouts on many tasks) may not develop this capability. The learning signal is "did you get this specific grid right," not "did you learn to learn."
3. **Exact match is harsh**: For ARC, a single wrong cell means 0 reward. This makes the reward very sparse, especially for larger grids. Partial credit (fraction of correct cells) can help but may encourage surface-level matching.
4. **Domain-specific reasoning**: ARC requires a specific kind of visual/spatial reasoning about grids. This may not transfer to other reasoning domains. A model good at ARC may not be good at logical deduction, and vice versa.
5. **Verification is trivial but generation is hard**: The verification function is just array equality — the complexity is entirely in the task, not the verifier. This means the RL challenge is all exploration, with minimal help from the reward function.
6. **Heterogeneous task distribution**: Unlike SAT or math where all problems use the same formalism, automated reasoning tasks span many different formats and rule systems. A single model must handle diverse task types.

## Connections

- **logic-propositional.md**: Many deduction tasks reduce to propositional reasoning.
- **logic-first-order.md**: Multi-step deduction with quantifiers is FOL reasoning.
- **logic-puzzles.md**: Constraint puzzles are a structured subset of abstract reasoning.
- **math-competition.md**: Competition math requires creative reasoning that overlaps with abstract reasoning.
- **graph-theory.md**: ARC tasks involving connectivity, paths, or graph-like structures connect to graph theory.
