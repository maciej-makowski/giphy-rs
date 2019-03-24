use super::model::{SearchRequest, SearchResponse};

pub struct Api {
    _url: String,
    _key: String,
    _client: reqwest::Client
}

impl Api {
    pub fn new(url: String, key: String, client: reqwest::Client) -> Api {
        Api {
            _url: url,
            _key: key,
            _client: client
        }
    }

    pub fn search(&self, req: &SearchRequest) -> Result<SearchResponse, reqwest::Error> {
        let endpoint = format!("{}/search", self._url);

        let response = self._client.get(&endpoint)
            .query(&[("q", req.query)])
            .send()?;

        let search_response = response
            .error_for_status()?
            .json::<SearchResponse>()?;

        Ok(search_response)
    }
}

#[cfg(test)]
mod test {
    use std::env;
    use dotenv::dotenv;
    use mockito::{server_url, mock, Matcher};

    use crate::v1::SearchRequest;
    use super::*;

    #[test]
    fn api_search_200_ok() {
        dotenv().ok();
        let api_key = env::var("GIPHY_API_KEY_TEST")
            .unwrap_or_else(|e| panic!("Error retrieving env variable: {:?}", e));
        let api_root = server_url();
        let _m = mock("GET", Matcher::Regex(r"/search.*".to_string()))
            .with_status(200)
            .with_body_from_file("data/example-search-response.json")
            .create();

        let client = reqwest::Client::new();
        let api = Api::new(api_root, api_key, client);

        let req = SearchRequest::new("rage");
        let response = api.search(&req)
            .unwrap_or_else(|e| panic!("Error while calling search endpoint: {:?}", e));

        assert!(response.pagination.count > 0);
    }
}

