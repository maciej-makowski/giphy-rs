use super::model::{SearchResponse, GiphyRequest};

/// [Search endpoint] request
/// 
/// [Search endpoint]: https://developers.giphy.com/docs/#operation--gifs-search-get
#[derive(Serialize)]
pub struct SearchRequest<'p> {
    #[serde(rename = "q")]
    pub(crate) query: &'p str,

    pub(crate) limit: Option<u32>,

    pub(crate) offset: Option<u32>,
}

impl <'p> SearchRequest<'p> {
    /// Creates new [Search endpoint] request
    ///
    /// [Search endpoint]: https://developers.giphy.com/docs/#operation--gifs-search-get
    pub fn new(query: &'p str) -> SearchRequest<'p> {
        SearchRequest { query, limit: None, offset: None }
    }

    /// Limits the maximum number of GIF objects returned from [Search] request
    ///
    /// [Search]: https://developers.giphy.com/docs/#operation--gifs-search-get
    pub fn with_limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }

    /// Specifies the number of GIF objects to skip when making [Search] request
    ///
    /// [Search]: https://developers.giphy.com/docs/#operation--gifs-search-get
    pub fn with_offset(mut self, value: u32) -> Self {
        self.offset = Some(value);
        self
    }
}

impl <'p> GiphyRequest<SearchResponse> for SearchRequest<'p> {
    fn get_endpoint(&self) -> &'static str {
        "/v1/gifs/search"
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_request_simple() {
        let req = SearchRequest::new("hello");

        assert_eq!(req.get_endpoint(), "/v1/gifs/search");
        assert_eq!(req.query, "hello");
        assert_eq!(req.limit, None);
        assert_eq!(req.offset, None);
    }

    #[test]
    fn search_request() {
        let req = SearchRequest::new("hello")
            .with_limit(100)
            .with_offset(5);

        assert_eq!(req.get_endpoint(), "/v1/gifs/search");
        assert_eq!(req.query, "hello");
        assert_eq!(req.limit, Some(100));
        assert_eq!(req.offset, Some(5));
    }
}
