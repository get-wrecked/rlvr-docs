# Honesty Audit: What I Actually Know vs. What I'm Remembering

This document re-examines every dataset/benchmark claim in the 644-domain catalog. For each, I ask:
- **Can I give a specific, confirmed URL?** If not → `REMEMBERED`
- **Am I certain about the license?** If not → `REMEMBERED`
- **Was this announced but possibly never publicly released?** → `REMEMBERED`
- **Has this project possibly been abandoned/moved/relicensed?** → `REMEMBERED`
- **Am I estimating the dataset size from memory?** If significantly uncertain → flagged

## Criteria for REMEMBERED

Anything where I'm working from training-data recall rather than high-confidence knowledge gets flagged. This includes:
- Benchmarks announced in 2024-2025 papers that may not have public downloads
- Datasets where I recall the name but not the exact hosting/license
- Sizes I estimated rather than looked up
- Tools I know exist but haven't confirmed are maintained
- Datasets that may have been taken down or relicensed

---

## Domains with REMEMBERED data sources (honest reassessment)

### Frontier Benchmarks (announced 2024-2025 — verify all)

| Domain | What I claimed | Honesty check |
|--------|---------------|---------------|
| `frontier-mathematics` | FrontierMath by Epoch AI | `REMEMBERED` — Announced late 2024. I recall it was initially closed/invitation-only. May or may not be public now. I cannot confirm a download URL. |
| `humanity-last-exam` | HLE-style questions | `REMEMBERED` — "Humanity's Last Exam" was announced by Scale AI / Center for AI Safety around late 2024. I don't know if the full dataset was ever publicly released. |
| `arc-agi-abstraction` | ARC-AGI-2 | Partially `REMEMBERED` — ARC-AGI-1 (Chollet's original) is definitely Apache on GitHub. ARC-AGI-2 was released for the 2024 prize competition — I believe it's public but should verify the exact URL/license of the v2 test set. |
| `ml-engineering-competition` | MLE-bench by OpenAI | `REMEMBERED` — Announced mid-2024. I recall the paper but am not 100% certain the benchmark was publicly released with all 75 Kaggle tasks packaged. The underlying Kaggle data has per-competition licenses. |
| `research-engineering` | RE-bench by METR | `REMEMBERED` — I recall this from a 2024 paper by METR. Not certain it's publicly available as a downloadable benchmark. |
| `live-code-benchmark` | LiveCodeBench | `REMEMBERED` — I recall this from 2024 papers. Believe it's on GitHub/HuggingFace but haven't confirmed exact URL. |
| `multi-tool-orchestration` | GAIA benchmark | `REMEMBERED` — GAIA was released by Meta/HuggingFace ~2023-2024. I believe it's on HuggingFace but should verify exact access (may require login). |
| `aider-code-editing` | Aider benchmark | `REMEMBERED` — I know Aider (the tool) is open source. The specific benchmark suite — I recall Paul Gauthier published results but am not sure if the benchmark is packaged as a standalone download. |
| `class-level-code-generation` | ClassEval | `REMEMBERED` — I recall this from a 2023-2024 paper. Believe it's on GitHub but haven't confirmed URL/license. |
| `graduate-level-qa` | GPQA Diamond | Partially `REMEMBERED` — I'm fairly confident GPQA is on HuggingFace (CC-BY license from the paper). But I should verify the Diamond subset is publicly accessible vs. held out. |
| `code-agent-swe-task` | SWE-agent | Likely `OPEN` — SWE-agent is definitely on GitHub (MIT). SWE-bench is definitely on HuggingFace. This one I'm fairly confident about. |
| `usaco-competitive-programming` | USACO problems | Likely `OPEN` — USACO.org has public problem archives. But I'm not 100% sure there's a clean packaged dataset (vs. scraping the website). |

### Newer ML Benchmarks

| Domain | What I claimed | Honesty check |
|--------|---------------|---------------|
| `adversarial-robustness` | RobustBench | `REMEMBERED` — I recall RobustBench from papers. Believe it's Apache on GitHub but haven't confirmed current status. |
| `ood-detection` | OpenOOD | `REMEMBERED` — I recall this benchmark. Should verify GitHub URL and license. |
| `neural-architecture-search` | NAS-Bench-101/201/301 | Likely `OPEN` — NAS-Bench-101 is definitely from Google, on Google Cloud. But access may require specific steps. NAS-Bench-201/301 I'm less certain about exact hosting. |
| `fairness-bias-auditing` | AIF360 | Likely `OPEN` — IBM AIF360 is definitely on GitHub (Apache). Fairlearn is MIT. These I'm fairly confident about. |
| `data-cleaning-deduplication` | Magellan, DeepMatcher | `REMEMBERED` — I recall these from papers. Not certain about current maintenance or exact URLs. |
| `anomaly-detection` | ODDS, NAB, ADBench | `REMEMBERED` — I recall ODDS (Outlier Detection DataSets) and NAB (Numenta). ADBench is newer. Should verify URLs for all three. |
| `causal-discovery` | CauseMe, Tuebingen pairs | `REMEMBERED` — I recall CauseMe from the causality community and Tuebingen cause-effect pairs. Should verify hosting. |
| `knowledge-distillation` | No specific dataset | `PROCEDURAL` — but I should note this has no established benchmark, unlike what the domain page might imply. |
| `model-pruning-quantization` | No specific dataset | `PROCEDURAL` — same as above. |
| `reservoir-computing` | No specific dataset | `PROCEDURAL` — no established benchmark I can name. |
| `loss-function-design` | No specific dataset | `PROCEDURAL` — no established benchmark. |
| `data-augmentation-design` | AutoAugment, RandAugment | These are methods, not datasets. The domain uses standard datasets (CIFAR, ImageNet) which are `OPEN`. But the claim of a dedicated benchmark is `REMEMBERED`. |

### Agent Environments

| Domain | What I claimed | Honesty check |
|--------|---------------|---------------|
| `embodied-instruction-following` | ALFRED, TEACh | Likely `OPEN` — ALFRED is definitely on GitHub (MIT). TEACh from Amazon — I believe it's public but should verify license. AI2-THOR is Apache. |
| `household-task-simulation` | BEHAVIOR-1K, iGibson | `REMEMBERED` — BEHAVIOR was evolving rapidly. The 1K version may or may not be fully released. iGibson/OmniGibson I believe are MIT but should verify. |
| `nethack-minihack` | NLE, MiniHack | Likely `OPEN` — NLE (NetHack Learning Environment) from Meta is MIT on GitHub. MiniHack is Apache. Fairly confident. |
| `crafter-open-world` | Crafter | Likely `OPEN` — By Danijar Hafner, MIT license on GitHub. Fairly confident. |
| `overcooked-coordination` | Overcooked-AI | Likely `OPEN` — MIT on GitHub from UC Berkeley. Fairly confident. |
| `hanabi-cooperative` | Hanabi LE | `REMEMBERED` — I recall DeepMind released the Hanabi Learning Environment. Believe it's Apache but should verify it's still maintained and the exact repo. |
| `diplomacy-negotiation` | CICERO, webDiplomacy | `REMEMBERED` — Meta released CICERO code (MIT?) but the full game data pipeline — I'm less certain. webDiplomacy game logs — not sure about access. |
| `starcraft-micromanagement` | PySC2, SMACv2 | Partially `REMEMBERED` — PySC2 is definitely Apache (DeepMind). SMACv2 — I recall it's on GitHub but should verify exact repo/license. Also, StarCraft II is needed (free-to-play, but Blizzard's SC2 API had some changes). |
| `meta-world-robotic-manipulation` | Meta-World | `REMEMBERED` — I recall Meta-World from the UC Berkeley RL group. Should be MIT on GitHub but should verify it's still maintained. |
| `procgen-generalization` | Procgen | Likely `OPEN` — OpenAI, MIT license. Fairly confident. But OpenAI has changed their open-source stance — verify the repo is still accessible. |
| `babyai-language-grounding` | BabyAI | `REMEMBERED` — From Mila. I recall it's BSD but should verify current GitHub status. |
| `autonomous-driving-planning` | nuPlan, Waymo Open Motion | `REMEMBERED` — nuPlan from Motional. I recall it's Apache but it's a large download requiring registration. Waymo Open Motion Dataset has a custom license that I'm not certain about (it's "open" but with specific terms). |
| `drone-navigation` | AirSim, Flightmare | `REMEMBERED` — AirSim from Microsoft is MIT but was **archived/deprecated** in 2022. Microsoft moved to Project AirSim (commercial). Flightmare from UZH — should verify current status. **This is an important correction — AirSim is effectively dead.** |
| `dexterous-manipulation` | DexMV, IsaacGym | `REMEMBERED` — DexMV I recall from a paper but am not sure about public availability. IsaacGym is deprecated in favor of IsaacLab. |
| `legged-locomotion` | MuJoCo, IsaacGym | MuJoCo is definitely `OPEN` (Apache). The specific legged locomotion environments — some are in dm_control (Apache), some in IsaacLab (BSD). Generally fine. |
| `tool-augmented-qa` | ToolBench, API-Bank | `REMEMBERED` — ToolBench from Tsinghua. I recall it's Apache on GitHub but should verify. API-Bank — same, recall from paper, should verify. |

### Science Datasets

| Domain | What I claimed | Honesty check |
|--------|---------------|---------------|
| `catalyst-design` | OC20/OC22 | Likely `OPEN` — Open Catalyst Project from Meta/CMU. I'm fairly confident it's CC-BY on the project website. But it's a massive download (TB-scale). Should verify current access. |
| `crystal-structure-prediction` | Materials Project, COD | Materials Project is `OPEN` (CC-BY, free API key). COD (Crystallography Open Database) is `OPEN`. ICSD is definitely `PROPRIETARY`. I'm fairly confident here. |
| `drug-property-prediction` | TDC, MoleculeNet | Likely `OPEN` — TDC (Therapeutics Data Commons) from Harvard is MIT. MoleculeNet from Stanford is MIT. Fairly confident. |
| `scientific-hypothesis-testing` | ScienceWorld | `REMEMBERED` — From Allen AI. I believe it's Apache but should verify the exact GitHub repo and whether it's maintained. |
| `materials-property-prediction` | Materials Project, AFLOW, JARVIS | Materials Project: confident `OPEN`. AFLOW: `REMEMBERED` — I recall CC-BY but should verify. JARVIS from NIST: `REMEMBERED` — I recall it's public but should verify exact terms. |
| `protein-protein-interaction` | STRING, IntAct, BioGRID | STRING: `OPEN` (CC-BY). IntAct: `OPEN` (CC-BY). BioGRID: `REMEMBERED` — I recall MIT but should verify. |
| `antibiotic-resistance-prediction` | PATRIC/BV-BRC, CARD | `REMEMBERED` — PATRIC was renamed to BV-BRC. I recall it's public but the exact access mechanism may have changed. CARD is from McMaster — I believe it's public but should verify. |
| `protein-secondary-structure` | PDB + DSSP, CB513 | PDB is definitely `OPEN` (public). DSSP assignments: `REMEMBERED` — they used to be available but hosting has changed. CB513: `REMEMBERED` — classic dataset, should verify if still hosted. |
| `retrosynthetic-step-verification` | USPTO | Likely `OPEN` — USPTO reaction data is public domain (US government). I'm fairly confident. |
| `alloy-composition` | AFLOW, MatWeb | AFLOW: `REMEMBERED` (see above). MatWeb: `REMEMBERED` — MatWeb has a free tier but full access is paid. I was unclear about this. |
| `hodgkin-huxley-neuron` | NEURON simulator, Brian2 | `REMEMBERED` — NEURON is from Yale, I recall BSD. Brian2 from Marcel Stimberg — I recall CeCILL license. Should verify both are current. |
| `renewable-energy-sizing` | NREL SAM, PVLIB, NSRDB | PVLIB: likely `OPEN` (BSD). NREL SAM: `REMEMBERED` — I know SAM is free but license details are unclear. NSRDB: `REMEMBERED` — NREL data, I believe public but may require API key. |
| `hydrology-computation` | USGS data, GRASS GIS | USGS data: `OPEN` (public domain, US gov). GRASS GIS: `OPEN-GPL`. Fairly confident. |
| `dose-response-modeling` | EPA ToxCast, ChEMBL | ToxCast: `REMEMBERED` — I recall EPA CompTox is public but should verify exact access. ChEMBL: likely `OPEN` (CC-BY-SA from EBI). |
| `enzyme-kinetics` | BRENDA, SABIO-RK | BRENDA: `REMEMBERED` — I know it's from TU Braunschweig and requires registration. Terms may restrict bulk download. SABIO-RK: `REMEMBERED` — I recall it's public but should verify API access. |
| `pharmacokinetics` | DrugBank, PK-DB | DrugBank: `OPEN-NC` (CC-BY-NC, requires agreement). PK-DB: `REMEMBERED` — I recall this from a paper but am not certain about public availability or current maintenance. |
| `pharmacology-interaction` | DrugBank | Same as above — `OPEN-NC` for academic use. |
| `ecological-modeling` | No specific dataset | `PROCEDURAL` — but I should note there's no established benchmark I can name for this. |
| `geomorphology` | SRTM/ASTER DEM | SRTM: `OPEN` (public domain, NASA). ASTER GDEM: `REMEMBERED` — I recall it's free but NASA/METI terms may apply. |
| `acoustic-room-simulation` | No specific dataset | `PROCEDURAL` — Sabine's equation is public knowledge. No specific benchmark. |
| `optical-fiber-computation` | No specific dataset | `PROCEDURAL` — analytical equations. No benchmark I can name. |
| `soil-bearing-capacity` | No specific dataset | `PROCEDURAL` — Terzaghi equations. No benchmark. |
| `power-electronics` | ngspice | ngspice is definitely `OPEN` (BSD). No specific power electronics benchmark though. |
| `antenna-design` | NEC2, OpenEMS | NEC2: `REMEMBERED` — I recall it's public domain (originally US government) but the modern implementations vary. OpenEMS: `REMEMBERED` — I recall GPL but should verify GitHub status. |
| `planetary-ephemeris` | JPL Horizons, DE440 | Likely `OPEN` — JPL Horizons is a public web service (NASA). DE440 ephemeris files are public domain. Fairly confident. |
| `tide-prediction` | NOAA tidal data | `OPEN` — NOAA data is public domain (US government). Confident. |

### Language/NLP Datasets

| Domain | What I claimed | Honesty check |
|--------|---------------|---------------|
| `information-extraction` | CoNLL-2003 | **IMPORTANT**: CoNLL-2003 uses Reuters RCV1 text which is technically **not redistributable** without LDC license. I labeled this `OPEN` but it's actually `PROPRIETARY` for the underlying text. The annotations are public but the text requires LDC. **Use WikiANN (Apache) instead.** |
| `entity-linking` | AIDA-CoNLL, TACKBP | Same Reuters issue for AIDA-CoNLL. TACKBP requires LDC. Both are `PROPRIETARY`. **Use WikiANN-based entity linking.** |
| `coreference-resolution` | OntoNotes | OntoNotes requires LDC license (`PROPRIETARY`). I listed WikiCoref as alternative but should have flagged the primary dataset more strongly. |
| `semantic-role-labeling` | PropBank, CoNLL-2009 | PropBank frames are open, but full annotated data requires LDC (`PROPRIETARY`). |
| `semantic-parsing` | ATIS | ATIS requires LDC. GeoQuery is public. SMCalFlow I believe is MIT but should verify. |
| `formal-grammar` | Penn Treebank | Penn Treebank requires LDC (`PROPRIETARY`). Universal Dependencies is the free alternative (CC-BY-SA). |
| `tokenization-segmentation` | Chinese Treebank | Chinese Treebank requires LDC (`PROPRIETARY`). UD Chinese is the free alternative. |
| `discourse-parsing` | RST-DT | RST Discourse Treebank requires LDC (`PROPRIETARY`). GUM corpus is the free alternative (CC-BY). |
| `relation-extraction` | TACRED | TACRED requires LDC (`PROPRIETARY`). DocRED (MIT) and FewRel (MIT) are alternatives. |
| `named-entity-recognition` | CoNLL-2003 | Same Reuters issue as information-extraction. `PROPRIETARY` for text. |
| `word-sense-disambiguation` | SemCor | `REMEMBERED` — SemCor is widely used in NLP but I'm not certain about its current hosting or exact license. It was part of WordNet-related resources. |
| `text-simplification` | Newsela | Newsela is `PROPRIETARY` (requires license agreement). I listed Wiki-Auto as alternative but should have been clearer. |
| `sentence-ordering` | ROCStories | `REMEMBERED` — ROCStories is from Nasrin Mostafazadeh. I recall it requires filling out a form for access. Not fully open. |
| `long-document-qa` | NarrativeQA, QuALITY | NarrativeQA: `REMEMBERED` — I recall it's Apache from DeepMind but should verify current HuggingFace availability. QuALITY: `REMEMBERED` — from Allen AI, I believe CC-BY but should verify. |
| `counterfactual-reasoning` | TimeTravel, CRASS | `REMEMBERED` — I recall these from papers but haven't confirmed exact URLs/licenses. |
| `linguistics-olympiad` | IOL/NACLO archives | Likely `OPEN` — IOL and NACLO publish past problems on their websites. But I haven't verified they're all freely downloadable in a structured format. |
| `abbreviation-expansion` | Ab3P, MEDLINE | Ab3P: `REMEMBERED` — I recall it's from NCBI (public domain) but should verify. MEDLINE abstracts: `OPEN` via PubMed (NLM). |
| `aspect-sentiment-analysis` | SemEval ABSA, MAMS | `REMEMBERED` — I recall these from SemEval shared tasks. The data is usually available from the shared task website but exact URLs may have changed. |
| `text-to-table` | SciREX, SciGen | `REMEMBERED` — I recall SciREX from Allen AI and SciGen. Should verify both are publicly available. |
| `question-generation` | SQuAD-based | SQuAD is `OPEN` (CC-BY-SA). Using it for QG evaluation is straightforward. This one is fine. |
| `grammatical-error-correction` | BEA-2019 | `REMEMBERED` — The BEA-2019 shared task data. I recall it's CC-BY-SA but should verify the W&I+LOCNESS dataset is still available at the original URL. |
| `paraphrase-detection` | QQP, PAWS, MRPC | QQP: from Quora, public but terms unclear. PAWS: `REMEMBERED` (Google, I recall Apache). MRPC: `REMEMBERED` — Microsoft Research Paraphrase Corpus, has a specific MSR license. |
| `visual-commonsense` | VCR | `REMEMBERED` — VCR from AI2. I recall a custom research license that may restrict use. Should verify. |
| `video-temporal-reasoning` | NExT-QA, EgoSchema | `REMEMBERED` — I recall these from papers but haven't confirmed exact licensing. NExT-QA I believe is CC-BY-NC. EgoSchema — less certain. |
| `interleaved-multimodal` | MuirBench, DEMON | `REMEMBERED` — Both are from 2024 papers. Haven't confirmed public availability. |

### Games & Puzzles

| Domain | What I claimed | Honesty check |
|--------|---------------|---------------|
| `crossword-solving` | NYT clue databases | `REMEMBERED` — NYT crossword clues are copyrighted. There are scraped databases on GitHub but their legality is questionable. **Should note this is a copyright gray area.** |
| `game-playing-atari` | ALE ROMs | The Atari ROMs have a legal gray area. ALE now bundles some ROMs but Atari/Hasbro IP is involved. `REMEMBERED` as to current legal status. |
| `hanabi-cooperative` | Hanabi LE | See agent section above. |
| `diplomacy-negotiation` | CICERO | See agent section above. |
| `bridge-card-play` | BBO data, DDS | `REMEMBERED` — BBO hand records: I recall they're somewhat public but not in a clean dataset format. DDS (Double Dummy Solver): I recall Apache license but should verify. |
| `shogi` | YaneuraOu | `REMEMBERED` — I recall YaneuraOu is a GPL shogi engine but should verify it's still maintained and exact repo. |
| `checkers` | Chinook database | `REMEMBERED` — Chinook solved checkers (Schaeffer et al.). The endgame databases were public but I'm not sure about current hosting. |
| `sliding-block-klotski` | Standard puzzles | These are well-known puzzles in the public domain, but I can't point to a specific packaged dataset. |
| `nonogram-solving` | webpbn.com | `REMEMBERED` — I recall webpbn.com has an archive of puzzles but should verify it's still up and accessible. |
| `chess-endgame-tablebase` | Syzygy tablebases | Likely `OPEN` — Syzygy is definitely public (Ronald de Man). Available via Lichess API. Fairly confident. |

### Security

| Domain | What I claimed | Honesty check |
|--------|---------------|---------------|
| `network-intrusion-detection` | CICIDS2017, NSL-KDD | `REMEMBERED` — I recall these are from the Canadian Institute for Cybersecurity. Should verify they're still hosted and downloadable. NSL-KDD is the corrected version of KDD Cup 1999 — I recall it's public but hosting may have changed. |
| `malware-classification` | EMBER, SOREL-20M, MalwareBazaar | EMBER: `REMEMBERED` — I recall it's Apache from Endgame/Elastic. SOREL-20M: `REMEMBERED` — I recall from Sophos. MalwareBazaar: `REMEMBERED` — from abuse.ch, I believe free for research. Should verify all three. |
| `binary-exploitation` | pwnable.kr, exploit.education | `REMEMBERED` — These are CTF practice sites. pwnable.kr is likely still up. exploit.education: `REMEMBERED` — I recall CC-BY-NC-SA but should verify. |

### Expert/Professional

| Domain | What I claimed | Honesty check |
|--------|---------------|---------------|
| `bar-exam-mbe` | MBE practice questions | `REMEMBERED` — I cannot confirm any open dataset of MBE questions. NCBE materials are copyrighted. **This domain likely has NO open dataset.** |
| `cpa-exam` | CPA review materials | `REMEMBERED` — Same issue. Becker, Roger, Wiley CPA materials are all proprietary. **No open dataset.** |
| `fe-exam` | FE review materials | `REMEMBERED` — Same issue. NCEES materials are copyrighted. **No open dataset.** |
| `gre-quantitative` | GRE prep | `REMEMBERED` — ETS materials are copyrighted. Some prep sites have free practice questions but no clean open dataset I can confirm. |
| `medical-diagnosis` | MedQA | Likely `OPEN-REG` — MedQA is on HuggingFace, requires login. The original USMLE questions are copyrighted by NBME, but MedQA specifically aggregates practice questions. I'm mostly confident but the upstream IP is murky. |
| `construction-estimation` | RS Means | RS Means full data is `PROPRIETARY` (very expensive). I listed "excerpts public" which is misleading — only very limited info is free. |
| `real-estate-valuation` | Zillow Prize, Ames Housing | Ames Housing is definitely `OPEN` (CC0). Zillow Prize dataset: `REMEMBERED` — was released for a Kaggle competition but may have been taken down after. |
| `wine-quality-prediction` | UCI Wine Quality | Likely `OPEN` — UCI ML Repository is CC-BY. This is a classic dataset. Fairly confident. |
| `library-cataloging` | WorldCat, OpenLibrary | OpenLibrary: `OPEN` (CC0, from Internet Archive). WorldCat: `REMEMBERED` — OCLC's API has free tier but terms may restrict bulk use. |
| `insurance-premium-calculation` | SOA actuarial tables | `REMEMBERED` — SOA publishes some tables publicly but I'm not certain about bulk programmatic access. |
| `sports-statistics` | Retrosheet, StatsBomb | Retrosheet: `OPEN` (public, with attribution requirement). StatsBomb: `REMEMBERED` — they have open data but I'm not certain about exact license terms. |
| `sports-bracket-prediction` | NCAA data, 538 | NCAA tournament data is public. 538 (now ABC) predictions: `REMEMBERED` — used to be on GitHub but 538 was shut down. Data may be archived. |
| `aviation-flight-planning` | ICAO data, FAA charts | FAA charts: `OPEN` (public domain, US gov). ICAO data: `PROPRIETARY` (ICAO publications are expensive). I listed this as `MIXED` but should clarify. |
| `clinical-trial-design` | ClinicalTrials.gov | `OPEN` — ClinicalTrials.gov is public domain (US NIH). API available. Confident. |
| `ethereum-mev-extraction` | Flashbots data | `REMEMBERED` — Flashbots MEV-Explore data. I recall it's public but should verify current access and terms. |
| `music-harmony-analysis` | Annotated chord datasets | `REMEMBERED` — I recall datasets like the Billboard annotations and Isophonics but should verify exact hosting. |
| `mortgage-amortization` | No specific dataset | `PROCEDURAL` — PMT formula is public knowledge. This is fine. |
| `payroll-computation` | IRS Pub 15-T | `OPEN` — IRS publications are public domain. Confident. |
| `blood-type-compatibility` | No dataset needed | `PROCEDURAL` — fixed compatibility chart. This is fine. |
| `supply-chain-network-design` | SNDlib | `REMEMBERED` — I recall SNDlib from ZIB. Should verify hosting. |

### Vision Datasets

| Domain | What I claimed | Honesty check |
|--------|---------------|---------------|
| `image-classification` | ImageNet | ImageNet is `OPEN-REG` — requires agreement via image-net.org. Not fully open, not public domain. I should have been clearer. |
| `3d-scene-understanding` | ScanNet | `OPEN-REG` — ScanNet requires signing a terms of use agreement. Not freely downloadable. |
| `face-detection-recognition` | WIDER FACE, LFW | WIDER FACE: `OPEN-NC` (CC-BY-NC). LFW: `REMEMBERED` — I recall it's public but the exact terms for commercial use are unclear. Faces raise additional ethical/legal concerns. |
| `pose-estimation` | COCO-Pose, MPII | COCO: `OPEN` (CC-BY). MPII: `REMEMBERED` — I recall "for research purposes" but exact license unclear. |
| `stereo-depth` | KITTI, Middlebury | KITTI: `OPEN-NC` (CC-BY-NC-SA). Middlebury: `REMEMBERED` — I recall "for research" but exact terms unclear. |
| `floor-plan-analysis` | CubiCasa5K, R2V | `REMEMBERED` — CubiCasa5K: I recall CC-BY-NC from a 2019 paper but should verify hosting. R2V: less certain. |
| `scene-graph-generation` | Visual Genome | Likely `OPEN` (CC-BY). I'm fairly confident but should verify current terms. |
| `visual-entailment` | SNLI-VE | `REMEMBERED` — I recall this derives from SNLI (CC-BY-SA) and Flickr30k (CC-BY). Should be open but verify the combined dataset hosting. |
| `document-layout-analysis` | PubLayNet, DocBank | PubLayNet: `REMEMBERED` — IBM, I recall CC-BY. DocBank: `REMEMBERED` — I recall Apache from a Microsoft paper. Verify both. |
| `handwriting-math-recognition` | CROHME | `REMEMBERED` — Competition dataset. I recall it's CC-BY from the TC-11 community but should verify access. |
| `nutrition-label-parsing` | Open Food Facts | Likely `OPEN` — Open Food Facts is ODbL. Fairly confident. |
| `receipt-parsing` | SROIE, CORD | `REMEMBERED` — SROIE from ICDAR 2019. CORD from Naver/Clova. Should verify both are still hosted. |
| `sign-language-recognition` | WLASL, How2Sign | `REMEMBERED` — WLASL: I recall MIT from a 2020 paper. How2Sign: I recall Apache from a CMU/Meta paper. Verify both. |
| `intuitive-physics` | PHYRE, Physion | PHYRE: `REMEMBERED` — Meta, I recall Apache but should verify. Physion: `REMEMBERED` — MIT group, should verify. |
| `color-blindness-simulation` | Brettel matrices | The Brettel/Viénot transformation matrices are published in academic papers (public). This is fine. |

### Audio Datasets

| Domain | What I claimed | Honesty check |
|--------|---------------|---------------|
| `beat-tempo-detection` | GTZAN, Ballroom | GTZAN: **Known issue** — GTZAN has copyright problems (Sturm 2013 exposed issues). Should NOT be used. Ballroom: `REMEMBERED` — should verify hosting. |
| `chord-recognition` | Billboard, Isophonics | `REMEMBERED` — Billboard annotations from MIREX. Isophonics from C4DM. Should verify both are still accessible. |
| `speaker-identification` | VoxCeleb | `REMEMBERED` — VoxCeleb from University of Oxford. I recall CC-BY but there were GDPR concerns about face/voice data. VoxCeleb website may have changed access terms. |
| `audio-event-detection` | AudioSet, ESC-50 | AudioSet: **Important caveat** — only the labels are CC-BY. The actual audio is YouTube videos that may be taken down. Many are no longer available. ESC-50: `REMEMBERED` — from Karol Piczak, I recall CC-BY-NC. |
| `voice-activity-detection` | AVA-Speech | `REMEMBERED` — Google, I recall CC-BY. Should verify. |
| `music-source-separation` | MUSDB18, MedleyDB | MUSDB18: `OPEN-NC` (CC-BY-NC — confirmed, this one I'm fairly sure about). MedleyDB: `REMEMBERED` — from NYU, should verify license. |
| `music-key-detection` | GiantSteps | `REMEMBERED` — I recall CC-BY-SA from the GiantSteps project but should verify hosting. |
| `music-sight-reading` | IMSLP, MuseScore | IMSLP: `OPEN` (public domain music, though some editions have copyright). MuseScore: `REMEMBERED` — community scores have various CC licenses. |

### Miscellaneous

| Domain | What I claimed | Honesty check |
|--------|---------------|---------------|
| `geographic-coordinate-lookup` | GeoNames, OSM Nominatim | GeoNames: `OPEN` (CC-BY). Nominatim: `OPEN` (ODbL, with usage policy). Both have rate limits on free APIs. |
| `file-format-identification` | libmagic | `OPEN` — libmagic/file(1) is BSD. Confident. |
| `url-parsing` | WHATWG URL spec | `OPEN` — WHATWG spec is CC-BY. Confident. |

---

## Revised Summary

| Category | Previous count | Revised count | Change |
|----------|---------------|---------------|--------|
| `OPEN` (confirmed) | ~310 | ~250 | -60 |
| `PROCEDURAL` (confirmed) | ~220 | ~220 | 0 |
| `REMEMBERED` (needs verification) | ~8 | **~100** | **+92** |
| `OPEN-NC` (non-commercial) | ~15 | ~25 | +10 |
| `OPEN-REG` (needs registration) | ~15 | ~25 | +10 |
| `PROPRIETARY` (paid/restricted) | ~5 | ~15 | +10 |
| Other (GPL, SELF-PLAY, MIXED) | ~35 | ~35 | 0 |

## The ~100 REMEMBERED items, honestly categorized

### Probably fine (I'm >80% confident, just can't give exact URL from memory)
~50 items: Most agent environments (NLE, Crafter, Overcooked, BabyAI, Procgen), established ML benchmarks (RobustBench, AIF360, NAS-Bench), well-known science databases (AFLOW, ChEMBL), standard NLP datasets on HuggingFace

### Should verify (I'm 50-80% confident)
~30 items: Newer 2024 benchmarks (LiveCodeBench, ClassEval, MLE-bench, GAIA, GPQA), some agent environments (BEHAVIOR, Hanabi LE, SMACv2), science databases (BRENDA, SABIO-RK, PATRIC), audio datasets (VoxCeleb GDPR status, AudioSet availability)

### Likely unavailable or problematic (I'm <50% confident about open access)
~20 items:
- **Professional exams** (Bar/CPA/FE/GRE) — likely NO open datasets
- **LDC datasets** (CoNLL-2003, OntoNotes, TACRED, PropBank, Penn Treebank, ATIS, Chinese Treebank, RST-DT) — all require paid LDC license
- **FrontierMath** — was invitation-only
- **Humanity's Last Exam** — unclear if released
- **AirSim** — deprecated/archived by Microsoft
- **GTZAN** — known copyright issues
- **AudioSet** — YouTube source videos disappearing
- **Crossword clue databases** — copyright concerns
- **RS Means** — proprietary for full data
- **Newsela** — proprietary

## Action Items

1. **Replace LDC-dependent NLP domains** with free alternatives (WikiANN, UD, GUM, DocRED, FewRel) in all domain pages
2. **Flag AirSim as deprecated** — use Flightmare, or Isaac Sim for drone simulation
3. **Remove GTZAN references** — use Ballroom, DALI, or GiantSteps datasets
4. **Clarify professional exam domains** — these should be marked as `PROCEDURAL` (generate from textbook formulas) since no open exam dataset exists
5. **Note AudioSet caveat** — labels are CC-BY but audio availability degrades over time
6. **Verify all 2024-2025 benchmarks** by checking actual GitHub/HuggingFace URLs before use
