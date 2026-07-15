use serde::Deserialize;

/// A lightweight problem entry as returned by the problem list query and
/// stored in the local cache.
#[derive(Debug, Clone)]
pub struct ProblemSummary {
    /// Internal question id.
    pub question_id: i64,
    /// Human-facing id shown on the website (e.g. "1", "1class").
    pub frontend_id: String,
    pub title: String,
    pub slug: String,
    pub difficulty: String,
    pub paid_only: bool,
    /// Acceptance rate as a percentage (0-100).
    pub ac_rate: f64,
    /// User status: "ac", "notac", or empty if unattempted.
    pub status: Option<String>,
    /// Comma-joined topic tag slugs.
    pub tags: Vec<String>,
}

/// Full problem detail used for `show`, `pick`, `test`, and `submit`.
#[derive(Debug, Clone)]
pub struct ProblemDetail {
    pub question_id: i64,
    pub frontend_id: String,
    pub title: String,
    pub slug: String,
    pub difficulty: String,
    /// HTML problem statement.
    pub content: String,
    pub code_snippets: Vec<CodeSnippet>,
    /// Default sample test input.
    pub sample_test_case: String,
    /// All example test cases, newline separated.
    pub example_testcases: String,
}

impl ProblemDetail {
    /// Find the code snippet matching a language slug.
    pub fn snippet_for(&self, lang_slug: &str) -> Option<&CodeSnippet> {
        self.code_snippets
            .iter()
            .find(|s| s.lang_slug.eq_ignore_ascii_case(lang_slug))
    }
}

#[derive(Debug, Clone)]
pub struct CodeSnippet {
    pub lang: String,
    pub lang_slug: String,
    pub code: String,
}

/// Result of polling the judge endpoint for a test or submit run.
#[derive(Debug, Clone, Deserialize)]
pub struct JudgeResult {
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub status_msg: Option<String>,
    #[serde(default)]
    pub run_success: Option<bool>,
    #[serde(default)]
    pub total_correct: Option<i64>,
    #[serde(default)]
    pub total_testcases: Option<i64>,
    #[serde(default)]
    pub status_runtime: Option<String>,
    #[serde(default)]
    pub status_memory: Option<String>,
    #[serde(default)]
    pub runtime_percentile: Option<f64>,
    #[serde(default)]
    pub memory_percentile: Option<f64>,
    /// Answers produced by the user's code (interpret/test only).
    #[serde(default)]
    pub code_answer: Option<Vec<String>>,
    /// Expected answers (interpret/test only).
    #[serde(default)]
    pub expected_code_answer: Option<Vec<String>>,
    /// Captured stdout per test case (interpret/test only).
    #[serde(default)]
    pub std_output_list: Option<Vec<String>>,
    /// Captured stdout (submit; often truncated).
    #[serde(default)]
    pub std_output: Option<String>,
    #[serde(default)]
    pub compare_result: Option<String>,
    #[serde(default)]
    pub full_compile_error: Option<String>,
    #[serde(default)]
    pub full_runtime_error: Option<String>,
    #[serde(default)]
    pub compile_error: Option<String>,
    #[serde(default)]
    pub runtime_error: Option<String>,
    #[serde(default)]
    pub last_testcase: Option<String>,
    #[serde(default)]
    pub expected_output: Option<String>,
}

impl JudgeResult {
    pub fn is_done(&self) -> bool {
        self.state == "SUCCESS"
    }

    pub fn accepted(&self) -> bool {
        self.status_msg.as_deref() == Some("Accepted")
    }
}

/// Solved-vs-total counts for one difficulty band.
#[derive(Debug, Clone)]
pub struct DifficultyStat {
    pub solved: i64,
    pub total: i64,
}

impl DifficultyStat {
    /// Percentage of problems solved in this band (0.0 when none exist).
    pub fn percent(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.solved as f64 * 100.0 / self.total as f64
        }
    }
}

/// A user's overall solve statistics, broken down by difficulty.
#[derive(Debug, Clone)]
pub struct ProfileStats {
    pub username: String,
    pub easy: DifficultyStat,
    pub medium: DifficultyStat,
    pub hard: DifficultyStat,
    pub total: DifficultyStat,
}
