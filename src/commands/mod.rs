pub mod auth;
pub mod cache_cmd;
pub mod config_cmd;
pub mod daily;
pub mod judge;
pub mod list;
pub mod pick;
pub mod show;

use anyhow::{bail, Result};

use crate::cache::Cache;
use crate::client::models::ProblemDetail;
use crate::client::LeetCodeClient;
use crate::config::Config;

/// Shared helper: resolve an id/slug key to a title slug using the cache when
/// possible, otherwise treat the key as a slug directly.
pub fn resolve_slug(cache: &Cache, key: &str) -> Result<String> {
    if let Some(p) = cache.find(key)? {
        return Ok(p.slug);
    }
    // Not in cache: assume the key is already a slug.
    Ok(key.to_string())
}

/// Fetch a problem detail for a key (id or slug), using the cache to map ids.
pub async fn fetch_detail(
    client: &LeetCodeClient,
    cache: &Cache,
    key: &str,
) -> Result<ProblemDetail> {
    let slug = resolve_slug(cache, key)?;
    client.problem_detail(&slug).await
}

/// Ensure we have credentials, erroring with a helpful message otherwise.
pub fn require_auth(cfg: &Config) -> Result<()> {
    if !cfg.is_authenticated() {
        bail!("not logged in. Run `lcx login` first (see README for how to get cookies)");
    }
    Ok(())
}
