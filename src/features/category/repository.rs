use crate::common::database::DbPool;
use crate::features::category::models;
use diesel::dsl::{count_star, max};
use diesel::prelude::*;
use diesel::{JoinOnDsl, OptionalExtension, QueryDsl, QueryResult, RunQueryDsl};
use crate::schema::category;
use crate::schema::category::id;

pub struct Repository;

impl Repository {
    pub async fn create_category(
        pool: &DbPool,
        new_category: Vec<models::Category>,
    ) -> QueryResult<usize> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        let rows_inserted = diesel::insert_into(category::table)
            .values(&new_category)
            .execute(&mut conn)
            .expect("Failed to create category");
        Ok(rows_inserted)
    }

    pub async fn get_categories(
        pool: &DbPool
    ) -> QueryResult<Vec<models::Category>> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        category::table
            .load::<models::Category>(&mut conn) // Loads all matching Content rows
    }

    pub async fn update_category(
        pool: &DbPool,
        update_category: models::Category,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        let result = diesel::update(category::table.filter(id.eq(update_category.id.clone())))
            .set(&update_category)
            .execute(&mut conn)?;
        Ok(result)
    }

    pub async fn delete_content(
        pool: &DbPool,
        content_uuid: String,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::contents::dsl::*;
        let mut conn = pool.get().expect("Failed to get DB connection");
        let deleted = diesel::delete(contents.filter(uuid.eq(content_uuid))).execute(&mut conn)?;
        Ok(deleted)
    }
}
