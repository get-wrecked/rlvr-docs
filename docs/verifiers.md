# Verifier Infrastructure

## Design Principles

Every verifier satisfies four properties:

1. **Deterministic**: Same input always produces the same score
2. **Zero false positives**: A score of 1.0 means the answer is definitively correct
3. **Tested against real data**: Every verifier has anti-hardcoding tests and is validated on real benchmark problems
4. **Fast**: Verification in milliseconds (except code execution, which uses subprocess timeout)

## Core Types

```rust
/// Result of a verification: a score from 0.0 (completely wrong) to 1.0 (perfect).
pub struct VerifyResult {
    pub score: f64,
    pub reason: String,
}

impl VerifyResult {
    pub fn correct() -> Self { ... }        // score: 1.0
    pub fn wrong(reason: &str) -> Self { ... }  // score: 0.0
    pub fn partial(score: f64, reason: &str) -> Self { ... }  // score: 0.0-1.0
}
```

## Verifier Catalog

### math_numerical (26 tests)
Extracts and compares numeric values from model output.

**Extraction pipeline:**
1. Check for `\boxed{...}` format (LaTeX math)
2. Check for `#### <number>` format (GSM8K convention)
3. Check for "The answer is X" pattern
4. Fall back to last number in response

**Comparison:** Integer problems require exact match. Real-valued problems use relative + absolute tolerance (rtol=1e-5, atol=1e-8).

**Validated against**: 1,319 real GSM8K test problems.

### math_equivalence (18 tests)
Tests symbolic equivalence of mathematical expressions.

**Techniques:**
- LaTeX normalization (`\frac{1}{2}` → `0.5`)
- Expression simplification
- Numerical evaluation at random points for symbolic expressions
- Handles equivalent forms: `x^2 + 2x + 1` = `(x+1)^2`

### exact_match (27 tests)
Normalized string comparison for factual answers.

**Normalization pipeline:**
1. Convert to lowercase
2. Remove articles ("a", "an", "the")
3. Strip punctuation
4. Collapse whitespace
5. Handle multi-answer questions (any correct answer scores 1.0)

**Also computes F1 score** for partial credit on token overlap.

### instruction_following (21 tests)
Checks whether model output satisfies a set of explicit constraints.

**15 constraint types:**
- Word count (min/max/exact)
- Sentence count
- Paragraph count
- Must include/exclude specific words or phrases
- Must start/end with specific text
- Format requirements (all caps, bullet points, numbered list)
- Language constraints
- Must contain specific sections/headers
- Response length in characters

**Validated against**: IFEval benchmark (541 problems).

### json_schema (20 tests)
Validates JSON output against a JSON Schema specification.

**Checks:**
- Type validation (string, number, integer, boolean, array, object, null)
- Required fields
- Numeric ranges (minimum, maximum)
- String patterns (regex)
- Array constraints (minItems, maxItems, uniqueItems)
- Nested object validation
- Enum values

### code_execution (16 tests)
Executes code in a sandboxed subprocess and checks results.

**Modes:**
1. **Stdin/stdout**: Provide input, check output matches expected
2. **Function call**: Import function, call with test cases, check return values
3. **Test suite**: Run provided unit tests, report pass/fail

**Safety:** Subprocess with timeout (default 10s), resource limits, no network access.

**Languages supported**: Python (primary), with architecture for additional languages.

### sudoku (16 tests)
Validates completed Sudoku grids against puzzle constraints.

**Checks:**
1. All rows contain digits 1-9 with no repeats
2. All columns contain digits 1-9 with no repeats
3. All 3x3 boxes contain digits 1-9 with no repeats
4. Given (pre-filled) cells are respected
5. **Partial credit**: Score = fraction of valid rows + columns + boxes

### chemical_equation (15 tests)
Verifies that chemical equations are properly balanced.

**Process:**
1. Parse chemical formulas (including parenthetical groups like `Ca(OH)2`)
2. Extract element counts from both sides
3. Apply coefficients
4. Check atom conservation for every element

### regex_synthesis (13 tests)
Verifies that a generated regex matches the specification.

**Process:**
1. Compile the regex (invalid regex = score 0)
2. Anchor to full string (`^...$`)
3. Test against all positive examples (must match)
4. Test against all negative examples (must not match)
5. Score = fraction of examples correctly handled

### date_time (13 tests)
Verifies date/time computation results.

**Operations:**
- Days between two dates
- Day of week for a given date
- Date addition/subtraction
- Leap year detection
- Date arithmetic with months/years

Uses the `chrono` crate for ground truth computation.

### unit_conversion (12 tests)
Verifies physical unit conversions.

**Coverage:** 60+ unit pairs across:
- Length (meters, feet, inches, miles, km, etc.)
- Weight (kg, lbs, oz, tons, etc.)
- Temperature (Celsius, Fahrenheit, Kelvin — special-case formulas)
- Volume (liters, gallons, cups, etc.)
- Speed, pressure, energy, and more

### sql_execution (11 tests)
Executes SQL queries against an in-memory SQLite database and compares results.

**Process:**
1. Create in-memory SQLite database
2. Load schema and data from task specification
3. Execute both gold query and model query
4. Compare result sets (order-insensitive by default)

### graph_properties (10 tests)
Verifies graph algorithm results.

**Algorithms:**
- Shortest path (Dijkstra)
- Connected components
- Topological sort
- Graph coloring (chromatic number)
- Cycle detection
- Minimum spanning tree

## Verifier Server API

The verifiers are designed to run as an HTTP service:

```
POST /verify
{
    "domain": "math_numerical",
    "task": { ... },
    "response": "The answer is 42"
}
→ { "score": 1.0, "reason": "correct" }

POST /verify/batch
[{ "domain": "...", "task": ..., "response": ... }, ...]
→ [{ "score": ..., "reason": ... }, ...]

POST /sample
{
    "domain": "math_numerical",
    "difficulty": 5,
    "count": 32
}
→ [task1, task2, ...]

GET /health
→ { "domains": 272, "status": "ok" }
```

## Running Tests

```bash
# All verifiers
cargo test

# Specific verifier
cargo test math_numerical
cargo test sudoku
cargo test code_execution

# With output
cargo test -- --nocapture
```

## Verification Types Taxonomy

| Type | Mechanism | False Positive Rate | Example |
|------|-----------|-------------------|---------|
| Formal proof checking | Lean/Coq type-checker | 0% | Theorem proving |
| Execution + test suite | Code runs, tests pass | ~0% (if tests are good) | Code generation |
| Exact match | Compare to known answer | 0% (possible false negatives) | Math, QA |
| Constraint satisfaction | All constraints verified | 0% | Sudoku, scheduling |
| Simulation-based | Simulator confirms specs | ~0% (within sim fidelity) | Circuit design |
| Schema validation | Conforms to formal schema | 0% for structure | JSON/XML output |
| Metric-based | BLEU, ROUGE, IoU | Noisy | Translation |

For AGI training, maximize time in the top 5 categories.
