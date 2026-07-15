use comfy_table::{Cell, Color, ContentArrangement, Table};
use owo_colors::OwoColorize;

use crate::client::models::{JudgeResult, ProblemDetail, ProblemSummary};

/// Return a colored difficulty label.
pub fn difficulty_colored(difficulty: &str) -> String {
    match difficulty {
        "Easy" => difficulty.green().to_string(),
        "Medium" => difficulty.yellow().to_string(),
        "Hard" => difficulty.red().to_string(),
        other => other.to_string(),
    }
}

fn status_symbol(status: Option<&str>) -> &'static str {
    match status {
        Some("ac") => "✔",
        Some("notac") => "✗",
        _ => " ",
    }
}

/// Render a table of problems for the `list` command.
pub fn problems_table(problems: &[ProblemSummary]) -> Table {
    let mut table = Table::new();
    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["", "#", "Title", "Difficulty", "AC%", "Paid"]);

    for p in problems {
        let diff_cell = match p.difficulty.as_str() {
            "Easy" => Cell::new(&p.difficulty).fg(Color::Green),
            "Medium" => Cell::new(&p.difficulty).fg(Color::Yellow),
            "Hard" => Cell::new(&p.difficulty).fg(Color::Red),
            _ => Cell::new(&p.difficulty),
        };
        table.add_row(vec![
            Cell::new(status_symbol(p.status.as_deref())),
            Cell::new(&p.frontend_id),
            Cell::new(&p.title),
            diff_cell,
            Cell::new(format!("{:.1}", p.ac_rate)),
            Cell::new(if p.paid_only { "🔒" } else { "" }),
        ]);
    }
    table
}

/// Convert an HTML problem statement into readable terminal text.
pub fn html_to_text(html: &str) -> String {
    html2text::from_read(html.as_bytes(), 100)
}

/// Print a full problem detail to stdout.
pub fn print_detail(detail: &ProblemDetail) {
    println!(
        "{} {}  [{}]",
        detail.frontend_id.bold(),
        detail.title.bold(),
        difficulty_colored(&detail.difficulty)
    );
    println!("https://leetcode.com/problems/{}/\n", detail.slug);
    println!("{}", html_to_text(&detail.content));
}

/// Format a test (interpret) result into a human-readable report.
pub fn format_test_result(result: &JudgeResult) -> String {
    if let Some(err) = compile_or_runtime_error(result) {
        return err;
    }

    let mut out = String::new();
    let status = result.status_msg.as_deref().unwrap_or("Unknown");
    let passed = result.run_success.unwrap_or(false)
        && result
            .compare_result
            .as_deref()
            .map(|c| c.chars().all(|ch| ch == '1') && !c.is_empty())
            .unwrap_or(false);

    let header = if passed {
        format!("{}  {}", "✔".green(), status.green().bold())
    } else {
        format!("{}  {}", "✗".red(), status.red().bold())
    };
    out.push_str(&header);
    out.push('\n');

    if let (Some(got), Some(expected)) = (&result.code_answer, &result.expected_code_answer) {
        out.push_str(&format!("\n{}   {:?}\n", "Output:".bold(), got));
        out.push_str(&format!("{} {:?}\n", "Expected:".bold(), expected));
    }
    if let Some(so) = stdout_text(result) {
        out.push_str(&format!("\n{}\n{}\n", "Stdout:".bold(), so));
    }
    if let Some(rt) = &result.status_runtime {
        out.push_str(&format!("Runtime: {rt}\n"));
    }
    out
}

/// Collect the stdout printed by the user's code, if any (prefers the per-case
/// list from `test`, falling back to the single `submit` string).
fn stdout_text(result: &JudgeResult) -> Option<String> {
    if let Some(list) = &result.std_output_list {
        let joined = list
            .iter()
            .filter(|s| !s.trim().is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join("\n");
        if !joined.trim().is_empty() {
            return Some(joined);
        }
    }
    if let Some(s) = &result.std_output {
        if !s.trim().is_empty() {
            return Some(s.clone());
        }
    }
    None
}

/// Format a submit result into a human-readable verdict.
pub fn format_submit_result(result: &JudgeResult) -> String {
    if let Some(err) = compile_or_runtime_error(result) {
        return err;
    }

    let mut out = String::new();
    let status = result.status_msg.as_deref().unwrap_or("Unknown");

    if result.accepted() {
        out.push_str(&format!("{}  {}\n", "✔".green(), status.green().bold()));
    } else {
        out.push_str(&format!("{}  {}\n", "✗".red(), status.red().bold()));
    }

    if let (Some(correct), Some(total)) = (result.total_correct, result.total_testcases) {
        out.push_str(&format!("Cases: {correct}/{total}\n"));
    }
    if let Some(rt) = &result.status_runtime {
        let pct = result
            .runtime_percentile
            .map(|p| format!(" (beats {p:.1}%)"))
            .unwrap_or_default();
        out.push_str(&format!("Runtime: {rt}{pct}\n"));
    }
    if let Some(mem) = &result.status_memory {
        let pct = result
            .memory_percentile
            .map(|p| format!(" (beats {p:.1}%)"))
            .unwrap_or_default();
        out.push_str(&format!("Memory: {mem}{pct}\n"));
    }
    if let Some(so) = stdout_text(result) {
        out.push_str(&format!("\n{}\n{}\n", "Stdout:".bold(), so));
    }
    if !result.accepted() {
        if let Some(last) = &result.last_testcase {
            if !last.is_empty() {
                out.push_str(&format!("\n{}\n{}\n", "Last testcase:".bold(), last));
            }
        }
        if let Some(exp) = &result.expected_output {
            if !exp.is_empty() {
                out.push_str(&format!("{} {}\n", "Expected:".bold(), exp));
            }
        }
    }
    out
}

fn compile_or_runtime_error(result: &JudgeResult) -> Option<String> {
    if let Some(err) = result
        .full_compile_error
        .as_ref()
        .or(result.compile_error.as_ref())
    {
        if !err.is_empty() {
            return Some(format!("{}\n{}", "Compile Error".red().bold(), err));
        }
    }
    if let Some(err) = result
        .full_runtime_error
        .as_ref()
        .or(result.runtime_error.as_ref())
    {
        if !err.is_empty() {
            return Some(format!("{}\n{}", "Runtime Error".red().bold(), err));
        }
    }
    None
}
