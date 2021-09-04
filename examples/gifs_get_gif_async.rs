use std::env;

use dotenv::dotenv;
use giphy::v1::gifs::GetGifRequest;
use giphy::v1::r#async::*;

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    let api_key = env::var("GIPHY_API_KEY_TEST")
        .unwrap_or_else(|e| panic!("Error retrieving env variable: {:?}", e));
    let client = reqwest::Client::new();
    let api = AsyncApi::new(api_key, client);

    let response = GetGifRequest::new("xT4uQulxzV39haRFjG")
        .send_to(&api)
        .await
        .unwrap();

    println!("Response: {:?}", response);
}
