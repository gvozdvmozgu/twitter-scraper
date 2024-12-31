#![warn(unreachable_pub, unused_qualifications)]
#![warn(clippy::use_self)]

mod config;
mod options;
mod pp;
mod scraper;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::load(concat!(env!("CARGO_PKG_NAME"), ".toml"))?;

    let scraper = match config.auth {
        config::AuthConfig::UsernamePassword { .. } => todo!(),
        config::AuthConfig::Cookie { cookie } => {
            scraper::from_cookies(&config.base, &config.bearer_token, &cookie)
        }
        config::AuthConfig::Token { .. } => todo!(),
    }?;

    match options::from_args() {
        options::Options::Tweets {
            query,
            search_mode,
            count,
            cursor,
        } => {
            let tweets = scraper.tweets(search_mode, &query, count, cursor).await?;
            pp::tweets(&tweets);
        }
        options::Options::Profiles {
            query,
            count,
            cursor,
        } => {
            let profiles = scraper.profiles(&query, count, cursor).await?;
            pp::profiles(&profiles);
        }
    }

    Ok(())
}
