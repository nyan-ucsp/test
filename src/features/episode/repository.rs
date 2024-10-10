use diesel::prelude::*;

use crate::common::database::DbPool;
use crate::features::episode::models;
use crate::schema::{albums, episodes};

pub struct Repository;

impl Repository {
    pub async fn create_episode(
        pool: &DbPool,
        new_episode: models::Episode,
    ) -> Result<models::Episode, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        diesel::insert_into(episodes::table)
            .values(&new_episode)
            .execute(&mut conn)
            .expect("Failed to create episode");
        let episode = episodes::table
            .order(episodes::id.desc())
            .first(&mut conn)?;
        Ok(episode)
    }

    pub async fn get_episodes_by_album_id(
        pool: &DbPool,
        id: i32,
        filters: models::FilterEpisodeRequest,
    ) -> Result<Vec<models::Episode>, diesel::result::Error> {
        use crate::schema::episodes::album_id;
        use crate::schema::episodes::title;
        let mut conn = pool.get().expect("Failed to get DB connection");
        let mut query = episodes::table.into_boxed();
        query = query
            .filter(album_id.eq(id))
            .filter(title.like(format!("%{}%", filters.title)));
        let result = query.get_results(&mut conn)?;
        Ok(result)
    }

    pub async fn get_album_uuid_by_episode_uuid(
        pool: &DbPool,
        episode_uuid: String,
    ) -> Result<String, diesel::result::Error> {
        use crate::schema::episodes::uuid;
        let mut conn = pool.get().expect("Failed to get DB connection");
        let result = albums::table
            .inner_join(episodes::table.on(episodes::album_id.eq(albums::id)))
            .select(albums::uuid)
            .filter(uuid.eq(episode_uuid))
            .first(&mut conn)?;
        Ok(result)
    }

    pub async fn get_episode_by_episode_uuid(
        pool: &DbPool,
        episode_uuid: String,
    ) -> Result<models::Episode, diesel::result::Error> {
        use crate::schema::episodes::uuid;
        let mut conn = pool.get().expect("Failed to get DB connection");
        let mut query = episodes::table.into_boxed();
        query = query.filter(uuid.eq(episode_uuid));
        let result = query.get_result(&mut conn)?;
        Ok(result)
    }

    pub async fn get_album_by_id(
        pool: &DbPool,
        album_id: i32,
    ) -> Result<models::EpisodeAlbum, diesel::result::Error> {
        use crate::schema::albums::id;
        let mut conn = pool.get().expect("Failed to get DB connection");
        let mut query = albums::table.into_boxed();
        query = query.filter(id.eq(album_id));
        let result = query.get_result(&mut conn)?;
        Ok(result)
    }

    pub async fn update_episode(
        pool: &DbPool,
        update_episode: models::Episode,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::episodes::uuid;

        let mut conn = pool.get().expect("Failed to get DB connection");

        let result = diesel::update(episodes::table.filter(uuid.eq(update_episode.uuid.clone())))
            .set(&update_episode)
            .execute(&mut conn)?;
        Ok(result)
    }

    pub async fn delete_episode(
        pool: &DbPool,
        episode_uuid: String,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::episodes::dsl::*;
        let mut conn = pool.get().expect("Failed to get DB connection");
        let deleted = diesel::delete(episodes.filter(uuid.eq(episode_uuid))).execute(&mut conn)?;
        Ok(deleted)
    }
}
