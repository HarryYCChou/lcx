use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::time::Duration;

use super::models::JudgeResult;
use super::LeetCodeClient;

#[derive(Debug, Deserialize)]
struct InterpretResponse {
    interpret_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SubmitResponse {
    submission_id: Option<serde_json::Value>,
}

impl LeetCodeClient {
    /// Run the given code against `data_input` (custom or sample test cases)
    /// without recording a submission. Returns the resolved judge result.
    pub async fn test(
        &self,
        slug: &str,
        question_id: i64,
        lang_slug: &str,
        code: &str,
        data_input: &str,
    ) -> Result<JudgeResult> {
        let referer = format!("{}/problems/{slug}/", self.base_url());
        let url = format!("{}/problems/{slug}/interpret_solution/", self.base_url());
        let body = serde_json::json!({
            "lang": lang_slug,
            "question_id": question_id.to_string(),
            "typed_code": code,
            "data_input": data_input,
        });

        let resp = self.post_json(&url, &referer, &body).await?;
        let status = resp.status();
        let text = resp.text().await.context("reading interpret response")?;
        if !status.is_success() {
            anyhow::bail!("test request failed ({status}): {text}");
        }
        let parsed: InterpretResponse = serde_json::from_str(&text)
            .with_context(|| format!("parsing interpret response: {text}"))?;
        let id = parsed
            .interpret_id
            .ok_or_else(|| anyhow!("no interpret_id returned (are you logged in?)"))?;

        self.poll_result(&id, &referer).await
    }

    /// Submit the given code and poll for the final verdict.
    pub async fn submit(
        &self,
        slug: &str,
        question_id: i64,
        lang_slug: &str,
        code: &str,
    ) -> Result<JudgeResult> {
        let referer = format!("{}/problems/{slug}/", self.base_url());
        let url = format!("{}/problems/{slug}/submit/", self.base_url());
        let body = serde_json::json!({
            "lang": lang_slug,
            "question_id": question_id.to_string(),
            "typed_code": code,
        });

        let resp = self.post_json(&url, &referer, &body).await?;
        let status = resp.status();
        let text = resp.text().await.context("reading submit response")?;
        if !status.is_success() {
            anyhow::bail!("submit request failed ({status}): {text}");
        }
        let parsed: SubmitResponse = serde_json::from_str(&text)
            .with_context(|| format!("parsing submit response: {text}"))?;
        let id = parsed
            .submission_id
            .ok_or_else(|| anyhow!("no submission_id returned (are you logged in?)"))?;
        let id = match id {
            serde_json::Value::String(s) => s,
            serde_json::Value::Number(n) => n.to_string(),
            other => other.to_string(),
        };

        self.poll_result(&id, &referer).await
    }

    /// Poll the check endpoint until the judge reports `SUCCESS` or we time out.
    async fn poll_result(&self, id: &str, referer: &str) -> Result<JudgeResult> {
        let url = format!("{}/submissions/detail/{id}/check/", self.base_url());
        let max_attempts = 60;
        for attempt in 0..max_attempts {
            let result: JudgeResult = self.get_json(&url, referer).await?;
            if result.is_done() {
                return Ok(result);
            }
            // Gentle backoff: 300ms ramping up to ~1s.
            let wait = Duration::from_millis(300 + (attempt.min(7) as u64) * 100);
            tokio::time::sleep(wait).await;
        }
        Err(anyhow!("judge timed out waiting for a result"))
    }
}
