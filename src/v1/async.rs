use super::model::GiphyRequest;
use futures::Future;
use serde::de::DeserializeOwned;
use std::marker::Send;

/// Implementation of Giphy API that uses asynchronous [`reqwest::async::Client`]
///
/// [`reqwest::async::Client`]: https://docs.rs/reqwest/0.9.12/reqwest/async/struct.Client.html
pub struct AsyncApi {
    url: String,
    key: String,
    client: reqwest::r#async::Client,
}

impl AsyncApi {
    /// Creates a new Giphy API Client
    pub fn new(url: String, key: String, client: reqwest::r#async::Client) -> AsyncApi {
        AsyncApi {
            url: url,
            key: key,
            client: client,
        }
    }
}

pub trait RunnableAsyncRequest<ResponseType> {
    /// Sends a request to an [AsyncApi]
    ///
    /// [SyncApi]: ./struct.AsyncApi.html
    fn send_to(
        &self,
        api: &AsyncApi,
    ) -> Box<Future<Item = ResponseType, Error = reqwest::Error> + Send>;
}

impl<'a, RequestType, ResponseType> RunnableAsyncRequest<ResponseType> for RequestType
where
    RequestType: GiphyRequest<ResponseType>,
    ResponseType: 'static + DeserializeOwned + Send,
{
    fn send_to(
        &self,
        api: &AsyncApi,
    ) -> Box<Future<Item = ResponseType, Error = reqwest::Error> + Send> {
        let endpoint = format!("{}/{}", api.url, self.get_endpoint());

        let future = api
            .client
            .get(&endpoint)
            .query(&[("api_key", &api.key)])
            .query(&self)
            .send()
            .and_then(reqwest::r#async::Response::error_for_status)
            .and_then(|mut response| response.json::<ResponseType>());

        Box::new(future)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::v1;
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
        let api = AsyncApi::new(api_root, api_key, client);

        let test_fut = v1::gifs::SearchRequest::new("rage")
            .send_to(&api)
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
        let api = AsyncApi::new(api_root, api_key, client);

        let test_fut = v1::gifs::TrendingRequest::new()
            .send_to(&api)
            .map(|response| {
                assert!(response.pagination.count > 0);
            })
            .map_err(|e| panic!("Error while calling search endpoint: {:?}", e));

        current_thread::run(test_fut);
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

        let client = reqwest::r#async::Client::new();
        let api = AsyncApi::new(api_root, api_key, client);

        let test_fut = v1::gifs::TranslateRequest::new("rage")
            .send_to(&api)
            .map(|response| {
                assert!(response.meta.status == 200);
            })
            .map_err(|e| panic!("Error while calling search endpoint: {:?}", e));

        current_thread::run(test_fut);
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

        let client = reqwest::r#async::Client::new();
        let api = AsyncApi::new(api_root, api_key, client);

        let test_fut = v1::gifs::RandomRequest::new()
            .send_to(&api)
            .map(|response| {
                assert!(response.meta.status == 200);
            })
            .map_err(|e| panic!("Error while calling search endpoint: {:?}", e));

        current_thread::run(test_fut);
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

        let client = reqwest::r#async::Client::new();
        let api = AsyncApi::new(api_root, api_key, client);

        let test_fut = v1::gifs::GetGifRequest::new("xT4uQulxzV39haRFjG")
            .send_to(&api)
            .map(|response| {
                assert!(response.meta.status == 200);
            })
            .map_err(|e| panic!("Error while calling search endpoint: {:?}", e));

        current_thread::run(test_fut);
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

        let client = reqwest::r#async::Client::new();
        let api = AsyncApi::new(api_root, api_key, client);

        let test_fut =
            v1::gifs::GetGifsRequest::new(vec!["xT4uQulxzV39haRFjG", "3og0IPxMM0erATueVW"])
                .send_to(&api)
                .map(|response| {
                    assert!(response.meta.status == 200);
                })
                .map_err(|e| panic!("Error while calling search endpoint: {:?}", e));

        current_thread::run(test_fut);
    }
}
