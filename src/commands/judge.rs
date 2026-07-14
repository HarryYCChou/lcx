use anyhow::{anyhow, bail, Result};
use std::path::Path;

use crate::cache::Cache;
use crate::client::models::ProblemDetail;
use crate::client::LeetCodeClient;
use crate::config::Config;
use crate::render;
use crate::solution;

struct Target {
    detail: ProblemDetail,
    lang_slug: String,
    code: String,
}

/// Resolve a test/submit target (file path or id/slug) into the problem detail,
/// language, and source code.
async fn resolve_target(
    cfg: &Config,
    cache: &Cache,
    client: &LeetCodeClient,
    target: &str,
    lang_override: Option<String>,
) -> Result<Target> {
    let path = Path::new(target);
    if path.is_file() {
        let (contents, meta) = solution::read_solution(path)?;
        let meta = meta.ok_or_else(|| {
            anyhow!("could not determine the problem for '{target}'. Use `lcx pick` to generate a file with metadata, or pass an id/slug.")
        })?;
        let lang_slug = lang_override.unwrap_or(meta.lang_slug);
        let detail = client.problem_detail(&meta.slug).await?;
        return Ok(Target {
            detail,
            lang_slug,
            code: contents,
        });
    }

    // Treat as id or slug.
    let slug = super::resolve_slug(cache, target)?;
    let detail = client.problem_detail(&slug).await?;
    let preferred = lang_override.clone().unwrap_or(cfg.lang.clone());
    let file = solution::find_existing(cfg, &detail.frontend_id, &detail.slug, &preferred)
        .ok_or_else(|| {
            anyhow!(
                "no solution file found for {} ({}). Run `lcx pick {}` first.",
                detail.frontend_id,
                detail.slug,
                target
            )
        })?;
    let (contents, meta) = solution::read_solution(&file)?;
    let lang_slug = lang_override
        .or_else(|| meta.map(|m| m.lang_slug))
        .unwrap_or(preferred);
    Ok(Target {
        detail,
        lang_slug,
        code: contents,
    })
}

/// Run a solution against sample or custom test cases.
pub async fn test(target: &str, case: Option<String>, lang: Option<String>) -> Result<()> {
    let cfg = Config::load()?;
    super::require_auth(&cfg)?;
    let cache = Cache::open()?;
    let client = LeetCodeClient::from_config(&cfg)?;

    let t = resolve_target(&cfg, &cache, &client, target, lang).await?;

    let input = case.unwrap_or_else(|| {
        if !t.detail.example_testcases.is_empty() {
            t.detail.example_testcases.clone()
        } else {
            t.detail.sample_test_case.clone()
        }
    });

    if input.trim().is_empty() {
        bail!("no test input available; provide one with --case");
    }

    println!("Testing {} ({})...", t.detail.frontend_id, t.lang_slug);
    let result = client
        .test(
            &t.detail.slug,
            t.detail.question_id,
            &t.lang_slug,
            &t.code,
            &input,
        )
        .await?;
    print!("{}", render::format_test_result(&result));
    Ok(())
}

/// Submit a solution and print the verdict.
pub async fn submit(target: &str, lang: Option<String>) -> Result<()> {
    let cfg = Config::load()?;
    super::require_auth(&cfg)?;
    let cache = Cache::open()?;
    let client = LeetCodeClient::from_config(&cfg)?;

    let t = resolve_target(&cfg, &cache, &client, target, lang).await?;

    println!("Submitting {} ({})...", t.detail.frontend_id, t.lang_slug);
    let result = client
        .submit(&t.detail.slug, t.detail.question_id, &t.lang_slug, &t.code)
        .await?;
    print!("{}", render::format_submit_result(&result));
    Ok(())
}
