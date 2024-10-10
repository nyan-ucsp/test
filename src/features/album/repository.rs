use diesel::dsl::count_star;
use diesel::prelude::*;

use crate::common::database::DbPool;
use crate::common::models::response_data::ResponseData;
use crate::features::album::models;
use crate::schema::albums;

pub struct Repository;

impl Repository {
    pub async fn create_album(
        pool: &DbPool,
        new_album: models::NewAlbum,
    ) -> Result<models::Album, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        diesel::insert_into(albums::table)
            .values(&new_album)
            .execute(&mut conn)
            .expect("Failed to create album");
        let album = albums::table.order(albums::id.desc()).first(&mut conn)?;
        Ok(album)
    }

    pub async fn get_albums(
        pool: &DbPool,
        filter_albums: models::GetAlbumRequest,
    ) -> Result<ResponseData<models::AlbumResponse>, diesel::result::Error> {
        use crate::schema::albums::*;
        let mut conn = pool.get().expect("Failed to get DB connection");
        let mut query = albums::table.into_boxed();
        let mut count_query = albums::table.into_boxed();

        if filter_albums.id.clone().unwrap_or(0) != 0 {
            let _id = filter_albums.id.unwrap();
            query = query.filter(id.eq(_id.clone()));
            count_query = count_query.filter(id.eq(_id));
        }

        if filter_albums.uuid.clone().unwrap_or(String::from("")) != "" {
            let _uuid = filter_albums.uuid.unwrap();
            query = query.filter(uuid.eq(_uuid.clone()));
            count_query = count_query.filter(uuid.eq(_uuid));
        }

        if filter_albums.title.clone().unwrap_or(String::from("")) != "" {
            let _title = filter_albums.title.unwrap();
            query = query.filter(title.like(format!("%{}%", _title.clone())));
            count_query = count_query.filter(title.like(format!("%{}%", _title)));
        }

        if filter_albums.completed.clone() != None {
            let _completed = filter_albums.completed.unwrap();
            query = query.filter(completed.eq(_completed.clone()));
            count_query = count_query.filter(completed.eq(_completed));
        }

        if filter_albums.tags.clone().unwrap_or(String::from("")) != "" {
            let _tag = filter_albums.tags.unwrap();
            query = query.filter(tags.like(format!("%{}%", _tag.clone())));
            count_query = count_query.filter(tags.like(format!("%{}%", _tag)));
        }

        if filter_albums.enable.clone() != None {
            let _enable = filter_albums.enable.unwrap();
            query = query.filter(enable.eq(_enable.clone()));
            count_query = count_query.filter(enable.eq(_enable));
        }

        if filter_albums.broken.clone() != None {
            let _broken = filter_albums.broken.unwrap();
            if _broken {
                query = query.filter(broken_at.is_not_null());
                count_query = count_query.filter(broken_at.is_not_null());
            } else {
                query = query.filter(broken_at.is_null());
                count_query = count_query.filter(broken_at.is_null());
            }
        }

        if filter_albums.min_age.clone().unwrap_or(0) != 0 {
            let _min_age = filter_albums.min_age.unwrap();
            query = query.filter(min_age.eq(_min_age.clone()));
            count_query = count_query.filter(min_age.eq(_min_age));
        }

        let total = count_query
            .select(count_star())
            .first::<i64>(&mut conn)
            .expect("failed to get total");

        let results = query
            .offset(filter_albums.offset.unwrap_or(0))
            .limit(filter_albums.limit.unwrap_or(20))
            .load::<models::Album>(&mut conn)
            .expect("error loading albums");

        let response_data = ResponseData::<models::AlbumResponse> {
            data: results
                .into_iter()
                .map(|r| models::AlbumResponse::from_album(r))
                .collect(),
            total,
        };
        Ok(response_data)
    }

    pub async fn get_album_by_uuid(
        pool: &DbPool,
        album_uuid: String,
    ) -> Result<models::Album, diesel::result::Error> {
        use crate::schema::albums::uuid;
        let mut conn = pool.get().expect("Failed to get DB connection");
        let mut query = albums::table.into_boxed();
        query = query.filter(uuid.eq(album_uuid));

        let result = query.get_result(&mut conn)?;
        Ok(result)
    }

    pub async fn update_album(
        pool: &DbPool,
        update_album: models::Album,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::albums::uuid;

        let mut conn = pool.get().expect("Failed to get DB connection");

        let result = diesel::update(albums::table.filter(uuid.eq(update_album.uuid.clone())))
            .set(&update_album)
            .execute(&mut conn)?;
        Ok(result)
    }

    pub async fn delete_album(
        pool: &DbPool,
        album_uuid: String,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::albums::dsl::*;
        let mut conn = pool.get().expect("Failed to get DB connection");

        let deleted = diesel::delete(albums.filter(uuid.eq(album_uuid))).execute(&mut conn)?;
        Ok(deleted)
    }
}
