extern crate reqwest;
extern crate iso639_1;
extern crate dotenv;

pub mod v1 {
    use std::convert::From;

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

    pub struct Api {
        pub (crate) url: String,
        pub (crate) key: String
    }

    impl Api {
        pub fn new(url: &str, key: &str) -> Api {
            Api {
                url: url.to_string(),
                key: key.to_string()
            }
        }

        pub fn search(&self, req: &SearchRequest) -> Result<String, ApiError> {
            let endpoint = format!("{}/search/api_key={}&q={}", 
                                   self.url,
                                   self.key,
                                   req.query
            );

            let response_text = reqwest::get(&endpoint)
                .map_err(|e| ApiError::from(e))?
                .text()?;

            Ok(response_text)
        }
    }

    #[cfg(test)]
    mod test {
        use std::env;
        use dotenv::dotenv;

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
        fn api_search_integration() {
            dotenv().ok();
            let api_key = env::var("GIPHY_API_KEY_TEST").unwrap();
            let api = Api::new(API_ROOT, &api_key);

            let req = SearchRequest::new("rage");
            let response_text = api.search(&req)
                .unwrap();

            assert!(response_text != "");
        }
    }
}

