//! Dataset Registry
//!
//! Maps every verifier domain to its dataset files and metadata.
//! Used by the training harness to sample tasks from the right datasets.

/// A dataset entry in the registry.
#[derive(Debug)]
pub struct DatasetEntry {
    pub name: &'static str,
    pub verifier: &'static str,
    pub path: &'static str,
    pub format: DataFormat,
    pub problems: EstimatedSize,
    pub downloaded: bool,
    pub stage: Stage,
}

#[derive(Debug)]
pub enum DataFormat {
    Jsonl,
    Json,
    Parquet,
    Csv,
    Archive(&'static str), // inner format description
    Procedural,
}

#[derive(Debug)]
pub enum EstimatedSize {
    Exact(usize),
    Approximate(usize),
    Unlimited,
}

#[derive(Debug, Clone, Copy)]
pub enum Stage {
    Pre,  // Stage 1: Rule recognition
    Mid,  // Stage 2: General systems
    Post, // Stage 3: Specific climbing
}

/// All registered datasets.
pub fn all_datasets() -> Vec<DatasetEntry> {
    vec![
        // ===== MATH =====
        DatasetEntry {
            name: "GSM8K train",
            verifier: "math_numerical",
            path: "raw/datasets/math/gsm8k_train.jsonl",
            format: DataFormat::Jsonl,
            problems: EstimatedSize::Exact(7473),
            downloaded: true,
            stage: Stage::Pre,
        },
        DatasetEntry {
            name: "GSM8K test",
            verifier: "math_numerical",
            path: "raw/datasets/math/gsm8k_test.jsonl",
            format: DataFormat::Jsonl,
            problems: EstimatedSize::Exact(1319),
            downloaded: true,
            stage: Stage::Pre,
        },
        DatasetEntry {
            name: "MMLU (57 subjects)",
            verifier: "exact_match",
            path: "raw/datasets/qa/mmlu.tar",
            format: DataFormat::Archive("csv per subject"),
            problems: EstimatedSize::Exact(15908),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "MATH (Hendrycks)",
            verifier: "math_equivalence",
            path: "raw/datasets/math/math_hendrycks/",
            format: DataFormat::Jsonl,
            problems: EstimatedSize::Exact(12500),
            downloaded: false,
            stage: Stage::Pre,
        },

        // ===== CODE =====
        DatasetEntry {
            name: "HumanEval",
            verifier: "code_execution",
            path: "raw/datasets/code/humaneval.jsonl",
            format: DataFormat::Jsonl,
            problems: EstimatedSize::Exact(164),
            downloaded: true,
            stage: Stage::Pre,
        },
        DatasetEntry {
            name: "MBPP",
            verifier: "code_execution",
            path: "raw/datasets/code/mbpp.jsonl",
            format: DataFormat::Jsonl,
            problems: EstimatedSize::Exact(974),
            downloaded: true,
            stage: Stage::Pre,
        },
        DatasetEntry {
            name: "WikiSQL",
            verifier: "sql_execution",
            path: "raw/datasets/code/wikisql.tar.bz2",
            format: DataFormat::Archive("jsonl"),
            problems: EstimatedSize::Exact(80654),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "APPS",
            verifier: "code_execution",
            path: "raw/datasets/code/apps/",
            format: DataFormat::Jsonl,
            problems: EstimatedSize::Approximate(10000),
            downloaded: false,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "SWE-bench",
            verifier: "code_execution",
            path: "raw/datasets/code/swebench/",
            format: DataFormat::Jsonl,
            problems: EstimatedSize::Exact(2294),
            downloaded: false,
            stage: Stage::Post,
        },

        // ===== QA =====
        DatasetEntry {
            name: "SQuAD 2.0 train",
            verifier: "exact_match",
            path: "raw/datasets/qa/squad_train.json",
            format: DataFormat::Json,
            problems: EstimatedSize::Approximate(130000),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "SQuAD 2.0 dev",
            verifier: "exact_match",
            path: "raw/datasets/qa/squad_dev.json",
            format: DataFormat::Json,
            problems: EstimatedSize::Approximate(11000),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "TriviaQA",
            verifier: "exact_match",
            path: "raw/datasets/qa/triviaqa.tar.gz",
            format: DataFormat::Archive("json"),
            problems: EstimatedSize::Exact(95000),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "HotpotQA val",
            verifier: "exact_match",
            path: "raw/datasets/qa/hotpotqa_val.parquet",
            format: DataFormat::Parquet,
            problems: EstimatedSize::Exact(7405),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "CommonsenseQA",
            verifier: "exact_match",
            path: "raw/datasets/qa/commonsenseqa_train.jsonl",
            format: DataFormat::Jsonl,
            problems: EstimatedSize::Exact(9741),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "COPA",
            verifier: "exact_match",
            path: "raw/datasets/qa/copa.tgz",
            format: DataFormat::Archive("xml"),
            problems: EstimatedSize::Exact(1000),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "WikiTableQuestions",
            verifier: "exact_match",
            path: "raw/datasets/qa/wikitablequestions.zip",
            format: DataFormat::Archive("tsv"),
            problems: EstimatedSize::Approximate(22000),
            downloaded: true,
            stage: Stage::Mid,
        },

        // ===== FACT VERIFICATION =====
        DatasetEntry {
            name: "FEVER",
            verifier: "exact_match",
            path: "raw/datasets/qa/fever_train.jsonl",
            format: DataFormat::Jsonl,
            problems: EstimatedSize::Exact(145449),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "TabFact",
            verifier: "exact_match",
            path: "raw/datasets/qa/tabfact_tables.zip",
            format: DataFormat::Archive("json+csv"),
            problems: EstimatedSize::Approximate(118000),
            downloaded: true,
            stage: Stage::Mid,
        },

        // ===== NLI & CLASSIFICATION =====
        DatasetEntry {
            name: "SNLI",
            verifier: "exact_match",
            path: "raw/datasets/classification/snli.zip",
            format: DataFormat::Archive("jsonl"),
            problems: EstimatedSize::Exact(570000),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "MultiNLI",
            verifier: "exact_match",
            path: "raw/datasets/classification/multinli.zip",
            format: DataFormat::Archive("jsonl"),
            problems: EstimatedSize::Exact(433000),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "ANLI",
            verifier: "exact_match",
            path: "raw/datasets/classification/anli.zip",
            format: DataFormat::Archive("jsonl"),
            problems: EstimatedSize::Exact(170000),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "SST-2",
            verifier: "exact_match",
            path: "raw/datasets/classification/sst2.zip",
            format: DataFormat::Archive("tsv"),
            problems: EstimatedSize::Approximate(70000),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "AG News",
            verifier: "exact_match",
            path: "raw/datasets/classification/agnews_test.csv",
            format: DataFormat::Csv,
            problems: EstimatedSize::Exact(7600),
            downloaded: true,
            stage: Stage::Mid,
        },

        // ===== COMMONSENSE & SCIENCE =====
        DatasetEntry {
            name: "HellaSwag",
            verifier: "exact_match",
            path: "raw/datasets/qa/hellaswag_val.jsonl",
            format: DataFormat::Jsonl,
            problems: EstimatedSize::Exact(10042),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "PIQA",
            verifier: "exact_match",
            path: "raw/datasets/qa/piqa_valid.jsonl",
            format: DataFormat::Jsonl,
            problems: EstimatedSize::Exact(1838),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "Winogrande",
            verifier: "exact_match",
            path: "raw/datasets/qa/winogrande.zip",
            format: DataFormat::Archive("jsonl"),
            problems: EstimatedSize::Approximate(44000),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "ARC",
            verifier: "exact_match",
            path: "raw/datasets/science/arc.zip",
            format: DataFormat::Archive("jsonl"),
            problems: EstimatedSize::Exact(7787),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "OpenBookQA",
            verifier: "exact_match",
            path: "raw/datasets/science/openbookqa.zip",
            format: DataFormat::Archive("jsonl"),
            problems: EstimatedSize::Exact(5957),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "SciQ",
            verifier: "exact_match",
            path: "raw/datasets/science/sciq.zip",
            format: DataFormat::Archive("json"),
            problems: EstimatedSize::Exact(13679),
            downloaded: true,
            stage: Stage::Mid,
        },

        // ===== MEDICAL =====
        DatasetEntry {
            name: "MedQA (USMLE)",
            verifier: "exact_match",
            path: "raw/datasets/medical/medqa_train.jsonl",
            format: DataFormat::Jsonl,
            problems: EstimatedSize::Approximate(10000),
            downloaded: true,
            stage: Stage::Post,
        },

        // ===== LOGIC & GAMES =====
        DatasetEntry {
            name: "Lichess Puzzles",
            verifier: "rule_based",
            path: "raw/datasets/logic/lichess_puzzles.csv.zst",
            format: DataFormat::Archive("csv"),
            problems: EstimatedSize::Approximate(3000000),
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "Sudoku",
            verifier: "sudoku",
            path: "raw/datasets/logic/sudoku_data.zip",
            format: DataFormat::Archive("csv"),
            problems: EstimatedSize::Approximate(1000000),
            downloaded: true,
            stage: Stage::Pre,
        },

        // ===== INSTRUCTION FOLLOWING =====
        DatasetEntry {
            name: "IFEval",
            verifier: "instruction_following",
            path: "raw/datasets/instruction/ifeval.jsonl",
            format: DataFormat::Jsonl,
            problems: EstimatedSize::Exact(541),
            downloaded: true,
            stage: Stage::Mid,
        },

        // ===== PROCEDURAL (UNLIMITED) =====
        DatasetEntry {
            name: "Unit Conversion (generated)",
            verifier: "unit_conversion",
            path: "(procedural)",
            format: DataFormat::Procedural,
            problems: EstimatedSize::Unlimited,
            downloaded: true, // always available
            stage: Stage::Pre,
        },
        DatasetEntry {
            name: "Date/Time (generated)",
            verifier: "date_time",
            path: "(procedural)",
            format: DataFormat::Procedural,
            problems: EstimatedSize::Unlimited,
            downloaded: true,
            stage: Stage::Pre,
        },
        DatasetEntry {
            name: "Chemical Equations (generated)",
            verifier: "chemical_equation",
            path: "(procedural)",
            format: DataFormat::Procedural,
            problems: EstimatedSize::Unlimited,
            downloaded: true,
            stage: Stage::Pre,
        },
        DatasetEntry {
            name: "Regex Tasks (generated)",
            verifier: "regex_synthesis",
            path: "(procedural)",
            format: DataFormat::Procedural,
            problems: EstimatedSize::Unlimited,
            downloaded: true,
            stage: Stage::Pre,
        },
        DatasetEntry {
            name: "JSON Schema Tasks (generated)",
            verifier: "json_schema",
            path: "(procedural)",
            format: DataFormat::Procedural,
            problems: EstimatedSize::Unlimited,
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "Instruction Tasks (generated)",
            verifier: "instruction_following",
            path: "(procedural)",
            format: DataFormat::Procedural,
            problems: EstimatedSize::Unlimited,
            downloaded: true,
            stage: Stage::Mid,
        },
        DatasetEntry {
            name: "Graph Tasks (generated)",
            verifier: "graph_properties",
            path: "(procedural)",
            format: DataFormat::Procedural,
            problems: EstimatedSize::Unlimited,
            downloaded: true,
            stage: Stage::Pre,
        },
    ]
}

/// Print a summary of the dataset registry.
pub fn print_summary() {
    let datasets = all_datasets();
    let downloaded: Vec<_> = datasets.iter().filter(|d| d.downloaded).collect();
    let pending: Vec<_> = datasets.iter().filter(|d| !d.downloaded).collect();

    println!("=== Dataset Registry ===");
    println!("Downloaded: {} datasets", downloaded.len());
    println!("Pending:    {} datasets", pending.len());
    println!();

    let total_problems: usize = downloaded.iter().map(|d| match d.problems {
        EstimatedSize::Exact(n) | EstimatedSize::Approximate(n) => n,
        EstimatedSize::Unlimited => 0,
    }).sum();

    let procedural: usize = downloaded.iter().filter(|d| matches!(d.problems, EstimatedSize::Unlimited)).count();

    println!("Total downloaded problems: ~{}", total_problems);
    println!("Procedural generators:     {} (unlimited)", procedural);
    println!();

    if !pending.is_empty() {
        println!("Pending downloads:");
        for d in &pending {
            println!("  - {} ({}) → {}", d.name, d.verifier, d.path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_has_entries() {
        let datasets = all_datasets();
        assert!(datasets.len() >= 30, "Should have at least 30 dataset entries");
    }

    #[test]
    fn all_verifiers_have_datasets() {
        let datasets = all_datasets();
        let verifiers: std::collections::HashSet<&str> = datasets.iter().map(|d| d.verifier).collect();

        // Every implemented verifier should have at least one dataset
        for expected in &[
            "math_numerical", "math_equivalence", "exact_match",
            "instruction_following", "code_execution", "sudoku",
            "regex_synthesis", "json_schema", "unit_conversion",
            "date_time", "chemical_equation", "sql_execution",
            "graph_properties",
        ] {
            assert!(
                verifiers.contains(expected),
                "Verifier '{}' has no dataset registered",
                expected
            );
        }
    }

    #[test]
    fn downloaded_files_exist() {
        let datasets = all_datasets();
        for d in &datasets {
            if d.downloaded && !matches!(d.format, DataFormat::Procedural) {
                let path = std::path::Path::new(d.path);
                assert!(
                    path.exists(),
                    "Dataset '{}' marked as downloaded but file missing: {}",
                    d.name, d.path
                );
            }
        }
    }

    #[test]
    fn summary_doesnt_panic() {
        print_summary();
    }
}
