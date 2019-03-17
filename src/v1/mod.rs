mod model;
mod sync;

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

