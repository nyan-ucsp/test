use std::collections::HashMap;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};
use crate::common::ne_parse::NEParse;
use crate::schema::contents;

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
    pub broken_at: Option<NaiveDateTime>,
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
            episode_id: NEParse::opt_immut_str_to_opt_i32(payload_data["episode_id"].as_str()).unwrap(),
            files: file_paths,
        }
    }
}
