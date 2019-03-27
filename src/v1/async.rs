use super::model::{SearchRequest, SearchResponse, TrendingRequest, TrendingResponse};
use futures::Future;

/// Implementation of Giphy API that uses asynchronous [`reqwest::async::Client`]
///
/// [`reqwest::async::Client`]: https://docs.rs/reqwest/0.9.12/reqwest/async/struct.Client.html
pub struct Api {
    _url: String,
    _key: String,
    _client: reqwest::r#async::Client,
}

impl Api {
    /// Creates a new Giphy API Client
    pub fn new(url: String, key: String, client: reqwest::r#async::Client) -> Api {
        Api {
            _url: url,
            _key: key,
            _client: client,
        }
    }

    /// Performs search against Giphy [Search endpoint]
    ///
    /// [Search endpoint]: https://developers.giphy.com/docs/#operation--gifs-search-get
    pub fn search(
        &self,
        req: &SearchRequest,
    ) -> impl Future<Item = SearchResponse, Error = reqwest::Error> {
        let endpoint = format!("{}/gifs/search", self._url);

        self._client
            .get(&endpoint)
            .query(&[("api_key", self._key.clone())])
            .query(&req)
            .send()
            .and_then(reqwest::r#async::Response::error_for_status)
            .and_then(|mut response| response.json::<SearchResponse>())
    }

    /// Performs search against Giphy [Trending endpoint]
    ///
    /// [Trending endpoint]: https://developers.giphy.com/docs/#path--gifs-trending
    pub fn trending(
        &self,
        req: &TrendingRequest,
    ) -> impl Future<Item = TrendingResponse, Error = reqwest::Error> {
        let endpoint = format!("{}/gifs/trending", self._url);

        self._client
            .get(&endpoint)
            .query(&[("api_key", self._key.clone())])
            .query(&req)
            .send()
            .and_then(reqwest::r#async::Response::error_for_status)
            .and_then(|mut response| response.json::<TrendingResponse>())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenv::dotenv;
    use mockito::{mock, server_url, Matcher};
    use std::env;
    use tokio::runtime::current_thread;

    #[test]
    fn api_search_200_ok() {
        dotenv().ok();
        let api_key = env::var("GIPHY_API_KEY_TEST")
            .unwrap_or_else(|e| panic!("Error retrieving env variable: {:?}", e));
        let api_root = server_url();
        let _m = mock(
            "GET",
            Matcher::Regex(r"/gifs/search.*api_key=.+q=.+".to_string()),
        )
        .with_status(200)
        .with_body_from_file("data/example-search-response.json")
        .create();

        let client = reqwest::r#async::Client::new();
        let api = Api::new(api_root, api_key, client);

        let req = SearchRequest::new("rage");
        let test_fut = api
            .search(&req)
            .map(|response| {
                assert!(response.pagination.count > 0);
            })
            .map_err(|e| panic!("Error while calling search endpoint: {:?}", e));

        current_thread::run(test_fut);
    }

    #[test]
    fn api_trending_200_ok() {
        dotenv().ok();
        let api_key = env::var("GIPHY_API_KEY_TEST")
            .unwrap_or_else(|e| panic!("Error retrieving env variable: {:?}", e));
        let api_root = server_url();
        let _m = mock(
            "GET",
            Matcher::Regex(r"/gifs/trending.*api_key=.+".to_string()),
        )
        .with_status(200)
        .with_body_from_file("data/example-trending-response.json")
        .create();

        let client = reqwest::r#async::Client::new();
        let api = Api::new(api_root, api_key, client);

        let req = TrendingRequest::new();
        let test_fut = api
            .trending(&req)
            .map(|response| {
                assert!(response.pagination.count > 0);
            })
            .map_err(|e| panic!("Error while calling search endpoint: {:?}", e));

        current_thread::run(test_fut);
    }
}
