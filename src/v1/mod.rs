mod model;
mod sync;
mod asn;

use std::convert::From;
pub use model::*;

pub static API_ROOT: &str = "https://api.giphy.com/v1/gifs";

#[derive(Debug)]
pub struct ApiError {
    text: String
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        ApiError {
            text: format!("{:?}", error)
        }
    }
}

pub struct SearchRequest<'a> {
    query: &'a str,
    limit: Option<u32>,
    offset: Option<u32>,
    lang: Option<iso639_1::Iso639_1>
}

impl <'a> SearchRequest<'a> {
    pub fn new (query: &'a str) -> SearchRequest<'a> {
        SearchRequest {
            query,
            limit: None,
            offset: None,
            lang: None
        }
    }

    pub fn limit(&mut self, limit: u32) {
        self.limit = Some(limit);
    }

    pub fn offset(&mut self, offset: u32) {
        self.offset = Some(offset);
    }

    pub fn lang(&mut self, lang: &iso639_1::Iso639_1) {
        self.lang = Some(lang.clone());
    }
}

#[cfg(test)]
mod test {
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
}

