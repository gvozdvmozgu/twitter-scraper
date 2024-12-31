use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::profile::LegacyUserRaw;
use crate::types::profile::Profile;
use crate::types::tweets::PlaceRaw;
use crate::types::tweets::Tweet;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Hashtag {
    pub(crate) text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineUserMentionBasicRaw {
    pub(crate) id_str: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) screen_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineMediaBasicRaw {
    pub(crate) media_url_https: Option<String>,
    pub(crate) r#type: Option<String>,
    pub(crate) url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineUrlBasicRaw {
    pub(crate) expanded_url: Option<String>,
    pub(crate) url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ExtSensitiveMediaWarningRaw {
    pub(crate) adult_content: Option<bool>,
    pub(crate) graphic_violence: Option<bool>,
    pub(crate) other: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct VideoVariant {
    pub(crate) bitrate: Option<i32>,
    pub(crate) url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct VideoInfo {
    pub(crate) variants: Option<Vec<VideoVariant>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineMediaExtendedRaw {
    pub(crate) id_str: Option<String>,
    pub(crate) media_url_https: Option<String>,
    pub(crate) ext_sensitive_media_warning: Option<ExtSensitiveMediaWarningRaw>,
    pub(crate) r#type: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) video_info: Option<VideoInfo>,
    pub(crate) ext_alt_text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SearchResultRaw {
    pub(crate) rest_id: Option<String>,
    pub(crate) __typename: Option<String>,
    pub(crate) core: Option<UserResultsCore>,
    pub(crate) views: Option<Views>,
    pub(crate) note_tweet: Option<NoteTweet>,
    pub(crate) quoted_status_result: Option<QuotedStatusResult>,
    pub(crate) legacy: Option<LegacyTweetRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct UserResultsCore {
    pub(crate) user_results: Option<UserResults>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct UserResults {
    pub(crate) result: Option<UserResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct UserResult {
    pub(crate) is_blue_verified: Option<bool>,
    pub(crate) legacy: Option<LegacyUserRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Views {
    pub(crate) count: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct NoteTweet {
    pub(crate) note_tweet_results: Option<NoteTweetResults>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct NoteTweetResults {
    pub(crate) result: Option<NoteTweetResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct NoteTweetResult {
    pub(crate) text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct QuotedStatusResult {
    pub(crate) result: Option<Box<SearchResultRaw>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineResultRaw {
    pub(crate) result: Option<Box<TimelineResultRaw>>,
    pub(crate) rest_id: Option<String>,
    pub(crate) __typename: Option<String>,
    pub(crate) core: Option<TimelineCore>,
    pub(crate) views: Option<TimelineViews>,
    pub(crate) note_tweet: Option<TimelineNoteTweet>,
    pub(crate) quoted_status_result: Option<Box<TimelineQuotedStatus>>,
    pub(crate) legacy: Option<Box<LegacyTweetRaw>>,
    pub(crate) tweet: Option<Box<TimelineResultRaw>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineCore {
    pub(crate) user_results: Option<TimelineUserResults>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineUserResults {
    pub(crate) result: Option<TimelineUserResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineUserResult {
    pub(crate) is_blue_verified: Option<bool>,
    pub(crate) legacy: Option<LegacyUserRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineViews {
    pub(crate) count: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineNoteTweet {
    pub(crate) note_tweet_results: Option<TimelineNoteTweetResults>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineNoteTweetResults {
    pub(crate) result: Option<TimelineNoteTweetResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineNoteTweetResult {
    pub(crate) text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineQuotedStatus {
    pub(crate) result: Option<Box<TimelineResultRaw>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct LegacyTweetRaw {
    pub(crate) bookmark_count: Option<i32>,
    pub(crate) conversation_id_str: Option<String>,
    pub(crate) created_at: Option<String>,
    pub(crate) favorite_count: Option<i32>,
    pub(crate) full_text: Option<String>,
    pub(crate) entities: Option<TweetEntities>,
    pub(crate) extended_entities: Option<TweetExtendedEntities>,
    pub(crate) id_str: Option<String>,
    pub(crate) in_reply_to_status_id_str: Option<String>,
    pub(crate) place: Option<PlaceRaw>,
    pub(crate) reply_count: Option<i32>,
    pub(crate) retweet_count: Option<i32>,
    pub(crate) retweeted_status_id_str: Option<String>,
    pub(crate) retweeted_status_result: Option<TimelineRetweetedStatus>,
    pub(crate) quoted_status_id_str: Option<String>,
    pub(crate) time: Option<String>,
    pub(crate) user_id_str: Option<String>,
    pub(crate) ext_views: Option<TweetExtViews>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TweetEntities {
    pub(crate) hashtags: Option<Vec<Hashtag>>,
    pub(crate) media: Option<Vec<TimelineMediaBasicRaw>>,
    pub(crate) urls: Option<Vec<TimelineUrlBasicRaw>>,
    pub(crate) user_mentions: Option<Vec<TimelineUserMentionBasicRaw>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TweetExtendedEntities {
    pub(crate) media: Option<Vec<TimelineMediaExtendedRaw>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineRetweetedStatus {
    pub(crate) result: Option<TimelineResultRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TweetExtViews {
    pub(crate) state: Option<String>,
    pub(crate) count: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineGlobalObjectsRaw {
    pub(crate) tweets: Option<HashMap<String, Option<LegacyTweetRaw>>>,
    pub(crate) users: Option<HashMap<String, Option<LegacyUserRaw>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineDataRawCursor {
    pub(crate) value: Option<String>,
    pub(crate) cursor_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineDataRawEntity {
    pub(crate) id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineDataRawModuleItem {
    pub(crate) client_event_info: Option<ClientEventInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClientEventInfo {
    pub(crate) details: Option<ClientEventDetails>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ClientEventDetails {
    pub(crate) guide_details: Option<GuideDetails>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct GuideDetails {
    pub(crate) transparent_guide_details: Option<TransparentGuideDetails>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TransparentGuideDetails {
    pub(crate) trend_metadata: Option<TrendMetadata>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TrendMetadata {
    pub(crate) trend_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineDataRawAddEntry {
    pub(crate) content: Option<TimelineEntryContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineDataRawPinEntry {
    pub(crate) content: Option<TimelinePinContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelinePinContent {
    pub(crate) item: Option<TimelineItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineDataRawReplaceEntry {
    pub(crate) content: Option<TimelineReplaceContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineReplaceContent {
    pub(crate) operation: Option<TimelineOperation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineDataRawInstruction {
    pub(crate) add_entries: Option<TimelineAddEntries>,
    pub(crate) pin_entry: Option<TimelineDataRawPinEntry>,
    pub(crate) replace_entry: Option<TimelineDataRawReplaceEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineAddEntries {
    pub(crate) entries: Option<Vec<TimelineDataRawAddEntry>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineDataRaw {
    pub(crate) instructions: Option<Vec<TimelineDataRawInstruction>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineV1 {
    pub(crate) global_objects: Option<TimelineGlobalObjectsRaw>,
    pub(crate) timeline: Option<TimelineDataRaw>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct QueryTweetsResponse {
    pub(crate) tweets: Vec<Tweet>,
    pub(crate) next: Option<String>,
    pub(crate) previous: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct QueryProfilesResponse {
    pub(crate) profiles: Vec<Profile>,
    pub(crate) next: Option<String>,
    pub(crate) previous: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineEntryContent {
    pub(crate) item: Option<TimelineItem>,
    pub(crate) operation: Option<TimelineOperation>,
    pub(crate) timeline_module: Option<TimelineModule>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineItem {
    pub(crate) content: Option<TimelineContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineContent {
    pub(crate) tweet: Option<TimelineDataRawEntity>,
    pub(crate) user: Option<TimelineDataRawEntity>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineOperation {
    pub(crate) cursor: Option<TimelineDataRawCursor>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineModule {
    pub(crate) items: Option<Vec<TimelineModuleItemWrapper>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TimelineModuleItemWrapper {
    pub(crate) item: Option<TimelineDataRawModuleItem>,
}
