---
domain: visual-grounding
category: vision/grounding
verification_type: constraint
dataset_scale: >500K referring expressions
difficulty_range: easy/medium/hard
modality: multimodal
status: verified
---

# Visual Grounding

## Overview

Visual grounding (referring expression comprehension) requires localizing an object in an image given a natural-language description. The agent must output a bounding box around the described object. This is a clean RLVR domain because verification is geometric: compute the Intersection over Union (IoU) between the predicted and ground truth bounding boxes, and reward if IoU exceeds a threshold (typically 0.5). The domain tests the fundamental ability to connect language to spatial visual content — a prerequisite for many agent tasks. Unlike image classification (which identifies what is present) or VQA (which answers questions), grounding demands precise spatial localization.

## Verification Mechanism

1. **IoU thresholding** (constraint-based): Primary metric. IoU = area(intersection) / area(union) of predicted and ground truth bounding boxes. Standard threshold: IoU >= 0.5 counts as correct. Binary reward. Can use multiple thresholds (0.5, 0.75, 0.9) for difficulty scaling.
2. **Pointing accuracy** (constraint-based): For point-based grounding (predict center point of object), check if the predicted point falls within the ground truth bounding box. Binary.
3. **IoU as continuous reward** (constraint-based): Use IoU directly as a [0,1] continuous reward signal. More informative than binary thresholding but may enable reward hacking (predicting very large boxes to inflate IoU).
4. **GIoU / DIoU** (constraint-based): Generalized IoU or Distance IoU metrics that also penalize non-overlapping boxes, providing gradient signal even when IoU = 0.
5. **Segmentation mask IoU** (constraint-based): For datasets with segmentation masks (RefCOCO has some), compute pixel-level IoU. More precise than bounding box IoU.

## Dataset Sources

- **RefCOCO**: https://github.com/lichengunc/refer — 142K referring expressions for 50K objects in 20K COCO images. Split into RefCOCO (short expressions), RefCOCO+ (no spatial language), and RefCOCOg (longer, more complex expressions). The standard visual grounding benchmark suite.
- **RefCOCO+**: Same images as RefCOCO but expressions avoid absolute spatial terms ("left", "right"). Forces appearance-based grounding.
- **RefCOCOg**: Longer, more natural expressions. Collected via a different protocol (one annotator describes, another locates).
- **Flickr30K Entities**: https://github.com/BryanPlummer/flickr30k_entities — 275K bounding boxes linked to 158K phrases in 31K Flickr images. Phrase grounding: ground each noun phrase in a caption to its bounding box.
- **Visual Genome**: https://homes.cs.washington.edu/~ranjay/visualgenome/ — 108K images with 3.8M object instances, 2.8M attributes, 2.3M relationships. Provides bounding boxes with descriptions.
- **ReferItGame**: https://www.mpi-inf.mpg.de/departments/computer-vision-and-machine-learning/research/vision-and-language/referitgame — 20K images with 130K referring expressions. Earlier, smaller dataset.
- **Cops-Ref**: https://github.com/zfchenUnique/Cops-Ref — Challenging referring expressions requiring fine-grained discrimination between similar objects.
- **ScreenSpot**: https://github.com/njucckevin/SeeClick — GUI element grounding. "Click the submit button." Specialized for agent tasks.
- **gRefCOCO**: Generalized referring expression dataset supporting expressions that refer to multiple objects or no objects.

## Task Format

**Referring expression comprehension**:
- Input: Image + referring expression (e.g., "the woman in the red dress standing next to the fountain").
- Output: Bounding box as [x_min, y_min, x_max, y_max] in absolute or normalized coordinates.
- Verification: IoU >= 0.5 with ground truth box.

**Phrase grounding**:
- Input: Image + caption with marked phrases (e.g., "[A man] is throwing [a frisbee] to [his dog] in [the park]").
- Output: Bounding box for each marked phrase.
- Verification: Per-phrase IoU, average over all phrases.

**Point-based grounding**:
- Input: Image + description.
- Output: (x, y) point coordinate.
- Verification: Point falls within ground truth bounding box.

**GUI element grounding**:
- Input: Screenshot + element description (e.g., "the search bar at the top of the page").
- Output: Bounding box or click coordinates.
- Verification: IoU or point-in-box.

## Difficulty Curriculum

1. **Unique objects** (easy): "the dog" when there is only one dog. No disambiguation needed.
2. **Attribute-based grounding** (easy-medium): "the red car" — one distinguishing attribute.
3. **Spatial grounding** (medium): "the person on the left" — requires spatial reasoning.
4. **Relational grounding** (medium-hard): "the cup next to the laptop" — requires understanding relationships.
5. **Multi-attribute grounding** (hard): "the tall woman in the blue jacket holding a phone" — multiple constraints.
6. **Fine-grained discrimination** (hard): "the slightly taller of the two men" — requires detailed comparison.
7. **Contextual grounding** (very hard): "the person who just scored" — requires scene understanding.
8. **Ambiguous expressions** (very hard): Expressions where the referent is genuinely hard to determine. Cops-Ref level.
9. **Small/occluded objects** (very hard): Objects that are small or partially hidden.

## Limitations & Risks

- **Bounding box imprecision**: Ground truth bounding boxes are human-annotated and inherently imprecise. Annotators disagree on tight vs loose boxes, especially for irregularly shaped objects. The IoU threshold partially accounts for this but cannot fully resolve it.
- **IoU gaming**: An agent can learn to predict slightly larger boxes to increase IoU with various ground truths, rather than precisely localizing objects. GIoU partially addresses this.
- **Expression ambiguity**: Some referring expressions are genuinely ambiguous — "the man" in a crowd could refer to several individuals. Datasets try to avoid this but it exists.
- **Dataset bias**: RefCOCO datasets are built on COCO images, which have their own distribution biases (Western-centric, certain object categories overrepresented).
- **Output format for LLMs**: LLMs must output bounding box coordinates as numbers, which is an unusual output modality. Format errors (wrong coordinate order, non-numeric output) are common failure modes distinct from actual grounding failure.
- **Single-object assumption**: Most benchmarks assume each expression refers to exactly one object. Multi-object grounding is underexplored (gRefCOCO addresses this).

## Connections

- [[visual-question-answering]] — VQA asks "what?" about an image; grounding asks "where?" — complementary tasks
- [[spatial-reasoning]] — Spatial grounding requires spatial reasoning
- [[gui-navigation]] — GUI element grounding is directly used in GUI navigation agents
- [[image-classification]] — Classification and grounding are both object-centric vision tasks
- [[web-navigation]] — Element identification in web navigation is a form of grounding
