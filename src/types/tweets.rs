use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Tweet {
    pub(crate) ext_views: Option<i32>,
    pub(crate) created_at: Option<String>,
    pub(crate) bookmark_count: Option<i32>,
    pub(crate) conversation_id: Option<String>,
    pub(crate) hashtags: Vec<String>,
    pub(crate) id: Option<String>,
    pub(crate) in_reply_to_status: Option<Box<Tweet>>,
    pub(crate) in_reply_to_status_id: Option<String>,
    pub(crate) is_quoted: Option<bool>,
    pub(crate) is_pin: Option<bool>,
    pub(crate) is_reply: Option<bool>,
    pub(crate) is_retweet: Option<bool>,
    pub(crate) is_self_thread: Option<bool>,
    pub(crate) likes: Option<i32>,
    pub(crate) name: Option<String>,
    pub(crate) mentions: Vec<Mention>,
    pub(crate) permanent_url: Option<String>,
    pub(crate) photos: Vec<Photo>,
    pub(crate) place: Option<PlaceRaw>,
    pub(crate) quoted_status: Option<Box<Tweet>>,
    pub(crate) quoted_status_id: Option<String>,
    pub(crate) replies: Option<i32>,
    pub(crate) retweets: Option<i32>,
    pub(crate) retweeted_status: Option<Box<Tweet>>,
    pub(crate) retweeted_status_id: Option<String>,
    pub(crate) text: Option<String>,
    pub(crate) thread: Vec<Tweet>,
    pub(crate) time_parsed: Option<DateTime<Utc>>,
    pub(crate) timestamp: Option<i64>,
    pub(crate) urls: Vec<String>,
    pub(crate) user_id: Option<String>,
    pub(crate) username: Option<String>,
    pub(crate) videos: Vec<Video>,
    pub(crate) views: Option<i32>,
    pub(crate) sensitive_content: Option<bool>,
    pub(crate) poll: Option<PollV2>,
    pub(crate) quote_count: Option<i32>,
    pub(crate) reply_count: Option<i32>,
    pub(crate) retweet_count: Option<i32>,
    pub(crate) screen_name: Option<String>,
    pub(crate) thread_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Mention {
    pub(crate) id: String,
    pub(crate) username: Option<String>,
    pub(crate) name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Photo {
    pub(crate) id: String,
    pub(crate) url: String,
    pub(crate) alt_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Video {
    pub(crate) id: String,
    pub(crate) preview: String,
    pub(crate) url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PlaceRaw {
    pub(crate) id: Option<String>,
    pub(crate) place_type: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) full_name: Option<String>,
    pub(crate) country_code: Option<String>,
    pub(crate) country: Option<String>,
    pub(crate) bounding_box: Option<BoundingBox>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BoundingBox {
    #[serde(rename = "type")]
    pub(crate) type_: Option<String>,
    pub(crate) coordinates: Option<Vec<Vec<Vec<f64>>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PollV2 {
    pub(crate) id: Option<String>,
    pub(crate) end_datetime: Option<String>,
    pub(crate) voting_status: Option<String>,
    pub(crate) options: Vec<PollOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PollOption {
    pub(crate) position: Option<i32>,
    pub(crate) label: String,
    pub(crate) votes: Option<i32>,
}
