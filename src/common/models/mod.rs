use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod file_metadata;
pub mod response_data;
pub mod response_message;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TmpFile {
    pub path: String,
}
