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
    pub async fn from_payload_data(payload_data: HashMap<String, Value>) -> Self {
        let file_paths = if payload_data.contains_key("images") {
            NEParse::opt_immut_vec_serde_json_value_to_vec_string(payload_data["images"].as_array())
        } else {
            vec![]
        };

        AddEpisodeContentsRequest {
            episode_id: NEParse::opt_immut_str_to_opt_i32(payload_data["episode_id"].as_str())
                .unwrap(),
            files: file_paths,
        }
    }
}
