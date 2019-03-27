use super::model::{SearchRequest, SearchResponse, TrendingRequest, TrendingResponse};

/// Implementation of Giphy API that uses synchronous [`reqwest::Client`]
///
/// [`reqwest::Client`]: https://docs.rs/reqwest/0.9.12/reqwest/struct.Client.html
pub struct Api {
    _url: String,
    _key: String,
    _client: reqwest::Client,
}

impl Api {
    /// Creates a new Giphy API Client
    pub fn new(url: String, key: String, client: reqwest::Client) -> Api {
        Api {
            _url: url,
            _key: key,
            _client: client,
        }
    }

    /// Performs search against Giphy [Search endpoint]
    ///
    /// [Search Endpoint]: https://developers.giphy.com/docs/#operation--gifs-search-get
    pub fn search(&self, req: &SearchRequest) -> Result<SearchResponse, reqwest::Error> {
        let endpoint = format!("{}/gifs/search", self._url);

        let response = self
            ._client
            .get(&endpoint)
            .query(&[("api_key", self._key.clone())])
            .query(&req)
            .send()?;

        let search_response = response.error_for_status()?.json::<SearchResponse>()?;

        Ok(search_response)
    }

    /// Performs search against Giphy [Trending endpoint]
    ///
    /// [Trending endpoint]: https://developers.giphy.com/docs/#path--gifs-trending
    pub fn trending(&self, req: &TrendingRequest) -> Result<TrendingResponse, reqwest::Error> {
        let endpoint = format!("{}/gifs/trending", self._url);

        let response = self
            ._client
            .get(&endpoint)
            .query(&[("api_key", self._key.clone())])
            .query(&req)
            .send()?;

        let trending_response = response.error_for_status()?.json::<TrendingResponse>()?;

        Ok(trending_response)
    }
}

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use mockito::{mock, server_url, Matcher};
    use std::env;

    use super::*;
    use crate::v1::SearchRequest;

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

        let client = reqwest::Client::new();
        let api = Api::new(api_root, api_key, client);

        let req = SearchRequest::new("rage");
        let response = api
            .search(&req)
            .unwrap_or_else(|e| panic!("Error while calling search endpoint: {:?}", e));

        assert!(response.pagination.count > 0);
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

        let client = reqwest::Client::new();
        let api = Api::new(api_root, api_key, client);

        let req = TrendingRequest::new();
        let response = api
            .trending(&req)
            .unwrap_or_else(|e| panic!("Error while calling search endpoint: {:?}", e));

        assert!(response.pagination.count > 0);
    }
}
