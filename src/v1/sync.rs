use serde::de::DeserializeOwned;

use super::model::{GiphyRequest};

/// Implementation of Giphy API that uses synchronous [`reqwest::Client`]
///
/// [`reqwest::Client`]: https://docs.rs/reqwest/0.9.12/reqwest/struct.Client.html
pub struct SyncApi {
    url: String,
    key: String,
    client: reqwest::Client,
}

impl SyncApi {
    /// Creates a new synchronous Giphy API Client
    pub fn new(url: String, key: String, client: reqwest::Client) -> SyncApi {
        SyncApi {
            url: url,
            key: key,
            client: client,
        }
    }
}

pub trait RunnableSyncRequest<ResponseType> {
    fn send_to(&self, api: &SyncApi) -> Result<ResponseType, reqwest::Error>;
}

impl <RequestType, ResponseType> RunnableSyncRequest<ResponseType> for RequestType
    where RequestType: GiphyRequest<ResponseType>,
          ResponseType: DeserializeOwned {

    fn send_to(&self, api: &SyncApi) -> Result<ResponseType, reqwest::Error> { 
        let endpoint = format!("{}/{}", api.url, self.get_endpoint());

        let response = api.client
            .get(&endpoint)
            .query(&[("api_key", api.key.clone())])
            .query(&self)
            .send()?;

        let response_data = response.error_for_status()?
            .json::<ResponseType>()?;

        Ok(response_data)
    }
}

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use mockito::{mock, server_url, Matcher};
    use std::env;

    use super::*;
    use crate::v1;

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
        let api = SyncApi::new(api_root, api_key, client);

        let response = v1::gifs::SearchRequest::new("rage")
            .send_to(&api)
            .unwrap_or_else(|e| panic!("Error while calling search endpoint: {:?}", e));

        assert!(response.pagination.count > 0);
    }
}

    /*
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

    /// Performs request against Giphy [Trending endpoint]
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
    */
