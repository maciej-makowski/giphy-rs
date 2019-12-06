use super::model::*;
use std::default::Default;

/// Giphy [Search endpoint] request
///
/// [Search endpoint]: https://developers.giphy.com/docs/api/
#[derive(Serialize)]
pub struct SearchRequest<'p> {
    #[serde(rename = "q")]
    pub(crate) query: &'p str,

    pub(crate) limit: Option<u32>,

    pub(crate) offset: Option<u32>,
}

impl<'p> SearchRequest<'p> {
    /// Creates new [Search endpoint] request
    ///
    /// [Search endpoint]: https://developers.giphy.com/docs/api/
    pub fn new(query: &'p str) -> SearchRequest<'p> {
        SearchRequest {
            query,
            limit: None,
            offset: None,
        }
    }

    /// Limits the maximum number of GIF objects returned from [Search] request
    ///
    /// [Search]: https://developers.giphy.com/docs/api/
    pub fn with_limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }

    /// Specifies the number of GIF objects to skip when making [Search] request
    ///
    /// [Search]: https://developers.giphy.com/docs/api/
    pub fn with_offset(mut self, value: u32) -> Self {
        self.offset = Some(value);
        self
    }
}

impl<'p> GiphyRequest<PaginatedGifListResponse> for SearchRequest<'p> {
    fn get_endpoint(&self) -> &'static str {
        "v1/gifs/search"
    }
}

/// Giphy [Trending endpoint] request
///
/// [Trending endpoint]: https://developers.giphy.com/docs/api/
#[derive(Serialize, Default)]
pub struct TrendingRequest<'a> {
    pub(crate) rating: Option<&'a str>,

    pub(crate) limit: Option<u32>,

    pub(crate) offset: Option<u32>,
}

impl<'a> TrendingRequest<'a> {
    /// Creates new [Trending endpoint] request
    ///
    /// [Trending endpoint]: https://developers.giphy.com/docs/api/
    pub fn new() -> TrendingRequest<'a> {
        Default::default()
    }

    /// Specifies the rating of GIF objects returned from [Trending] request
    ///
    /// [Trending]: https://developers.giphy.com/docs/api/
    pub fn with_rating<'b: 'a>(mut self, rating: &'b str) -> Self {
        self.rating = Some(rating);
        self
    }

    /// Limits the maximum number of GIF objects returned from [Trending] request
    ///
    /// [Trending]: https://developers.giphy.com/docs/api/
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Specifies the number of GIF objects to skip when making [Trending] request
    ///
    /// [Trending]: https://developers.giphy.com/docs/api/
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
}

impl<'p> GiphyRequest<PaginatedGifListResponse> for TrendingRequest<'p> {
    fn get_endpoint(&self) -> &'static str {
        "v1/gifs/trending"
    }
}

/// Giphy [Translate endpoint] request
///
/// [Translate endpoint]: https://developers.giphy.com/docs/api/
#[derive(Serialize, Default)]
pub struct TranslateRequest<'a> {
    #[serde(rename = "s")]
    pub(crate) phrase: &'a str,

    pub(crate) weirdness: Option<u8>,
}

impl<'a> TranslateRequest<'a> {
    /// Creates new [Translate endpoint] request
    ///
    /// [Translate endpoint]: https://developers.giphy.com/docs/api/
    pub fn new(phrase: &'a str) -> TranslateRequest {
        TranslateRequest {
            phrase,
            weirdness: None,
        }
    }

    /// Specifies the weirdness value for [Translate] request
    ///
    /// [Translate]: https://developers.giphy.com/docs/api/
    pub fn with_weirdness(mut self, value: u8) -> Self {
        self.weirdness = Some(value);
        self
    }
}

impl<'p> GiphyRequest<SingleGifResponse> for TranslateRequest<'p> {
    fn get_endpoint(&self) -> &'static str {
        "v1/gifs/translate"
    }
}

/// Giphy [Random endpoint] request
///
/// [Random endpoint]: https://developers.giphy.com/docs/api/
#[derive(Serialize, Default)]
pub struct RandomRequest<'a, 'b> {
    pub(crate) tag: Option<&'a str>,

    pub(crate) rating: Option<&'b str>,
}

impl<'a, 'b> RandomRequest<'a, 'b> {
    /// Creates new [Random endpoint] request
    ///
    /// [Random endpoint]: https://developers.giphy.com/docs/api/
    pub fn new() -> RandomRequest<'a, 'b> {
        Default::default()
    }

    /// Filters [Random] request by specific tag
    ///
    /// [Random]: https://developers.giphy.com/docs/api/
    pub fn with_tag(mut self, value: &'a str) -> Self {
        self.tag = Some(value);
        self
    }

    /// Filters [Random] request by specific rating
    ///
    /// [Random]: https://developers.giphy.com/docs/api/
    pub fn with_rating(mut self, value: &'b str) -> Self {
        self.rating = Some(value);
        self
    }
}

impl<'a, 'b> GiphyRequest<SingleGifResponse> for RandomRequest<'a, 'b> {
    fn get_endpoint(&self) -> &'static str {
        "v1/gifs/random"
    }
}

/// Giphy [GIF by id endpoint] request
///
/// [GIF by id endpoint]: https://developers.giphy.com/docs/api/
#[derive(Serialize)]
pub struct GetGifRequest {
    #[serde(skip)]
    pub(crate) endpoint: String,
}

impl GetGifRequest {
    /// Created new [GIF by id] request
    ///
    /// [GIF by id]: https://developers.giphy.com/docs/api/
    pub fn new(gif_id: &str) -> GetGifRequest {
        GetGifRequest {
            endpoint: format!("v1/gifs/{}", gif_id),
        }
    }
}

impl GiphyRequest<SingleGifResponse> for GetGifRequest {
    fn get_endpoint(&self) -> &str {
        &self.endpoint
    }
}

/// Giphy [GIFs by id endpoint] request
///
/// [GIFs by id endpoint]: https://developers.giphy.com/docs/api/
#[derive(Serialize)]
pub struct GetGifsRequest {
    pub(crate) ids: String,
}

impl GetGifsRequest {
    /// Created new [GIFs by id] request
    ///
    /// [GIFs by id endpoint]: https://developers.giphy.com/docs/api/
    pub fn new(ids: Vec<&str>) -> GetGifsRequest {
        GetGifsRequest { ids: ids.join(",") }
    }
}

impl GiphyRequest<PaginatedGifListResponse> for GetGifsRequest {
    fn get_endpoint(&self) -> &str {
        "v1/gifs"
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_request() {
        let req = SearchRequest::new("hello").with_limit(100).with_offset(5);

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
        let req = TranslateRequest::new("rage").with_weirdness(10);

        assert_eq!(req.get_endpoint(), "v1/gifs/translate");
        assert_eq!(req.phrase, "rage");
        assert_eq!(req.weirdness, Some(10));
    }

    #[test]
    fn random_request() {
        let req = RandomRequest::new().with_tag("burrito").with_rating("g");

        assert_eq!(req.get_endpoint(), "v1/gifs/random");
        assert_eq!(req.tag, Some("burrito"));
        assert_eq!(req.rating, Some("g"));
    }

    #[test]
    fn get_gif_request() {
        let req = GetGifRequest::new("xT4uQulxzV39haRFjG");
        assert_eq!(req.get_endpoint(), "v1/gifs/xT4uQulxzV39haRFjG");
    }

    #[test]
    fn get_gifs_request() {
        let ids = vec!["xT4uQulxzV39haRFjG", "3og0IPxMM0erATueVW"];
        let req = GetGifsRequest::new(ids);
        assert_eq!(req.get_endpoint(), "v1/gifs");
        assert_eq!(req.ids, "xT4uQulxzV39haRFjG,3og0IPxMM0erATueVW");
    }
}
