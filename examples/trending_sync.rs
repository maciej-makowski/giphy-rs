use std::env;

use dotenv::dotenv;
use giphy::v1::{SyncApi, TrendingRequest};

pub fn main() {
    dotenv().ok();
    let api_key = env::var("GIPHY_API_KEY_TEST")
        .unwrap_or_else(|e| panic!("Error retrieving env variable: {:?}", e));
    let client = reqwest::Client::new();
    let api = SyncApi::new(giphy::v1::API_ROOT.to_string(), api_key, client);

    let mut req = TrendingRequest::new();
    req.limit(1);

    let response = api
        .trending(&req)
        .unwrap_or_else(|e| panic!("Error while calling search endpoint: {:?}", e));

    println!("Response: {:?}", response);
}
