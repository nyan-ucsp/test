use std::collections::HashMap;

use chrono::Utc;
use diesel::prelude::*;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::common::models::file_metadata::ImageMetadata;
use crate::common::utils::{get_data_directory, get_file_metadata, parse_option_date_time, parse_string_vec};
use crate::schema::albums;

#[derive(
    Debug,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    ToSchema,
    AsChangeset,
    Clone,
    IntoParams,
    PartialEq,
    Eq
)]
#[diesel(table_name = albums)]
pub struct Album {
    pub id: i32,
    pub uuid: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub covers: String,
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

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    ToSchema,
    IntoParams,
    PartialEq,
    Eq
)]
pub struct AlbumResponse {
    pub id: i32,
    pub uuid: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub covers: Vec<String>,
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
            covers: album.covers
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
            released_at: album.released_at,
            broken_at: album.broken_at,
            created_at: album.created_at,
            updated_at: album.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateAlbumRequest {
    #[schema(example = "Title")]
    pub title: String,
    #[schema(example = "Description")]
    pub description: String,
    /// Album Cover **Accept Only JPG**
    #[schema(value_type = String, format = Binary)]
    pub image: String,
    /// Album Description Images **Accept Only JPG**
    #[schema(value_type = Option < Vec < String >>, format = Binary)]
    pub covers: Option<Vec<String>>,
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
    /// Start released date of your album (2024-07-27T15:50:50.993251+00:00)
    #[schema(example = "2024-07-27T15:50:50.993251+00:00")]
    pub released_at: Option<String>,
}

impl CreateAlbumRequest {
    pub async fn from_payload_data(payload_data: HashMap<String, Value>) -> Self {
        let image_paths: Vec<String> = parse_string_vec(payload_data["image"].as_array());
        let mut cover_paths: Vec<String> = vec![];
        if !payload_data["covers"].is_null() {
            cover_paths = parse_string_vec(Some(payload_data["covers"].as_array().unwrap_or(&Vec::new())));
        }

        CreateAlbumRequest {
            title: payload_data["title"].as_str().unwrap().to_string(),
            description: payload_data["description"].as_str().unwrap().to_string(),
            image: image_paths.first().unwrap().to_string(),
            covers: Option::from(cover_paths),
            completed: payload_data["completed"].as_bool(),
            tags: payload_data["tags"].as_str().map(|value| value.to_string()),
            enable: payload_data["enable"].as_bool(),
            min_age: payload_data["min_age"].as_i64().map(|value| value.try_into().unwrap()),
            released_at: parse_option_date_time(payload_data["released_at"].as_str()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateAlbumRequest {
    #[schema(example = "Title")]
    pub title: String,
    #[schema(example = "Description")]
    pub description: String,
    /// Album Default Cover **Accept Only JPG**
    #[schema(value_type = Option < String >, format = Binary)]
    pub image: Option<String>,
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
    /// Start released date of your album (2024-07-27T15:50:50.993251+00:00)
    #[schema(example = "2024-07-27T15:50:50.993251+00:00")]
    pub released_at: Option<String>,
    /// Set empty if your album fix
    #[schema(example = "")]
    pub broken_at: Option<String>,
}

impl UpdateAlbumRequest {
    pub async fn from_payload_data(payload_data: HashMap<String, Value>) -> Self {
        let mut image_paths: Vec<String> = vec![];
        if !payload_data["image"].is_null() {
            image_paths = payload_data["image"].as_array().unwrap_or(&Vec::new()).into_iter().map(|s| s.as_str().unwrap().to_string()).collect();
        }

        UpdateAlbumRequest {
            title: payload_data["title"].as_str().unwrap().to_string(),
            description: payload_data["description"].as_str().unwrap().to_string(),
            image: Some(image_paths.first().unwrap().to_string()),
            completed: payload_data["completed"].as_bool(),
            tags: payload_data["tags"].as_str().map(|value| value.to_string()),
            enable: payload_data["enable"].as_bool(),
            min_age: payload_data["min_age"].as_i64().map(|value| value.try_into().unwrap()),
            released_at: parse_option_date_time(payload_data["released_at"].as_str()),
            broken_at: parse_option_date_time(payload_data["released_at"].as_str()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddAlbumCoverRequest {
    /// Album Covers **Accept Only JPG**
    #[schema(value_type = Vec < String >, format = Binary)]
    pub covers: Vec<String>,
}


impl AddAlbumCoverRequest {
    pub async fn from_payload_data(payload_data: HashMap<String, Value>) -> Self {
        let mut cover_paths = parse_string_vec(payload_data["covers"].as_array());

        AddAlbumCoverRequest {
            covers: cover_paths
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
    pub covers: String,
    pub tags: Option<String>,
    pub enable: bool,
    pub min_age: i32,
    pub url: String,
    pub content_type: String,
    pub width: i32,
    pub height: i32,
    pub bytes: i32,
    pub released_at: Option<String>,
    pub created_at: Option<String>,
}

impl NewAlbum {
    pub fn from_request(req: CreateAlbumRequest) -> Self {
        let album_uuid = Uuid::new_v4().to_string();
        let image_uuid = Uuid::new_v4().to_string();
        let file_metadata = get_file_metadata(&req.image);
        let format = file_metadata.original_name.split(".").last().unwrap();
        let url = format!("{}/{}/{}.{}", get_data_directory(), album_uuid, image_uuid, format);
        let covers: Vec<String> = req.covers.unwrap_or(vec![]).into_iter().map(|s| format!("{}/{}/{}.{}", get_data_directory(), album_uuid, Uuid::new_v4().to_string(), s.split(".").last().unwrap())).collect();

        NewAlbum {
            uuid: album_uuid,
            title: req.title,
            description: req.description,
            completed: req.completed.unwrap_or(false),
            covers: covers.join(","),
            tags: req.tags,
            enable: req.enable.unwrap_or(true),
            min_age: req.min_age.unwrap_or(0),
            url,
            content_type: file_metadata.content_type,
            width: file_metadata.image_data.clone().unwrap_or(ImageMetadata::default()).width as i32,
            height: file_metadata.image_data.unwrap_or(ImageMetadata::default()).height as i32,
            bytes: file_metadata.size as i32,
            released_at: req.released_at,
            created_at: Option::from(Utc::now().to_rfc3339()),
        }
    }
}