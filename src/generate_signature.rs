use base64::prelude::*;
use chrono::Utc;
use dotenv::dotenv;
use hmac::{Hmac, Mac};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use sha1::Sha1;
use std::env;
use textnonce::TextNonce;

pub fn generate_nonce() -> String {
    let nonce = TextNonce::sized(16).expect("failed getting nonce");
    nonce.to_string()
}

pub fn generate_oauth_signature() -> String {
    dotenv().ok();
    let timestamp = (Utc::now().timestamp_millis() / 1000).to_string();
    let consumer_key = env::var("CONSUMER_KEY").expect("consumer key is missing");
    let consumer_secret = env::var("CONSUMER_SECRET").expect("cosumer secret is missing");
    let access_token = env::var("ACCESS_TOKEN").expect("access token is missing");
    let token_secret = env::var("TOKEN_SECRET").expect("token secret is missing");
    let http_method = "POST";
    let base_url = "https://api.twitter.com/2/tweets";
    let nonce = generate_nonce();

    let mut signing_params: Vec<(String, String)> = Vec::new();
    signing_params.push(("oauth_consumer_key".to_string(), consumer_key.to_string()));
    signing_params.push(("oauth_nonce".to_string(), nonce.to_string()));
    signing_params.push((
        "oauth_signature_method".to_string(),
        "HMAC-SHA1".to_string(),
    ));
    signing_params.push(("oauth_token".to_string(), access_token.to_string()));
    signing_params.push(("oauth_timestamp".to_string(), timestamp.to_string()));
    signing_params.push(("oauth_version".to_string(), "1.0".to_string()));

    signing_params.sort();
    let param_string = signing_params
        .into_iter()
        .map(|(key, value)| {
            format!(
                "{}={}",
                utf8_percent_encode(&key, NON_ALPHANUMERIC),
                utf8_percent_encode(&value, NON_ALPHANUMERIC)
            )
        })
        .collect::<Vec<String>>()
        .join("&");
    let signature_base_string = format!(
        "{}&{}&{}",
        http_method,
        utf8_percent_encode(base_url, NON_ALPHANUMERIC),
        utf8_percent_encode(&param_string, NON_ALPHANUMERIC)
    );

    let signing_key = format!("{}&{}", consumer_secret, token_secret);
    let mut mac = Hmac::<Sha1>::new_from_slice(signing_key.as_bytes())
        .expect("Hmac can take key of any size");
    mac.update(signature_base_string.as_bytes());
    let signature = BASE64_STANDARD.encode(mac.finalize().into_bytes());
    // let signature = base64_url::encode(&mac.finalize().into_bytes());

    // format!("OAuth oauth_consumer_key=\"{}\", oauth_nonce=\"{}\", oauth_signature=\"{}\", oauth_signature_method=\"HMAC-SHA1\", oauth_timestamp=\"{}\", oauth_token=\"{}\", oauth_version=\"1.0\"",
    //     utf8_percent_encode(&consumer_key, NON_ALPHANUMERIC),
    //     utf8_percent_encode(&nonce, NON_ALPHANUMERIC),
    //     utf8_percent_encode(&signature, NON_ALPHANUMERIC),
    //     utf8_percent_encode(&timestamp, NON_ALPHANUMERIC),
    //     utf8_percent_encode(&access_token, NON_ALPHANUMERIC)
    // )
    // format!("include_entities=true&oauth_consumer_key={}&oauth_nonce={}&oauth_signature_method=HMAC-SHA1&oauth_timestamp={}&oauth_token={}&oauth_version=1.0&oauth_signature={}&status=Hello",
    // utf8_percent_encode(&consumer_key, NON_ALPHANUMERIC),
    // utf8_percent_encode(&nonce, NON_ALPHANUMERIC),
    // utf8_percent_encode(&timestamp, NON_ALPHANUMERIC),
    // utf8_percent_encode(&access_token, NON_ALPHANUMERIC),
    // utf8_percent_encode(&signature, NON_ALPHANUMERIC),
    // )
    signature
}
