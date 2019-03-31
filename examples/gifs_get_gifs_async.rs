use std::env;

use dotenv::dotenv;
use futures::future::Future;
use giphy::v1::gifs::GetGifsRequest;
use giphy::v1::r#async::*;

use tokio;

pub fn main() {
    dotenv().ok();
    let api_key = env::var("GIPHY_API_KEY_TEST")
        .unwrap_or_else(|e| panic!("Error retrieving env variable: {:?}", e));
    let client = reqwest::r#async::Client::new();
    let api = AsyncApi::new(api_key, client);

    let test_fut = GetGifsRequest::new(vec!["xT4uQulxzV39haRFjG", "3og0IPxMM0erATueVW"])
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
