use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::common::database::DbPool;
use crate::features::episode::models;
use crate::schema::episodes;

pub struct Repository;

impl Repository {
    pub async fn create_episode(pool: &DbPool, new_episode: models::Episode) -> Result<models::Episode, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        diesel::insert_into(episodes::table)
            .values(&new_episode)
            .execute(&mut conn).expect("Failed to create episode");
        let last_episode: models::Episode = episodes::table.order(episodes::id.desc()).first(&mut conn)?;
        Ok(last_episode)
    }
}