use std::env;

use dotenv::dotenv;
use giphy::v1::sync::*;
use giphy::v1::gifs::GetGifsRequest;

pub fn main() {
    dotenv().ok();
    let api_key = env::var("GIPHY_API_KEY_TEST")
        .unwrap_or_else(|e| panic!("Error retrieving env variable: {:?}", e));
    let client = reqwest::Client::new();
    let api = SyncApi::new(giphy::v1::API_ROOT.to_string(), api_key, client);

    let response = GetGifsRequest::new(vec!("xT4uQulxzV39haRFjG", "3og0IPxMM0erATueVW"))
        .send_to(&api)
        .unwrap_or_else(|e| panic!("Error while calling search endpoint: {:?}", e));

    println!("Response: {:?}", response);
}

