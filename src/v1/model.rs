use std::default::Default;

/// Default API URL for Giphy v1 API
pub static API_ROOT: &str = "https://api.giphy.com/v1";

/// Giphy [`Meta`] object representation
///
/// [`Meta`]: https://developers.giphy.com/docs/#metacontent-object
#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub msg: String,
    pub status: i32,
    pub response_id: String,
}

/// Giphy [`Pagination`] object representation
///
/// [`Pagination`]: https://developers.giphy.com/docs/#pagination-object
#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub count: i32,
    pub total_count: i32,
    pub offset: i32,
}

/// Giphy [`User`] object representation
///
/// [`User`]: https://developers.giphy.com/docs/#user-object
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub avatar_url: String,
    pub banner_url: String,
    pub profile_url: String,
    pub username: String,
    pub display_name: String,
    pub twitter: Option<String>,
}

/// Giphy Animated [`Images`] object representation
///
/// [`Images`]: https://developers.giphy.com/docs/#images-object
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageAnimated {
    pub url: Option<String>,
    pub width: String,
    pub height: String,
    pub size: Option<String>,
    pub mp4: Option<String>,
    pub mp4_size: Option<String>,
    pub webp: Option<String>,
    pub webp_size: Option<String>,
}

/// Giphy Still [`Images`] object representation
///
/// [`Images`]: https://developers.giphy.com/docs/#images-object
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageStill {
    pub url: String,
    pub width: String,
    pub height: String,
}

/// Giphy Looping [`Images`] object representation
///
/// [`Images`]: https://developers.giphy.com/docs/#images-object
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageLooping {
    pub mp4: String,
}

/// Giphy MP4 Preview [`Images`] object representation
///
/// [`Images`]: https://developers.giphy.com/docs/#images-object
#[derive(Serialize, Deserialize, Debug)]
pub struct ImagePreviewMp4 {
    pub mp4: String,
    pub mp4_size: String,
    pub width: String,
    pub height: String,
}

/// Giphy GIF Preview [`Images`] object representation
///
/// [`Images`]: https://developers.giphy.com/docs/#images-object
#[derive(Serialize, Deserialize, Debug)]
pub struct ImagePreviewGif {
    pub url: String,
    pub size: String,
    pub width: String,
    pub height: String,
}

/// Giphy [`Images`] object representation
///
/// [`Images`]: https://developers.giphy.com/docs/#images-object
#[derive(Serialize, Deserialize, Debug)]
pub struct Images {
    pub fixed_height: ImageAnimated,
    pub fixed_height_still: ImageStill,
    pub fixed_height_downsampled: ImageAnimated,
    pub fixed_width: ImageAnimated,
    pub fixed_width_still: ImageStill,
    pub fixed_width_downsampled: ImageAnimated,
    pub fixed_height_small: ImageAnimated,
    pub fixed_height_small_still: ImageStill,
    pub fixed_width_small: ImageAnimated,
    pub fixed_width_small_still: ImageStill,
    pub downsized: ImageAnimated,
    pub downsized_still: ImageStill,
    pub downsized_large: ImageAnimated,
    pub downsized_medium: ImageAnimated,
    pub downsized_small: ImageAnimated,
    pub original: ImageAnimated,
    pub original_still: ImageStill,
    pub looping: ImageLooping,
    pub preview: ImagePreviewMp4,
    pub preview_gif: ImagePreviewGif,
}

/// Giphy [`Gif`] object representation
///
/// [`Gif`]: https://developers.giphy.com/docs/#gif-object
#[derive(Serialize, Deserialize, Debug)]
pub struct Gif {
    #[serde(alias = "type")]
    pub gif_type: String,
    pub id: String,
    pub slug: String,
    pub url: String,
    pub bitly_url: String,
    pub embed_url: String,
    pub username: String,
    pub source: String,
    pub rating: String,
    pub user: Option<User>,
    pub source_tld: String,
    pub source_post_url: String,
    pub update_datetime: Option<String>,
    pub create_datetime: Option<String>,
    pub import_datetime: Option<String>,
    pub trending_datetime: Option<String>,
    pub images: Images,
    pub title: String,
}

/// Giphy [Search endpoint] response object representation
///
/// [Search endpoint]: https://developers.giphy.com/docs/#operation--gifs-search-get
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    pub data: Vec<Gif>,
    pub pagination: Pagination,
    pub meta: Meta,
}

/// Giphy [Search endpoint] request parameters
///
/// [Search endpoint]: https://developers.giphy.com/docs/#operation--gifs-search-get
#[derive(Serialize)]
pub struct SearchRequest<'a> {
    #[serde(rename = "q")]
    pub(crate) query: &'a str,

    pub(crate) limit: Option<u32>,

    pub(crate) offset: Option<u32>,
}

impl<'a> SearchRequest<'a> {
    /// Creates new [Search endpoint] request parameters
    ///
    /// [Search endpoint]: https://developers.giphy.com/docs/#operation--gifs-search-get
    pub fn new(query: &'a str) -> SearchRequest<'a> {
        SearchRequest {
            query,
            limit: None,
            offset: None,
        }
    }

    /// Limits the maximum number of GIF objects returned from [Search] request
    ///
    /// [Search]: https://developers.giphy.com/docs/#operation--gifs-search-get
    pub fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Specifies the number of GIF objects to skip when making [Search] request
    ///
    /// [Search]: https://developers.giphy.com/docs/#operation--gifs-search-get
    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.offset = Some(offset);
        self
    }
}

/// Giphy [Trending endpoint] response object representation
///
/// [Trending endpoint]: https://developers.giphy.com/docs/#path--gifs-trending
#[derive(Serialize, Deserialize, Debug)]
pub struct TrendingResponse {
    pub data: Vec<Gif>,
    pub pagination: Pagination,
    pub meta: Meta,
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
}
