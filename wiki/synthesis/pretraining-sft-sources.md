---
title: Pre-Training & SFT Data Sources
description: All corpora needed for policy initialization before RLVR begins
---

# Pre-Training & SFT Data Sources

These datasets are **prerequisites** for RLVR — they produce the base policy that RL then improves. Without a competent base policy, RL has nothing to work with.

## Pre-Training Corpora (Policy Initialization)

These are the raw text/code/multimodal corpora for training the base VLM.

| Corpus | Size | Content | URL |
|--------|------|---------|-----|
| **FineWeb** | 15T tokens | Curated web text (HuggingFace) | `huggingface.co/datasets/HuggingFaceFW/fineweb` |
| **FineWeb-Edu** | 1.3T tokens | Education-filtered web text | `huggingface.co/datasets/HuggingFaceFW/fineweb-edu` |
| **RedPajama-v2** | 30T tokens | Multi-source (CC, GitHub, arXiv, books) | `together.ai/redpajama` |
| **SlimPajama** | 627B tokens | Deduplicated RedPajama subset | `huggingface.co/datasets/cerebras/SlimPajama-627B` |
| **Dolma** | 3T tokens | AI2's curated pre-training mix | `huggingface.co/datasets/allenai/dolma` |
| **The Pile** | 825GB | EleutherAI's diverse mix | `the-eye.eu/public/AI/pile` |
| **RefinedWeb** | 600B tokens | Falcon's web corpus | `huggingface.co/datasets/tiiuae/falcon-refinedweb` |
| **ROOTS** | 1.6TB | BigScience multilingual corpus | `huggingface.co/bigscience-data` |
| **ProofPile-2** | 55B tokens | Math-heavy pre-training | `huggingface.co/datasets/EleutherAI/proof-pile-2` |
| **peS2o** | 40B tokens | Semantic Scholar science papers | `huggingface.co/datasets/allenai/peS2o` |
| **The Stack v2** | 67TB | Code in 600+ languages | `huggingface.co/datasets/bigcode/the-stack-v2` |
| **StarCoderData** | 783GB | Curated code for StarCoder | `huggingface.co/datasets/bigcode/starcoderdata` |
| **OpenWebText2** | 65GB | Reddit-filtered web text | `openwebtext2.readthedocs.io` |

**Recommended pre-training mix** for RLVR base policy:
- 40% web text (FineWeb-Edu for reasoning-heavy)
- 25% code (The Stack v2 / StarCoderData)
- 15% science/math (ProofPile-2 + peS2o)
- 10% books/encyclopedias (from Dolma)
- 10% multilingual (from ROOTS)

## SFT / Instruction Tuning (Bridge to RL)

After pre-training, SFT on high-quality instructions teaches the model to follow instructions and produce structured output — making it a viable starting point for RL.

| Dataset | Size | Focus | URL |
|---------|------|-------|-----|
| **FLAN Collection** | 1,836 tasks | Diverse NLP tasks | `huggingface.co/datasets/Muennighoff/flan` |
| **OpenAssistant (OASST2)** | 161K messages | Multi-turn dialogue | `huggingface.co/datasets/OpenAssistant/oasst2` |
| **UltraChat** | 1.5M convos | Diverse conversations | `huggingface.co/datasets/stingning/ultrachat` |
| **OpenHermes 2.5** | 1M instructions | Curated instruction mix | `huggingface.co/datasets/teknium/OpenHermes-2.5` |
| **OpenOrca** | 4M examples | Orca-style reasoning traces | `huggingface.co/datasets/Open-Orca/OpenOrca` |
| **Tulu 2 Mix** | 326K | AI2's curated SFT mix | `huggingface.co/datasets/allenai/tulu-v2-sft-mixture` |
| **ShareGPT (Vicuna)** | 70K convos | Real user conversations | Various HF mirrors |
| **WizardLM-Evol** | 250K | Complexity-evolved instructions | `huggingface.co/datasets/WizardLM/...` |
| **NuminaMath-CoT** | 860K | Math with chain-of-thought | `huggingface.co/datasets/AI-MO/NuminaMath-CoT` |
| **MAmmoTH** | 260K | Math reasoning traces | `huggingface.co/datasets/TIGER-Lab/MAmmoTH` |
| **Magicoder-OSS** | 75K | Code instruction following | `huggingface.co/datasets/ise-uiuc/Magicoder-OSS-Instruct-75K` |
| **CodeFeedback** | 66K | Code with feedback | `huggingface.co/datasets/m-a-p/CodeFeedback-Filtered-Instruction` |
| **Deita 10K** | 10K | Quality-selected diverse | `huggingface.co/datasets/hkust-nlp/deita-10k-v0` |
| **Capybara** | 16K | Multi-turn | `huggingface.co/datasets/LDJnr/Capybara` |

**Recommended SFT mix** for RLVR transition:
- General instruction following: Tulu 2 Mix + OpenHermes + FLAN
- Math reasoning: NuminaMath-CoT + MAmmoTH
- Code: Magicoder-OSS + CodeFeedback
- Total: ~2-3M examples, 1-2 epochs

## Additional RL Verification Datasets (Missing from MANIFEST)

| Dataset | Size | Domain | URL |
|---------|------|--------|-----|
| Omni-MATH | 4K+ | math competition | `huggingface.co/datasets/KbsdJames/Omni-MATH` |
| TheoremQA | 800 | multi-domain math | `huggingface.co/datasets/wenhu/TheoremQA` |
| BigCodeBench | 1.1K | code tasks | `huggingface.co/datasets/bigcode/bigcodebench` |
| CRUXEval | 800 | code understanding | `github.com/facebookresearch/cruxeval` |
| ARC-AGI | 1K | abstract reasoning | `arcprize.org` |
| LeanDojo Benchmark4 | 98K theorems | formal proofs | `leandojo.org` |
| Visual Genome | 108K images | visual QA | `visualgenome.org` |
| Objaverse | 800K 3D | 3D objects | `objaverse.allenai.org` |
| OPUS-100 | 55M pairs | translation | `opus.nlpl.eu` |
| ConceptNet | 21M edges | knowledge graph | `conceptnet.io` |
| ATOMIC 2020 | 1.33M tuples | commonsense KG | `allenai.org/data/atomic-2020` |

## Environments & Simulators (for Agentic RL)

| Environment | Tasks | Domain | URL |
|-------------|-------|--------|-----|
| WebArena | 812 | web navigation | `webarena.dev` |
| VisualWebArena | 910 | visual web nav | `jykoh.com/vwa` |
| OSWorld | 369 | computer use | `os-world.github.io` |
| SWE-bench (Docker) | 2.3K | code repair | `swebench.com` |
| Gymnasium | 100+ | classic RL (Atari, MuJoCo) | `gymnasium.farama.org` |
| MineRL | Minecraft | open-world | `minerl.io` |
| NetHack LE | roguelike | strategy | `github.com/facebookresearch/nle` |
| TextWorld | proc-gen | text adventure | `github.com/microsoft/TextWorld` |
| ScienceWorld | 30 | science experiments | `sciworld.apps.allenai.org` |
| PettingZoo | 50+ | multi-agent | `pettingzoo.farama.org` |
| Habitat 3.0 | sim | embodied AI | `aihabitat.org` |
| E2B | sandbox | code execution | `e2b.dev` |

## The Full Pipeline

```
Pre-Training Corpora (FineWeb, Stack, ProofPile)
    ↓ 1-10T tokens, develops base capabilities
SFT Datasets (Tulu, NuminaMath, Magicoder)  
    ↓ 2-3M examples, teaches instruction following
RLVR (this project: 272 domains, 13 verifiers)
    ↓ Unlimited, develops reasoning + capability
AGI-Level Agent
```

The pre-training and SFT stages are **upstream** of this project. We either:
1. Use an existing pre-trained + SFT'd model (Qwen2-VL, LLaVA, etc.)
2. Train our own from these corpora

Option 1 is faster. Option 2 gives more control over the pre-training mix (e.g., heavier on math/code/science to match RLVR domains).
