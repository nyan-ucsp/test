use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResponseData<T> {
    pub data: Vec<T>,
    pub total: i64,
}
