use crate::common::database::DbPool;
use crate::features::content::models;
use crate::schema::contents::episode_id;
use crate::schema::{albums, contents, episodes};
use diesel::dsl::count_star;
use diesel::prelude::*;
use diesel::{JoinOnDsl, OptionalExtension, QueryDsl, QueryResult, RunQueryDsl};

pub struct Repository;

impl Repository {
    pub async fn create_contents(
        pool: &DbPool,
        new_contents: Vec<models::Content>,
    ) -> QueryResult<usize> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        let rows_inserted = diesel::insert_into(contents::table)
            .values(&new_contents)
            .execute(&mut conn)
            .expect("Failed to create contents");
        Ok(rows_inserted)
    }

    pub async fn get_contents_by_episode_id(
        pool: &DbPool,
        episode_id_val: i32,
    ) -> QueryResult<Vec<models::Content>> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        contents::table
            .filter(contents::episode_id.eq(episode_id_val))
            .load::<models::Content>(&mut conn) // Loads all matching Content rows
    }

    pub async fn get_total_content_count_by_episode_id(
        pool: &DbPool,
        episode_id_val: i32,
    ) -> QueryResult<Option<i64>> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        let mut count_query = contents::table.into_boxed();
        count_query = count_query.filter(episode_id.eq(episode_id_val));

        count_query
            .select(count_star())
            .first::<i64>(&mut conn)
            .optional()
    }

    pub async fn get_album_uuid_by_episode_id(
        pool: &DbPool,
        episode_id_val: i32,
    ) -> QueryResult<Option<String>> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        episodes::table
            .inner_join(albums::table.on(episodes::album_id.eq(albums::id)))
            .select(albums::uuid)
            .filter(episodes::id.eq(episode_id_val))
            .first::<String>(&mut conn)
            .optional() // Returns `None` if no match is found
    }

    pub async fn get_content_by_uuid(
        pool: &DbPool,
        content_uuid: String,
    ) -> QueryResult<Option<models::Content>> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        contents::table
            .filter(contents::uuid.eq(content_uuid))
            .first::<models::Content>(&mut conn)
            .optional() // Returns `None` if no match is found
    }
    pub async fn get_episode_uuid_by_id(
        pool: &DbPool,
        episode_id_val: i32,
    ) -> QueryResult<Option<String>> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        episodes::table
            .select(episodes::uuid)
            .filter(episodes::id.eq(episode_id_val))
            .first::<String>(&mut conn)
            .optional() // Returns `None` if no episode is found with the given ID
    }

    pub async fn get_episode_id_by_uuid(
        pool: &DbPool,
        episode_uuid_val: String,
    ) -> QueryResult<Option<i32>> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        episodes::table
            .select(episodes::id)
            .filter(episodes::uuid.eq(episode_uuid_val))
            .first::<Option<i32>>(&mut conn) // Returns `None` if no episode is found with the given ID
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
