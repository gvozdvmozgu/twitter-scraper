use serde::Deserialize;

use crate::types::profile::parse_profile;

use super::{
    v1::{QueryProfilesResponse, QueryTweetsResponse},
    v2::{parse_legacy_tweet, SearchEntryRaw},
};

#[derive(Debug, Deserialize)]
pub(crate) struct SearchTimeline {
    pub(crate) data: Option<SearchData>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SearchData {
    pub(crate) search_by_raw_query: Option<SearchByRawQuery>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SearchByRawQuery {
    pub(crate) search_timeline: Option<SearchTimelineData>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SearchTimelineData {
    pub(crate) timeline: Option<TimelineData>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TimelineData {
    pub(crate) instructions: Option<Vec<SearchInstruction>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SearchInstruction {
    pub(crate) entries: Option<Vec<SearchEntryRaw>>,
    pub(crate) entry: Option<SearchEntryRaw>,
    #[serde(rename = "type")]
    pub(crate) instruction_type: Option<String>,
}

pub(crate) fn parse_tweets(timeline: &SearchTimeline) -> QueryTweetsResponse {
    let mut bottom_cursor = None;
    let mut top_cursor = None;
    let mut tweets = Vec::new();

    let instructions = timeline
        .data
        .as_ref()
        .and_then(|data| data.search_by_raw_query.as_ref())
        .and_then(|search| search.search_timeline.as_ref())
        .and_then(|timeline| timeline.timeline.as_ref())
        .and_then(|timeline| timeline.instructions.as_ref())
        .unwrap_or(const { &Vec::new() });

    for instruction in instructions {
        if let Some(instruction_type) = &instruction.instruction_type {
            if instruction_type == "TimelineAddEntries"
                || instruction_type == "TimelineReplaceEntry"
            {
                if let Some(entry) = &instruction.entry {
                    if let Some(content) = &entry.content {
                        match content.cursor_type.as_deref() {
                            Some("Bottom") => {
                                bottom_cursor = content.value.clone();
                                continue;
                            }
                            Some("Top") => {
                                top_cursor = content.value.clone();
                                continue;
                            }
                            _ => {}
                        }
                    }
                }

                let entries = instruction
                    .entries
                    .as_ref()
                    .unwrap_or(const { &Vec::new() });

                for entry in entries {
                    (|| -> Option<()> {
                        let content = entry.content.as_ref()?;

                        if let Some(item_content) = content
                            .item_content
                            .as_ref()
                            .filter(|it| it.tweet_display_type.as_deref() == "Tweet".into())
                        {
                            let tweet_results = item_content.tweet_results.as_ref()?;
                            let result = tweet_results.result.as_ref()?;

                            let user_legacy = result
                                .core
                                .as_ref()
                                .and_then(|core| core.user_results.as_ref())
                                .and_then(|user_results| user_results.result.as_ref())
                                .and_then(|result| result.legacy.as_ref());

                            if let Ok(tweet_result) =
                                parse_legacy_tweet(user_legacy, result.legacy.as_deref())
                            {
                                if tweet_result.views.is_none() {
                                    if let Some(Ok(view_count)) = result
                                        .views
                                        .as_ref()
                                        .and_then(|views| views.count.as_ref())
                                        .map(|count| count.parse::<i32>())
                                    {
                                        let mut tweet = tweet_result;
                                        tweet.views = Some(view_count);
                                        tweets.push(tweet);
                                    }
                                } else {
                                    tweets.push(tweet_result);
                                }
                            }
                        } else if let Some(cursor_type) = &content.cursor_type {
                            match cursor_type.as_str() {
                                "Bottom" => bottom_cursor = content.value.clone(),
                                "Top" => top_cursor = content.value.clone(),
                                _ => {}
                            }
                        }

                        Some(())
                    })();
                }
            }
        }
    }

    QueryTweetsResponse {
        tweets,
        next: bottom_cursor,
        previous: top_cursor,
    }
}

pub(crate) fn parse_users(timeline: &SearchTimeline) -> QueryProfilesResponse {
    let instructions = timeline
        .data
        .as_ref()
        .and_then(|d| d.search_by_raw_query.as_ref())
        .and_then(|s| s.search_timeline.as_ref())
        .and_then(|t| t.timeline.as_ref())
        .and_then(|t| t.instructions.as_ref())
        .unwrap_or(const { &Vec::new() });

    let (mut bottom_cursor, mut top_cursor) = (None, None);
    let mut profiles = Vec::new();

    for instr in instructions {
        if !matches!(
            instr.instruction_type.as_deref(),
            Some("TimelineAddEntries" | "TimelineReplaceEntry")
        ) {
            continue;
        }

        if let Some((cursor_type, value)) = instr
            .entry
            .as_ref()
            .and_then(|e| e.content.as_ref())
            .and_then(|c| c.cursor_type.as_deref().map(|t| (t, c.value.clone())))
        {
            match cursor_type {
                "Bottom" => {
                    bottom_cursor = value;
                    continue;
                }
                "Top" => {
                    top_cursor = value;
                    continue;
                }
                _ => {}
            }
        }

        let entries = instr.entries.as_ref().unwrap_or(const { &Vec::new() });

        for entry in entries {
            (|| -> Option<()> {
                let content = entry.content.as_ref()?;

                if let Some(item) = &content.item_content {
                    if item.user_display_type.as_deref() == Some("User") {
                        let result = item.user_results.as_ref()?.result.as_ref()?;
                        let legacy = result.legacy.as_ref()?;
                        let mut profile = parse_profile(legacy, result.is_blue_verified);
                        if profile.id.is_empty() {
                            profile.id = result.rest_id.clone().unwrap_or_default();
                        }
                        profiles.push(profile);
                    }
                } else if let Some(t) = content.cursor_type.as_deref() {
                    match t {
                        "Bottom" => bottom_cursor = content.value.clone(),
                        "Top" => top_cursor = content.value.clone(),
                        _ => {}
                    }
                }

                Some(())
            })();
        }
    }

    QueryProfilesResponse {
        profiles,
        next: bottom_cursor,
        previous: top_cursor,
    }
}
