use anyhow::Result;

use crate::cache::Cache;
use crate::client::LeetCodeClient;
use crate::config::Config;
use crate::render;

/// Show a problem's description (and optionally its starter code).
pub async fn run(key: &str, lang: Option<String>, show_code: bool) -> Result<()> {
    let cfg = Config::load()?;
    let cache = Cache::open()?;
    let client = LeetCodeClient::from_config(&cfg)?;

    let detail = super::fetch_detail(&client, &cache, key).await?;
    render::print_detail(&detail);

    if show_code {
        let lang_slug = lang.unwrap_or(cfg.lang.clone());
        match detail.snippet_for(&lang_slug) {
            Some(snippet) => {
                println!("\n--- {} ---", snippet.lang);
                println!("{}", snippet.code);
            }
            None => println!("\nNo starter code for language '{lang_slug}'."),
        }
    }
    Ok(())
}
