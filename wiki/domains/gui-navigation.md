---
domain: gui-navigation
category: agent/mobile
verification_type: constraint | outcome | diff
dataset_scale: >30K tasks (benchmarks); >700K demonstrations (AITW)
difficulty_range: easy/medium/hard/superhuman
modality: multimodal
status: verified
---

# GUI Navigation (Mobile)

## Overview

GUI navigation tasks require an agent to interact with graphical user interfaces — primarily mobile apps but also desktop applications — to complete specified goals. The agent observes screenshots (and optionally accessibility trees / view hierarchies), and outputs touch actions (tap, swipe, type) or click actions. This is a cornerstone agent domain: mastering GUI interaction enables automating virtually any software task. Verification checks whether the app reaches the goal state, making this a clean RLVR environment. The domain has seen explosive growth in benchmarks and methods since 2023.

## Verification Mechanism

1. **Screen state matching** (constraint-based): After the agent completes its action sequence, check the final screen state against goal conditions. E.g., verify that a specific screen is showing, a toggle is in the correct position, a text field contains the right value. Implementation: accessibility tree inspection, view hierarchy queries, or OCR on screenshots.
2. **Element state checking** (constraint-based): Check specific UI elements: button pressed, checkbox checked, text entered, item selected. Query via accessibility APIs (Android: UIAutomator, iOS: XCTest).
3. **App state / intent matching** (outcome-based): Verify the app reached the correct state. E.g., correct contact is displayed, correct settings page is open, correct item is in cart. Check via Android intent extras, app database state, or screen content.
4. **Action sequence matching** (diff-based): Compare the agent's action sequence to a reference trajectory. Metrics: action-level accuracy, partial sequence match. Weaker than state-based verification (many correct paths exist) but useful when state checking is infeasible.
5. **Screenshot similarity** (diff-based): Compare final screenshot to a reference screenshot. Pixel similarity, SSIM, or learned perceptual metrics. Used as a fallback when structured state checking is unavailable.
6. **Task completion binary** (outcome-based): For end-to-end evaluation, human judges or scripted checks determine if the task was completed. Some benchmarks provide automated verification scripts (AITW, ScreenSpot).

## Dataset Sources

- **Android-in-the-Wild (AITW)**: https://github.com/google-research/google-research/tree/master/android_in_the_wild — 715K human demonstrations across 30K unique tasks on Android apps. Action types: tap, type, swipe, go back, go home. The largest mobile agent dataset.
- **AITW (Curated subset)**: Higher-quality filtered subset used in multiple papers.
- **ScreenSpot**: https://github.com/njucckevin/SeeClick — 1.2K screen grounding tasks across mobile, desktop, and web. Tests ability to locate UI elements from descriptions.
- **AndroidControl**: https://github.com/google-research/google-research/tree/master/android_control — 15K tasks with high/low-level instructions on Android.
- **AITZ (Android in the Zoo)**: Refined AITW with better task descriptions and verification.
- **OmniAct**: https://github.com/CriticalOptimisation/OmniACT — Cross-platform (mobile + desktop) action completion dataset.
- **AppAgent**: https://github.com/mnotgod96/AppAgent — Framework for LLM-based mobile app interaction.
- **MobileAgentBench**: Benchmark for mobile agent evaluation with automated verification.
- **AitW-AITW-v2**: Updated version of AITW with improved annotations.
- **Rico**: https://interactionmining.org/rico — 66K unique Android UI screens with view hierarchies. Not a task dataset but invaluable for understanding UI structure.
- **UIBert / Screen2Words**: UI understanding pretraining datasets.
- **Mind2Web (mobile subset)**: Mobile web navigation tasks.

## Task Format

**Step-by-step navigation**:
- Input: Screenshot of current app state + view hierarchy (optional) + task instruction (e.g., "Open Settings and turn on Dark Mode").
- Output: Next action: `tap(x, y)`, `type("text")`, `swipe(x1, y1, x2, y2)`, `press_back`, `press_home`.
- Verification: After full action sequence, check that Dark Mode is enabled via settings API or screen state.

**Grounding + action**:
- Input: Screenshot + instruction referring to a specific element (e.g., "Tap the search icon").
- Output: Tap coordinates (x, y).
- Verification: Tapped coordinates fall within the target element's bounding box. ScreenSpot format.

**Multi-app workflow**:
- Input: Complex task spanning multiple apps (e.g., "Look up the restaurant's phone number in Maps and call it").
- Output: Action sequence across apps.
- Verification: Phone call initiated to the correct number.

**Low-level instruction following**:
- Input: Step-by-step instructions (e.g., "1. Tap the hamburger menu 2. Select 'Account' 3. Tap 'Change password'").
- Output: Action sequence following each step.
- Verification: Per-step action correctness.

## Difficulty Curriculum

1. **Single tap on labeled element** (easy): "Tap the 'Submit' button." ScreenSpot level.
2. **Type into field** (easy): "Type 'hello' in the search bar."
3. **Navigate menus** (easy-medium): "Go to Settings > Display > Brightness."
4. **Form completion** (medium): Fill out a multi-field form.
5. **Search and select** (medium): "Find the nearest Starbucks on Maps."
6. **Multi-step task** (medium-hard): "Create a new contact named John with phone 555-0123."
7. **Cross-app task** (hard): "Share the photo from Gallery to WhatsApp."
8. **Error recovery** (hard): Task requires handling unexpected states (popups, errors, different app versions).
9. **Complex workflows** (very hard): "Book a flight, add it to calendar, and email the confirmation to your boss."
10. **Unfamiliar apps** (superhuman): Navigate apps the agent has never seen, relying on general GUI understanding.

## Limitations & Risks

- **App version fragility**: Apps update frequently. A task verified on app version 1.0 may be impossible on version 2.0 due to UI changes. Benchmarks become stale.
- **Emulator overhead**: Android emulators are slow. Each step requires rendering, screenshot capture, and state extraction. RL training loops are bottlenecked by this latency.
- **Verification is task-specific**: Each task needs its own verification logic. Scaling to thousands of tasks requires significant engineering for verification scripts.
- **Non-deterministic app behavior**: Apps may show different content based on time, location, user state, network conditions. This makes reproducibility difficult.
- **Privacy and safety**: Mobile agents that can tap and type on real devices can access personal data, make purchases, send messages. Sandbox environments are critical.
- **Accessibility tree limitations**: Not all apps provide clean accessibility trees. Many elements lack useful labels. Screenshot-only operation is harder but more robust.
- **Android bias**: Most benchmarks are Android-only. iOS is harder to automate (stricter sandboxing) and underrepresented. Cross-platform generalization is untested.

## Connections

- [[web-navigation]] — Web navigation is the browser counterpart of GUI navigation
- [[computer-use]] — GUI navigation is a core component of general computer use
- [[visual-grounding]] — UI element grounding is a prerequisite for GUI navigation
- [[file-system-tasks]] — File manager app navigation is a specific GUI task type
- [[map-navigation]] — Map app navigation is a specific GUI task type
