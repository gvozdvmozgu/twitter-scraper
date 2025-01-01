use anyhow::Context;
use secrecy::SecretString;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub(crate) enum AuthConfig {
    User {
        username: String,
        password: SecretString,
        email: Option<String>,
    },
    Cookie {
        cookie: SecretString,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) base: reqwest::Url,
    pub(crate) bearer_token: SecretString,
    pub(crate) auth: AuthConfig,
}

pub(crate) fn load(path: &str) -> anyhow::Result<Config> {
    let text = std::fs::read_to_string(path).with_context(|| format!("reading `{path}`"))?;
    toml::from_str(&text).map_err(Into::into)
}
