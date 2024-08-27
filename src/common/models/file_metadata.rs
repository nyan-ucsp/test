use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FileMetadata {
    pub content_type: String,
    pub original_name: String,
    pub size: u64,
    pub image_data: Option<ImageMetadata>,
    pub video_data: Option<VideoMetadata>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
}

impl ImageMetadata {
    pub fn default() -> Self {
        ImageMetadata {
            width: 0,
            height: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct VideoMetadata {
    pub thumbnail: String,
    pub width: u32,
    pub height: u32,
}

impl VideoMetadata {
    pub fn default() -> Self {
        VideoMetadata {
            thumbnail: String::from(""),
            width: 0,
            height: 0,
        }
    }
}