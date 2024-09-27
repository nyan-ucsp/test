use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::features::album::models::AlbumResponse;

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[aliases(
    ResponseDataAlbum = ResponseData<AlbumResponse>
)]
pub struct ResponseData<T> {
    pub data: Vec<T>,
    pub total: i64,
}
