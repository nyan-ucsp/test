use crate::common::database::DbPool;
use crate::features::content::models;
use crate::schema::contents;
use diesel::RunQueryDsl;

pub struct Repository;

impl Repository {
    pub async fn create_contents(
        pool: &DbPool,
        new_contents: Vec<models::Content>,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        let rows_inserted = diesel::insert_into(contents::table)
            .values(&new_contents)
            .execute(&mut conn)
            .expect("Failed to create contents");
        Ok(rows_inserted)
    }


    pub async fn get_contents_by_episode_uuid(
        pool: &DbPool,
        new_contents: Vec<models::Content>,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        let rows_inserted = diesel::insert_into(contents::table)
            .values(&new_contents)
            .execute(&mut conn)
            .expect("Failed to create contents");
        Ok(rows_inserted)
    }


}
