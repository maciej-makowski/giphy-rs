
use super::{SearchRequest, SearchResponse};

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

    use super::*;

    #[test]
    fn search_request_new() {
        let query = "test";
        let req = SearchRequest::new(query);

        assert_eq!(req.query, query);
        assert_eq!(req.offset, None);
        assert_eq!(req.limit, None);
        assert_eq!(req.lang, None);
    }

    #[test]
    fn search_request_offset() {
        let offset = 10u32;
        let mut req = SearchRequest::new("");
        req.offset(offset);

        assert_eq!(req.offset, Some(offset));
    }

    #[test]
    fn search_request_limit() {
        let limit = 10u32;
        let mut req = SearchRequest::new("");
        req.limit(limit);

        assert_eq!(req.limit, Some(limit));
    }

    #[test]
    fn search_request_lang() {
        let lang = iso639_1::Iso639_1::En;
        let mut req = SearchRequest::new("");
        req.lang(&lang);

        assert_eq!(req.lang, Some(lang));
    }

    #[test]
    fn api_search_200_ok() {
        dotenv().ok();
        let api_key = env::var("GIPHY_API_KEY_TEST").unwrap();
        let api_root = server_url();
        let _m = mock("GET", Matcher::Regex(r"/search.*".to_string()))
            .with_status(200)
            .with_body_from_file("data/example-search-response.json")
            .create();

        let client = reqwest::Client::new();
        let api = Api::new(api_root, api_key, client);

        let req = SearchRequest::new("rage");
        let response = api.search(&req)
            .unwrap();

        assert!(response.pagination.count > 0);
    }
}

