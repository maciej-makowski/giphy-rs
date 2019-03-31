use std::env;

use dotenv::dotenv;
use giphy::v1::gifs::TrendingRequest;
use giphy::v1::sync::*;

pub fn main() {
    dotenv().ok();
    let api_key = env::var("GIPHY_API_KEY_TEST")
        .unwrap_or_else(|e| panic!("Error retrieving env variable: {:?}", e));
    let client = reqwest::Client::new();
    let api = SyncApi::new(giphy::v1::API_ROOT.to_string(), api_key, client);

    let response = TrendingRequest::new()
        .with_limit(1)
        .send_to(&api)
        .unwrap_or_else(|e| panic!("Error while calling trending endpoint: {:?}", e));

    println!("Response: {:?}", response);
}
