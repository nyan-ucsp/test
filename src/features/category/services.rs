use crate::common::database::DbPool;
use crate::common::models::file_metadata::ImageMetadata;
use crate::common::models::response_data::ResponseData;
use crate::common::utils::{
    delete_file_if_exists, get_data_directory, get_file_metadata, get_project_directory,
    move_file_and_replace,
};
use crate::features::category::models;
use crate::features::category::repository::Repository;
use chrono::Utc;
use uuid::Uuid;

pub struct Service;

impl Service {
    pub async fn add_category(
        pool: &DbPool,
        req: models::AddCategoryRequest,
    ) -> Result<&str, &str> {
        match Repository::create_category(pool, models::Category{id: None, name: req.name }).await {
            Ok(size) => if size>0 { Ok("Successfully added") }else { Err("Failed to add category") }
            Err(_) => Err("Failed to add category"),
        }
    }

    pub async fn get_categories(
        pool: &DbPool,
    ) -> Result<ResponseData<models::CategoryResponse>, diesel::result::Error> {
        match Repository::get_categories(pool).await {
            Ok(categories) => {
                Ok(ResponseData::<models::CategoryResponse> {
                    data: models::CategoryResponse::from_categories(categories.clone()),
                    total: categories.len() as i64,
                })
            }
            Err(e) => Err(e),
        }
    }

    pub async fn update_category(
        pool: &DbPool,
        update_category: models::UpdateCategoryRequest,
    ) -> Result<models::CategoryResponse, diesel::result::Error> {
        match Repository::update_category(pool, models::Category{ id: Some(update_category.id), name: update_category.name }).await {
            Ok(category) => Ok(category),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_category(
        pool: &DbPool,
        category_id: i32,
    ) -> Result<&str, diesel::result::Error> {
        match Repository::delete_category(pool, category_id).await {
            Ok(size) => {
                if size>0 {
                   Ok("Successfully deleted")
                } else {
                    Err(diesel::result::Error::NotFound)
                }
            }
            Err(e) => Err(e),
        }
    }
}
