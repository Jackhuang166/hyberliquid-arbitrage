use serde_json::json;

use crate::generate_signature::generate_oauth_signature;

pub async fn create_tweet_not_working() -> Result<(), Box<dyn std::error::Error>> {
    let signature = generate_oauth_signature();

    println!("signature: {}", signature);

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.twitter.com/2/tweets")
        .header("Authorization", signature)
        .header("Content-Type", "application/json")
        .json(&json!({"text": "Hello"}))
        .send()
        .await?;

    println!("asdasdas: {:#?}", &response);
    println!("tweet response: {:#?}", response.text().await);

    Ok(())
}
