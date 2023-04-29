use crate::TweetResult;
use egg_mode::tweet::Tweet;
use egg_mode::Token;

pub async fn get_account_ids(token: &Token, users_list: Vec<String>) -> TweetResult<Vec<u64>> {
    let account_ids: Vec<u64> = egg_mode::user::lookup(users_list, token)
        .await?
        .into_iter()
        .map(|u| u.response.id)
        .collect();
    Ok(account_ids)
}

pub async fn get_tweet_url(tweet: &Tweet) -> Option<String> {
    let tweet_id = tweet.id;
    let screen_name = &tweet.user.as_ref()?.screen_name;
    Some(format!(
        "https://twitter.com/{}/status/{}",
        screen_name, tweet_id
    ))
}
pub async fn tweet_has_url(tweet: &Tweet) -> bool {
    !tweet.entities.urls.is_empty()
}
pub async fn tweet_has_media(tweet: &Tweet) -> bool {
    tweet.extended_entities.is_some() || tweet.entities.media.is_some()
}
