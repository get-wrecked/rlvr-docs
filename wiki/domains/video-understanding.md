---
domain: video-understanding
category: vision/video
verification_type: exact_match
dataset_scale: >500K QA pairs
difficulty_range: easy/medium/hard
modality: multimodal
status: verified
---

# Video Understanding

## Overview

Video understanding tasks require answering questions or making predictions about video content: actions, events, temporal sequences, causal relationships, and dynamic scenes. Compared to image-based VQA, video adds the temporal dimension — understanding what happens over time, in what order, and why. For RLVR, the key constraint is verifiability: we restrict to questions with short, factual, exact-match answers rather than open-ended descriptions. Video QA is increasingly important as video data dominates the internet and agents need to understand demonstrations, tutorials, meetings, and surveillance footage.

## Verification Mechanism

1. **Exact answer match** (exact match): Most video QA benchmarks have short factual answers. Verification: normalized string comparison. E.g., "What did the person pick up?" -> "a red ball."
2. **Multiple choice selection** (exact match): Many benchmarks (NExT-QA, EgoSchema) use multiple choice. Perfectly unambiguous verification.
3. **Temporal localization accuracy** (constraint-based): For moment retrieval tasks ("When does the person start running?"), verify that the predicted time interval overlaps sufficiently with the ground truth (temporal IoU >= threshold).
4. **Action recognition accuracy** (exact match): Classify the action in a video clip. Standard classification metric.
5. **Numeric answer match** (exact match): For counting questions ("How many times does the ball bounce?"), exact numeric comparison.

## Dataset Sources

- **ActivityNet-QA**: http://activity-net.org/ — 58K QA pairs on 5.8K videos of human activities. Open-ended questions about actions, spatial relationships, temporal ordering, and counting.
- **MSRVTT-QA**: https://github.com/xudejing/video-question-answering — 244K QA pairs on 10K video clips. Generated from video captions. Standard benchmark.
- **NExT-QA**: https://github.com/doc-doc/NExT-QA — 52K QA pairs on 5.4K videos. Focus on causal and temporal reasoning. Multiple choice format. Three types: causal (why), temporal (when/before/after), descriptive.
- **EgoSchema**: https://egoschema.github.io/ — 5K multiple-choice questions on 5K egocentric (first-person) video clips from Ego4D. 3 minutes long, requiring long-form video understanding.
- **MSVD-QA**: 50K QA pairs on 1.9K short video clips from YouTube.
- **TGIF-QA**: https://github.com/YunseokJANG/tgif-qa — 165K QA pairs on animated GIFs. Four task types: repetition counting, repeating action, state transition, frame QA.
- **VideoMME**: https://video-mme.github.io/ — Comprehensive video understanding evaluation for multimodal models. 900 videos, 2.7K questions, diverse domains.
- **MVBench**: https://github.com/OpenGVLab/Ask-Anything — 20 video understanding tasks with 4K questions covering temporal, spatial, and causal reasoning.
- **Perception Test**: https://github.com/google-deepmind/perception_test — 11.6K questions on 38s average videos. Tests memory, abstraction, physics, semantics.
- **Ego4D**: https://ego4d-data.org/ — 3,670 hours of egocentric video. Multiple benchmarks: episodic memory, forecasting, hand-object interaction.
- **Charades-STA**: Temporal grounding benchmark. Localize activity descriptions in videos.
- **QVHighlights**: https://github.com/jayleicn/moment_detr — Moment retrieval and highlight detection in YouTube videos.

## Task Format

**Open-ended video QA**:
- Input: Video (as frames or clip) + question (e.g., "What does the person do after sitting down?").
- Output: Short text answer (e.g., "opens a laptop").
- Verification: Exact match or soft match against ground truth answers.

**Multiple choice video QA**:
- Input: Video + question + 5 answer options.
- Output: Selected option (A-E).
- Verification: Exact match. NExT-QA, EgoSchema format.

**Temporal grounding / moment retrieval**:
- Input: Video + text query (e.g., "The person picks up the phone").
- Output: Start and end timestamps (e.g., [12.5, 15.3] seconds).
- Verification: Temporal IoU >= 0.5 (or other threshold) with ground truth interval.

**Repetition counting**:
- Input: Video + "How many times does [action] occur?"
- Output: Integer count.
- Verification: Exact numeric match.

**Action recognition**:
- Input: Short video clip.
- Output: Action label from fixed vocabulary (e.g., "playing basketball").
- Verification: Exact match.

## Difficulty Curriculum

1. **Static scene description** (easy): Questions answerable from a single frame. "What color is the car?"
2. **Simple action recognition** (easy-medium): "What is the person doing?" Short clip, single action.
3. **Object state change** (medium): "What happens to the glass?" Requires tracking over time.
4. **Temporal ordering** (medium): "What does the person do before/after X?"
5. **Counting actions** (medium-hard): "How many times does the ball bounce?" Requires temporal attention.
6. **Causal reasoning** (hard): "Why did the person open the umbrella?" Requires inference. NExT-QA causal subset.
7. **Long-form video understanding** (hard): EgoSchema-style 3-minute videos requiring sustained attention and memory.
8. **Multi-event sequences** (very hard): "Describe the sequence of events that led to the accident." Requires temporal reasoning over multiple events.
9. **Predictive reasoning** (very hard): "What will happen next?" Requires understanding dynamics and intent.

## Limitations & Risks

- **Computational cost**: Processing video (even sampled frames) is far more expensive than images. A 3-minute video at 1 fps is 180 frames. This makes RL training loops expensive.
- **Frame sampling sensitivity**: Most models sample sparse frames from videos. The correct answer may depend on a frame that was not sampled, leading to false negatives.
- **Static shortcut**: Many video QA questions can be answered from a single frame, not requiring temporal reasoning. Models learn this shortcut, achieving good accuracy without understanding video dynamics. Verified by frame-shuffling experiments that show minimal performance drop.
- **Answer distribution bias**: Like VQA, video QA has skewed answer distributions. Frequent answers ("yes", "no", "2", "walking") dominate.
- **Annotation quality**: Video annotation is expensive and more error-prone than image annotation. Temporal boundaries are fuzzy. Multiple valid actions may be happening simultaneously.
- **Temporal granularity**: "After" and "before" can be ambiguous — immediately after, or at any point after? Temporal reasoning questions need careful specification.
- **Limited scale for open-ended QA**: The largest video QA datasets (~250K) are much smaller than image QA datasets (>5M). Most video QA data is generated from captions, limiting question diversity.

## Connections

- [[visual-question-answering]] — Video QA extends image QA with temporal reasoning
- [[game-playing-atari]] — Both require processing visual sequences over time
- [[spatial-reasoning]] — Video spatial reasoning adds the temporal dimension
- [[multimodal-reasoning]] — Video understanding is a key component of multimodal reasoning
- [[gui-navigation]] — Screen recordings of GUI interactions are videos
