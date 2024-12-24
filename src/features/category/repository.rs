use crate::common::database::DbPool;
use crate::features::category::models;
use crate::schema::category;
use diesel::prelude::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};
use crate::schema::category::id;

pub struct Repository;

impl Repository {
    pub async fn create_category(
        pool: &DbPool,
        new_category: models::Category,
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
    ) -> Result<models::CategoryResponse, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        let result = diesel::update(category::table.filter(id.eq(update_category.id.clone())))
            .set(&update_category)
            .execute(&mut conn)?;
        Ok(models::CategoryResponse{ id: update_category.id, name: update_category.name })
    }

    pub async fn delete_category(
        pool: &DbPool,
        category_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::category::dsl::*;
        let mut conn = pool.get().expect("Failed to get DB connection");
        let deleted = diesel::delete(category.filter(id.eq(category_id))).execute(&mut conn)?;
        Ok(deleted)
    }
}
