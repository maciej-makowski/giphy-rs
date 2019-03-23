use std::convert::From;

pub static API_ROOT: &str = "https://api.giphy.com/v1/gifs";

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub msg: String,
    pub status: i32,
    pub response_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub count: i32,
    pub total_count: i32,
    pub offset: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub avatar_url: String,
    pub banner_url: String,
    pub profile_url: String,
    pub username: String,
    pub display_name: String,
    pub twitter: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageAnimated {
    pub url: Option<String>,
    pub width: String,
    pub height: String,
    pub size: Option<String>,
    pub mp4: Option<String>,
    pub mp4_size: Option<String>,
    pub webp: Option<String>,
    pub webp_size: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageStill {
    pub url: String,
    pub width: String,
    pub height: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageLooping {
    pub mp4: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImagePreviewMp4 {
    pub mp4: String,
    pub mp4_size: String,
    pub width: String,
    pub height: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImagePreviewGif {
    pub url: String,
    pub size: String,
    pub width: String,
    pub height: String
}

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
    pub preview_gif: ImagePreviewGif
}

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
    pub title: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    pub data: Vec<Gif>,
    pub pagination: Pagination,
    pub meta: Meta
}


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
    pub (crate) query: &'a str,
    pub (crate) limit: Option<u32>,
    pub (crate) offset: Option<u32>,
    pub (crate) lang: Option<iso639_1::Iso639_1>
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

