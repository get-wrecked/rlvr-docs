---
domain: Grammar Induction & Parsing
category: computational-linguistics
verification_type: execution
dataset_scale: 100K+ (from treebanks + parsing benchmarks)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Grammar Induction & Parsing

## Overview
Parse sentences into constituent or dependency trees, or induce grammars from examples. Verification: compare parsed tree to gold standard (exact match or labeled attachment score), or verify that an induced grammar generates the correct language.

## Verification Mechanism
```python
def verify(task_type: str, input_text: str, prediction: Any, gold: Any) -> float:
    if task_type == "dependency_parse":
        # Labeled Attachment Score (LAS)
        pred_deps = parse_conllu(prediction)
        gold_deps = parse_conllu(gold)
        correct = sum(1 for p, g in zip(pred_deps, gold_deps) 
                     if p.head == g.head and p.label == g.label)
        return correct / len(gold_deps)
    
    elif task_type == "constituency_parse":
        # F1 score on labeled brackets
        pred_tree = Tree.fromstring(prediction)
        gold_tree = Tree.fromstring(gold)
        pred_spans = extract_labeled_spans(pred_tree)
        gold_spans = extract_labeled_spans(gold_tree)
        
        precision = len(pred_spans & gold_spans) / max(len(pred_spans), 1)
        recall = len(pred_spans & gold_spans) / max(len(gold_spans), 1)
        return 2 * precision * recall / max(precision + recall, 1e-10)
    
    elif task_type == "grammar_induction":
        # Check if induced grammar generates all positive examples
        # and rejects all negative examples
        grammar = parse_grammar(prediction)
        parser = EarleyParser(grammar)
        
        for pos_example in gold["positive"]:
            if not parser.accepts(pos_example):
                return 0.0
        for neg_example in gold["negative"]:
            if parser.accepts(neg_example):
                return 0.0
        return 1.0
```

## Dataset Sources
- **Penn Treebank**: 40K parsed English sentences.
- **Universal Dependencies**: 200+ treebanks in 100+ languages.
- **OntoNotes**: 1.7M words with syntactic annotations.
- **English Web Treebank**: Web text with syntactic trees.
- **SPMRL**: Morphologically rich language parsing.
- **Grammar induction benchmarks**: Grammatical vs. ungrammatical sentence pairs.

## Task Format
- **Input**: "Parse: 'The quick brown fox jumps over the lazy dog'"
- **Output**: "(S (NP (DT The) (JJ quick) (JJ brown) (NN fox)) (VP (VBZ jumps) (PP (IN over) (NP (DT the) (JJ lazy) (NN dog)))))"

## Difficulty Curriculum
- Level 1: Simple SVO sentences
- Level 3: Complex NPs, PP attachment
- Level 5: Relative clauses, coordination
- Level 7: Garden path sentences, long-distance dependencies
- Level 9: Parsing low-resource languages, ambiguous sentences

## Limitations & Risks
- LAS/F1 are well-established metrics with minor inter-annotator disagreement.
- Grammar induction is more challenging — accept any grammar that generates the correct language.
- Annotation conventions vary between frameworks (PTB vs. UD).

## Connections
- [[formal-language-theory]] — formal grammars
- [[semantic-parsing]] — parsing to meaning representation
- [[spelling-grammar]] — grammatical error detection
