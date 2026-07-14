use anyhow::Result;

use crate::cache::Cache;
use crate::client::LeetCodeClient;
use crate::config::Config;

const PAGE_SIZE: i64 = 500;

/// Manage the local problem cache.
pub async fn run(update: bool, clear: bool) -> Result<()> {
    let mut cache = Cache::open()?;

    if clear {
        cache.clear()?;
        println!("Cache cleared.");
        return Ok(());
    }

    if update {
        let cfg = Config::load()?;
        let client = LeetCodeClient::from_config(&cfg)?;
        println!("Fetching problem list from LeetCode...");

        let mut all = Vec::new();
        let mut skip = 0;
        loop {
            let (total, batch) = client
                .list_problems(PAGE_SIZE, skip, serde_json::json!({}))
                .await?;
            let fetched = batch.len() as i64;
            all.extend(batch);
            skip += fetched;
            print!("\r  {} / {} problems", all.len(), total);
            use std::io::Write;
            std::io::stdout().flush().ok();
            if fetched == 0 || skip >= total {
                break;
            }
        }
        println!();

        cache.replace_all(&all)?;
        println!("Cached {} problems.", all.len());
        return Ok(());
    }

    // Default: report cache status.
    let n = cache.count()?;
    if n == 0 {
        println!("Cache is empty. Run `lcx cache --update` to populate it.");
    } else {
        println!("{n} problems cached.");
    }
    Ok(())
}
