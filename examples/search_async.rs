use std::env;

use dotenv::dotenv;
use giphy::v1::{AsyncApi, SearchRequest};
use futures::future::Future;
use tokio;

pub fn main() {
    dotenv().ok();
    let api_key = env::var("GIPHY_API_KEY_TEST")
        .unwrap_or_else(|e| panic!("Error retrieving env variable: {:?}", e));
    let client = reqwest::r#async::Client::new();
    let api = AsyncApi::new(giphy::v1::API_ROOT.to_string(), api_key, client);

    let mut req = SearchRequest::new("rage");
    req.limit(5);

    let test_fut = api.search(&req)
        .map(|response| {
            println!("Response: {:?}", response);
            ()
        })
        .map_err(|e| {
            println!("Error: {:?}", e);
            ()
        });

    tokio::run(test_fut);
}
