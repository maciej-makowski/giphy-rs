use std::env;

use dotenv::dotenv;
use futures::future::Future;
use giphy::v1::r#async::*;
use giphy::v1::gifs::TrendingRequest;

use tokio;

pub fn main() {
    dotenv().ok();
    let api_key = env::var("GIPHY_API_KEY_TEST")
        .unwrap_or_else(|e| panic!("Error retrieving env variable: {:?}", e));
    let client = reqwest::r#async::Client::new();
    let api = AsyncApi::new(giphy::v1::API_ROOT.to_string(), api_key, client);

    let test_fut = TrendingRequest::new()
        .with_limit(1)
        .send_to(&api)
        .map(|response| {
            println!("Response: {:?}", response);
            std::process::exit(0)
        })
        .map_err(|e| {
            println!("Error: {:?}", e);
            std::process::exit(1)
        });

    tokio::run(test_fut);
}
