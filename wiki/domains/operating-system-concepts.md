---
domain: Operating System Concepts (Scheduling, Memory, Concurrency)
category: systems-cs
verification_type: execution
dataset_scale: 50K+ (from OS courses + simulation)
difficulty_range: medium/hard
modality: text
status: strong_hypothesis
---

# Operating System Concepts

## Overview
Solve OS problems with verifiable numerical answers: compute CPU scheduling outcomes (turnaround time, waiting time), page replacement outcomes (page faults), deadlock detection, synchronization correctness. Verification: simulate the algorithm and compare.

## Verification Mechanism
```python
def verify(task_type: str, problem: dict, answer: Any) -> float:
    if task_type == "cpu_scheduling":
        # Simulate the scheduling algorithm
        result = simulate_scheduler(
            algorithm=problem["algorithm"],  # FCFS, SJF, RR, Priority
            processes=problem["processes"],
            quantum=problem.get("quantum")
        )
        
        if answer.get("average_turnaround") is not None:
            if abs(answer["average_turnaround"] - result.avg_turnaround) < 0.01:
                return 1.0
        
        if answer.get("gantt_chart") is not None:
            if answer["gantt_chart"] == result.gantt_chart:
                return 1.0
        
        return 0.0
    
    elif task_type == "page_replacement":
        result = simulate_page_replacement(
            algorithm=problem["algorithm"],  # FIFO, LRU, Optimal
            reference_string=problem["reference_string"],
            frame_count=problem["frames"]
        )
        return 1.0 if answer["page_faults"] == result.page_faults else 0.0
    
    elif task_type == "deadlock_detection":
        has_deadlock = detect_deadlock(
            problem["allocation_matrix"],
            problem["request_matrix"],
            problem["available"]
        )
        return 1.0 if answer == has_deadlock else 0.0
    
    elif task_type == "disk_scheduling":
        result = simulate_disk_scheduler(
            algorithm=problem["algorithm"],  # FCFS, SSTF, SCAN, C-SCAN
            requests=problem["requests"],
            head_position=problem["head"]
        )
        return 1.0 if answer["total_seek"] == result.total_seek else 0.0
```

## Dataset Sources
- **OS textbook exercises**: Silberschatz, Tanenbaum — with solutions.
- **OS courses**: MIT 6.828, CMU 15-410, Berkeley CS162.
- **Operating System Exams**: Past exam papers with solutions.
- **Procedural generation**: Generate random process sets, simulate scheduling.

## Task Format
- **Input**: "Three processes P1(burst=5), P2(burst=3), P3(burst=8) arrive at time 0. Compute average turnaround time under Round Robin with quantum=3."
- **Output**: "Average turnaround time = 12.33"

## Difficulty Curriculum
- Level 1: FCFS scheduling, simple page replacement
- Level 3: Round Robin, LRU, Banker's algorithm
- Level 5: Multi-level queue scheduling, working set model
- Level 7: Concurrent program correctness (mutex, semaphore)
- Level 9: Distributed OS algorithms, real-time scheduling

## Connections
- [[scheduling]] — abstract scheduling
- [[memory-management]] — low-level memory
- [[distributed-systems]] — distributed OS
