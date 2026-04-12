---
title: Exhaustive Domain Audit
description: Systematic scan for missing RLVR domains across all branches of knowledge
---

# Exhaustive Domain Audit

Scanned every branch of knowledge against the 243 existing domains. Below are genuinely missing domains that are:
1. Distinct from existing coverage (not a sub-task)
2. Have concrete, automatable verification
3. Have internet-scale datasets

## Missing: NLP Tasks (distinct from existing coverage)

### Coreference Resolution
- **What**: Link pronouns/mentions to their antecedents ("John went to the store. **He** bought milk.")
- **Verification**: B3/MUC/CEAF-φ4 metrics against gold clusters
- **Datasets**: OntoNotes 5.0 (1.7M words), CoNLL-2012 (2K docs), LitBank, GAP
- **Why distinct**: Not covered by information-extraction (which is NER/relations) or reading-comprehension

### Dialogue State Tracking
- **What**: Track slot-value pairs in task-oriented conversation ("I want a cheap Italian restaurant in the center")
- **Verification**: Joint goal accuracy — all slots must match gold state simultaneously
- **Datasets**: MultiWOZ (10K dialogues), SGD (20K dialogues), DSTC2-11
- **Why distinct**: Conversation-level state tracking, not single-turn QA or extraction

### Semantic Role Labeling
- **What**: "Who did what to whom?" — label predicate-argument structure
- **Verification**: F1 on labeled spans against gold (standard CoNLL eval)
- **Datasets**: PropBank (113K predicates), CoNLL-2005, CoNLL-2009, FrameNet
- **Why distinct**: Not IE (which extracts entities/relations) or parsing (which is syntactic)

### Semantic Textual Similarity
- **What**: Score sentence pair similarity on 0-5 scale
- **Verification**: Pearson/Spearman correlation with gold human scores
- **Datasets**: STS Benchmark (8.6K pairs), SICK (10K), STS12-16, PAWS
- **Why distinct**: Continuous similarity scoring, not binary NLI

### Morphological Inflection
- **What**: Given lemma + morphological features, produce inflected form ("run" + PAST → "ran")
- **Verification**: Exact string match
- **Datasets**: UniMorph (400+ languages, millions of forms), SIGMORPHON shared tasks
- **Why distinct**: Sub-word linguistic transformation, not grammar correction

### Word Sense Disambiguation
- **What**: Given word in context, identify correct sense from sense inventory
- **Verification**: Match gold sense key
- **Datasets**: SemEval WSD tasks, WordNet sense-tagged corpora, WSD Evaluation Framework
- **Why distinct**: Not entity-linking (which links to KB entities, not word senses)

### Text Normalization
- **What**: Convert non-standard text to standard form ("Feb 3rd" → "February third", "$3.50" → "three dollars and fifty cents")
- **Verification**: Exact string match against gold
- **Datasets**: Google Text Normalization (1.1B tokens), ITN datasets
- **Why distinct**: Not spelling/grammar (which fixes errors), this normalizes valid but non-standard forms

### Data-to-Text Generation
- **What**: Generate natural language from structured data (tables, graphs, RDF triples)
- **Verification**: Check all source facts are mentioned and accurate (slot error rate)
- **Datasets**: WebNLG (25K instances), E2E NLG (50K), ToTTo (120K), DART
- **Why distinct**: Not QA (which extracts from text) — this goes DATA → TEXT with verifiable faithfulness

## Missing: Vision Tasks

### Human Pose Estimation
- **What**: Detect body keypoints (joints) in images
- **Verification**: PCK (Percentage of Correct Keypoints) / OKS (Object Keypoint Similarity)
- **Datasets**: COCO-Pose (250K+ people), MPII (40K), CrowdPose, OCHuman
- **Why distinct**: Not segmentation (pixel masks) or detection (boxes) — keypoint localization

### Optical Flow Estimation
- **What**: Estimate per-pixel motion between consecutive video frames
- **Verification**: End-Point Error (EPE) against ground truth flow fields
- **Datasets**: Sintel (1K+ frame pairs), KITTI Flow (400 pairs), FlyingChairs (22K)
- **Why distinct**: Dense motion estimation, not object detection or video QA

### Stereo Depth Estimation
- **What**: Compute depth from stereo image pair
- **Verification**: Disparity error / absolute depth error against LiDAR ground truth
- **Datasets**: KITTI Stereo (400 pairs), SceneFlow (39K), Middlebury Stereo
- **Why distinct**: Two-view geometry problem, not monocular depth (covered in 3d-scene)

### Medical Image Segmentation
- **What**: Segment organs, tumors, lesions in medical images (CT, MRI, X-ray)
- **Verification**: Dice coefficient / Hausdorff distance against expert annotations
- **Datasets**: Medical Segmentation Decathlon (10 tasks), BraTS (brain tumors), ACDC (cardiac), LiTS (liver)
- **Why distinct**: Different from general segmentation — specialized anatomy, different image modalities, clinical significance

### Face Recognition / Detection
- **What**: Detect faces in images and/or identify individuals
- **Verification**: mAP for detection, accuracy/TAR@FAR for recognition
- **Datasets**: WIDER Face (32K images), LFW (13K faces), MegaFace, IJB-C
- **Why distinct**: Specific biometric task with its own benchmarks, not general detection

## Missing: Biomedical

### Forward Reaction Prediction
- **What**: Given reactants + reagents, predict products
- **Verification**: Compare predicted product SMILES to known products (exact or Tanimoto similarity)
- **Datasets**: USPTO (1.8M reactions), Pistachio, ORD (Open Reaction Database)
- **Why distinct**: Forward direction of retrosynthesis — different task, different models

### Protein-Ligand Docking
- **What**: Predict how a small molecule binds to a protein (pose + affinity)
- **Verification**: RMSD of predicted pose vs crystal structure, correlation of predicted vs measured affinity
- **Datasets**: PDBbind (23K complexes), CASF benchmark, DUD-E (decoys)
- **Why distinct**: Binding prediction, not protein design or molecular generation

### Spectrometry Interpretation
- **What**: Given NMR/mass/IR spectrum, identify the molecule or functional groups
- **Verification**: Match molecular formula or SMILES against ground truth
- **Datasets**: SDBS (35K spectra), MassBank (100K+), HMDB, NIST Chemistry WebBook
- **Why distinct**: Signal → molecule inference, not property prediction

### ECG / Biosignal Interpretation
- **What**: Classify cardiac arrhythmias from ECG waveforms
- **Verification**: Match cardiologist-validated labels (normal/AF/VT/etc.)
- **Datasets**: MIT-BIH (48 records, 110K beats), PhysioNet/CinC (40K+ recordings), PTB-XL (22K 12-lead ECGs)
- **Why distinct**: Physiological signal processing, not text-based medical diagnosis

### Medical Coding (ICD/CPT)
- **What**: Assign diagnosis/procedure codes to clinical notes
- **Verification**: Match gold ICD-10/CPT codes (micro/macro F1)
- **Datasets**: MIMIC-III with ICD codes (50K+ admissions), CodiEsp (Spanish), CANTEMIST
- **Why distinct**: Multi-label classification of clinical text with standardized code taxonomy

## Missing: Chemistry & Physics (specific)

### Chemical Equation Balancing
- **What**: Balance a chemical equation (e.g., _Fe + _O₂ → _Fe₂O₃)
- **Verification**: Count atoms on both sides — must be equal for every element
- **Datasets**: Chemistry textbook exercises (unlimited procedural generation)
- **Why distinct**: Pure constraint satisfaction on chemical formulas, not property prediction

### Phase Diagram Computation
- **What**: Determine phase (solid/liquid/gas) at given temperature and pressure
- **Verification**: Compare to known phase diagrams / Antoine equation
- **Datasets**: NIST Chemistry WebBook, CRC Handbook
- **Why distinct**: Thermodynamic state computation, not general chemistry

## Missing: Mathematics (specific)

### Combinatorial Enumeration
- **What**: Count objects satisfying constraints (how many ways to arrange N items with restrictions?)
- **Verification**: Exact integer match against computed answer
- **Datasets**: Competition math, OEIS sequences, combinatorics textbooks
- **Why distinct**: Counting problems are a distinct mathematical skill from optimization or algebra

### Recurrence Relation Solving
- **What**: Given a recurrence (e.g., a(n) = 2a(n-1) + 3a(n-2)), find closed-form solution
- **Verification**: Evaluate closed form and recurrence for first 100 terms — must match
- **Datasets**: OEIS (360K+ sequences with formulas), combinatorics textbooks
- **Why distinct**: Specific algebraic technique, not general symbolic math

## Missing: Social Science / Behavioral

### Recommender Systems
- **What**: Predict user ratings or rankings for items
- **Verification**: RMSE/MAE for ratings, NDCG/Precision@K for rankings
- **Datasets**: MovieLens (25M ratings), Amazon Reviews (233M), Netflix Prize, Last.fm
- **Why distinct**: Collaborative filtering / content-based prediction, not general ML

### Emotion Recognition
- **What**: Classify emotion from text, audio, or facial expressions
- **Verification**: Match gold labels (Ekman categories or valence-arousal scores)
- **Datasets**: GoEmotions (58K Reddit comments), IEMOCAP (12h dialogues), FER-2013 (35K face images), AffectNet
- **Why distinct**: Cross-modal emotion inference, not just text classification

## Missing: Signal Processing (specific)

### Audio Source Separation
- **What**: Separate mixed audio into individual sources (vocals, drums, bass, etc.)
- **Verification**: SDR (Signal-to-Distortion Ratio) against ground truth isolated tracks
- **Datasets**: MUSDB18 (150 tracks), DSD100, FUSS
- **Why distinct**: Not transcription (pitch detection) — signal decomposition

## Missing: Systems / Software (specific)

### Git Merge Conflict Resolution
- **What**: Given a merge conflict (<<<< ==== >>>>), produce the correct resolution
- **Verification**: Resolution matches the actual human-resolved version from git history
- **Datasets**: GitHub merge commits (millions extractable), MergeConflictBench
- **Why distinct**: Not code-repair (which fixes bugs) — this resolves conflicting changes

### Log Analysis / Anomaly Detection
- **What**: Identify anomalous patterns in system logs
- **Verification**: Match against labeled anomalies (F1/AUC)
- **Datasets**: HDFS logs (11M lines), BGL (5M lines), Thunderbird, Loghub (2B+ log messages)
- **Why distinct**: Not general text classification — structured log patterns, temporal dependencies

## Missing: Creative / Design

### Pixel Art Generation
- **What**: Generate pixel art matching descriptions (constrained to low resolution, limited palette)
- **Verification**: Palette constraint check + visual similarity to reference
- **Datasets**: Pixel art communities (DeviantArt, itch.io assets), sprite databases
- **Why distinct**: More constrained than general image generation, fully programmatic palette/resolution checks

### Typographic Layout
- **What**: Arrange text elements on a page/poster satisfying design constraints
- **Verification**: No text overlap, readability (font size ≥ threshold), alignment, grid compliance
- **Datasets**: Magazine layouts, poster datasets, Crello design dataset
- **Why distinct**: Not HTML/CSS (which is web) — print/poster layout with different constraints

## Summary

| Category | Missing Domains | Total New |
|---|---|---|
| NLP | Coreference, dialogue state, SRL, STS, morphology, WSD, text norm, data-to-text | 8 |
| Vision | Pose estimation, optical flow, stereo depth, medical segmentation, face detection | 5 |
| Biomedical | Forward reaction, docking, spectrometry, ECG, medical coding | 5 |
| Chemistry/Physics | Equation balancing, phase diagrams | 2 |
| Mathematics | Combinatorial enumeration, recurrence relations | 2 |
| Social Science | Recommender systems, emotion recognition | 2 |
| Signal Processing | Audio source separation | 1 |
| Systems | Merge conflict resolution, log anomaly detection | 2 |
| Creative | Pixel art, typographic layout | 2 |
| **Total** | | **~29** |

This would bring the total to approximately **272 domains**.

After this, I believe coverage is genuinely exhaustive for domains that satisfy all three criteria (distinct + verifiable + internet-scale datasets). Remaining uncovered areas are either:
- Sub-tasks of existing domains (e.g., "sentiment analysis" = text-classification)
- Not programmatically verifiable (creative writing quality, humor, persuasion)
- Too niche for internet-scale datasets (paleontology, philately)
