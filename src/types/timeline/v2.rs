use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::types::{
    profile::LegacyUserRaw,
    tweets::{Mention, Photo, Tweet, Video},
};

use super::v1::{LegacyTweetRaw, TimelineMediaExtendedRaw, TimelineResultRaw};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Timeline {
    pub(crate) timeline: Option<TimelineItems>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineContent {
    pub(crate) instructions: Option<Vec<TimelineInstruction>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineData {
    pub(crate) user: Option<TimelineUser>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineEntities {
    pub(crate) hashtags: Option<Vec<Hashtag>>,
    pub(crate) user_mentions: Option<Vec<UserMention>>,
    pub(crate) urls: Option<Vec<UrlEntity>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineEntry {
    #[serde(rename = "entryId")]
    pub(crate) entry_id: Option<String>,
    pub(crate) content: Option<EntryContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineEntryItemContent {
    pub(crate) item_type: Option<String>,
    pub(crate) tweet_display_type: Option<String>,
    pub(crate) tweet_result: Option<TweetResult>,
    pub(crate) tweet_results: Option<TweetResult>,
    pub(crate) user_display_type: Option<String>,
    pub(crate) user_results: Option<TimelineUserResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineEntryItemContentRaw {
    #[serde(rename = "itemType")]
    pub(crate) item_type: Option<String>,
    #[serde(rename = "tweetDisplayType")]
    pub(crate) tweet_display_type: Option<String>,
    #[serde(rename = "tweetResult")]
    pub(crate) tweet_result: Option<TweetResultRaw>,
    pub(crate) tweet_results: Option<TweetResultRaw>,
    #[serde(rename = "userDisplayType")]
    pub(crate) user_display_type: Option<String>,
    pub(crate) user_results: Option<TimelineUserResultRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineItems {
    pub(crate) instructions: Option<Vec<TimelineInstruction>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineUser {
    pub(crate) result: Option<TimelineUserResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineUserResult {
    pub(crate) rest_id: Option<String>,
    pub(crate) legacy: Option<LegacyUserRaw>,
    pub(crate) is_blue_verified: Option<bool>,
    pub(crate) timeline_v2: Option<Box<TimelineV2>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineUserResultRaw {
    pub(crate) result: Option<TimelineUserResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineV2 {
    pub(crate) data: Option<TimelineData>,
    pub(crate) timeline: Option<TimelineItems>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ThreadedConversation {
    pub(crate) data: Option<ThreadedConversationData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ThreadedConversationData {
    pub(crate) threaded_conversation_with_injections_v2: Option<TimelineContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TweetResult {
    pub(crate) result: Option<TimelineResultRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TweetResultRaw {
    pub(crate) result: Option<TimelineResultRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct EntryContent {
    #[serde(rename = "cursorType")]
    pub(crate) cursor_type: Option<String>,
    pub(crate) value: Option<String>,
    pub(crate) items: Option<Vec<EntryItem>>,
    #[serde(rename = "itemContent")]
    pub(crate) item_content: Option<TimelineEntryItemContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct EntryItem {
    #[serde(rename = "entryId")]
    pub(crate) entry_id: Option<String>,
    pub(crate) item: Option<ItemContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ItemContent {
    pub(crate) content: Option<TimelineEntryItemContent>,
    #[serde(rename = "itemContent")]
    pub(crate) item_content: Option<TimelineEntryItemContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Hashtag {
    pub(crate) text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct UrlEntity {
    pub(crate) expanded_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct UserMention {
    pub(crate) id_str: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) screen_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineInstruction {
    pub(crate) entries: Option<Vec<TimelineEntry>>,
    pub(crate) entry: Option<TimelineEntry>,
    #[serde(rename = "type")]
    pub(crate) type_: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SearchEntryRaw {
    #[serde(rename = "entryId")]
    pub(crate) entry_id: String,
    #[serde(rename = "sortIndex")]
    pub(crate) sort_index: String,
    pub(crate) content: Option<SearchEntryContentRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SearchEntryContentRaw {
    #[serde(rename = "cursorType")]
    pub(crate) cursor_type: Option<String>,
    #[serde(rename = "entryType")]
    pub(crate) entry_type: Option<String>,
    #[serde(rename = "__typename")]
    pub(crate) typename: Option<String>,
    pub(crate) value: Option<String>,
    pub(crate) items: Option<Vec<SearchEntryItemRaw>>,
    #[serde(rename = "itemContent")]
    pub(crate) item_content: Option<TimelineEntryItemContentRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SearchEntryItemRaw {
    pub(crate) item: Option<SearchEntryItemInnerRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SearchEntryItemInnerRaw {
    pub(crate) content: Option<TimelineEntryItemContentRaw>,
}

pub(crate) fn parse_legacy_tweet(
    user: Option<&LegacyUserRaw>,
    tweet: Option<&LegacyTweetRaw>,
) -> anyhow::Result<Tweet> {
    let tweet = tweet.ok_or(anyhow::format_err!(
        "Tweet was not found in the timeline object",
    ))?;

    let user = user.ok_or(anyhow::format_err!(
        "User was not found in the timeline object",
    ))?;

    let id_str = tweet
        .id_str
        .as_ref()
        .or(tweet.conversation_id_str.as_ref())
        .ok_or(anyhow::format_err!("Tweet ID was not found in object"))?;

    let hashtags = tweet
        .entities
        .as_ref()
        .and_then(|e| e.hashtags.as_ref())
        .map(|h| h.iter().filter_map(|h| h.text.clone()).collect())
        .unwrap_or_default();

    let mentions = tweet
        .entities
        .as_ref()
        .and_then(|e| e.user_mentions.as_ref())
        .map(|mentions| {
            mentions
                .iter()
                .map(|m| Mention {
                    id: m.id_str.clone().unwrap_or_default(),
                    name: m.name.clone(),
                    username: m.screen_name.clone(),
                })
                .collect()
        })
        .unwrap_or_default();

    let (photos, videos, _) = if let Some(extended_entities) = &tweet.extended_entities {
        if let Some(media) = &extended_entities.media {
            parse_media_groups(media)
        } else {
            (Vec::new(), Vec::new(), false)
        }
    } else {
        (Vec::new(), Vec::new(), false)
    };

    let mut tweet = Tweet {
        bookmark_count: tweet.bookmark_count,
        conversation_id: tweet.conversation_id_str.clone(),
        id: Some(id_str.clone()),
        hashtags,
        likes: tweet.favorite_count,
        mentions,
        name: user.name.clone(),
        permanent_url: Some(format!(
            "https://twitter.com/{}/status/{}",
            user.screen_name.as_ref().unwrap_or(&String::new()),
            id_str
        )),
        photos,
        replies: tweet.reply_count,
        retweets: tweet.retweet_count,
        text: tweet.full_text.clone(),
        thread: Vec::new(),
        urls: tweet
            .entities
            .as_ref()
            .and_then(|e| e.urls.as_ref())
            .map(|urls| urls.iter().filter_map(|u| u.expanded_url.clone()).collect())
            .unwrap_or_default(),
        user_id: tweet.user_id_str.clone(),
        username: user.screen_name.clone(),
        videos,
        is_quoted: Some(false),
        is_reply: Some(false),
        is_retweet: Some(false),
        is_pin: Some(false),
        sensitive_content: Some(false),
        quoted_status: None,
        quoted_status_id: tweet.quoted_status_id_str.clone(),
        in_reply_to_status_id: tweet.in_reply_to_status_id_str.clone(),
        retweeted_status: None,
        retweeted_status_id: None,
        views: None,
        time_parsed: None,
        timestamp: None,
        place: tweet.place.clone(),
        in_reply_to_status: None,
        is_self_thread: None,
        poll: None,
        created_at: tweet.created_at.clone(),
        ext_views: None,
        quote_count: None,
        reply_count: None,
        retweet_count: None,
        screen_name: None,
        thread_id: None,
    };

    if let Some(created_at) = &tweet.created_at {
        if let Ok(time) = chrono::DateTime::parse_from_str(created_at, "%a %b %d %H:%M:%S %z %Y") {
            tweet.time_parsed = Some(time.with_timezone(&Utc));
            tweet.timestamp = Some(time.timestamp());
        }
    }

    if let Some(views) = tweet.ext_views {
        tweet.views = Some(views);
    }

    Ok(tweet)
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct QueryTweetsResponse {
    pub(crate) tweets: Vec<Tweet>,
    pub(crate) next: Option<String>,
    pub(crate) previous: Option<String>,
}

pub(crate) fn parse_media_groups(
    media: &[TimelineMediaExtendedRaw],
) -> (Vec<Photo>, Vec<Video>, bool) {
    let mut photos = Vec::new();
    let mut videos = Vec::new();
    let mut sensitive_content = false;

    for m in media
        .iter()
        .filter(|m| m.id_str.is_some() && m.media_url_https.is_some())
    {
        match m.r#type.as_deref() {
            Some("photo") => {
                photos.push(Photo {
                    id: m.id_str.clone().unwrap(),
                    url: m.media_url_https.clone().unwrap(),
                    alt_text: m.ext_alt_text.clone(),
                });
            }
            Some("video") => {
                videos.push(parse_video(m));
            }
            _ => {}
        }

        if let Some(warning) = &m.ext_sensitive_media_warning {
            sensitive_content = warning.adult_content.unwrap_or(false)
                || warning.graphic_violence.unwrap_or(false)
                || warning.other.unwrap_or(false);
        }
    }

    (photos, videos, sensitive_content)
}

fn parse_video(m: &TimelineMediaExtendedRaw) -> Video {
    let mut video = Video {
        id: m.id_str.clone().unwrap(),
        preview: m.media_url_https.clone().unwrap(),
        url: None,
    };

    let mut max_bitrate = 0;
    if let Some(video_info) = &m.video_info {
        if let Some(variants) = &video_info.variants {
            for variant in variants {
                if let (Some(bitrate), Some(url)) = (&variant.bitrate, &variant.url) {
                    if *bitrate > max_bitrate {
                        let mut variant_url = url.clone();
                        if let Some(idx) = variant_url.find("?tag=10") {
                            variant_url = variant_url[..idx + 1].to_string();
                        }
                        video.url = Some(variant_url);
                        max_bitrate = *bitrate;
                    }
                }
            }
        }
    }

    video
}
