---
domain: Ontology Construction & OWL Reasoning
category: knowledge-representation
verification_type: rule
dataset_scale: 10K+ (from existing ontologies)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Ontology Construction & OWL Reasoning

## Overview
Build or extend OWL ontologies, reason over them (classification, subsumption, consistency checking), and answer queries. Verification: use OWL reasoners (HermiT, Pellet, ELK) to check logical consistency and entailment.

## Verification Mechanism
```python
from owlready2 import *

def verify(task_type: str, ontology_code: str, properties: dict) -> float:
    try:
        onto = get_ontology("file://temp.owl").load()
    except:
        return 0.0
    
    if task_type == "consistency":
        try:
            with onto:
                sync_reasoner_hermit()
            is_consistent = True
        except OwlReadyInconsistentOntologyError:
            is_consistent = False
        
        return 1.0 if properties["expected_consistent"] == is_consistent else 0.0
    
    elif task_type == "classification":
        with onto:
            sync_reasoner_hermit()
        
        # Check inferred class hierarchy
        score = 0
        for (sub, sup) in properties["expected_subsumptions"]:
            sub_class = onto[sub]
            sup_class = onto[sup]
            if sup_class in sub_class.is_a:
                score += 1
        return score / len(properties["expected_subsumptions"])
    
    elif task_type == "query":
        with onto:
            sync_reasoner_hermit()
        results = list(onto.search(**properties["query"]))
        expected = set(properties["expected_results"])
        actual = set(str(r) for r in results)
        return set_f1(actual, expected)
```

## Dataset Sources
- **Gene Ontology (GO)**: 45K+ terms, widely used in biology.
- **SNOMED CT**: 350K+ medical concepts.
- **BFO (Basic Formal Ontology)**: Upper-level ontology.
- **OWL benchmarks**: University ontology, pizza ontology (classic tutorial).
- **OBO Foundry**: 200+ biomedical ontologies.
- **Schema.org**: Web vocabulary.
- **DBpedia ontology**: Wikipedia-derived ontology.

## Task Format
- **Input**: "Extend this ontology by adding a class 'ElectricVehicle' that is a subclass of 'Vehicle', has property 'batteryCapacity' with range xsd:float, and is disjoint with 'InternalCombustionVehicle'"
- **Output**: OWL axioms in Turtle/OWL-XML format

## Difficulty Curriculum
- Level 1: Define classes and basic subsumption
- Level 3: Properties, restrictions (someValuesFrom, allValuesFrom)
- Level 5: Complex class expressions, disjointness, equivalence
- Level 7: OWL2 features (property chains, keys, datatypes)
- Level 9: Large-scale ontology integration, consistency repair

## Limitations & Risks
- OWL reasoning complexity ranges from polynomial (EL) to undecidable (OWL Full). Use OWL2 DL profiles.
- Reasoner timeouts for very complex ontologies.
- Multiple valid ontology designs exist — focus on logical properties rather than specific axiom choice.

## Connections
- [[knowledge-graph-completion]] — KG as lightweight ontology
- [[semantic-parsing]] — NL to formal representation
- [[logic-first-order]] — description logic is a fragment of FOL
