use std::env;

use dotenv::dotenv;
use reqwest;
use reqwest_oauth1::OAuthClientProvider;
use serde_json::json;

pub async fn create_tweet(tweet_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let consumer_key = env::var("CONSUMER_KEY").expect("consumer key is missing");
    let consumer_secret = env::var("CONSUMER_SECRET").expect("cosumer secret is missing");
    let access_token = env::var("ACCESS_TOKEN").expect("access token is missing");
    let token_secret = env::var("TOKEN_SECRET").expect("token secret is missing");

    let secrets = reqwest_oauth1::Secrets::new(consumer_key, consumer_secret)
        .token(access_token, token_secret);

    let endpoint = "https://api.twitter.com/2/tweets";
    let tweet_data = json!({"text": tweet_text}).to_string();

    let client = reqwest::Client::new();
    let resp = client
        .oauth1(secrets)
        .post(endpoint)
        .header("Content-Type", "application/json")
        .body(tweet_data)
        .send()
        .await?;

    println!("tweet response: {:#?}", resp.text().await);
    Ok(())
}
