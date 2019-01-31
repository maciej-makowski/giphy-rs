extern crate reqwest;
extern crate iso639_1;
extern crate dotenv;

use std::convert::From;

static API_ROOT: &str = "https://api.giphy.com/v1/gifs";

#[derive(Debug)]
pub struct Error {
    text: String
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error {
            text: format!("{:?}", error)
        }
    }
}

pub struct SearchRequest {
    api_key: String,
    query: String,
    limit: Option<u32>,
    offset: Option<u32>,
    lang: Option<iso639_1::Iso639_1>
}

impl SearchRequest {
    pub fn new(api_key: String, query: String) -> SearchRequest {
        SearchRequest {
            api_key,
            query,
            limit: None,
            offset: None, 
            lang: None
        }
    }
}

pub fn search(req: &SearchRequest) -> Result<String, Error> {
    let endpoint = format!("{}/search/api_key={}&q={}", API_ROOT, req.api_key, req.query);
    let response_text = reqwest::get(&endpoint).map_err(|e| Error::from(e))?.text()?;

    Ok(response_text)
}


#[cfg(test)]
mod tests {
    use std::env;
    use dotenv::dotenv;
    use super::*;

    #[test]
    fn calls_search_endpoint() {
        dotenv().ok();
        let api_key = env::var("GIPHY_API_KEY_TEST").unwrap();
        let req = SearchRequest::new(String::from(api_key), String::from("rage"));
        let response_text = search(&req).unwrap();
        assert_ne!(response_text, "");
    }
}

