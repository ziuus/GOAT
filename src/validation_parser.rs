use crate::code_retry::{
    ValidationFailure, ValidationFailureEvidence, ValidationFailureKind, ValidationFailureLocation,
    ValidationFailureSeverity, ValidationFailureSource,
};
use std::collections::HashMap;

/// Parses output from various build/test tools and extracts structured `ValidationFailure`s.
pub struct ValidationParser;

impl ValidationParser {
    pub fn parse_output(
        command: &str,
        stdout: &str,
        stderr: &str,
        exit_code: Option<i32>,
    ) -> Vec<ValidationFailure> {
        let combined = format!("{}\n{}", stdout, stderr);

        if command.contains("cargo") {
            Self::parse_cargo(&combined, command, exit_code)
        } else if command.contains("next build") || command.contains("npm run build") {
            Self::parse_nextjs(&combined, command, exit_code)
        } else if command.contains("tsc") {
            Self::parse_tsc(&combined, command, exit_code)
        } else if command.contains("eslint") {
            Self::parse_eslint(&combined, command, exit_code)
        } else {
            Self::parse_generic(&combined, command, exit_code)
        }
    }

    fn parse_cargo(output: &str, command: &str, exit_code: Option<i32>) -> Vec<ValidationFailure> {
        let mut failures = Vec::new();
        let lines: Vec<&str> = output.lines().collect();

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];
            if line.contains("error[")
                || (line.starts_with("error:") && !line.contains("could not compile"))
            {
                let kind = if command.contains("test") {
                    ValidationFailureKind::RustTestFailure
                } else if command.contains("fmt") {
                    ValidationFailureKind::RustFormatError
                } else {
                    ValidationFailureKind::RustCompileError
                };

                let message = line.to_string();
                let mut file_path = None;
                let mut line_num = None;
                let mut col_num = None;

                // Try to find the file reference in the next few lines `--> src/file.rs:line:col`
                let mut excerpt = String::new();
                for j in 0..5 {
                    if i + j < lines.len() {
                        let peek = lines[i + j];
                        excerpt.push_str(peek);
                        excerpt.push('\n');
                        if peek.trim().starts_with("-->") {
                            let parts: Vec<&str> =
                                peek.trim().trim_start_matches("--> ").split(':').collect();
                            if !parts.is_empty() {
                                file_path = Some(parts[0].to_string());
                            }
                            if parts.len() > 1 {
                                line_num = parts[1].parse().ok();
                            }
                            if parts.len() > 2 {
                                col_num = parts[2].parse().ok();
                            }
                        }
                    }
                }

                failures.push(ValidationFailure {
                    id: uuid::Uuid::new_v4().to_string(),
                    kind,
                    severity: ValidationFailureSeverity::High,
                    location: ValidationFailureLocation {
                        file_path,
                        line: line_num,
                        column: col_num,
                    },
                    evidence: ValidationFailureEvidence {
                        command: command.to_string(),
                        exit_code,
                        source: ValidationFailureSource {
                            raw_excerpt: excerpt,
                            normalized_message: message,
                        },
                    },
                    likely_cause: "Rust compiler or test error".to_string(),
                    suggested_action: "Review code and fix typing, borrowing, or logic".to_string(),
                    confidence: 0.9,
                    related_artifacts: vec![],
                });
            }
            i += 1;
        }

        if failures.is_empty() && exit_code.unwrap_or(0) != 0 {
            failures.extend(Self::parse_generic(output, command, exit_code));
        }

        failures
    }

    fn parse_nextjs(output: &str, command: &str, exit_code: Option<i32>) -> Vec<ValidationFailure> {
        let mut failures = Vec::new();
        let lines: Vec<&str> = output.lines().collect();

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];
            if line.contains("Failed to compile.")
                || line.contains("Type error:")
                || line.contains("Syntax error")
                || line.contains("Syntax Error")
            {
                // Next.js error block found
                let mut excerpt = String::new();
                let mut file_path = None;
                let mut line_num = None;

                // Backtrack or look ahead to find `./src/...`
                for j in (0..=2).rev() {
                    if i >= j {
                        let peek = lines[i - j];
                        if peek.starts_with("./") {
                            let parts: Vec<&str> = peek.split(':').collect();
                            file_path = Some(parts[0].to_string());
                            if parts.len() > 1 {
                                line_num = parts[1].parse().ok();
                            }
                        }
                    }
                }

                for j in 0..10 {
                    if i + j < lines.len() {
                        excerpt.push_str(lines[i + j]);
                        excerpt.push('\n');
                    }
                }

                failures.push(ValidationFailure {
                    id: uuid::Uuid::new_v4().to_string(),
                    kind: ValidationFailureKind::NextjsBuildError,
                    severity: ValidationFailureSeverity::High,
                    location: ValidationFailureLocation {
                        file_path,
                        line: line_num,
                        column: None,
                    },
                    evidence: ValidationFailureEvidence {
                        command: command.to_string(),
                        exit_code,
                        source: ValidationFailureSource {
                            raw_excerpt: excerpt,
                            normalized_message: line.to_string(),
                        },
                    },
                    likely_cause: "Next.js compilation or type error".to_string(),
                    suggested_action: "Fix the syntax or type definitions in the React component"
                        .to_string(),
                    confidence: 0.85,
                    related_artifacts: vec![],
                });
            }
            i += 1;
        }

        if failures.is_empty() && exit_code.unwrap_or(0) != 0 {
            failures.extend(Self::parse_generic(output, command, exit_code));
        }

        failures
    }

    fn parse_tsc(output: &str, command: &str, exit_code: Option<i32>) -> Vec<ValidationFailure> {
        let mut failures = Vec::new();
        // src/file.ts(10,5): error TS2322: ...
        for line in output.lines() {
            if line.contains("error TS") {
                let parts: Vec<&str> = line.split("): ").collect();
                let mut file_path = None;
                let mut line_num = None;

                if parts.len() == 2 {
                    let file_loc_parts: Vec<&str> = parts[0].split('(').collect();
                    if file_loc_parts.len() == 2 {
                        file_path = Some(file_loc_parts[0].to_string());
                        let nums: Vec<&str> = file_loc_parts[1].split(',').collect();
                        if !nums.is_empty() {
                            line_num = nums[0].parse().ok();
                        }
                    }
                }

                failures.push(ValidationFailure {
                    id: uuid::Uuid::new_v4().to_string(),
                    kind: ValidationFailureKind::TypescriptTypeError,
                    severity: ValidationFailureSeverity::High,
                    location: ValidationFailureLocation {
                        file_path,
                        line: line_num,
                        column: None,
                    },
                    evidence: ValidationFailureEvidence {
                        command: command.to_string(),
                        exit_code,
                        source: ValidationFailureSource {
                            raw_excerpt: line.to_string(),
                            normalized_message: line.to_string(),
                        },
                    },
                    likely_cause: "TypeScript type mismatch".to_string(),
                    suggested_action: "Update type definitions or fix assignment".to_string(),
                    confidence: 0.9,
                    related_artifacts: vec![],
                });
            }
        }
        failures
    }

    fn parse_eslint(output: &str, command: &str, exit_code: Option<i32>) -> Vec<ValidationFailure> {
        let mut failures = Vec::new();
        // File path line:
        // /path/to/file.ts
        //   10:5  error  'foo' is assigned a value but never used  no-unused-vars
        let mut current_file = None;
        for line in output.lines() {
            if line.starts_with('/') || line.starts_with("./") {
                current_file = Some(line.trim().to_string());
            } else if line.contains("error") && current_file.is_some() {
                failures.push(ValidationFailure {
                    id: uuid::Uuid::new_v4().to_string(),
                    kind: ValidationFailureKind::EslintError,
                    severity: ValidationFailureSeverity::Medium,
                    location: ValidationFailureLocation {
                        file_path: current_file.clone(),
                        line: None,
                        column: None,
                    },
                    evidence: ValidationFailureEvidence {
                        command: command.to_string(),
                        exit_code,
                        source: ValidationFailureSource {
                            raw_excerpt: line.to_string(),
                            normalized_message: line.to_string(),
                        },
                    },
                    likely_cause: "Linting rule violation".to_string(),
                    suggested_action: "Fix the lint rule or add an ignore comment".to_string(),
                    confidence: 0.9,
                    related_artifacts: vec![],
                });
            }
        }
        failures
    }

    fn parse_generic(
        output: &str,
        command: &str,
        exit_code: Option<i32>,
    ) -> Vec<ValidationFailure> {
        if exit_code.unwrap_or(0) == 0 {
            return vec![];
        }

        // Generic fallback - grab last few non-empty lines
        let lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
        let excerpt = if lines.len() > 10 {
            lines[lines.len() - 10..].join("\n")
        } else {
            lines.join("\n")
        };

        vec![ValidationFailure {
            id: uuid::Uuid::new_v4().to_string(),
            kind: ValidationFailureKind::Unknown,
            severity: ValidationFailureSeverity::High,
            location: ValidationFailureLocation {
                file_path: None,
                line: None,
                column: None,
            },
            evidence: ValidationFailureEvidence {
                command: command.to_string(),
                exit_code,
                source: ValidationFailureSource {
                    raw_excerpt: excerpt,
                    normalized_message: "Command exited with non-zero status".to_string(),
                },
            },
            likely_cause: "Execution failed with non-zero exit code".to_string(),
            suggested_action: "Analyze command output".to_string(),
            confidence: 0.5,
            related_artifacts: vec![],
        }]
    }

    pub fn cluster_failures(
        failures: Vec<ValidationFailure>,
    ) -> Vec<crate::code_retry::ValidationFailureCluster> {
        if failures.is_empty() {
            return vec![];
        }

        let mut file_clusters: HashMap<String, Vec<ValidationFailure>> = HashMap::new();
        let mut generic_cluster = Vec::new();

        for f in failures {
            if let Some(ref fp) = f.location.file_path {
                file_clusters.entry(fp.clone()).or_default().push(f);
            } else {
                generic_cluster.push(f);
            }
        }

        let mut clusters = Vec::new();
        for (file_path, mut group) in file_clusters {
            let primary = group.remove(0);
            clusters.push(crate::code_retry::ValidationFailureCluster {
                primary_failure: primary,
                secondary_failures: group,
                likely_first_fix_target: file_path,
                confidence_score: 0.8,
            });
        }

        if !generic_cluster.is_empty() {
            let primary = generic_cluster.remove(0);
            clusters.push(crate::code_retry::ValidationFailureCluster {
                primary_failure: primary,
                secondary_failures: generic_cluster,
                likely_first_fix_target: "Unknown (check command environment)".to_string(),
                confidence_score: 0.5,
            });
        }

        clusters
    }
}
