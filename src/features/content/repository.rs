use crate::common::database::DbPool;
use diesel::prelude::*;
use crate::features::content::models;
use crate::schema::{albums, contents, episodes};
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

    pub async fn get_contents_by_episode_uuid(
        pool: &DbPool,
        uuid: String,
    ) -> QueryResult<Vec<models::Content>> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        contents::table
            .filter(contents::uuid.eq(uuid))
            .load::<models::Content>(&mut conn) // Loads all matching Content rows
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
}
