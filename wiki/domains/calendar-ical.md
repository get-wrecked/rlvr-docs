---
domain: calendar-ical
category: format-constrained
verification_type: execution | constraint
dataset_scale: ~medium (synthetic generation)
difficulty_range: easy/medium
modality: text
status: strong_hypothesis
---

# Calendar / iCal Generation

## Overview

Calendar generation tasks require producing valid iCal (ICS) files from natural language event descriptions. The iCalendar format (RFC 5545) is the universal standard for calendar data exchange, used by Google Calendar, Apple Calendar, Outlook, and virtually every scheduling application.

This domain tests the model's ability to parse natural language time expressions, handle timezones, produce standard-compliant structured output, and manage edge cases (recurring events, all-day events, multi-day events, timezone conversions). Verification is exact: parse the output with an iCalendar library and check that all fields match the specification.

## Verification Mechanism

**Type: Execution (parse) + Constraint satisfaction (field checking)**

```python
from icalendar import Calendar
from datetime import datetime
import pytz

def verify_ical(model_output: str, expected_fields: dict) -> float:
    try:
        cal = Calendar.from_ical(model_output)
    except ValueError:
        return 0.0  # Invalid iCal format
    
    score = 0.2  # Successfully parsed
    events = [c for c in cal.walk() if c.name == 'VEVENT']
    
    if len(events) != expected_fields.get('event_count', 1):
        return score
    
    event = events[0]
    checks = {
        'summary': str(event.get('SUMMARY', '')) == expected_fields.get('summary', ''),
        'dtstart': event.get('DTSTART').dt == expected_fields.get('start'),
        'dtend': event.get('DTEND').dt == expected_fields.get('end'),
        'location': str(event.get('LOCATION', '')) == expected_fields.get('location', ''),
        'rrule': check_rrule(event.get('RRULE'), expected_fields.get('rrule')),
    }
    
    score += sum(checks.values()) * 0.16  # 5 checks * 0.16 = 0.8
    return score
```

**Verification checks:**
1. **Parsability:** Does the output parse as valid iCalendar? (icalendar library)
2. **Required properties:** DTSTART, DTEND (or DURATION), SUMMARY present.
3. **Field correctness:** Each field matches expected values (datetime comparison with timezone awareness).
4. **Recurrence rules:** RRULE correctly encodes frequency, interval, count/until.
5. **Timezone handling:** VTIMEZONE component present when needed, TZID references correct.
6. **RFC compliance:** PRODID, VERSION, CALSCALE headers present.

**Verification confidence: HIGH.** iCalendar parsing is well-defined by RFC 5545. The icalendar Python library provides robust parsing. Field comparison is exact (datetime equality, string equality). The only ambiguity is in natural language interpretation — "next Tuesday" depends on context.

## Dataset Sources

- **Synthetic generation (primary):** Generate event descriptions from templates:
  - "Meeting with {person} on {date} at {time} in {location} for {duration}"
  - "Weekly standup every {day} at {time} from {start_date} to {end_date}"
  - "Birthday party: {date}, {start_time}-{end_time}, at {venue}"
  - Include timezone variations, all-day events, recurring events.
- **Google Calendar / Outlook export data:** Users can export their calendars as ICS files (privacy-sensitive — use synthetic).
- **RFC 5545 examples:** The specification itself contains numerous example ICS snippets.
- **Public event calendars:** Conference schedules, sports schedules, academic calendars published as ICS.
- **Scheduling benchmark datasets:** Meeting scheduling datasets that include temporal information.

## Task Format

**Input:**
```
Create an iCal event for: Team meeting on March 15, 2025 from 2:00 PM to 3:30 PM EST
in Conference Room B. Recurring weekly until June 15, 2025.
```

**Output:**
```
BEGIN:VCALENDAR
VERSION:2.0
PRODID:-//RLVR//EN
BEGIN:VEVENT
DTSTART;TZID=America/New_York:20250315T140000
DTEND;TZID=America/New_York:20250315T153000
SUMMARY:Team Meeting
LOCATION:Conference Room B
RRULE:FREQ=WEEKLY;UNTIL=20250615T235959Z
END:VEVENT
END:VCALENDAR
```

**Input (complex):**
```
Create calendar events for a 3-day conference:
- Day 1 (April 10): Keynote 9:00-10:30 AM, Workshop A 11:00-12:30 PM, Lunch 12:30-1:30 PM
- Day 2 (April 11): Panel 9:00-10:00 AM, Workshop B 10:30-12:00 PM, Networking 5:00-7:00 PM
- Day 3 (April 12): Closing remarks 9:00-10:00 AM
All times Pacific. Location: San Francisco Convention Center.
```

## Difficulty Curriculum

1. **Easy:** Single event, explicit date/time, no timezone complexity, no recurrence.
2. **Medium:** Recurring events (daily, weekly, monthly), timezone specification, all-day events, events with attendees.
3. **Hard:** Complex recurrence rules (every 2nd Tuesday, except holidays), timezone conversions, multi-event calendars, VALARM (reminders), attendee RSVP status.
4. **Very hard:** Interpret ambiguous natural language ("next business day after Easter"), handle DST transitions, free/busy queries, calendar merging.

## Limitations & Risks

- **Narrow domain:** iCal generation is a specific format skill with limited transfer to broader reasoning.
- **Natural language ambiguity:** "Next Friday" is ambiguous without a reference date. Tasks must either specify absolute dates or provide a reference date.
- **Timezone complexity:** DST transitions, historical timezone changes, and unusual offsets create edge cases that are hard to generate comprehensive test data for.
- **Format strictness:** iCalendar has specific line-folding rules (lines > 75 octets must be folded). This is a formatting detail that may trip up the model without adding reasoning value.
- **Limited dataset diversity:** Calendar events are repetitive — the same patterns (meeting, appointment, conference) recur. Difficulty comes from edge cases rather than domain breadth.

## Connections

- [[date-time-computation]] — Date/time reasoning is the core skill exercised.
- [[data-formatting]] — iCal is structured data formatting with domain-specific rules.
- [[protocol-compliance]] — iCalendar is defined by an RFC, similar to protocol compliance.
- [[config-generation]] — Both involve generating standards-compliant structured text.
