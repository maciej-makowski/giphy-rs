use serde::de::DeserializeOwned;
use futures::Future;
use super::model::{GiphyRequest};

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
    fn send_to(&self, api: &AsyncApi) -> Box<Future<Item = ResponseType, Error = reqwest::Error>>;
}

impl <RequestType, ResponseType: 'static> RunnableAsyncRequest<ResponseType> for RequestType
    where RequestType: GiphyRequest<ResponseType>,
          ResponseType: DeserializeOwned
{
    fn send_to(&self, api: &AsyncApi) -> Box<Future<Item = ResponseType, Error = reqwest::Error>> {
        let endpoint = format!("{}/{}", api.url, self.get_endpoint());

        let future = api.client
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
    use dotenv::dotenv;
    use mockito::{mock, server_url, Matcher};
    use std::env;
    use tokio::runtime::current_thread;
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
}

/*

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
*/
