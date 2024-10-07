use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::features::album::models::AlbumResponse;
use crate::features::episode::models::EpisodeResponse;

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[aliases(
    ResponseDataAlbum = ResponseData<AlbumResponse>,
    ResponseDataEpisode = ResponseData<EpisodeResponse>
)]
pub struct ResponseData<T> {
    pub data: Vec<T>,
    pub total: i64,
}
