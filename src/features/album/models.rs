use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::common::models::file_metadata::ImageMetadata;
use crate::common::ne_parse::NEParse;
use crate::common::utils::{get_data_directory, get_file_metadata};
use crate::schema::albums;

#[derive(
    Debug,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    ToSchema,
    Identifiable,
    AsChangeset,
    Clone,
    IntoParams,
    PartialEq,
    Eq,
)]
#[diesel(table_name = albums)]
pub struct Album {
    pub id: i32,
    pub uuid: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub images: String,
    pub tags: Option<String>,
    pub enable: bool,
    pub min_age: i32,
    pub url: String,
    pub content_type: String,
    pub width: i32,
    pub height: i32,
    pub bytes: i32,
    pub released_at: Option<NaiveDateTime>,
    pub broken_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, IntoParams, PartialEq, Eq)]
pub struct AlbumResponse {
    pub id: i32,
    pub uuid: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub images: Vec<String>,
    pub tags: Option<String>,
    pub enable: bool,
    pub min_age: i32,
    pub url: String,
    pub content_type: String,
    pub width: i32,
    pub height: i32,
    pub bytes: i32,
    pub released_at: Option<String>,
    pub broken_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl AlbumResponse {
    pub fn from_album(album: Album) -> Self {
        AlbumResponse {
            id: album.id,
            uuid: album.uuid,
            title: album.title,
            description: album.description,
            completed: album.completed,
            images: album
                .images
                .split(",")
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect::<Vec<String>>(),
            tags: album.tags,
            enable: album.enable,
            min_age: album.min_age,
            url: album.url,
            content_type: album.content_type,
            width: album.width,
            height: album.height,
            bytes: album.bytes,
            released_at: NEParse::opt_naive_datetime_to_utc_opt_string(album.released_at),
            broken_at: NEParse::opt_naive_datetime_to_utc_opt_string(album.broken_at),
            created_at: NEParse::opt_naive_datetime_to_utc_opt_string(album.created_at),
            updated_at: NEParse::opt_naive_datetime_to_utc_opt_string(album.updated_at),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateAlbumRequest {
    #[schema(example = "Title")]
    pub title: String,
    #[schema(example = "Description")]
    pub description: String,
    /// Album Cover
    #[schema(value_type = String, format = Binary)]
    pub cover: String,
    /// Album Description Images
    #[schema(value_type = Option < Vec < String >>, format = Binary)]
    pub images: Option<Vec<String>>,
    /// Completed or End album default is false
    #[schema(example = false)]
    pub completed: Option<bool>,
    /// Tags or your album #niceanimation
    #[schema(example = "#niceanime")]
    pub tags: Option<String>,
    /// User visible your album default is true
    #[schema(example = true)]
    pub enable: Option<bool>,
    /// Minimum age of your album default is 0
    #[schema(example = 0)]
    pub min_age: Option<i32>,
    /// Start released date of your album
    pub released_at: Option<NaiveDateTime>,
}

impl CreateAlbumRequest {
    pub async fn from_payload_data(payload_data: HashMap<String, Value>) -> Self {
        let image_paths: Vec<String> = if payload_data.contains_key("cover") {
            NEParse::opt_immut_vec_serde_json_value_to_vec_string(payload_data["cover"].as_array())
        } else {
            vec![]
        };
        let mut cover_paths: Vec<String> = vec![];
        if payload_data.contains_key("images") && !payload_data["images"].is_null() {
            cover_paths = NEParse::opt_immut_vec_serde_json_value_to_vec_string(Some(
                payload_data["images"].as_array().unwrap_or(&Vec::new()),
            ));
        }
        CreateAlbumRequest {
            title: payload_data["title"].as_str().unwrap().to_string(),
            description: payload_data["description"].as_str().unwrap().to_string(),
            cover: image_paths.first().unwrap().to_string(),
            images: Option::from(cover_paths),
            completed: if payload_data.contains_key("completed") {
                NEParse::opt_immut_str_to_option_bool(payload_data["completed"].as_str())
            } else {
                Some(false)
            },
            tags: if payload_data.contains_key("tags") {
                payload_data["tags"].as_str().map(|value| value.to_string())
            } else {
                Some(String::from(""))
            },
            enable: if payload_data.contains_key("enable") {
                NEParse::opt_immut_str_to_option_bool(payload_data["enable"].as_str())
            } else {
                Some(true)
            },
            min_age: if payload_data.contains_key("min_age") {
                NEParse::opt_immut_str_to_opt_i32(payload_data["min_age"].as_str())
            } else {
                Some(0)
            },
            released_at: if payload_data.contains_key("released_at") {
                NEParse::opt_immut_str_to_opt_naive_datetime(payload_data["released_at"].as_str())
            } else {
                None
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateAlbumRequest {
    #[schema(example = "Title")]
    pub title: String,
    #[schema(example = "Description")]
    pub description: String,
    /// Album Default Cover
    #[schema(value_type = Option < String >, format = Binary)]
    pub cover: Option<String>,
    /// Completed or End album default is false
    #[schema(example = false)]
    pub completed: Option<bool>,
    /// Tags or your album #niceanimation
    #[schema(example = "#niceanime")]
    pub tags: Option<String>,
    /// User visible your album default is true
    #[schema(example = true)]
    pub enable: Option<bool>,
    /// Minimum age of your album default is 0
    #[schema(example = 0)]
    pub min_age: Option<i32>,
    /// Start released date of your album
    pub released_at: Option<NaiveDateTime>,
    /// Set empty if your album fix
    pub broken_at: Option<NaiveDateTime>,
}

impl UpdateAlbumRequest {
    pub async fn from_payload_data(payload_data: HashMap<String, Value>) -> Self {
        let mut image_paths: Vec<String> = vec![];
        if !payload_data["cover"].is_null() {
            image_paths = payload_data["cover"]
                .as_array()
                .unwrap_or(&Vec::new())
                .into_iter()
                .map(|s| s.as_str().unwrap().to_string())
                .collect();
        }

        UpdateAlbumRequest {
            title: payload_data["title"].as_str().unwrap().to_string(),
            description: payload_data["description"].as_str().unwrap().to_string(),
            cover: if image_paths.is_empty() {
                None
            } else {
                Some(image_paths.first().unwrap().to_string())
            },
            completed: NEParse::opt_immut_str_to_option_bool(payload_data["completed"].as_str()),
            tags: payload_data["tags"].as_str().map(|value| value.to_string()),
            enable: NEParse::opt_immut_str_to_option_bool(payload_data["enable"].as_str()),
            min_age: NEParse::opt_immut_str_to_opt_i32(payload_data["min_age"].as_str()),
            released_at: NEParse::opt_immut_str_to_opt_naive_datetime(
                payload_data["released_at"].as_str(),
            ),
            broken_at: NEParse::opt_immut_str_to_opt_naive_datetime(
                payload_data["broken_at"].as_str(),
            ),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddAlbumImagesRequest {
    /// Album Images
    #[schema(value_type = Vec < String >, format = Binary)]
    pub images: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RemoveAlbumImagesRequest {
    /// Album Images
    pub images: Vec<String>,
}

impl AddAlbumImagesRequest {
    pub async fn from_payload_data(payload_data: HashMap<String, Value>) -> Self {
        let cover_paths = if payload_data.contains_key("images") {
            NEParse::opt_immut_vec_serde_json_value_to_vec_string(payload_data["images"].as_array())
        } else {
            vec![]
        };

        AddAlbumImagesRequest {
            images: cover_paths,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetAlbumRequest {
    /// Album ID (Exact Value)
    #[schema(example = 0)]
    pub id: Option<i32>,
    ///Filter album UUID (Exact Value)
    #[schema(example = "")]
    pub uuid: Option<String>,
    /// Filter album title
    #[schema(example = "")]
    pub title: Option<String>,
    /// Filter completed or end
    #[schema(example = "null")]
    pub completed: Option<bool>,
    /// Filter tags
    #[schema(example = "")]
    pub tags: Option<String>,
    /// Filter enable
    #[schema(example = "null")]
    pub enable: Option<bool>,
    #[schema(example = "null")]
    pub broken: Option<bool>,
    /// Filter minimum age
    #[schema(example = 0)]
    pub min_age: Option<i32>,
    /// Filter offset
    #[schema(example = 0)]
    pub offset: Option<i64>,
    /// Filter limit (Get all for 0)
    #[schema(example = 20)]
    pub limit: Option<i64>,
}

#[derive(Debug, Insertable, ToSchema, Deserialize)]
#[diesel(table_name = albums)]
pub struct NewAlbum {
    pub uuid: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub images: String,
    pub tags: Option<String>,
    pub enable: bool,
    pub min_age: i32,
    pub url: String,
    pub content_type: String,
    pub width: i32,
    pub height: i32,
    pub bytes: i32,
    pub released_at: Option<NaiveDateTime>,
}

impl NewAlbum {
    pub fn from_request(req: CreateAlbumRequest) -> Self {
        let album_uuid = Uuid::new_v4().to_string();
        let image_uuid = Uuid::new_v4().to_string();
        let file_metadata = get_file_metadata(&req.cover);
        let format = file_metadata.original_name.split(".").last().unwrap();
        let url = format!(
            "{}/{}/{}.{}",
            get_data_directory(),
            album_uuid,
            image_uuid,
            format
        );
        let images: Vec<String> = req
            .images
            .unwrap_or(vec![])
            .into_iter()
            .map(|s| {
                format!(
                    "{}/{}/{}.{}",
                    get_data_directory(),
                    album_uuid,
                    Uuid::new_v4().to_string(),
                    s.split(".").last().unwrap()
                )
            })
            .collect();

        NewAlbum {
            uuid: album_uuid,
            title: req.title,
            description: req.description,
            completed: req.completed.unwrap_or(false),
            images: images.join(","),
            tags: req.tags,
            enable: req.enable.unwrap_or(true),
            min_age: req.min_age.unwrap_or(0),
            url,
            content_type: file_metadata.content_type,
            width: file_metadata
                .image_data
                .clone()
                .unwrap_or(ImageMetadata::default())
                .width as i32,
            height: file_metadata
                .image_data
                .unwrap_or(ImageMetadata::default())
                .height as i32,
            bytes: file_metadata.size as i32,
            released_at: req.released_at,
        }
    }
}
