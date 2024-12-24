use crate::features::album::models::AlbumResponse;
use crate::features::episode::models::EpisodeResponse;
use crate::features::content::models::ContentResponse;
use crate::features::category::models::CategoryResponse;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[aliases(
    ResponseDataCategory = ResponseData<CategoryResponse>,
    ResponseDataAlbum = ResponseData<AlbumResponse>,
    ResponseDataEpisode = ResponseData<EpisodeResponse>,
    ResponseDataContent = ResponseData<ContentResponse>,
)]
pub struct ResponseData<T> {
    pub data: Vec<T>,
    pub total: i64,
}
