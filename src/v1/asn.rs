use super::{SearchRequest, SearchResponse};

pub struct Api {
    _url: String,
    _key: String,
    _client: reqwest::async::Client
}

impl Api {
    pub fn new(url: String, key: String, client: reqwest::async::Client) -> Api {
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
