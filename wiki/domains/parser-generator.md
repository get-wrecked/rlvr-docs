---
domain: Parser Generator
category: Code & Software
verification_type: execution
dataset_scale: ~200+ grammars in ANTLR repository
difficulty_range: medium/hard
modality: code
status: strong_hypothesis
---

# Parser Generator

## Overview
Given a grammar specification (BNF/EBNF) or example input/output pairs, generate a working parser. Verification via parse success/failure on test corpora and AST structure comparison.

## Verification Mechanism
1. Parser correctly accepts valid inputs from test suite
2. Parser correctly rejects invalid inputs
3. Parsed AST matches expected structure for each input

## Dataset Sources
- ANTLR grammar repository (200+ grammars)
- tree-sitter grammar collection
- University compiler course assignments
- Parsing benchmarks

## Task Format
**Input**: Grammar specification (BNF/EBNF) or example input/output pairs
**Output**: Parser implementation (PEG, recursive descent, LALR)

## Difficulty Curriculum
1. Simple arithmetic expression parser
2. JSON/XML parser
3. Programming language subset
4. Ambiguous grammar handling
5. Error recovery and reporting

## Connections
[[compiler-construction]], [[formal-grammar]], [[formal-language-theory]]
