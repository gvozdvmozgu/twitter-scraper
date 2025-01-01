#![warn(unreachable_pub, unused_qualifications)]
#![warn(clippy::use_self)]

use anyhow::Context as _;
use types::timeline::v1::{QueryProfilesResponse, QueryTweetsResponse};

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
            mut query,
            from,
            search_mode,
            count,
            cursor,
            output,
            all,
        } => {
            if let Some(from) = from {
                query = format!("from:{} {}", from, query);
            }

            let tweets = if all {
                let mut cursor = None;
                let mut tweets = QueryTweetsResponse::default();

                loop {
                    let new = scraper
                        .tweets(search_mode, &query, count, cursor)
                        .await
                        .context("failed to scrape tweets")?;

                    if new.tweets.is_empty() {
                        break tweets;
                    }

                    cursor = new.next;
                    tweets.merge(new.tweets);
                }
            } else {
                scraper
                    .tweets(search_mode, &query, count, cursor)
                    .await
                    .context("failed to scrape tweets")?
            };

            match output {
                options::Output::PrettyPrint => pp::tweets(&tweets),
                options::Output::Json => {
                    let json = serde_json::to_string_pretty(&tweets)?;
                    std::fs::write("tweets.json", json).context("writing `tweets.json`")?;
                }
            }
        }
        options::Options::Profiles {
            query,
            count,
            cursor,
            output,
            all,
        } => {
            let profiles = if all {
                let mut cursor = None;
                let mut profiles = QueryProfilesResponse::default();

                loop {
                    let new = scraper
                        .profiles(&query, count, cursor)
                        .await
                        .context("failed to scrape tweets")?;

                    if new.profiles.is_empty() {
                        break profiles;
                    }

                    cursor = new.next;
                    profiles.merge(new.profiles);
                }
            } else {
                scraper
                    .profiles(&query, count, cursor)
                    .await
                    .context("failed to scrape tweets")?
            };

            match output {
                options::Output::PrettyPrint => pp::profiles(&profiles),
                options::Output::Json => {
                    let json = serde_json::to_string_pretty(&profiles)?;
                    std::fs::write("profiles.json", json).context("writing `profiles.json`")?;
                }
            }
        }
    }

    Ok(())
}
