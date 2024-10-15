use crate::common::database::DbPool;
use crate::common::models::file_metadata::ImageMetadata;
use crate::common::models::response_data::ResponseData;
use crate::common::utils::{
    delete_directory_if_exists, delete_file_if_exists, get_data_directory, get_file_metadata,
    get_project_directory, move_file_and_replace,
};
use crate::features::episode::models;
use crate::features::episode::repository::*;
use chrono::Utc;
use uuid::Uuid;

pub struct Service;
impl Service {
    pub async fn create_episode(
        pool: &DbPool,
        req_data: models::CreateEpisodeRequest,
    ) -> Result<models::EpisodeResponse, &str> {
        let data = req_data.clone().file;
        match Repository::get_album_by_id(pool, req_data.clone().album_id).await {
            Ok(album) => {
                let album_uuid = album.uuid;
                let mut new_episode = models::Episode::from_create_request(req_data, album_uuid);
                if !data.is_none() {
                    let metadata = get_file_metadata(data.clone().unwrap().as_str());
                    new_episode.content_type = Some(metadata.content_type.clone());
                    new_episode.width = metadata
                        .image_data
                        .clone()
                        .unwrap_or(ImageMetadata::default())
                        .width as i32;
                    new_episode.height = metadata
                        .image_data
                        .clone()
                        .unwrap_or(ImageMetadata::default())
                        .height as i32;
                    new_episode.bytes = metadata.size as i32;
                }
                match Repository::create_episode(pool, new_episode).await {
                    Ok(episode) => {
                        let response = models::EpisodeResponse::from_episode(episode);
                        if !response.url.is_none() {
                            let des_file_path = format!(
                                "{}/{}",
                                get_project_directory(),
                                response.url.clone().unwrap()
                            );
                            move_file_and_replace(data.unwrap().as_str(), &*des_file_path);
                        }
                        Ok(response)
                    }
                    Err(e) => {
                        eprintln!("Error: {e}");
                        Err("Failed to create episode")
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                Err("Album not found")
            }
        }
    }

    pub async fn get_episodes_by_album_id(
        pool: &DbPool,
        album_id: i32,
        filter_episodes: models::FilterEpisodeRequest,
    ) -> Result<ResponseData<models::EpisodeResponse>, diesel::result::Error> {
        match Repository::get_episodes_by_album_id(pool, album_id, filter_episodes).await {
            Ok(episodes) => {
                let data = models::EpisodeResponse::from_episodes(episodes);
                let total = data.len() as i64;
                let response = ResponseData { data, total };
                Ok(response)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn delete_episode(
        pool: &DbPool,
        episode_uuid: String,
    ) -> Result<usize, diesel::result::Error> {
        match Repository::get_album_uuid_by_episode_uuid(pool, episode_uuid.clone()).await {
            Ok(album_uuid) => {
                let filepath = format!("{}/{}/{}", get_data_directory(), album_uuid, episode_uuid,);
                delete_directory_if_exists(&filepath);
                Repository::delete_episode(pool, episode_uuid).await
            }
            Err(e) => Err(e),
        }
    }
    pub async fn update_episode(
        pool: &DbPool,
        episode_uuid: String,
        update_episode: models::UpdateEpisodeRequest,
    ) -> Result<models::EpisodeResponse, &str> {
        match Repository::get_episode_by_episode_uuid(pool, episode_uuid.clone()).await {
            Ok(episode) => {
                match Repository::get_album_uuid_by_episode_uuid(pool, episode_uuid.clone()).await {
                    Ok(album_uuid) => {
                        let old_url = episode.url.clone();
                        let mut new_episode = models::Episode {
                            id: episode.id,
                            album_id: episode.album_id,
                            uuid: episode.uuid,
                            title: update_episode.title.unwrap_or(episode.title),
                            url: episode.url,
                            content_type: episode.content_type,
                            width: episode.width,
                            height: episode.height,
                            bytes: episode.bytes,
                            broken_at: episode.broken_at,
                            created_at: episode.created_at,
                            updated_at: episode.updated_at,
                        };
                        if let Some(src_path) = update_episode.file.clone() {
                            let new_uuid = Uuid::new_v4().to_string();
                            let format = src_path.split(".").last().unwrap();
                            let metadata = get_file_metadata(src_path.as_str());
                            new_episode.content_type = Some(metadata.content_type);
                            new_episode.width = metadata
                                .image_data
                                .clone()
                                .unwrap_or(ImageMetadata::default())
                                .width as i32;
                            new_episode.height = metadata
                                .image_data
                                .clone()
                                .unwrap_or(ImageMetadata::default())
                                .height as i32;
                            new_episode.bytes = metadata.size as i32;
                            let new_url = format!(
                                "{}/{}/{}/{}.{}",
                                get_data_directory(),
                                album_uuid,
                                episode_uuid,
                                new_uuid,
                                format
                            );
                            new_episode.url = Some(new_url);
                        }
                        new_episode.updated_at = Some(Utc::now().naive_utc());
                        match Repository::update_episode(pool, new_episode.clone()).await {
                            Ok(size) if size > 0 => {
                                if old_url != new_episode.url {
                                    if !old_url.is_none() {
                                        let old_file_path = format!(
                                            "{}/{}",
                                            get_project_directory(),
                                            old_url.unwrap()
                                        );
                                        //Delete old file
                                        delete_file_if_exists(&old_file_path);
                                    }
                                    if let (Some(src_file_path), Some(new_ep_url)) =
                                        (update_episode.file, new_episode.url.clone())
                                    {
                                        //Move new data
                                        let des_file_path =
                                            format!("{}/{}", get_project_directory(), new_ep_url);
                                        move_file_and_replace(&src_file_path, &des_file_path);
                                    }
                                }
                                Ok(models::EpisodeResponse::from_episode(new_episode))
                            }
                            Ok(_) => Err("Album not found"),
                            Err(e) => {
                                eprintln!("Error: {e}");
                                Err("Failed to update album")
                            }
                        }
                    }
                    Err(_) => Err("Album UUID not found"),
                }
            }
            Err(_) => Err("Episode not found"),
        }
    }
}
