---
domain: Process Mining & Workflow Discovery
category: business-analytics
verification_type: execution
dataset_scale: 10K+ (from event log benchmarks)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Process Mining & Workflow Discovery

## Overview
Given event logs (sequences of activities with timestamps), discover the underlying process model (Petri net, BPMN diagram), check conformance, or predict next activities. Verification: replay the event log on the discovered model and measure fitness/precision.

## Verification Mechanism
```python
import pm4py

def verify(event_log: pd.DataFrame, process_model: Any, task: str) -> float:
    if task == "discovery":
        # Fitness: how well does the model replay the log?
        fitness = pm4py.fitness_token_based_replay(
            event_log, process_model["net"], 
            process_model["im"], process_model["fm"]
        )
        
        # Precision: does the model allow only observed behavior?
        precision = pm4py.precision_token_based_replay(
            event_log, process_model["net"],
            process_model["im"], process_model["fm"]
        )
        
        # F1-like combination
        f1 = 2 * fitness["average_trace_fitness"] * precision / \
             max(fitness["average_trace_fitness"] + precision, 1e-10)
        return f1
    
    elif task == "conformance":
        # Check if specific traces conform to model
        alignments = pm4py.conformance_diagnostics_alignments(
            event_log, process_model["net"],
            process_model["im"], process_model["fm"]
        )
        fitness = sum(a["fitness"] for a in alignments) / len(alignments)
        return fitness
    
    elif task == "next_activity":
        # Predict next activity accuracy
        correct = 0
        for trace in event_log.groupby("case_id"):
            prefix = trace[:-1]
            actual_next = trace.iloc[-1]["activity"]
            predicted = predict_next(process_model, prefix)
            if predicted == actual_next:
                correct += 1
        return correct / len(event_log["case_id"].unique())
```

## Dataset Sources
- **BPI Challenge datasets**: Annual process mining competition datasets (2011-2023).
- **Process Mining Group datasets**: TU Eindhoven benchmarks.
- **IEEE Task Force on Process Mining**: Standard datasets.
- **Synthetic event logs**: pm4py can generate synthetic logs from known models.
- **Healthcare process logs**: Patient pathway datasets.
- **IT service management logs**: Ticket handling processes.

## Task Format
- **Input**: Event log CSV (case_id, activity, timestamp) + "Discover the process model"
- **Output**: Petri net or BPMN model (in standard format)

## Difficulty Curriculum
- Level 1: Simple sequential processes
- Level 3: Parallel activities, choice
- Level 5: Loops, long-running processes
- Level 7: Noise handling, incomplete logs
- Level 9: Multi-perspective models (resource, time, data-aware)

## Limitations & Risks
- Fitness and precision can be in tension — a flower model has perfect fitness but zero precision. Balance both.
- Process models are not unique — many valid models may exist for the same log.
- pm4py provides well-established metrics for evaluation.

## Connections
- [[workflow-automation]] — automating discovered processes
- [[simulation-modeling]] — simulating discovered processes
- [[data-science-eda]] — exploratory analysis of event data
