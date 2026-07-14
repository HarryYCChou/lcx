use anyhow::Result;

use crate::client::LeetCodeClient;
use crate::config::Config;
use crate::render;

/// Show today's daily challenge, optionally generating a solution file.
pub async fn run(pick: bool) -> Result<()> {
    let cfg = Config::load()?;
    let client = LeetCodeClient::from_config(&cfg)?;

    let (date, daily) = client.daily().await?;
    println!(
        "Daily Challenge ({date}): {} {}  [{}]",
        daily.question.question_frontend_id,
        daily.question.title,
        render::difficulty_colored(&daily.question.difficulty)
    );
    println!("https://leetcode.com{}", daily.link);

    if pick {
        super::pick::run(&daily.question.title_slug, None, true).await?;
    }
    Ok(())
}
