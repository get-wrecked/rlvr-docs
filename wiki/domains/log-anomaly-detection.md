---
domain: Log Anomaly Detection
category: systems
verification_type: exact_match
dataset_scale: 5M-2B+ lines (BGL, HDFS, Loghub)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Log Anomaly Detection

## Overview

Log anomaly detection identifies unusual patterns in system logs that indicate failures, security breaches, or degraded performance. The model analyzes sequences of log messages and classifies them as normal or anomalous, optionally identifying the type of anomaly (hardware failure, software crash, security intrusion, performance degradation).

RLVR verification compares predicted labels against human-annotated anomaly labels or system-confirmed failure events. For session-level detection (is this log session anomalous?), this is a binary classification verified by F1 or AUC. For line-level detection, it is sequence labeling with per-line F1.

## Verification Mechanism

```python
import numpy as np
from sklearn.metrics import f1_score, precision_recall_fscore_support, roc_auc_score

def verify_anomaly_detection(predicted_labels: list, gold_labels: list,
                               predicted_scores: list = None) -> dict:
    """
    Session-level or line-level anomaly detection verification.
    predicted_labels: list of 0 (normal) or 1 (anomalous)
    gold_labels: ground truth labels
    """
    tp = sum(1 for p, g in zip(predicted_labels, gold_labels) if p == 1 and g == 1)
    fp = sum(1 for p, g in zip(predicted_labels, gold_labels) if p == 1 and g == 0)
    fn = sum(1 for p, g in zip(predicted_labels, gold_labels) if p == 0 and g == 1)

    precision = tp / (tp + fp) if (tp + fp) > 0 else 0.0
    recall = tp / (tp + fn) if (tp + fn) > 0 else 0.0
    f1 = 2 * precision * recall / (precision + recall) if (precision + recall) > 0 else 0.0

    result = {
        "precision": precision,
        "recall": recall,
        "f1": f1,
        "reward": f1
    }

    if predicted_scores is not None:
        try:
            auc = roc_auc_score(gold_labels, predicted_scores)
            result["auc_roc"] = auc
            result["reward"] = (f1 + auc) / 2  # combine F1 and AUC
        except ValueError:
            pass

    return result

def verify_root_cause(predicted_cause: str, gold_cause: str,
                       valid_causes: list = None) -> dict:
    """Verify root cause identification."""
    match = predicted_cause.strip().lower() == gold_cause.strip().lower()
    return {"correct": match, "reward": 1.0 if match else 0.0}
```

## Dataset Sources

- **HDFS (Hadoop Distributed File System)**: 11,175,629 log lines from a 203-node Hadoop cluster. 2.9% anomaly rate. Labeled at block_id level (session-level labels). One of the most used log datasets.
- **BGL (Blue Gene/L)**: 4,747,963 log messages from a Blue Gene/L supercomputer at LLNL. Each line labeled as alert or non-alert. Continuous time-series log data.
- **Loghub**: Collection of 16 log datasets totaling 2B+ lines, including:
  - Thunderbird: 211M messages from a supercomputer
  - Spirit: 272M messages
  - OpenStack: 207K log lines with labeled anomalies
  - Apache, Linux, Zookeeper, Hadoop, Spark, and more
- **LANL Unified Host and Network Dataset**: 1.6B network flow records, 1B authentication events from Los Alamos National Lab. Red team intrusion labels.
- **AIT Log Dataset v2**: 130M log lines from simulated enterprise network with labeled attack scenarios.
- **Drain-parsed datasets**: Many Loghub datasets have been parsed into log templates using Drain parser, providing structured event sequences.

## Task Format

- **Input**: A sequence of log messages (a session or time window).
```
Analyze the following log session for anomalies:

081109 203615 148 INFO dfs.DataNode$PacketResponder: PacketResponder 1 for block blk_38865049 terminating
081109 203615 35 INFO dfs.FSNamesystem: BLOCK* NameSystem.addStoredBlock: blockMap updated: 10.251.73.220 is added to blk_38865049 size 67108864
081109 203615 35 INFO dfs.FSNamesystem: BLOCK* NameSystem.delete: blk_38865049 is added to invalidSet of 10.251.43.115
081109 203618 35 WARN dfs.FSNamesystem: BLOCK* NameSystem.addStoredBlock: Redundant addStoredBlock request received for blk_38865049 on 10.251.203.80 size 67108864
081109 203619 35 ERROR dfs.ReplicationMonitor: Block blk_38865049 has replication factor 0. No replicas found.

Is this session normal or anomalous? If anomalous, identify the type.
```
- **Output**: Classification and explanation.
```
Anomalous.
Type: Block replication failure.
Evidence: Block blk_38865049 was deleted from one node (invalidSet) and
subsequently has replication factor 0 with no replicas found. This indicates
a data loss event.
```

## Difficulty Curriculum

- Level 1: Obvious anomalies with ERROR/FATAL level messages in otherwise clean logs
- Level 2: Known anomaly patterns (connection timeout sequences, out-of-memory patterns)
- Level 3: Anomalies identified by unusual log template sequences (no explicit error messages)
- Level 4: Subtle timing anomalies (operations taking 10x longer than usual)
- Level 5: Multi-component anomalies (failure propagation across services)
- Level 6: Anomalies in high-volume logs (finding the needle in 100K+ lines)
- Level 7: Previously unseen anomaly types (zero-shot detection)
- Level 8: Security intrusion detection (subtle indicators of compromise)
- Level 9: Correlated anomalies across multiple log sources (host + network + application), adversarial log injection (attacker modifies logs to hide intrusion)

## Limitations & Risks

- **Label quality**: Most log anomaly datasets use automated labeling (correlating with known failure events). Labels are incomplete -- some anomalies may not have been detected in the original system.
- **Anomaly definition ambiguity**: What counts as "anomalous" depends on the operational context. A retry that succeeds is normal behavior for a distributed system but might be flagged as anomalous by a naive detector.
- **Evolving systems**: Log formats and normal behavior patterns change with software updates. Models trained on historical data degrade as the system evolves.
- **Class imbalance**: Anomalies are rare (typically 1-5% of sessions). F1 is more informative than accuracy, but the low base rate makes high recall difficult.
- **Context window limitations**: Anomalies may require understanding hundreds or thousands of log lines in context. Current LLMs have limited context windows relative to real-world log volumes.
- **Log parsing sensitivity**: Raw log messages must be parsed into templates (removing variable parts like timestamps, IP addresses, file paths). Parsing errors propagate to detection errors.
- **Privacy concerns**: System logs may contain sensitive information (usernames, IP addresses, file paths). Data must be anonymized for RLVR training.

## Connections

- [[cicd-pipeline]] — Build and deployment logs are a common source of anomalies
- [[merge-conflict-resolution]] — Both involve understanding system-level software engineering artifacts
- [[reading-comprehension]] — Log analysis is a form of structured text comprehension
- [[medical-coding]] — Both involve classification from lengthy text documents
