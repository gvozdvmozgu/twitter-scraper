use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Profile {
    pub(crate) id: String,
    pub(crate) username: String,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) location: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) protected: bool,
    pub(crate) verified: bool,
    pub(crate) followers_count: i32,
    pub(crate) following_count: i32,
    pub(crate) tweets_count: i32,
    pub(crate) listed_count: i32,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) profile_image_url: Option<String>,
    pub(crate) profile_banner_url: Option<String>,
    pub(crate) pinned_tweet_id: Option<String>,
    pub(crate) is_blue_verified: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserProfile {
    pub(crate) id: String,
    pub(crate) id_str: String,
    pub(crate) name: String,
    pub(crate) screen_name: String,
    pub(crate) location: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) protected: bool,
    pub(crate) followers_count: i32,
    pub(crate) friends_count: i32,
    pub(crate) listed_count: i32,
    pub(crate) created_at: String,
    pub(crate) favourites_count: i32,
    pub(crate) verified: bool,
    pub(crate) statuses_count: i32,
    pub(crate) profile_image_url_https: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LegacyUserRaw {
    pub(crate) created_at: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) entities: Option<UserEntitiesRaw>,
    #[serde(default)]
    pub(crate) favourites_count: i32,
    #[serde(default)]
    pub(crate) followers_count: i32,
    #[serde(default)]
    pub(crate) friends_count: i32,
    #[serde(default)]
    pub(crate) media_count: i32,
    #[serde(default)]
    pub(crate) statuses_count: i32,
    pub(crate) id_str: Option<String>,
    #[serde(default)]
    pub(crate) listed_count: i32,
    pub(crate) name: Option<String>,
    pub(crate) location: String,
    pub(crate) geo_enabled: Option<bool>,
    pub(crate) pinned_tweet_ids_str: Option<Vec<String>>,
    pub(crate) profile_background_color: Option<String>,
    pub(crate) profile_banner_url: Option<String>,
    pub(crate) profile_image_url_https: Option<String>,
    #[serde(default)]
    pub(crate) protected: bool,
    pub(crate) screen_name: Option<String>,
    #[serde(default)]
    pub(crate) verified: bool,
    pub(crate) has_custom_timelines: Option<bool>,
    pub(crate) has_extended_profile: Option<bool>,
    pub(crate) url: Option<String>,
    pub(crate) can_dm: Option<bool>,
    #[serde(rename = "userId")]
    pub(crate) user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserEntitiesRaw {
    pub(crate) url: Option<UserUrlEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserUrlEntity {
    pub(crate) urls: Option<Vec<ExpandedUrl>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExpandedUrl {
    pub(crate) expanded_url: Option<String>,
}

pub(crate) fn parse_profile(user: &LegacyUserRaw, is_blue_verified: Option<bool>) -> Profile {
    let mut profile = Profile {
        id: user.user_id.clone().unwrap_or_default(),
        username: user.screen_name.clone().unwrap_or_default(),
        name: user.name.clone().unwrap_or_default(),
        description: user.description.clone(),
        location: Some(user.location.clone()),
        url: user.url.clone(),
        protected: user.protected,
        verified: user.verified,
        followers_count: user.followers_count,
        following_count: user.friends_count,
        tweets_count: user.statuses_count,
        listed_count: user.listed_count,
        is_blue_verified: Some(is_blue_verified.unwrap_or_default()),
        created_at: user
            .created_at
            .as_ref()
            .and_then(|date_str| {
                DateTime::parse_from_str(date_str, "%a %b %d %H:%M:%S %z %Y")
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            })
            .unwrap_or_else(Utc::now),
        profile_image_url: user
            .profile_image_url_https
            .as_ref()
            .map(|url| url.replace("_normal", "")),
        profile_banner_url: user.profile_banner_url.clone(),
        pinned_tweet_id: user
            .pinned_tweet_ids_str
            .as_ref()
            .and_then(|ids| ids.first().cloned()),
    };

    let url = user
        .entities
        .as_ref()
        .and_then(|entities| entities.url.as_ref())
        .and_then(|url_entity| url_entity.urls.as_ref())
        .and_then(|urls| urls.first())
        .and_then(|first_url| first_url.expanded_url.as_ref());

    if let Some(expanded_url) = url {
        profile.url = Some(expanded_url.clone());
    }

    profile
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserResults {
    pub(crate) result: UserResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "__typename")]
pub(crate) enum UserResult {
    User(UserData),
    UserUnavailable(UserUnavailable),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserData {
    pub(crate) id: String,
    pub(crate) rest_id: String,
    pub(crate) affiliates_highlighted_label: Option<serde_json::Value>,
    pub(crate) has_graduated_access: bool,
    pub(crate) is_blue_verified: bool,
    pub(crate) profile_image_shape: String,
    pub(crate) legacy: LegacyUserRaw,
    pub(crate) smart_blocked_by: bool,
    pub(crate) smart_blocking: bool,
    pub(crate) legacy_extended_profile: Option<serde_json::Value>,
    pub(crate) is_profile_translatable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserUnavailable {
    pub(crate) reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserRaw {
    pub(crate) data: UserRawData,
    pub(crate) errors: Option<Vec<TwitterApiErrorRaw>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserRawData {
    pub(crate) user: UserRawUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserRawUser {
    pub(crate) result: UserRawResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserRawResult {
    pub(crate) rest_id: Option<String>,
    pub(crate) is_blue_verified: Option<bool>,
    pub(crate) legacy: LegacyUserRaw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TwitterApiErrorRaw {
    pub(crate) message: String,
    pub(crate) code: i32,
}
