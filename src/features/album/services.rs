use chrono::Utc;
use uuid::Uuid;

use crate::common::database::DbPool;
use crate::common::models::file_metadata::ImageMetadata;
use crate::common::models::response_data::ResponseData;
use crate::common::utils::{delete_directory_if_exists, delete_file_if_exists, get_data_directory, get_file_metadata, get_project_directory, move_file_and_replace};
use crate::features::album::models;
use crate::features::album::repository::Repository;

pub struct Service;

impl Service {
    pub async fn create_album(
        pool: &DbPool,
        req_data: models::CreateAlbumRequest,
    ) -> Result<models::Album, diesel::result::Error> {
        let src_file_path = req_data.image.clone();
        let new_album = models::NewAlbum::from_request(req_data);
        let des_file_path = format!("{}/{}", get_project_directory(), new_album.url.clone());
        match Repository::create_album(pool, new_album).await {
            Ok(album) => {
                move_file_and_replace(&*src_file_path, &*des_file_path);
                Ok(album)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_albums(
        pool: &DbPool,
        filter_albums: models::GetAlbumRequest,
    ) -> Result<ResponseData<models::Album>, diesel::result::Error> {
        Repository::get_albums(pool, filter_albums).await
    }

    pub async fn update_album(
        pool: &DbPool,
        album_uuid: String,
        update_album: models::UpdateAlbumRequest,
    ) -> Result<models::Album, diesel::result::Error> {
        let album = Repository::get_album_by_uuid(pool, album_uuid).await.unwrap();
        let old_url = album.url.clone();
        let mut new_album = models::Album {
            id: album.id,
            uuid: album.uuid.clone(),
            title: update_album.title,
            description: update_album.description,
            completed: update_album.completed.unwrap_or(album.completed),
            tags: update_album.tags,
            enable: update_album.enable.unwrap_or(album.enable),
            min_age: update_album.min_age.unwrap_or(album.min_age),
            url: album.url,
            content_type: "".to_string(),
            width: album.width,
            height: album.height,
            bytes: album.bytes,
            released_at: update_album.released_at,
            broken_at: update_album.broken_at,
            created_at: album.created_at,
            updated_at: album.updated_at,
        };
        if let Some(src_path) = update_album.image.clone() {
            let file_meta_data = get_file_metadata(&src_path.clone());
            let key = Uuid::new_v4().to_string();
            let format = file_meta_data.original_name.split(".").last().unwrap();
            let new_url = format!("{}/{}/{}.{}", get_data_directory(), album.uuid, key, format);
            new_album.url = new_url;
            new_album.width = file_meta_data.image_data.clone().unwrap_or(ImageMetadata::default()).width as i32;
            new_album.height = file_meta_data.image_data.unwrap_or(ImageMetadata::default()).height as i32;
            new_album.bytes = file_meta_data.size as i32;
            new_album.updated_at = Option::from(Utc::now().to_rfc3339());
        }
        match Repository::update_album(pool, new_album.clone()).await {
            Ok(size) if size > 0 =>
                {
                    if new_album.url != old_url {
                        let old_file_path = format!("{}/{}", get_project_directory(), old_url);
                        //Delete old file
                        delete_file_if_exists(&old_file_path);
                        //Move new data
                        let src_file_path = update_album.image.unwrap();
                        let des_file_path = format!("{}/{}", get_project_directory(), new_album.url);
                        move_file_and_replace(&src_file_path, &des_file_path);
                    }
                    Ok(new_album)
                }
            Ok(_) => {
                //Delete tmp data
                Err(diesel::result::Error::NotFound)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn delete_album(
        pool: &DbPool,
        album_uuid: String,
    ) -> Result<usize, diesel::result::Error> {
        let filepath = format!("{}/{}", get_data_directory(), album_uuid, );
        delete_directory_if_exists(&filepath);
        Repository::delete_album(pool, album_uuid).await
    }
}
