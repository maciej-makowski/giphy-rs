use std::env;

use dotenv::dotenv;
use giphy::v1::gifs::RandomRequest;
use giphy::v1::sync::*;

pub fn main() {
    dotenv().ok();
    let api_key = env::var("GIPHY_API_KEY_TEST")
        .unwrap_or_else(|e| panic!("Error retrieving env variable: {:?}", e));
    let client = reqwest::Client::new();
    let api = SyncApi::new(api_key, client);

    let response = RandomRequest::new()
        .send_to(&api)
        .unwrap_or_else(|e| panic!("Error while calling search endpoint: {:?}", e));

    println!("Response: {:?}", response);
}
