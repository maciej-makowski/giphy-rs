use std::default::Default;
pub use super::model::{SearchResponse, TrendingResponse, TranslateResponse, RandomResponse, GiphyRequest};

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
        "v1/gifs/search"
    }
}

/// Giphy [Trending endpoint] request
///
/// [Trending endpoint]: https://developers.giphy.com/docs/#path--gifs-trending
#[derive(Serialize, Default)]
pub struct TrendingRequest<'a> {
    pub(crate) rating: Option<&'a str>,

    pub(crate) limit: Option<u32>,

    pub(crate) offset: Option<u32>,
}

impl<'a> TrendingRequest<'a> {
    /// Creates new [Trending endpoint] request
    ///
    /// [Trending endpoint]: https://developers.giphy.com/docs/#path--gifs-trending
    pub fn new() -> TrendingRequest<'a> {
        Default::default()
    }

    /// Specifies the rating of GIF objects returned from [Trending] request
    ///
    /// [Trending]: https://developers.giphy.com/docs/#path--gifs-trending
    pub fn with_rating<'b: 'a>(mut self, rating: &'b str) -> Self {
        self.rating = Some(rating);
        self
    }

    /// Limits the maximum number of GIF objects returned from [Trending] request
    ///
    /// [Trending]: https://developers.giphy.com/docs/#path--gifs-trending
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Specifies the number of GIF objects to skip when making [Trending] request
    ///
    /// [Trending]: https://developers.giphy.com/docs/#path--gifs-trending
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
}

impl <'p> GiphyRequest<TrendingResponse> for TrendingRequest<'p> {
    fn get_endpoint(&self) -> &'static str {
        "v1/gifs/trending"
    }
}

/// Giphy [Translate endpoint] request
///
/// [Translate endpoint]: https://developers.giphy.com/docs/#path--gifs-translate
#[derive(Serialize, Default)]
pub struct TranslateRequest<'a> {
    #[serde(rename = "s")]
    pub (crate) phrase: &'a str,

    pub (crate) weirdness: Option<u8>
}

impl <'a> TranslateRequest<'a> {
    /// Creates new [Translate endpoint] request
    ///
    /// [Translate endpoint]: https://developers.giphy.com/docs/#path--gifs-translate
    pub fn new(phrase: &'a str) -> TranslateRequest {
        TranslateRequest { phrase, weirdness: None }
    }

    /// Specifies the weirdness value for [Translate] request
    ///
    /// [Translate]: https://developers.giphy.com/docs/#path--gifs-translate
    pub fn with_weirdness(mut self, value: u8) -> Self {
        self.weirdness = Some(value);
        self
    }
}

impl <'p> GiphyRequest< TranslateResponse> for TranslateRequest<'p> {
    fn get_endpoint(&self) -> &'static str {
        "v1/gifs/translate"
    }
}

/// Giphy [Random endpoint] request
///
/// [Random endpoint]: https://developers.giphy.com/docs/#path--gifs-random
#[derive(Serialize, Default)]
pub struct RandomRequest<'a, 'b> {
    pub(crate) tag: Option<&'a str>,

    pub(crate) rating: Option<&'b str>,
}

impl <'a, 'b> RandomRequest<'a, 'b> {
    /// Creates new [Random endpoint] request
    ///
    /// [Random endpoint]: https://developers.giphy.com/docs/#path--gifs-random
    pub fn new() -> RandomRequest<'a, 'b> {
        Default::default()
    }

    /// Filters [Random] request by specific tag
    ///
    /// [Random]: https://developers.giphy.com/docs/#path--gifs-random
    pub fn with_tag(mut self, value: &'a str) -> Self {
        self.tag = Some(value);
        self
    }

    /// Filters [Random] request by specific rating
    ///
    /// [Random]: https://developers.giphy.com/docs/#path--gifs-random
    pub fn with_rating(mut self, value: &'b str) -> Self {
        self.rating = Some(value);
        self
    }
}

impl <'a, 'b> GiphyRequest<RandomResponse> for RandomRequest<'a, 'b> {
    fn get_endpoint(&self) -> &'static str {
        "v1/gifs/random"
    }
}

#[derive(Serialize)]
pub struct GetGifRequest {
    #[serde(skip)]
    pub(crate) endpoint: String
}

impl GetGifRequest {
    pub fn new(gif_id: &str) -> GetGifRequest {
        GetGifRequest {
            endpoint: format!("v1/gifs/{}", gif_id)
        }
    }
}

impl GiphyRequest<RandomResponse> for GetGifRequest {
    fn get_endpoint(&self) -> &str {
        &self.endpoint
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_request() {
        let req = SearchRequest::new("hello")
            .with_limit(100)
            .with_offset(5);

        assert_eq!(req.get_endpoint(), "v1/gifs/search");
        assert_eq!(req.query, "hello");
        assert_eq!(req.limit, Some(100));
        assert_eq!(req.offset, Some(5));
    }

    #[test]
    fn trending_request() {
        let req = TrendingRequest::new()
            .with_rating("g")
            .with_limit(100)
            .with_offset(5);

        assert_eq!(req.get_endpoint(), "v1/gifs/trending");
        assert_eq!(req.rating, Some("g"));
        assert_eq!(req.limit, Some(100));
        assert_eq!(req.offset, Some(5));
    }

    #[test]
    fn translate_request() {
        let req = TranslateRequest::new("rage")
            .with_weirdness(10);

        assert_eq!(req.get_endpoint(), "v1/gifs/translate");
        assert_eq!(req.phrase, "rage");
        assert_eq!(req.weirdness, Some(10));
    }

    #[test]
    fn random_request() {
        let req = RandomRequest::new()
            .with_tag("burrito")
            .with_rating("g");

        assert_eq!(req.get_endpoint(), "v1/gifs/random");
        assert_eq!(req.tag, Some("burrito"));
        assert_eq!(req.rating, Some("g"));
    }

    #[test]
    fn get_gif_request() {
        let req = GetGifRequest::new("xT4uQulxzV39haRFjG");
        assert_eq!(req.get_endpoint(), "v1/gifs/xT4uQulxzV39haRFjG");
    }
}

