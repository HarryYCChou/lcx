use anyhow::Result;

use crate::cache::{Cache, ListFilter};
use crate::config::Config;
use crate::render;

/// List cached problems matching the given filters.
pub async fn run(
    difficulty: Option<String>,
    tag: Option<String>,
    status: Option<String>,
    query: Option<String>,
    limit: i64,
) -> Result<()> {
    let _cfg = Config::load()?;
    let cache = Cache::open()?;

    if cache.count()? == 0 {
        println!("Problem cache is empty. Run `lcx cache --update` to fetch the problem list.");
        return Ok(());
    }

    let filter = ListFilter {
        difficulty,
        tag,
        status,
        query,
        limit: Some(limit),
    };
    let problems = cache.query(&filter)?;

    if problems.is_empty() {
        println!("No problems matched your filters.");
        return Ok(());
    }

    println!("{}", render::problems_table(&problems));
    Ok(())
}
