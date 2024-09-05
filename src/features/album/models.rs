use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::common::models::file_metadata::ImageMetadata;
use crate::common::utils::{get_data_directory, get_file_metadata};
use crate::schema::albums;

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize, ToSchema, AsChangeset, Clone, IntoParams,  PartialEq, Eq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateAlbumRequest {
    #[schema(example = "Title")]
    pub title: String,
    #[schema(example = "Description")]
    pub description: String,
    /// Album Cover **Accept Only JPG**
    #[schema(value_type = String, format = Binary)]
    pub image: String,
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateAlbumRequest {
    #[schema(example = "Title")]
    pub title: String,
    #[schema(example = "Description")]
    pub description: String,
    /// Album Cover **Accept Only JPG**
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

        NewAlbum {
            uuid: album_uuid,
            title: req.title,
            description: req.description,
            completed: req.completed.unwrap_or(false),
            covers: String::from(""),
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
