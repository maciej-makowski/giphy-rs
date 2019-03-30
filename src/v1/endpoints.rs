/*
impl<'a> SearchRequest<'a> {
    pub fn new(query: &'a str) -> SearchRequest<'a> {
        SearchRequest {
            query,
            limit: None,
            offset: None,
        }
    }

    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.offset = Some(offset);
        self
    }
}

/// Giphy [Trending endpoint] request parameters
///
/// [Trending endpoint]: https://developers.giphy.com/docs/#path--gifs-trending
#[derive(Serialize, Default)]
pub struct TrendingRequest<'a> {
    pub(crate) rating: Option<&'a str>,

    pub(crate) limit: Option<u32>,

    pub(crate) offset: Option<u32>,
}

impl<'a> TrendingRequest<'a> {
    /// Creates new [Trending endpoint] request parameters
    ///
    /// [Trending endpoint]: https://developers.giphy.com/docs/#path--gifs-trending
    pub fn new() -> TrendingRequest<'a> {
        Default::default()
    }

    /// Specifies the rating of GIF objects returned from [Trending] request
    ///
    /// [Trending]: https://developers.giphy.com/docs/#path--gifs-trending
    pub fn rating<'b: 'a>(&mut self, rating: &'b str) -> &mut Self {
        self.rating = Some(rating);
        self
    }

    /// Limits the maximum number of GIF objects returned from [Trending] request
    ///
    /// [Trending]: https://developers.giphy.com/docs/#path--gifs-trending
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Specifies the number of GIF objects to skip when making [Trending] request
    ///
    /// [Trending]: https://developers.giphy.com/docs/#path--gifs-trending
    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.offset = Some(offset);
        self
    }
}

/// Giphy [Trending endpoint] request parameters
///
/// [Translate endpoint]: https://developers.giphy.com/docs/#path--gifs-translate
#[derive(Serialize, Default)]
pub struct TranslateRequest<'a> {
    #[serde(alias = "s")]
    pub (crate) phrase: &'a str,

    pub (crate) weirdness: Option<u8>
}

impl <'a> TranslateRequest<'a> {
    pub fn new(phrase: &'a str) -> TranslateRequest {
        TranslateRequest { phrase, weirdness: None }
    }

    pub fn weirdness(&mut self, value: u8) -> &mut Self {
        self.weirdness = Some(value);
        self
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
    fn trending_request_new() {
        let req = TrendingRequest::new();
        assert_eq!(req.rating, None);
        assert_eq!(req.limit, None);
        assert_eq!(req.offset, None);
    }

    #[test]
    fn trending_request_rating() {
        let mut req = TrendingRequest::new();
        req.rating("10");
        assert_eq!(req.rating, Some("10"));
    }

    #[test]
    fn trending_request_limit() {
        let mut req = TrendingRequest::new();
        req.limit(13);
        assert_eq!(req.limit, Some(13));
    }

    #[test]
    fn trending_request_offset() {
        let mut req = TrendingRequest::new();
        req.offset(13);
        assert_eq!(req.offset, Some(13));
    }

    #[test]
    fn translate_request_new() {
        let req = TranslateRequest::new("hello");
        assert_eq!(req.phrase, "hello");
        assert_eq!(req.weirdness, None);
    }

    #[test]
    fn translate_request_weirdness() {
        let mut req = TranslateRequest::new("hello");
        req.weirdness(10);
        assert_eq!(req.phrase, "hello");
        assert_eq!(req.weirdness, Some(10));
    }
}
*/
