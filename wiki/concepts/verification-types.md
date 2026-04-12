---
concept: Verification Types
description: Taxonomy of verification mechanisms for RLVR rewards
---

# Verification Types

The heart of RLVR is the verification function: given a task and a proposed solution, compute a reward without human involvement. This page catalogs all verification types used across domains in this wiki.

## 1. Exact Match

**Mechanism**: Compare output string to known correct answer(s).

**Implementation**:
```python
def verify(output: str, gold: str | list[str]) -> float:
    normalized_output = normalize(output)  # strip whitespace, lowercase, etc.
    if isinstance(gold, list):
        return 1.0 if normalized_output in [normalize(g) for g in gold] else 0.0
    return 1.0 if normalized_output == normalize(gold) else 0.0
```

**Strengths**: Zero false positives. Trivial to implement.
**Weaknesses**: False negatives when there are multiple valid forms (e.g., "1/2" vs "0.5" vs "one half"). Mitigated by normalization and accepting multiple gold answers.

**Used in**: Math (numerical answers), factual QA, trivia, text classification, cloze completion, entity linking.

## 2. Execution-Based

**Mechanism**: Execute the output (as code) and verify behavior.

**Subtypes**:

### 2a. Test Suite Execution
```python
def verify(code: str, tests: list[TestCase]) -> float:
    try:
        result = execute_in_sandbox(code, timeout=30)
        passed = sum(1 for t in tests if run_test(result, t))
        return passed / len(tests)
    except (SyntaxError, RuntimeError, TimeoutError):
        return 0.0
```

### 2b. Output Comparison
```python
def verify(code: str, input_data: Any, expected_output: Any) -> float:
    actual = execute_in_sandbox(code, input_data, timeout=30)
    return 1.0 if actual == expected_output else 0.0
```

### 2c. Property Testing
```python
def verify(code: str, properties: list[Callable]) -> float:
    for prop in properties:
        for _ in range(100):  # random test cases
            if not prop(code):
                return 0.0
    return 1.0
```

**Strengths**: Very reliable. Tests specify behavior precisely.
**Weaknesses**: Requires sandboxing. Tests may not cover all edge cases. Time limits needed.

**Used in**: Code generation, SQL, regex, shell commands, data wrangling, web scraping, compilers.

## 3. Formal Verification

**Mechanism**: A proof checker / type checker verifies the output.

```python
def verify(proof: str, theorem: str, system: str = "lean4") -> float:
    result = run_proof_checker(system, theorem, proof, timeout=300)
    return 1.0 if result.success else 0.0
```

**Strengths**: Mathematically perfect. Zero false positives, zero false negatives.
**Weaknesses**: Limited to domains with formal specifications. Proof search is expensive.

**Used in**: Formal theorem proving (Lean4, Coq, Isabelle), TLA+ specifications, Alloy models.

## 4. Constraint Satisfaction

**Mechanism**: Check that output satisfies all stated constraints.

```python
def verify(solution: Any, constraints: list[Constraint]) -> float:
    satisfied = sum(1 for c in constraints if c.check(solution))
    return satisfied / len(constraints)  # or binary: all-or-nothing
```

**Strengths**: Very flexible. Can express complex requirements. No reference answer needed.
**Weaknesses**: Must define constraints precisely. Constraints may be incomplete (solution satisfies all constraints but is still "wrong" by intent).

**Used in**: Logic puzzles (Sudoku, etc.), scheduling, constrained writing, instruction following, combinatorial optimization.

## 5. Simulation-Based

**Mechanism**: Run a simulator and check if the output achieves desired properties.

```python
def verify(design: Any, specs: dict, simulator: str) -> float:
    result = run_simulation(simulator, design, timeout=600)
    score = 0.0
    for spec_name, spec_value in specs.items():
        if result.metrics[spec_name] meets spec_value:
            score += 1.0
    return score / len(specs)
```

**Strengths**: Can verify complex, continuous-valued outcomes. Bridges theory and practice.
**Weaknesses**: Simulation fidelity may not match reality. Can be slow. Some simulations are stochastic.

**Used in**: Physics, circuit design, control systems, fluid dynamics, robotics, structural engineering.

## 6. Rule-Based

**Mechanism**: Check output against a set of domain-specific rules.

```python
def verify(output: Any, rules: RuleSet) -> float:
    violations = rules.check(output)
    return 1.0 if len(violations) == 0 else max(0, 1 - len(violations) / rules.total)
```

**Strengths**: Encodes domain expertise. No reference answer needed.
**Weaknesses**: Rules may be incomplete. Hard to encode all rules for complex domains.

**Used in**: Chess (legal moves), music theory (voice leading), protocol compliance, accessibility, formal poetry.

## 7. Diff-Based (Structural Comparison)

**Mechanism**: Compare output structure to reference, allowing for superficial differences.

### 7a. AST Diff (for code)
```python
def verify(output_code: str, reference_code: str) -> float:
    output_ast = parse_ast(output_code)
    ref_ast = parse_ast(reference_code)
    return ast_similarity(output_ast, ref_ast)
```

### 7b. Visual Diff (for UI/documents)
```python
def verify(output_html: str, reference_screenshot: Image) -> float:
    rendered = headless_render(output_html)
    return structural_similarity(rendered, reference_screenshot)
```

### 7c. Schema Validation
```python
def verify(output: str, schema: dict) -> float:
    try:
        parsed = json.loads(output)
        jsonschema.validate(parsed, schema)
        return 1.0
    except (json.JSONDecodeError, jsonschema.ValidationError):
        return 0.0
```

**Used in**: HTML/CSS generation, document parsing, structured output, config generation.

## 8. Outcome-Based

**Mechanism**: Evaluate based on the end result of an action sequence.

```python
def verify(actions: list[Action], environment: Env) -> float:
    state = environment.reset()
    for action in actions:
        state = environment.step(action)
    return environment.reward(state)
```

**Strengths**: Naturally captures the full task. No need to specify intermediate steps.
**Weaknesses**: Sparse reward (only at end). Credit assignment is hard.

**Used in**: Games (win/loss), web navigation, computer use, GUI navigation, interactive fiction.

## 9. Metric-Based (Approximate)

**Mechanism**: Compute a continuous metric that correlates with quality.

```python
def verify(output: str, reference: str, metric: str = "bleu") -> float:
    if metric == "bleu":
        return sacrebleu.compute(output, reference)
    elif metric == "rouge":
        return rouge_scorer.score(output, reference)["rougeL"].fmeasure
    elif metric == "iou":
        return intersection_over_union(output_box, reference_box)
```

**Strengths**: Applicable to many domains. Continuous signal aids training.
**Weaknesses**: Imperfect correlation with true quality. Can be gamed. Not truly "verifiable" in the strict sense.

**Used in**: Translation, summarization, visual grounding, image similarity.

## Recommendation for RLVR Training

**Primary reward**: Use types 1-6 (exact, execution, formal, constraint, simulation, rule). These give clean, reliable signal.

**Supplementary reward**: Use types 7-9 (diff, outcome, metric) as auxiliary objectives or for domains where exact verification isn't possible.

**Never**: Use LLM-as-judge as a primary reward signal — it introduces the model's own biases into the training loop, creating a degenerate feedback cycle.
