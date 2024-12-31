use secrecy::ExposeSecret as _;

pub(crate) struct Scraper {
    client: reqwest::Client,
    bearer_token: secrecy::SecretString,
    csrf: secrecy::SecretString,
}

impl Scraper {
    fn from_cookies(base: &url::Url, bearer_token: &str, cookies: &str) -> anyhow::Result<Self> {
        let bearer_token = format!("Bearer {bearer_token}");

        let jar = reqwest::cookie::Jar::default();
        let mut csrf = None;

        for cookie in cookies.split(';') {
            if let Some(raw) = cookie.trim().strip_prefix("ct0=") {
                csrf = Some(raw.into());
            }

            jar.add_cookie_str(cookie, base);
        }

        let client = reqwest::ClientBuilder::new()
            .cookie_provider(jar.into())
            .build()?;

        let Some(csrf) = csrf else {
            anyhow::bail!("missing csrf token");
        };

        Ok(Self {
            client,
            bearer_token: bearer_token.into(),
            csrf,
        })
    }

    pub(crate) async fn tweets(
        &self,
        search_mode: SearchMode,
        query: &str,
        count: u32,
        cursor: Option<String>,
    ) -> anyhow::Result<crate::types::timeline::v1::QueryTweetsResponse> {
        self.tweet_timeline(search_mode, query, count, cursor)
            .await
            .map(|timeline| crate::types::timeline::search::parse_tweets(&timeline))
    }

    pub(crate) async fn profiles(
        &self,
        query: &str,
        count: u32,
        cursor: Option<String>,
    ) -> anyhow::Result<crate::types::timeline::v1::QueryProfilesResponse> {
        self.tweet_timeline(SearchMode::Users, query, count, cursor)
            .await
            .map(|timeline| crate::types::timeline::search::parse_users(&timeline))
    }

    async fn tweet_timeline(
        &self,
        search_mode: SearchMode,
        query: &str,
        mut count: u32,
        cursor: Option<String>,
    ) -> anyhow::Result<crate::types::timeline::search::SearchTimeline> {
        if count > 50 {
            count = 50
        }

        let mut variables = serde_json::json!({
            "rawQuery": query,
            "count": count,
            "querySource": "typed_query",
            "product": "Top"
        });

        if let Some(cursor) = cursor {
            variables["cursor"] = cursor.into();
        }

        match search_mode {
            SearchMode::Latest => {
                variables["product"] = "Latest".into();
            }
            SearchMode::Photos => {
                variables["product"] = "Photos".into();
            }
            SearchMode::Videos => {
                variables["product"] = "Videos".into();
            }
            SearchMode::Users => {
                variables["product"] = "People".into();
            }
            _ => {}
        }

        let features = serde_json::json!({
            "rweb_lists_timeline_redesign_enabled": true,
            "responsive_web_graphql_exclude_directive_enabled": true,
            "verified_phone_label_enabled": false,
            "creator_subscriptions_tweet_preview_api_enabled": true,
            "responsive_web_graphql_timeline_navigation_enabled": true,
            "responsive_web_graphql_skip_user_profile_image_extensions_enabled": false,
            "tweetypie_unmention_optimization_enabled": true,
            "responsive_web_edit_tweet_api_enabled": true,
            "graphql_is_translatable_rweb_tweet_is_translatable_enabled": true,
            "view_counts_everywhere_api_enabled": true,
            "longform_notetweets_consumption_enabled": true,
            "responsive_web_twitter_article_tweet_consumption_enabled": false,
            "tweet_awards_web_tipping_enabled": false,
            "freedom_of_speech_not_reach_fetch_enabled": true,
            "standardized_nudges_misinfo": true,
            "tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled": true,
            "longform_notetweets_rich_text_read_enabled": true,
            "longform_notetweets_inline_media_enabled": true,
            "responsive_web_media_download_video_enabled": false,
            "responsive_web_enhance_cards_enabled": false,
        });

        let field_toggles = serde_json::json!({
            "withArticleRichContentState": false
        });

        let params = &[
            ("variables", serde_json::to_string(&variables)?),
            ("features", serde_json::to_string(&features)?),
            ("fieldToggles", serde_json::to_string(&field_toggles)?),
        ];

        let request = self
            .client
            .get("https://api.twitter.com/graphql/nK1dw4oV3k4w5TdtcAdSww/SearchTimeline")
            .query(params)
            .header("Authorization", self.bearer_token.expose_secret())
            .header("x-csrf-token", self.csrf.expose_secret())
            .build()?;

        self.client
            .execute(request)
            .await?
            .json::<crate::types::timeline::search::SearchTimeline>()
            .await
            .map_err(Into::into)
    }
}

pub(crate) fn from_cookies(
    base: &url::Url,
    bearer_token: &str,
    cookies: &str,
) -> anyhow::Result<Scraper> {
    Scraper::from_cookies(base, bearer_token, cookies)
}

#[derive(Debug, Clone, Copy, strum::EnumString, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum SearchMode {
    Top,
    Latest,
    Photos,
    Videos,
    Users,
}
