---
domain: calendar-scheduling
category: agent/productivity
verification_type: constraint
dataset_scale: unlimited (procedurally generated from constraint templates)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Calendar Scheduling

## Overview

Calendar scheduling tasks require an agent to schedule meetings, events, and appointments while satisfying multiple constraints: participant availability, room capacity, time zone differences, duration requirements, buffer times, recurring patterns, and priority orderings. This is an excellent RLVR domain because verification is purely constraint-based — either all constraints are satisfied or they are not — and tasks can be generated procedurally in unlimited quantity at precisely controlled difficulty. The domain is practically valuable (scheduling is a major productivity bottleneck) and tests constraint satisfaction, temporal reasoning, and optimization under constraints.

## Verification Mechanism

1. **Constraint satisfaction checking** (constraint-based): For each constraint, verify it is satisfied by the proposed schedule. Constraints include:
   - **No overlaps**: No participant is double-booked. Check all event pairs for time overlap per participant.
   - **Availability windows**: Each event falls within all participants' available times. Check bounds.
   - **Duration requirements**: Each event has the required duration. Simple comparison.
   - **Buffer time**: Minimum gap between consecutive events for each participant. Check all consecutive pairs.
   - **Room capacity**: Number of attendees <= room capacity.
   - **Time zone correctness**: Events are at valid local times for all participants across time zones.
   - **Recurring event correctness**: Recurring events follow the specified pattern (weekly, biweekly, etc.).
   - **Priority ordering**: Higher-priority events are scheduled before lower-priority ones, or get preferred time slots.
   - Implementation: Iterate over all constraints, return binary (all satisfied / any violated) or count of satisfied constraints.
2. **Optimality checking** (constraint-based): Beyond feasibility, check if the schedule optimizes for objectives: minimize total travel time, maximize preference satisfaction, minimize fragmentation. Compare to an optimal or known-good solution computed by a constraint solver (OR-Tools, CP-SAT).
3. **Output format validation** (rule-based): Check that the output is a valid schedule format: each event has start time, end time, participants, room (if applicable). Well-formed datetime values.

## Dataset Sources

- **Procedural generation (primary)**: Calendar scheduling tasks are ideal for procedural generation:
  - Define N participants with random availability windows (working hours, time zones, exceptions).
  - Define M events to schedule, each with required participants, duration, and constraints.
  - Use a constraint solver to verify the instance is feasible before presenting to the agent.
  - Control difficulty by varying: number of participants (2-20), number of events (1-50), constraint tightness (loose-tight), time horizons (1 day to 1 month), time zone diversity.
  - Frameworks: Google OR-Tools CP-SAT, OptaPlanner, custom generators.
- **CalendarBench**: Benchmark for LLM calendar reasoning. Tasks involving reading calendars, finding free slots, scheduling events.
- **NATURAL PLAN benchmark (scheduling subset)**: https://github.com/google-deepmind/natural-plan — Meeting scheduling tasks requiring temporal reasoning.
- **TimeQA-style datasets**: Temporal reasoning datasets that can be adapted for scheduling.
- **Real calendar anonymized datasets**: Enterprise calendar data (anonymized) for realistic constraint distributions. Limited availability due to privacy.
- **Synthetic enterprise scenarios**: Generate realistic business scenarios: weekly team meetings, 1:1s, cross-team syncs, all-hands, with realistic constraints.

## Task Format

**Simple slot finding**:
- Input: Calendar state (existing events for each participant) + new event requirements (participants, duration).
- Output: Proposed time slot (start datetime, end datetime).
- Verification: No conflicts with existing events, all participants available, duration correct.

**Multi-event scheduling**:
- Input: Set of events to schedule, each with participants, duration, and constraints. Existing calendar state.
- Output: Complete schedule (list of events with times, rooms).
- Verification: All constraints satisfied.

**Rescheduling**:
- Input: Existing schedule + change request (e.g., "Move the team meeting to accommodate Alice's new conflict on Tuesday").
- Output: Updated schedule.
- Verification: Original constraints still met + new constraint satisfied + minimal disruption.

**Constraint extraction + scheduling**:
- Input: Natural language description (e.g., "Schedule a 1-hour meeting with Bob and Carol sometime next week. Bob is free Mon-Wed mornings. Carol can't do Monday. Prefer earlier in the week.").
- Output: Proposed event.
- Verification: Extracted constraints are satisfied.

**Optimization**:
- Input: Scheduling problem with soft constraints / preferences.
- Output: Schedule that satisfies all hard constraints and maximizes soft constraint satisfaction.
- Verification: Hard constraints all met. Score based on soft constraint satisfaction.

## Difficulty Curriculum

1. **Single event, two people, same timezone** (easy): Find a free slot for two people. Minimal constraints.
2. **Single event, multiple people** (easy-medium): Find a common free slot for 3-5 people.
3. **Multiple events, no interactions** (medium): Schedule 3-5 independent events (different participants).
4. **Multiple events, shared participants** (medium-hard): Participants appear in multiple events, creating scheduling dependencies.
5. **Time zone management** (hard): Participants in 3+ time zones. Must find reasonable local times for all.
6. **Tight constraints** (hard): Dense calendars with few available slots. May require creative scheduling.
7. **Recurring events** (hard): Schedule weekly recurring events that don't conflict with existing recurrences.
8. **Optimization** (very hard): Minimize total meeting hours, maximize focus time blocks, balance meeting load across days.
9. **Dynamic rescheduling** (very hard): Handle cascading changes when one event moves and conflicts arise.
10. **Large-scale scheduling** (superhuman): 20+ participants, 50+ events, multiple rooms, hard and soft constraints, over a month-long horizon.

## Limitations & Risks

- **Natural language ambiguity**: "Morning" means different things to different people. "Next week" is ambiguous near weekends. Converting natural language to precise constraints is itself error-prone and not fully verifiable.
- **Preference vs constraint**: The distinction between hard constraints (must satisfy) and soft preferences (should satisfy) is often unclear in natural language. "I prefer mornings" vs "I'm only available mornings."
- **Real-world complexity**: Real scheduling involves human social dynamics (seniority, preferences, meeting fatigue) that are hard to encode as formal constraints.
- **Feasibility detection**: Some constraint sets are infeasible (no valid schedule exists). The agent should detect and report this, but verification of "correctly identified as infeasible" requires proving infeasibility.
- **Evaluation of optimality**: For optimization tasks, the optimal solution may be hard to compute (NP-hard in general). Comparing to a good heuristic solution is a proxy, not ground truth.
- **Synthetic vs real**: Procedurally generated scheduling problems may not capture the messy reality of real enterprise calendars (recurring exceptions, tentative events, buffer preferences, meeting type priorities).
- **Limited existing benchmarks**: Unlike coding or math, there are few established scheduling benchmarks. Most evaluation requires custom task generation and verification.

## Connections

- [[email-tasks]] — Scheduling often involves email communication (meeting invites, availability requests)
- [[computer-use]] — Calendar management is a core computer use task
- [[map-navigation]] — Time-constrained routing connects to scheduling (travel time between meetings)
- [[competitive-programming-interactive]] — Both involve constraint satisfaction and optimization
- [[puzzle-games]] — Scheduling is fundamentally a constraint satisfaction puzzle
