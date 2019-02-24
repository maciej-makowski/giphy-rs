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
