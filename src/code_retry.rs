use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationFailureKind {
    RustCompileError,
    RustTestFailure,
    RustFormatError,
    TypescriptTypeError,
    NextjsBuildError,
    EslintError,
    ImportError,
    MissingDependency,
    ConfigError,
    RouteError,
    ApiContractError,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationFailureSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationFailureSource {
    pub raw_excerpt: String,
    pub normalized_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationFailureLocation {
    pub file_path: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationFailureEvidence {
    pub command: String,
    pub exit_code: Option<i32>,
    pub source: ValidationFailureSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationFailure {
    pub id: String,
    pub kind: ValidationFailureKind,
    pub severity: ValidationFailureSeverity,
    pub location: ValidationFailureLocation,
    pub evidence: ValidationFailureEvidence,
    pub likely_cause: String,
    pub suggested_action: String,
    pub confidence: f32, // 0.0 to 1.0
    pub related_artifacts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationFailureCluster {
    pub primary_failure: ValidationFailure,
    pub secondary_failures: Vec<ValidationFailure>,
    pub likely_first_fix_target: String,
    pub confidence_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationFailureFixHypothesis {
    pub description: String,
    pub expected_outcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationFailureAnalysis {
    pub session_id: String,
    pub clusters: Vec<ValidationFailureCluster>,
    pub fix_hypothesis: ValidationFailureFixHypothesis,
}

// --------------------------------------------------------
// Retry Plan Models
// --------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BuilderRetryRiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BuilderRetryApprovalNeed {
    None,
    Standard,
    HighRisk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderRetryStep {
    pub order: usize,
    pub action: String,
    pub target_file: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderRetryPatchCandidate {
    pub affected_files: Vec<String>,
    pub steps: Vec<BuilderRetryStep>,
    pub diff_preview_summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderRetryValidationPlan {
    pub expected_commands: Vec<crate::code_execution::ValidationCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderRetryPlan {
    pub id: String,
    pub validation_session_id: String,
    pub failed_command: String,
    pub failure_summary: String,
    pub suspected_root_cause: String,
    pub proposed_patch_intent: String,
    pub patch_candidate: BuilderRetryPatchCandidate,
    pub risk_level: BuilderRetryRiskLevel,
    pub approval_need: BuilderRetryApprovalNeed,
    pub validation_plan: BuilderRetryValidationPlan,
    pub checkpoint_ref: Option<String>,
    pub rollback_ref: Option<String>,
    pub max_retry_count: usize,
    pub current_retry_attempt: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BuilderRetryLoopState {
    Idle,
    Analyzing,
    PlanGenerated,
    AwaitingApproval,
    Patching,
    Validating,
    Success,
    MaxRetriesExceeded,
    Aborted,
}
