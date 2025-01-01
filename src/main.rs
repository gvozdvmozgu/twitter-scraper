#![warn(unreachable_pub, unused_qualifications)]
#![warn(clippy::use_self)]

use anyhow::Context as _;

mod auth;
mod config;
mod options;
mod pp;
mod scraper;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::load(concat!(env!("CARGO_PKG_NAME"), ".toml"))?;
    let scraper = scraper::from_config(config).await?;

    match options::from_args() {
        options::Options::Tweets {
            query,
            search_mode,
            count,
            cursor,
        } => {
            let tweets = scraper
                .tweets(search_mode, &query, count, cursor)
                .await
                .context("failed to scrape tweets")?;
            pp::tweets(&tweets);
        }
        options::Options::Profiles {
            query,
            count,
            cursor,
        } => {
            let profiles = scraper
                .profiles(&query, count, cursor)
                .await
                .context("failed to scrape profiles")?;
            pp::profiles(&profiles);
        }
    }

    Ok(())
}
