---
title: Domain Matrix
description: Feature matrix of all 212 RLVR domains across key dimensions
---

# RLVR Domain Matrix

## Capability Dimensions

Each domain develops specific cognitive capabilities. For AGI, we need coverage across ALL dimensions:

### Dimension 1: Formal Reasoning
Domains that develop rigorous, step-by-step logical thinking:
- math-formal-proofs, logic-propositional, logic-first-order, smt-solving
- automated-theorem-proving, type-theory-puzzles, boolean-function-learning
- theorem-formalization, proof-repair, formal-language-theory
- mathematical-programming, proof-complexity
**~15 domains, strongest verification (formal/execution)**

### Dimension 2: Empirical Reasoning
Domains requiring reasoning about the physical world:
- physics-simulation, chemistry-computation, biology-sequence
- materials-science, fluid-dynamics, climate-science
- epidemiology-modeling, astronomy-computation
**~15 domains, simulation-based verification**

### Dimension 3: Computational Reasoning
Domains requiring algorithm design and implementation:
- code-generation, code-optimization, code-repair
- compiler-construction, competitive-coding-interactive
- database-query-optimization, program-synthesis-from-io
- graph-algorithm-execution, ml-pipeline-optimization
**~25 domains, execution-based verification**

### Dimension 4: Perceptual Reasoning
Domains requiring visual/spatial/auditory understanding:
- visual-question-answering, image-segmentation, 3d-scene-understanding
- chart-understanding, document-ocr-extraction, spatial-reasoning
- audio-speech-recognition, music-audio-processing
- optical-character-recognition, video-understanding
**~15 domains, metric-based verification**

### Dimension 5: Linguistic Reasoning
Domains requiring language understanding and generation:
- information-extraction, fact-verification, reading-comprehension
- natural-language-inference, semantic-parsing
- translation-reference, text-classification
**~25 domains, exact match verification**

### Dimension 6: Strategic Reasoning
Domains requiring planning, strategy, and decision-making:
- chess, go-game, multi-agent-coordination
- negotiation-bargaining, planning-classical
- scheduling, economic-modeling
**~10 domains, outcome/simulation verification**

### Dimension 7: Creative Reasoning (Constrained)
Domains requiring creativity within verifiable constraints:
- constrained-writing, poetry-formal, music-generation-midi
- game-design-level-generation, image-generation-constrained
- crossword-construction, cad-modeling
**~15 domains, constraint verification**

### Dimension 8: Agentic Capability
Domains requiring multi-step interaction with environments:
- web-navigation, computer-use, gui-navigation
- interactive-fiction, file-system-tasks
- workflow-automation, natural-language-to-api
**~12 domains, outcome verification**

### Dimension 9: Domain Expertise
Domains requiring deep specialized knowledge:
- medical-diagnosis, legal-reasoning, cybersecurity-ctf
- molecular-generation, protein-design, chip-design-rtl
- distributed-systems, financial-calculation
**~20 domains, various verification**

### Dimension 10: Meta-Cognitive
Domains requiring reasoning about reasoning:
- instruction-following, educational-content
- code-documentation, causal-reasoning
- scientific-experiment-design
- bayesian-network-reasoning
**~8 domains, constraint/execution verification**

## Coverage Analysis

| Dimension | # Domains | Data Availability | Verification Quality | Current RLVR Usage |
|-----------|-----------|-------------------|---------------------|--------------------|
| Formal Reasoning | 15 | Good (procedural) | Excellent | High (DeepSeek-R1) |
| Empirical Reasoning | 15 | Medium (simulation) | Good | Low |
| Computational Reasoning | 25 | Excellent (GitHub) | Excellent | High (code RL) |
| Perceptual Reasoning | 15 | Excellent (COCO etc) | Good | Low |
| Linguistic Reasoning | 25 | Excellent (web) | Good | Medium |
| Strategic Reasoning | 10 | Good (self-play) | Excellent | Medium (games) |
| Creative (Constrained) | 15 | Good (procedural) | Good | Very Low |
| Agentic Capability | 12 | Medium (benchmarks) | Good | Very Low |
| Domain Expertise | 20 | Good (exams/papers) | Good | Very Low |
| Meta-Cognitive | 8 | Medium | Good | Very Low |

**Key gaps**: Perceptual reasoning, agentic capability, and domain expertise are severely underrepresented in current RLVR training. This wiki fills those gaps.

## Priority Matrix

For maximum capability gain per engineering effort:

### Priority 1 (High impact, low effort — do first)
These have large datasets, simple verification, and broad capability development:
- instruction-following (IFEval — develops general instruction adherence)
- fact-verification (FEVER — develops factual reasoning)
- question-answering-closed (TriviaQA — develops knowledge recall)
- logic-puzzles (Sudoku etc. — develops constraint reasoning)
- commonsense-reasoning (multiple benchmarks — develops world knowledge)
- text-classification (many datasets — develops categorization)
- information-extraction (CoNLL etc. — develops structured thinking)

### Priority 2 (High impact, medium effort)
Strong datasets, moderate verification infrastructure needed:
- web-navigation (WebArena — develops agentic capability)
- visual-question-answering (VQAv2 — develops vision reasoning)
- sql-generation (Spider — develops structured querying)
- data-wrangling (DS-1000 — develops data skills)
- scientific-paper-extraction (SciERC — develops scientific reasoning)
- regex-synthesis (develops pattern thinking)
- shell-commands (develops system interaction)

### Priority 3 (High impact, high effort)
Significant infrastructure but very valuable:
- computer-use (OSWorld — develops general computer operation)
- physics-simulation (PyBullet — develops physical reasoning)
- circuit-design (SPICE — develops engineering thinking)
- molecular-generation (RDKit — develops scientific creativity)
- chip-design-rtl (Verilog/Iverilog — develops hardware thinking)
- distributed-systems (TLA+ — develops systems reasoning)

### Priority 4 (Research frontier)
Novel domains that push toward superhuman:
- theorem-discovery (pushing math boundaries)
- protein-design (pushing biology boundaries)
- scientific-experiment-design (pushing methodology)
- game-design-level-generation (pushing creativity)
- symbolic-regression (pushing pattern discovery)

## Interaction Effects

Training on multiple domains creates synergies:

1. **Math + Code** → Mathematical programming, symbolic computation
2. **Logic + Code** → Formal verification, type theory
3. **Science + Code** → Computational science, simulation
4. **Language + Code** → Semantic parsing, NL-to-code
5. **Vision + Code** → UI generation, chart understanding
6. **Games + Planning** → Strategic reasoning, multi-step thinking
7. **Security + Code** → Vulnerability analysis, exploitation
8. **Agent + Vision** → Computer use, GUI navigation

These interactions mean the total capability is **greater than the sum of individual domains**. This is why breadth matters for AGI.
