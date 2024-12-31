use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub(crate) enum AuthConfig {
    UsernamePassword { username: String, password: String },
    Cookie { cookie: String },
    Token { token: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) base: reqwest::Url,
    pub(crate) bearer_token: String,
    pub(crate) auth: AuthConfig,
}

pub(crate) fn load(path: &str) -> anyhow::Result<Config> {
    let text = std::fs::read_to_string(path).with_context(|| format!("reading `{path}`"))?;
    toml::from_str(&text).map_err(Into::into)
}
