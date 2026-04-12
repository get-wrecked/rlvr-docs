---
domain: Scheduling & Resource Allocation
category: optimization
verification_type: constraint
dataset_scale: unlimited (procedurally generated)
difficulty_range: easy/medium/hard/superhuman
modality: text
status: verified
---

# Scheduling & Resource Allocation

## Overview
Given a set of jobs/tasks with durations, dependencies, resource requirements, and constraints (deadlines, capacity), produce a valid schedule. This is a core operations research problem with massive practical importance. Verification is straightforward: check all constraints are satisfied.

## Verification Mechanism
```python
def verify(problem: SchedulingProblem, schedule: dict[Task, TimeSlot]) -> float:
    violations = 0
    total_constraints = 0
    
    # Check all tasks are scheduled
    for task in problem.tasks:
        total_constraints += 1
        if task not in schedule:
            violations += 1
            continue
        
        # Check deadline
        total_constraints += 1
        if schedule[task].end > task.deadline:
            violations += 1
        
        # Check duration
        total_constraints += 1
        if schedule[task].duration < task.required_duration:
            violations += 1
    
    # Check dependencies (task A before task B)
    for dep in problem.dependencies:
        total_constraints += 1
        if schedule[dep.before].end > schedule[dep.after].start:
            violations += 1
    
    # Check resource capacity at each time slot
    for t in time_range(schedule):
        for resource in problem.resources:
            total_constraints += 1
            usage = sum(task.resource_req[resource] 
                       for task, slot in schedule.items() 
                       if slot.contains(t))
            if usage > resource.capacity:
                violations += 1
    
    if violations == 0:
        # Bonus for optimization (makespan, cost, etc.)
        makespan = max(slot.end for slot in schedule.values())
        return 1.0  # or scale by optimality
    return max(0, 1 - violations / total_constraints)
```

## Dataset Sources
- **OR-Library**: Standard scheduling benchmark instances (job shop, flow shop, resource-constrained).
- **PSPLIB**: Project scheduling problem library. 2000+ instances.
- **Taillard's benchmarks**: Job shop scheduling instances.
- **University timetabling**: ITC competition datasets (International Timetabling Competition).
- **Nurse scheduling**: NHS and other healthcare scheduling benchmarks.
- **Airline crew scheduling**: Public benchmark instances.
- **Procedural generation**: Sample random tasks, dependencies, resources. Control difficulty by number of tasks and tightness of constraints.

## Task Format
- **Input**: List of tasks with durations, deadlines, dependencies, resource requirements
- **Output**: Assignment of start times (and optionally resources) to each task

## Difficulty Curriculum
- Level 1: 5 tasks, no dependencies, single resource
- Level 3: 20 tasks with dependencies, multiple resources
- Level 5: 50 tasks, tight deadlines, optimization objective
- Level 7: 100+ tasks, multiple objectives (minimize cost AND makespan)
- Level 9: Real-world-scale (1000+ tasks, complex constraints)

## Limitations & Risks
- Optimal scheduling is NP-hard. We verify feasibility (polynomial) and measure quality (compare to known bounds).
- Agent might produce trivially feasible but terrible schedules. Use optimization bonus to incentivize quality.

## Connections
- [[combinatorics-optimization]] — scheduling is a core combinatorial optimization problem
- [[planning-classical]] — planning and scheduling are closely related
- [[calendar-scheduling]] — personal scheduling as an application
