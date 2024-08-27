use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod response_message;
pub mod file_metadata;
pub mod response_data;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TmpFile {
    pub path: String,
}
