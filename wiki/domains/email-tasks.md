---
domain: email-tasks
category: agent/productivity
verification_type: constraint | rule
dataset_scale: generatable from templates; limited benchmarks
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Email Tasks

## Overview

Email tasks require an agent to draft, organize, reply to, and manage emails meeting specific criteria. This is a practically important RLVR domain — email is a dominant professional communication channel — but verification is PARTIALLY LIMITED. We can verify structural and mechanical aspects of emails (correct recipients, required keywords present, appropriate length, attachment references, formatting) but cannot fully verify quality dimensions like tone, persuasiveness, or appropriateness. This page focuses on the verifiable subset and honestly flags where verification breaks down.

## Verification Mechanism

**What CAN be verified (structural/mechanical):**

1. **Recipient checking** (rule-based): Verify correct To/CC/BCC fields. Exact match against required recipients. Binary.
2. **Subject line checking** (rule-based): Check subject contains required keywords, follows required format, or matches a template. Regex or keyword matching.
3. **Keyword/phrase presence** (rule-based): Verify that specific information is included in the body. E.g., "The email must mention the project deadline of March 15." Keyword/phrase search.
4. **Attachment reference checking** (rule-based): If the task requires mentioning attachments, verify the email references them. Regex for "attached", "attachment", "please find enclosed", etc.
5. **Length constraints** (rule-based): Email is within specified word/character limits. Simple count.
6. **Structural requirements** (rule-based): Check for required sections (greeting, body, signature, closing). Pattern matching.
7. **Information extraction accuracy** (exact match): If the email must contain specific data from a source (e.g., "Include the quarterly revenue figure of $4.2M"), verify the data is correct.
8. **Reply threading** (rule-based): For reply tasks, verify the email is properly threaded (includes original message reference, addresses the right person).
9. **No-send constraints** (rule-based): Verify the email does NOT contain prohibited content (confidential information, specific phrases to avoid). Negative keyword matching.

**What CANNOT be reliably verified (quality/pragmatic):**

- Tone (professional, friendly, urgent) — requires subjective judgment
- Persuasiveness — no programmatic metric
- Cultural appropriateness — context-dependent
- Whether the email will achieve its intended effect — unknowable
- Naturalness of language — LLM-as-judge territory, not RLVR

## Dataset Sources

- **Enron Email Corpus**: https://www.cs.cmu.edu/~enron/ — 500K+ real business emails from Enron Corporation. The standard large-scale email dataset. Can be used to generate realistic email tasks (reply to this thread, summarize this chain, draft a follow-up).
- **AESLC (Annotated Enron Subject Line Corpus)**: https://github.com/ryanzhumich/AESLC — Enron emails with human-annotated subject lines. Verification: subject line match.
- **ShortAnswer Email Benchmark**: Task-oriented email generation with verifiable outputs.
- **Template-based generation**: Generate email tasks from templates:
  - Define a scenario (meeting request, project update, complaint response).
  - Specify required elements (recipients, key information, constraints).
  - Generate the instruction and verification criteria.
  - Can produce unlimited tasks with controlled difficulty.
- **Customer service datasets**: 
  - Ubuntu Dialogue Corpus: Technical support threads.
  - Taskmaster: Task-oriented dialogues that can be adapted to email format.
- **Business communication templates**: Thousands of email templates for various scenarios. Each template defines a structure that can be verified.
- **Gmail/Outlook synthetic benchmarks**: Synthesized from common email patterns with verifiable criteria.

## Task Format

**Email drafting with constraints**:
- Input: Scenario description + requirements (e.g., "Draft an email to john@company.com requesting a meeting next week about the Q3 budget review. CC: finance-team@company.com. Mention the deadline is October 15. Keep it under 200 words.").
- Output: Complete email (To, CC, Subject, Body).
- Verification: Check recipients, keyword presence ("Q3 budget review", "October 15"), length, CC field.

**Email reply**:
- Input: Original email thread + instruction (e.g., "Reply accepting the meeting but suggest Thursday instead of Wednesday. Mention you'll bring the updated figures.").
- Output: Reply email.
- Verification: Addresses original sender, references specific content (Thursday, updated figures), proper threading.

**Email organization**:
- Input: Set of emails + instruction (e.g., "Categorize these emails as: urgent, normal, or archive").
- Output: Categorization for each email.
- Verification: Exact match against ground truth categories (if human-labeled) or rule-based (emails with "URGENT" in subject or deadline within 24h = urgent).

**Email summarization with extraction**:
- Input: Long email thread + "Extract all action items and their assignees."
- Output: Structured list of action items.
- Verification: Exact match on extracted items against ground truth (when available).

**Email triage**:
- Input: Set of incoming emails + priority rules + "Which emails need immediate response?"
- Output: Prioritized list.
- Verification: Matches rule-based prioritization.

## Difficulty Curriculum

1. **Simple forwarding** (easy): Forward an email to a specified recipient with a brief note.
2. **Template filling** (easy): Fill in a template email with provided information.
3. **Constrained drafting** (easy-medium): Draft an email with 3-4 verifiable requirements.
4. **Reply with context** (medium): Reply to an email thread, incorporating specific information.
5. **Multi-constraint drafting** (medium-hard): 6+ constraints including length, tone indicators, required/forbidden content.
6. **Chain management** (hard): Manage a multi-email thread: summarize, identify action items, draft follow-up.
7. **Cross-reference drafting** (hard): Draft email pulling information from multiple source documents/emails.
8. **Negotiation emails** (very hard): Draft emails in a negotiation sequence. Verifiable constraints: maintain specific positions, reference specific terms, stay within authorized limits.
9. **Complex organizational email** (very hard): Coordinate across multiple recipients with different information needs, threading requirements, and access levels.

## Limitations & Risks

- **PARTIAL VERIFIABILITY IS THE CENTRAL LIMITATION**: The most important aspects of email quality (clarity, tone, professionalism, effectiveness) cannot be verified programmatically. RLVR can only train on the structural/mechanical aspects. An email that passes all structural checks could still be terrible.
- **Goodhart's Law risk**: Training on verifiable proxies (keyword presence, length, format) without quality checks could produce emails that "check the boxes" but are unnatural, repetitive, or awkward. The model optimizes for what is measured, not what matters.
- **Privacy concerns**: Email datasets contain personal and business communications. The Enron corpus, while public, raises ethical questions. Synthetic data avoids this but may lack realism.
- **Tone and style are unverifiable**: Instructing "write in a professional tone" has no programmatic verifier. Any regex-based proxy will be easily gamed.
- **Context sensitivity**: The appropriate email response depends heavily on organizational context, relationships, and history that cannot be fully captured in a task description.
- **Evaluation gap**: There is a wide gap between what we can verify (structural correctness) and what constitutes a good email (effectiveness in achieving its purpose). RLVR can only train on the former.

## Connections

- [[calendar-scheduling]] — Meeting scheduling often involves email coordination
- [[computer-use]] — Email management is a core computer use task
- [[web-navigation]] — Webmail interfaces combine email tasks with web navigation
- [[file-system-tasks]] — Email with attachments connects to file management
- [[spreadsheet-tasks]] — Extracting data for emails often involves spreadsheet sources
