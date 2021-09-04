use serde::de::DeserializeOwned;

use super::model::{GiphyRequest, API_ROOT};

/// Implementation of Giphy API that uses synchronous [`reqwest::Client`]
///
/// [`reqwest::Client`]: https://docs.rs/reqwest/0.11.4/reqwest/blocking/struct.Client.html
pub struct SyncApi {
    url: String,
    key: String,
    client: reqwest::blocking::Client,
}

impl SyncApi {
    /// Creates a new synchronous Giphy API Client
    pub fn new(key: String, client: reqwest::blocking::Client) -> SyncApi {
        SyncApi {
            url: API_ROOT.to_string(),
            key,
            client,
        }
    }

    pub fn new_with_url(url: String, key: String, client: reqwest::blocking::Client) -> SyncApi {
        SyncApi { url, key, client }
    }
}

pub trait RunnableSyncRequest<ResponseType> {
    /// Sends a request to a [SyncApi]
    ///
    /// [SyncApi]: ./struct.SyncApi.html
    fn send_to(&self, api: &SyncApi) -> Result<ResponseType, reqwest::Error>;
}

impl<'a, RequestType, ResponseType> RunnableSyncRequest<ResponseType> for RequestType
where
    RequestType: GiphyRequest<ResponseType>,
    ResponseType: DeserializeOwned,
{
    fn send_to(&self, api: &SyncApi) -> Result<ResponseType, reqwest::Error> {
        let endpoint = format!("{}/{}", api.url, self.get_endpoint());

        let response = api
            .client
            .get(&endpoint)
            .query(&[("api_key", api.key.clone())])
            .query(&self)
            .send()?;

        let response_data = response.error_for_status()?.json::<ResponseType>()?;

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

        let client = reqwest::blocking::Client::new();
        let api = SyncApi::new_with_url(api_root, api_key, client);

        let response = v1::gifs::SearchRequest::new("rage")
            .send_to(&api)
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

        let client = reqwest::blocking::Client::new();
        let api = SyncApi::new_with_url(api_root, api_key, client);

        let response = v1::gifs::TrendingRequest::new()
            .send_to(&api)
            .unwrap_or_else(|e| panic!("Error while calling search endpoint: {:?}", e));

        assert!(response.pagination.count > 0);
    }

    #[test]
    fn api_translate_200_ok() {
        dotenv().ok();
        let api_key = env::var("GIPHY_API_KEY_TEST")
            .unwrap_or_else(|e| panic!("Error retrieving env variable: {:?}", e));
        let api_root = server_url();
        let _m = mock(
            "GET",
            Matcher::Regex(r"/gifs/translate.*api_key=.+&s=.+".to_string()),
        )
        .with_status(200)
        .with_body_from_file("data/example-translate-response.json")
        .create();

        let client = reqwest::blocking::Client::new();
        let api = SyncApi::new_with_url(api_root, api_key, client);

        let response = v1::gifs::TranslateRequest::new("rage")
            .send_to(&api)
            .unwrap_or_else(|e| panic!("Error while calling search endpoint: {:?}", e));

        assert!(response.meta.status == 200);
    }

    #[test]
    fn api_random_200_ok() {
        dotenv().ok();
        let api_key = env::var("GIPHY_API_KEY_TEST")
            .unwrap_or_else(|e| panic!("Error retrieving env variable: {:?}", e));
        let api_root = server_url();
        let _m = mock(
            "GET",
            Matcher::Regex(r"/gifs/random.*api_key=.+".to_string()),
        )
        .with_status(200)
        .with_body_from_file("data/example-random-response.json")
        .create();

        let client = reqwest::blocking::Client::new();
        let api = SyncApi::new_with_url(api_root, api_key, client);

        let response = v1::gifs::RandomRequest::new()
            .send_to(&api)
            .unwrap_or_else(|e| panic!("Error while calling search endpoint: {:?}", e));

        assert!(response.meta.status == 200);
    }

    #[test]
    fn api_get_gif_200_ok() {
        dotenv().ok();
        let api_key = env::var("GIPHY_API_KEY_TEST")
            .unwrap_or_else(|e| panic!("Error retrieving env variable: {:?}", e));
        let api_root = server_url();
        let _m = mock(
            "GET",
            Matcher::Regex(r"/gifs/xT4uQulxzV39haRFjG.*api_key=.+".to_string()),
        )
        .with_status(200)
        .with_body_from_file("data/example-get-gif-response.json")
        .create();

        let client = reqwest::blocking::Client::new();
        let api = SyncApi::new_with_url(api_root, api_key, client);

        let response = v1::gifs::GetGifRequest::new("xT4uQulxzV39haRFjG")
            .send_to(&api)
            .unwrap_or_else(|e| panic!("Error while calling search endpoint: {:?}", e));

        assert!(response.meta.status == 200);
    }

    #[test]
    fn api_get_gifs_200_ok() {
        dotenv().ok();
        let api_key = env::var("GIPHY_API_KEY_TEST")
            .unwrap_or_else(|e| panic!("Error retrieving env variable: {:?}", e));
        let api_root = server_url();
        let _m = mock(
            "GET",
            Matcher::Regex(r"/gifs.*api_key=.+ids=.+".to_string()),
        )
        .with_status(200)
        .with_body_from_file("data/example-get-gifs-response.json")
        .create();

        let client = reqwest::blocking::Client::new();
        let api = SyncApi::new_with_url(api_root, api_key, client);

        let response =
            v1::gifs::GetGifsRequest::new(vec!["xT4uQulxzV39haRFjG", "3og0IPxMM0erATueVW"])
                .send_to(&api)
                .unwrap_or_else(|e| panic!("Error while calling search endpoint: {:?}", e));

        assert!(response.meta.status == 200);
    }
}
