use futures::Future;
use super::{SearchRequest, SearchResponse};

pub struct Api {
    _url: String,
    _key: String,
    _client: reqwest::r#async::Client
}

impl Api {
    pub fn new(url: String, key: String, client: reqwest::r#async::Client) -> Api {
        Api {
            _url: url,
            _key: key,
            _client: client
        }
    }

    pub fn search(&self, req: &SearchRequest) -> impl Future<Item = SearchResponse, Error = reqwest::Error> {
        let endpoint = format!("{}/search", self._url);

        self._client.get(&endpoint)
            .query(&[("q", req.query)])
            .send()
            .and_then(|response| response.error_for_status())
            .and_then(|mut response| response.json::<SearchResponse>())
    }
}

#[cfg(test)]
mod test {
    use std::env;
    use dotenv::dotenv;
    use mockito::{server_url, mock, Matcher};
    use super::*;

    #[test]
    fn api_search_200_ok() {
        dotenv().ok();
        let api_key = env::var("GIPHY_API_KEY_TEST").unwrap();
        let api_root = server_url();
        let _m = mock("GET", Matcher::Regex(r"/search.*".to_string()))
            .with_status(200)
            .with_body_from_file("data/example-search-response.json")
            .create();

        let client = reqwest::r#async::Client::new();
        let api = Api::new(api_root, api_key, client);

        let req = SearchRequest::new("rage");
        let response = api.search(&req)
            .wait()
            .unwrap();

        assert!(response.pagination.count > 0);
    }
}

