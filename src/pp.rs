use crate::types::{
    profile::Profile,
    timeline::v1::{QueryProfilesResponse, QueryTweetsResponse},
    tweets::{Mention, Photo, PollV2, Tweet, Video},
};

pub(crate) fn tweets(response: &QueryTweetsResponse) {
    println!("==============================");
    println!("QueryTweetsResponse:");
    println!("==============================");

    for (index, tweet) in response.tweets.iter().enumerate() {
        println!("\nTweet #{}:", index + 1);
        self::tweet(tweet);
    }

    if let Some(ref next) = response.next {
        println!("\nNext Page: {}", next);
    } else {
        println!("\nNo next page.");
    }

    if let Some(ref previous) = response.previous {
        println!("Previous Page: {}", previous);
    } else {
        println!("No previous page.");
    }

    println!("==============================\n");
}

pub(crate) fn tweet(tweet: &Tweet) {
    println!("------------------------------");
    if let Some(ref text) = tweet.text {
        println!("Text: {}", text);
    } else {
        println!("Text: N/A");
    }

    if let Some(ref created_at) = tweet.created_at {
        println!("Created At: {}", created_at);
    } else {
        println!("Created At: N/A");
    }

    if let Some(likes) = tweet.likes {
        println!("Likes: {}", likes);
    } else {
        println!("Likes: N/A");
    }

    if let Some(retweets) = tweet.retweets {
        println!("Retweets: {}", retweets);
    } else {
        println!("Retweets: N/A");
    }

    if let Some(replies) = tweet.replies {
        println!("Replies: {}", replies);
    } else {
        println!("Replies: N/A");
    }

    if let Some(ref user_id) = tweet.user_id {
        println!("User ID: {}", user_id);
    } else {
        println!("User ID: N/A");
    }

    if let Some(ref username) = tweet.username {
        println!("Username: {}", username);
    } else {
        println!("Username: N/A");
    }

    if !tweet.mentions.is_empty() {
        println!("Mentions:");
        for mention in &tweet.mentions {
            self::mention(mention);
        }
    }

    if !tweet.photos.is_empty() {
        println!("Photos:");
        for photo in &tweet.photos {
            self::photo(photo);
        }
    }

    if !tweet.videos.is_empty() {
        println!("Videos:");
        for video in &tweet.videos {
            self::video(video);
        }
    }

    if let Some(ref poll) = tweet.poll {
        println!("Poll:");
        self::poll(poll);
    }

    println!("------------------------------");
}

pub(crate) fn mention(mention: &Mention) {
    println!(
        "  Mention: Username: {:?}, ID: {}",
        mention.username, mention.id
    );
}

pub(crate) fn photo(photo: &Photo) {
    println!("  Photo: URL: {}", photo.url);
}

pub(crate) fn video(video: &Video) {
    println!("  Video: URL: {:?}, ", video.url);
}

pub(crate) fn poll(poll: &PollV2) {
    println!("  Options: {:?}", poll.options);
    println!("  End Datetime {:?}", poll.end_datetime);
}

pub(crate) fn profiles(response: &QueryProfilesResponse) {
    println!("==============================");
    println!("QueryProfilesResponse:");
    println!("==============================");

    for (index, profile) in response.profiles.iter().enumerate() {
        println!("\nProfile #{}:", index + 1);
        self::profile(profile);
    }

    if let Some(ref next) = response.next {
        println!("\nNext Page: {}", next);
    } else {
        println!("\nNo next page.");
    }

    if let Some(ref previous) = response.previous {
        println!("Previous Page: {}", previous);
    } else {
        println!("No previous page.");
    }

    println!("==============================\n");
}

pub(crate) fn profile(profile: &Profile) {
    println!("------------------------------");
    println!("ID: {}", profile.id);
    println!("Username: {}", profile.username);
    println!("Name: {}", profile.name);

    if let Some(ref description) = profile.description {
        println!("Description: {}", description);
    } else {
        println!("Description: N/A");
    }

    if let Some(ref location) = profile.location {
        println!("Location: {}", location);
    } else {
        println!("Location: N/A");
    }

    if let Some(ref url) = profile.url {
        println!("URL: {}", url);
    } else {
        println!("URL: N/A");
    }

    println!("Protected: {}", profile.protected);
    println!("Verified: {}", profile.verified);
    println!("Followers: {}", profile.followers_count);
    println!("Following: {}", profile.following_count);
    println!("Tweets: {}", profile.tweets_count);
    println!("Listed: {}", profile.listed_count);

    println!("Created At: {}", profile.created_at);

    if let Some(ref profile_image_url) = profile.profile_image_url {
        println!("Profile Image URL: {}", profile_image_url);
    } else {
        println!("Profile Image URL: N/A");
    }

    if let Some(ref profile_banner_url) = profile.profile_banner_url {
        println!("Profile Banner URL: {}", profile_banner_url);
    } else {
        println!("Profile Banner URL: N/A");
    }

    if let Some(ref pinned_tweet_id) = profile.pinned_tweet_id {
        println!("Pinned Tweet ID: {}", pinned_tweet_id);
    } else {
        println!("Pinned Tweet ID: N/A");
    }

    if let Some(is_blue_verified) = profile.is_blue_verified {
        println!("Is Blue Verified: {}", is_blue_verified);
    } else {
        println!("Is Blue Verified: N/A");
    }

    println!("------------------------------");
}
