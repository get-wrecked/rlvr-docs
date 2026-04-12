---
domain: image-classification
category: vision/classification
verification_type: exact_match
dataset_scale: >10M labeled images
difficulty_range: easy/medium/hard
modality: vision
status: verified
---

# Image Classification

## Overview

Image classification — assigning a label to an image from a fixed set of categories — is the most foundational computer vision task and one of the simplest RLVR domains. Verification is trivially exact match: the predicted label either matches the ground truth or it does not. While modern vision models have largely "solved" standard benchmarks (ImageNet top-1 accuracy >90%), classification remains relevant for RLVR because: (1) it provides a massive, clean reward signal for pre-training visual representations, (2) fine-grained and domain-specific classification remains challenging, and (3) it serves as a building block for more complex multimodal tasks. The domain has the largest labeled datasets in computer vision.

## Verification Mechanism

1. **Exact label match** (exact match): Predicted class label matches ground truth label. Binary reward. The simplest possible verification.
2. **Top-K accuracy** (exact match): Correct label is within the model's top-K predictions. Standard for ImageNet (top-5 accuracy). Reward = 1 if correct label in top-K, 0 otherwise.
3. **Hierarchical match** (rule-based): For fine-grained classification, partial credit for predicting a nearby class in the taxonomy. E.g., predicting "Golden Retriever" when the answer is "Labrador Retriever" is closer than predicting "Cat." WordNet hierarchy provides the structure. More nuanced but less standard for RLVR.
4. **Multi-label match** (exact match): For images with multiple valid labels, compute precision/recall over predicted label set vs ground truth set. F1-based reward.

## Dataset Sources

- **ImageNet (ILSVRC)**: https://www.image-net.org/ — 1.28M training images, 1000 classes. The foundational large-scale classification benchmark. Well-curated labels, extensive study of failure modes.
- **ImageNet-21K**: ~14M images across 21,841 classes. Much larger but noisier labels.
- **CIFAR-10 / CIFAR-100**: https://www.cs.toronto.edu/~kriz/cifar.html — 60K 32x32 images in 10/100 classes. Classic small-scale benchmark. Still useful for fast experimentation.
- **Oxford Flowers 102**: https://www.robots.ox.ac.uk/~vgg/data/flowers/ — 8,189 images of 102 flower species. Fine-grained.
- **Stanford Cars**: https://ai.stanford.edu/~jkrause/cars/car_dataset.html — 16,185 images of 196 car makes/models/years.
- **CUB-200-2011**: https://www.vision.caltech.edu/datasets/cub_200_2011/ — 11,788 images of 200 bird species. Fine-grained, requires expert knowledge.
- **iNaturalist**: https://github.com/visipedia/inat_comp — Millions of images across 10K+ species (plants, animals, fungi). Extreme fine-grained classification. Long-tailed distribution.
- **Food-101**: https://data.vision.ee.ethz.ch/cvl/datasets_extra/food-101/ — 101K images of 101 food categories.
- **Places365**: http://places2.csail.mit.edu/ — 1.8M images of 365 scene categories. Scene recognition rather than object classification.
- **ObjectNet**: https://objectnet.dev/ — 50K images testing object recognition robustness. Controls for viewpoint, rotation, and background.
- **VTAB (Visual Task Adaptation Benchmark)**: https://google-research.github.io/task_adaptation/ — 19 diverse classification tasks for evaluating visual representations.

## Task Format

**Standard classification**:
- Input: Image + instruction (e.g., "What object is in this image?") or image + class options.
- Output: Class label (e.g., "golden retriever", "sports car", "pizza").
- Verification: Exact string match after normalization.

**Multiple choice classification**:
- Input: Image + "Is this a [A] dog [B] cat [C] bird [D] fish?"
- Output: Selected option (e.g., "A").
- Verification: Exact match.

**Fine-grained classification**:
- Input: Image + domain context (e.g., "What species of bird is this?").
- Output: Fine-grained label (e.g., "Painted Bunting").
- Verification: Exact match.

**Multi-label classification**:
- Input: Image + "What objects are present?"
- Output: Set of labels (e.g., {"person", "bicycle", "backpack"}).
- Verification: Set F1 against ground truth labels.

## Difficulty Curriculum

1. **CIFAR-10 level** (easy): 10 highly distinct categories. Low resolution.
2. **ImageNet coarse categories** (easy-medium): Distinguish between broad groups (animal vs vehicle vs food).
3. **ImageNet full 1000-class** (medium): Fine enough categories to be challenging (many dog breeds, many vehicle types).
4. **Fine-grained, common species** (medium-hard): CUB-200, Stanford Cars. Requires attention to detail.
5. **Fine-grained, rare species** (hard): iNaturalist long-tail. Rare species with few training examples.
6. **Adversarial/OOD images** (hard): ObjectNet-style unusual viewpoints, rotations, backgrounds.
7. **Ultra-fine-grained** (very hard): Subspecies-level identification, cultivar identification. Requires expert knowledge.
8. **Novel category detection** (very hard): Identify when an image does not belong to any known category.

## Limitations & Risks

- **Largely solved for common benchmarks**: Standard ImageNet classification is above human accuracy for most architectures. The marginal RLVR value of further training on well-trodden benchmarks is low.
- **Label noise**: Even curated datasets have ~5-10% label errors. ImageNet has known systematic errors (Re-evaluated labels: ImageNet-ReaL). This noise directly corrupts the reward signal.
- **Single-label assumption**: Most images contain multiple objects, but standard classification assigns a single label. The "correct" label can be ambiguous.
- **Distribution shift**: Models trained on ImageNet-style web images may fail on medical images, satellite imagery, or industrial inspection images. Domain-specific datasets are needed.
- **Simplistic as an RLVR task**: For LLMs, image classification may be too simple to drive interesting reasoning improvements. It primarily trains visual perception, not multi-step reasoning.
- **Dataset biases**: ImageNet has documented biases in demographics, geography, and category selection. These propagate through RLVR training.

## Connections

- [[visual-question-answering]] — VQA subsumes classification (classification is VQA with "What is this?" as the question)
- [[document-ocr-extraction]] — Document classification is a related task
- [[visual-grounding]] — Both are object-centric vision tasks; classification identifies "what," grounding identifies "where"
- [[spatial-reasoning]] — Scene classification (Places365) requires spatial understanding
- [[multimodal-reasoning]] — Classification provides the perceptual foundation for higher-level reasoning
