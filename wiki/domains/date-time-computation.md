---
domain: date-time-computation
category: miscellaneous
verification_type: exact_match
dataset_scale: ~infinite (template-generated)
difficulty_range: easy/medium/hard
modality: text
status: verified
---

# Date/Time Computation

## Overview

Date/time computation tasks require performing calculations with dates, times, and timezones: days between two dates, day-of-week calculation, timezone conversion, adding/subtracting durations, handling leap years, DST transitions, and related problems. These are deceptively tricky — the irregularities of the calendar system (months of different lengths, leap years, daylight saving time) make seemingly simple calculations error-prone.

Verification is exact: every date/time calculation has a single correct answer computable by standard datetime libraries. The domain exercises precise reasoning with irregular rule systems.

## Verification Mechanism

**Type: Exact match**

```python
from datetime import datetime, timedelta
import pytz

def verify_date_computation(task_type: str, params: dict, 
                             model_answer: str) -> float:
    if task_type == 'days_between':
        d1 = datetime.strptime(params['date1'], '%Y-%m-%d')
        d2 = datetime.strptime(params['date2'], '%Y-%m-%d')
        expected = abs((d2 - d1).days)
        return 1.0 if str(expected) == model_answer.strip() else 0.0
    
    elif task_type == 'day_of_week':
        d = datetime.strptime(params['date'], '%Y-%m-%d')
        expected = d.strftime('%A')  # e.g., "Monday"
        return 1.0 if expected.lower() == model_answer.strip().lower() else 0.0
    
    elif task_type == 'timezone_conversion':
        tz_from = pytz.timezone(params['from_tz'])
        tz_to = pytz.timezone(params['to_tz'])
        dt = tz_from.localize(datetime.strptime(params['datetime'], '%Y-%m-%d %H:%M'))
        expected = dt.astimezone(tz_to).strftime('%Y-%m-%d %H:%M')
        return 1.0 if expected == model_answer.strip() else 0.0
    
    elif task_type == 'add_duration':
        dt = datetime.strptime(params['start'], '%Y-%m-%d')
        delta = timedelta(days=params.get('days', 0))
        expected = (dt + delta).strftime('%Y-%m-%d')
        return 1.0 if expected == model_answer.strip() else 0.0
    
    # ... more task types
```

**Verification confidence: VERY HIGH.** Datetime libraries are authoritative. Every calculation has a single correct answer. The only ambiguity is output format (must specify expected format in the prompt or accept multiple valid formats).

## Dataset Sources

- **Template generation (primary):** Generate problems from templates with random dates:
  - "How many days between {date1} and {date2}?"
  - "What day of the week is {date}?"
  - "Convert {time} {tz1} to {tz2}"
  - "What date is {N} days after {date}?"
  - "If a meeting starts at {time} in {tz1}, what time is it in {tz2}?"
  Unlimited scale.
- **Timeanddate.com:** Reference answers for timezone conversions and date calculations.
- **IANA timezone database:** Authoritative timezone data for all regions.
- **Historical calendar questions:** "What day of the week was July 4, 1776?" — can verify against known answers.
- **Programming challenge sites:** Date/time problems from LeetCode, HackerRank.
- **Standardized test questions:** GRE/GMAT quantitative sections occasionally include date problems.

## Task Format

**Input (simple):**
```
How many days are there between March 1, 2024 and April 15, 2024?
```

**Output:**
```
45
```

**Input (timezone):**
```
A conference call is scheduled for 3:00 PM EST on March 10, 2024. 
What time is this in Tokyo (JST)?
```

**Output:**
```
5:00 AM JST on March 11, 2024
```

**Input (complex):**
```
A project started on January 29, 2024 and has a deadline of 90 business days later 
(excluding US federal holidays). What is the deadline date?
```

**Output:**
```
June 10, 2024
```

## Difficulty Curriculum

1. **Easy:** Days between two dates in the same month, day of week for recent dates, simple time addition.
2. **Medium:** Cross-month and cross-year calculations, basic timezone conversion (whole-hour offsets), leap year handling.
3. **Hard:** DST transition awareness (March/November in US, different dates globally), half-hour timezone offsets (India, Nepal), business day calculations, historical date questions (Julian/Gregorian calendar switch).
4. **Very hard:** "Business days excluding holidays" (requires holiday calendar), date arithmetic across international date line, recurring schedule calculations ("third Thursday of every month"), age calculation edge cases (born on Feb 29).

## Limitations & Risks

- **Output format sensitivity:** "March 15, 2024" vs "2024-03-15" vs "15/03/2024" — all correct. Must normalize or specify format.
- **Holiday definitions vary:** "Business days excluding holidays" requires knowing which holiday calendar to use. Must specify country/region.
- **Historical calendar complexity:** Before 1582, the Julian calendar was in use. Different countries switched at different times. Historical date questions before ~1900 can be ambiguous.
- **DST rules change:** Countries modify DST rules periodically. Must fix the timezone database version.
- **Moderate reasoning depth:** Most date calculations are algorithmic, not deeply reasoned. The domain builds precision more than insight.

## Connections

- [[calendar-ical]] — Calendar generation requires date/time computation skills.
- [[unit-conversion]] — Time unit conversions are a subset.
- [[financial-calculation]] — Date calculations are fundamental to interest and amortization computations.
- [[recipe-scaling]] — Both involve precise arithmetic with domain-specific rules.
