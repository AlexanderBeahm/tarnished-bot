use reqwest::{self, Client, Response};
use http::{HeaderMap, HeaderValue, header::{COOKIE, CONTENT_TYPE}};
use tokio;
use base64;
use serde::Deserialize;

/*TODO:
1. Establish Twitter API is up and available
2. Establish Twitter.com is up and available
3. Log in to Twitter API and get token
4. Store token and set request information for future calls
5. Construct POST
6. Send POST*/

#[derive(Deserialize)]
pub struct AuthResponse {
    token_type: String,
    access_token: String
}

pub async fn ping_twitter()  {
    //TODO Make safer and handle errors properly via Rust interfaces.
    let client = reqwest::Client::new();
    let res = client.get("https://twitter.com").send();
    let response_text = res.await.unwrap().text().await.unwrap();
}

pub async fn login_twitter() -> AuthResponse {
    let scopes = ["tweet.read", "tweet.write", "users.read"];
    //let key = "key";
    //let secret = "secret";

    //Use the .env file for this.
    let bearer_token = "test";

    let client = reqwest::Client::new();
    let res = client.post("https://api.twitter.com/2/oauth2/token")
    .bearer_auth(bearer_token)
    .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
    .query(&[("grant_type", "client_credentials")])
    .send().await.unwrap();

    println!("{:?}", res);

    let response_text = res.json::<AuthResponse>().await.unwrap();
    return response_text;
}

pub async fn post_twitter() {
    let client = reqwest::Client::new();
    let access_token = "test";
    let res = client.post("https://api.twitter.com/2/tweets")
    .header("Authorization", format!("Bearer {access_token}"))
    .header("Content-type", "application/json")
    .body("{\"text\": \"Tarnished O Tarnished\"}")
    .send();
}


// pub fn base64_encode(input: &str) -> &str {
//     return &base64::encode(input);
// }