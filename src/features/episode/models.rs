use std::collections::HashMap;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use crate::common::utils::{format_naive_as_utc_string, get_data_directory, parse_string_vec, try_parse_i32};
use crate::schema::episodes;

#[derive(
    Debug,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    Insertable,
    ToSchema,
    Identifiable,
    AsChangeset,
    Clone,
    IntoParams,
    PartialEq,
    Eq
)]
#[diesel(table_name = episodes)]
pub struct Episode {
    pub id: Option<i32>,
    pub album_id: i32,
    pub title: String,
    pub uuid: String,
    pub url: Option<String>,
    pub broken_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Episode {
    pub fn from_create_request(req: CreateEpisodeRequest) -> Self {
        let episode_uuid = Uuid::new_v4().to_string();
        let episode_file_uuid = Uuid::new_v4().to_string();
        Episode {
            id: None,
            album_id: req.album_id,
            title: req.title,
            uuid: episode_uuid.clone(),
            url: if req.file.clone().is_none() { None } else { Some(format!("{}/{}/{}.{}", get_data_directory(), episode_uuid, episode_file_uuid, req.file.unwrap().split(".").last().unwrap())) },
            broken_at: None,
            created_at: None,
            updated_at: None,
        }
    }
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
            broken_at: format_naive_as_utc_string(ep.broken_at),
            created_at: format_naive_as_utc_string(ep.created_at),
            updated_at: format_naive_as_utc_string(ep.created_at),
        }
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
            &&
            (
                !payload_data["title"].as_str().is_none()
                    && !try_parse_i32(payload_data["album_id"].as_str()).is_none()
            )
    }
    pub async fn from_payload_data(payload_data: HashMap<String, Value>) -> Self {
        let file_paths: Vec<String> = if payload_data.contains_key("file") { parse_string_vec(payload_data["file"].as_array()) } else { vec![] };
        CreateEpisodeRequest {
            album_id: try_parse_i32(payload_data["album_id"].as_str()).unwrap(),
            title: payload_data["title"].as_str().unwrap().to_string(),
            file: if file_paths.is_empty() { None } else { file_paths.first().cloned() },
        }
    }
}
