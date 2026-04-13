---
domain: iCal/vCard Generation & Parsing
category: Code
verification_type: execution
dataset_scale: unlimited (procedural)
difficulty_range: easy/medium
modality: code
status: strong_hypothesis
---

# iCal/vCard Generation & Parsing

## Overview
Generate valid iCalendar (.ics) and vCard (.vcf) files. Verify by parsing with icalendar/vobject libraries.

## Verification Mechanism
Parse generated file with reference library. Verify all fields present and correctly formatted per RFC 5545/6350.

## Dataset Sources
See wiki for specific URLs and download instructions.

## Task Format
**Input**: Problem specification
**Output**: Solution in appropriate format

## Difficulty Curriculum
Scales from basic to expert-level within the domain.

## Limitations & Risks
Domain-specific edge cases may require careful handling.

## Connections
[[calendar-ical]], [[structured-output-generation]]
