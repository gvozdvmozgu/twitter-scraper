use std::{sync::Arc, time::Duration};

use anyhow::Context;
use reqwest::cookie::{CookieStore, Jar};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

use crate::config::{AuthConfig, Config};

pub(crate) struct Auth<'config> {
    jar: Arc<Jar>,
    bearer_token: &'config SecretString,
    base: &'config url::Url,
    client: reqwest::Client,
    guest_token: Option<SecretString>,
    csrf: Option<SecretString>,
}

impl<'config> Auth<'config> {
    fn new(config: &'config Config) -> Self {
        let jar = Arc::new(Jar::default());

        Self {
            jar: jar.clone(),
            base: &config.base,
            bearer_token: &config.bearer_token,
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .cookie_provider(jar)
                .build()
                .unwrap(),
            guest_token: None,
            csrf: None,
        }
    }

    fn build_flow_request<T: Serialize>(
        &self,
        endpoint: &str,
        payload: &T,
    ) -> reqwest::RequestBuilder {
        let mut req = self
            .client
            .post(endpoint)
            .header(
                "x-guest-token",
                self.guest_token.as_ref().unwrap().expose_secret(),
            )
            .bearer_auth(self.bearer_token.expose_secret())
            .json(payload);

        if let Some(csrf) = &self.csrf {
            req = req.header("x-csrf-token", csrf.expose_secret());
        }

        req
    }

    async fn init_login(&mut self) -> anyhow::Result<FlowResponse> {
        let init_request = FlowInitRequest {
            flow_name: "login".to_string(),
            input_flow_data: serde_json::json!({
                "flow_context": {
                    "debug_overrides": {},
                    "start_location": { "location": "splash_screen" }
                }
            }),
        };

        let response = self
            .build_flow_request(
                "https://api.twitter.com/1.1/onboarding/task.json",
                &init_request,
            )
            .send()
            .await?;

        Ok(response.json().await?)
    }

    async fn execute_flow_task(
        &mut self,
        request: FlowTaskRequest,
    ) -> anyhow::Result<FlowResponse> {
        let response = self
            .build_flow_request("https://api.twitter.com/1.1/onboarding/task.json", &request)
            .send()
            .await?;

        self.jar.set_cookies(
            &mut response.headers().get_all("set-cookie").iter(),
            self.base,
        );

        if let Some(cookie) = self.jar.cookies(self.base) {
            if let Some(string) = cookie
                .to_str()
                .ok()
                .and_then(|s| s.split(';').find_map(|s| s.trim().strip_prefix("ct0=")))
            {
                self.csrf = Some(string.into());
            }
        }

        Ok(response.json().await?)
    }

    async fn fetch_guest_token(&mut self) -> anyhow::Result<()> {
        let response = self
            .client
            .post("https://api.twitter.com/1.1/guest/activate.json")
            .bearer_auth(self.bearer_token.expose_secret())
            .send()
            .await?;

        let response = response.json::<serde_json::Value>().await?;

        let guest_token = response
            .get("guest_token")
            .and_then(|token| token.as_str())
            .ok_or_else(|| anyhow::format_err!("Failed to get guest token"))?;

        self.guest_token = Some(guest_token.to_owned().into());

        Ok(())
    }

    async fn login(
        mut self,
        config: &AuthConfig,
    ) -> anyhow::Result<(reqwest::Client, SecretString)> {
        match config {
            AuthConfig::User {
                username,
                password,
                email,
            } => {
                self.fetch_guest_token().await?;

                let mut flow_response = self.init_login().await?;

                while let Some(subtask) = flow_response.subtasks.first() {
                    let subtask_id = subtask.subtask_id.as_str();
                    let flow_token = flow_response.flow_token;

                    flow_response = match subtask_id {
                        "LoginJsInstrumentationSubtask" => {
                            self.execute_flow_task(FlowTaskRequest {
                                flow_token,
                                subtask_inputs: vec![serde_json::json!({
                                    "subtask_id": subtask_id,
                                    "js_instrumentation": {
                                        "response": "{}",
                                        "link": "next_link"
                                    }
                                })],
                            })
                            .await
                        }
                        "LoginEnterUserIdentifierSSO" => {
                            self.execute_flow_task(FlowTaskRequest {
                                flow_token,
                                subtask_inputs: vec![serde_json::json!({
                                    "subtask_id": subtask_id,
                                    "settings_list": {
                                        "setting_responses": [
                                            {
                                                "key": "user_identifier",
                                                "response_data": {
                                                    "text_data": {
                                                        "result": username.clone()
                                                    }
                                                }
                                            }
                                        ],
                                        "link": "next_link"
                                    }
                                })],
                            })
                            .await
                        }
                        "LoginEnterAlternateIdentifierSubtask" => self
                            .execute_flow_task(FlowTaskRequest {
                                flow_token,
                                subtask_inputs: vec![serde_json::json!({
                                    "subtask_id": subtask_id,
                                    "enter_text": {
                                        "text": email,
                                        "link": "next_link"
                                    }
                                })],
                            })
                            .await
                            .context(anyhow::format_err!("Email is required")),
                        "LoginEnterPassword" => {
                            self.execute_flow_task(FlowTaskRequest {
                                flow_token,
                                subtask_inputs: vec![serde_json::json!({
                                    "subtask_id": subtask_id,
                                    "enter_password": {
                                        "password": password.expose_secret(),
                                        "link": "next_link"
                                    }
                                })],
                            })
                            .await
                        }
                        "LoginSuccessSubtask" => {
                            self.execute_flow_task(FlowTaskRequest {
                                flow_token,
                                subtask_inputs: vec![],
                            })
                            .await
                        }
                        name => anyhow::bail!("Unknown subtask: {}", name),
                    }?;
                }
            }
            AuthConfig::Cookie { cookie: cookies } => {
                for cookie in cookies.expose_secret().split(';') {
                    if let Some(raw) = cookie.trim().strip_prefix("ct0=") {
                        self.csrf = Some(raw.into());
                    }

                    self.jar.add_cookie_str(cookie, self.base);
                }

                if self.csrf.is_none() {
                    anyhow::bail!("missing csrf cookie");
                };

                if !cookies.expose_secret().contains("auth_token") {
                    anyhow::bail!("missing auth_token cookie");
                }
            }
        }

        Ok((self.client, self.csrf.unwrap()))
    }
}

pub(crate) async fn from_config(
    config: &Config,
) -> anyhow::Result<(reqwest::Client, SecretString)> {
    Auth::new(config).login(&config.auth).await
}

#[derive(Debug, Serialize)]
struct FlowInitRequest {
    flow_name: String,
    input_flow_data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct FlowResponse {
    flow_token: String,
    #[serde(default)]
    subtasks: Vec<Subtask>,
}

#[derive(Debug, Deserialize)]
struct Subtask {
    subtask_id: String,
}

#[derive(Debug, Serialize)]
struct FlowTaskRequest {
    flow_token: String,
    subtask_inputs: Vec<serde_json::Value>,
}
