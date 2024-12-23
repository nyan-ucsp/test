use crate::common::ne_parse::NEParse;
use crate::schema::contents;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};

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
#[diesel(table_name = contents)]
pub struct Content {
    pub id: Option<i32>,
    pub episode_id: i32,
    pub uuid: String,
    pub index_no: i32,
    pub url: String,
    pub ads_url: Option<String>,
    pub content_type: String,
    pub width: i32,
    pub height: i32,
    pub bytes: i32,
    pub broken_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, IntoParams, PartialEq, Eq)]
pub struct ContentResponse {
    pub id: Option<i32>,
    pub episode_id: i32,
    pub uuid: String,
    pub index_no: i32,
    pub url: String,
    pub ads_url: Option<String>,
    pub content_type: String,
    pub width: i32,
    pub height: i32,
    pub bytes: i32,
    pub broken_at: Option<String>,
}

impl ContentResponse {
    pub fn from_content(content: Content) -> Self {
        ContentResponse {
            id: content.id,
            episode_id: content.episode_id,
            uuid: content.uuid,
            index_no: content.index_no,
            url: content.url,
            ads_url: content.ads_url,
            content_type: content.content_type,
            width: content.width,
            height: content.height,
            bytes: content.bytes,
            broken_at: NEParse::opt_naive_datetime_to_utc_opt_string(content.broken_at),
        }
    }
    pub fn from_contents(contents: Vec<Content>) -> Vec<Self> {
        contents
            .into_iter()
            .map(|e| Self::from_content(e))
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddEpisodeContentsRequest {
    pub episode_id: i32,
    /// Episode Content Files
    #[schema(value_type = Vec < String >, format = Binary)]
    pub files: Vec<String>,
}

impl AddEpisodeContentsRequest {
    pub async fn from_payload_data<'a>(
        payload_data: HashMap<String, Value>,
    ) -> Result<Self, &'a str> {
        let file_paths = if payload_data.contains_key("files") {
            NEParse::opt_immut_vec_serde_json_value_to_vec_string(payload_data["files"].as_array())
        } else {
            vec![]
        };
        if file_paths.is_empty() {
            Err("Files cannot be empty")
        } else {
            Ok(AddEpisodeContentsRequest {
                episode_id: NEParse::opt_immut_str_to_opt_i32(payload_data["episode_id"].as_str())
                    .expect("Invalid episode id"),
                files: file_paths,
            })
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateContentRequest {
    /// Episode Content Files
    #[schema(value_type = Option < String >, format = Binary)]
    pub file: Option<String>,
    pub ads_url: Option<String>,
    pub index_no: Option<i32>,
}

impl UpdateContentRequest {
    pub async fn from_payload_data<'a>(
        payload_data: HashMap<String, Value>,
    ) -> Result<Self, &'a str> {
        let file_paths: Vec<String> = if payload_data.contains_key("file") {
            NEParse::opt_immut_vec_serde_json_value_to_vec_string(payload_data["file"].as_array())
        } else {
            vec![]
        };
        let ads_url = if payload_data.contains_key("ads_url") {
            NEParse::opt_immut_str_to_option_string(payload_data["ads_url"].as_str())
        } else {
            None
        };
        let index_no = if payload_data.contains_key("ads_url") {
            NEParse::opt_immut_str_to_opt_i32(payload_data["index_no"].as_str())
        } else {
            None
        };
        Ok(UpdateContentRequest {
            file: if file_paths.is_empty() { None } else { file_paths.first().cloned() },
            ads_url,
            index_no,
        })
    }
}
