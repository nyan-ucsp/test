use crate::common::ne_parse::NEParse;
use crate::common::utils::get_data_directory;
use crate::schema::{albums, episodes};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Queryable, Selectable, ToSchema, Clone, IntoParams, PartialEq, Eq)]
#[diesel(table_name = albums)]
pub struct EpisodeAlbum {
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

#[derive(
    Debug,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    ToSchema,
    Identifiable,
    AsChangeset,
    Insertable,
    Clone,
    IntoParams,
    PartialEq,
    Eq,
)]
#[diesel(table_name = episodes)]
pub struct Episode {
    pub id: Option<i32>,
    pub album_id: i32,
    pub uuid: String,
    pub title: String,
    pub url: Option<String>,
    pub broken_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Episode {
    pub fn from_create_request(req: CreateEpisodeRequest, album_uuid: String) -> Self {
        let episode_uuid = Uuid::new_v4().to_string();
        let episode_file_uuid = Uuid::new_v4().to_string();
        Episode {
            id: None,
            album_id: req.album_id,
            title: req.title,
            uuid: episode_uuid.clone(),
            url: if req.file.clone().is_none() {
                None
            } else {
                Some(format!(
                    "{}/{}/{}/{}.{}",
                    get_data_directory(),
                    album_uuid,
                    episode_uuid,
                    episode_file_uuid,
                    req.file.unwrap().split(".").last().unwrap()
                ))
            },
            broken_at: None,
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, IntoParams, PartialEq, Eq)]
pub struct EpisodeResponse {
    pub id: Option<i32>,
    pub album_id: i32,
    pub title: String,
    pub uuid: String,
    pub url: Option<String>,
    pub broken_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl EpisodeResponse {
    pub fn from_episode(ep: Episode) -> Self {
        EpisodeResponse {
            id: ep.id,
            album_id: ep.album_id,
            title: ep.title,
            uuid: ep.uuid,
            url: ep.url,
            broken_at: NEParse::opt_naive_datetime_to_utc_opt_string(ep.broken_at),
            created_at: NEParse::opt_naive_datetime_to_utc_opt_string(ep.created_at),
            updated_at: NEParse::opt_naive_datetime_to_utc_opt_string(ep.updated_at),
        }
    }

    pub fn from_episodes(eps: Vec<Episode>) -> Vec<Self> {
        eps.into_iter().map(|e| Self::from_episode(e)).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateEpisodeRequest {
    //Album ID
    #[schema(value_type = i32)]
    pub album_id: i32,
    #[schema(example = "Episode 1")]
    pub title: String,
    /// Episode File
    #[schema(value_type = Option<String>, format = Binary)]
    pub file: Option<String>,
}

impl CreateEpisodeRequest {
    pub async fn check_required_data(payload_data: HashMap<String, Value>) -> bool {
        (payload_data.contains_key("title") && payload_data.contains_key("album_id"))
            && (!payload_data["title"].as_str().is_none()
                && !NEParse::opt_immut_str_to_opt_i32(payload_data["album_id"].as_str()).is_none())
    }
    pub async fn from_payload_data(payload_data: HashMap<String, Value>) -> Self {
        let file_paths: Vec<String> = if payload_data.contains_key("file") {
            NEParse::opt_immut_vec_serde_json_value_to_vec_string(payload_data["file"].as_array())
        } else {
            vec![]
        };
        CreateEpisodeRequest {
            album_id: NEParse::opt_immut_str_to_opt_i32(payload_data["album_id"].as_str()).unwrap(),
            title: payload_data["title"].as_str().unwrap().to_string(),
            file: if file_paths.is_empty() {
                None
            } else {
                file_paths.first().cloned()
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateEpisodeRequest {
    #[schema(example = "Episode 1")]
    pub title: Option<String>,
    /// Episode File
    #[schema(value_type = Option<String>, format = Binary)]
    pub file: Option<String>,
}

impl UpdateEpisodeRequest {
    pub async fn from_payload_data(payload_data: HashMap<String, Value>) -> Self {
        let file_paths: Vec<String> = if payload_data.contains_key("file") {
            NEParse::opt_immut_vec_serde_json_value_to_vec_string(payload_data["file"].as_array())
        } else {
            vec![]
        };
        UpdateEpisodeRequest {
            title: NEParse::opt_immut_str_to_option_string(payload_data["title"].as_str()),
            file: if file_paths.is_empty() {
                None
            } else {
                file_paths.first().cloned()
            },
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FilterEpisodeRequest {
    #[schema(example = "")]
    pub title: String,
}
