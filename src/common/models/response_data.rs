use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::features::album::models::Album;

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[aliases(
    ResponseDataAlbum = ResponseData<Album>
)]
pub struct ResponseData<T> {
    pub data: Vec<T>,
    pub total: i64,
}
