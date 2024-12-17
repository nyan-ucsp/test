use crate::common::database::DbPool;
use crate::common::models::file_metadata::ImageMetadata;
use crate::common::models::response_data::ResponseData;
use crate::common::utils::{delete_directory_if_exists, get_data_directory, get_directory_from_file_path, get_file_metadata, get_project_directory, move_file_and_replace};
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

    pub async fn get_episode_by_episode_uuid(
        pool: &DbPool,
        episode_uuid: String,
    ) -> Result<models::EpisodeResponse, diesel::result::Error> {
        match Repository::get_episode_by_episode_uuid(pool, episode_uuid).await {
            Ok(episode) => {
                Ok(models::EpisodeResponse::from_episode(episode))
            }
            Err(e) => Err(e),
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
                let filepath = format!("{}/{}/{}", get_data_directory(), album_uuid, episode_uuid, );
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
        let file_url_exists = update_episode
            .file_url
            .as_ref()
            .map(|url| !url.trim().is_empty())
            .unwrap_or(false);

        let episode = Repository::get_episode_by_episode_uuid(pool, episode_uuid.clone())
            .await
            .map_err(|_| "Episode not found")?;

        let album_uuid = Repository::get_album_uuid_by_episode_uuid(pool, episode_uuid.clone())
            .await
            .map_err(|_| "Album UUID not found")?;

        let mut new_episode = build_updated_episode(&episode, &update_episode, file_url_exists);

        if let Some(src_path) = &update_episode.file {
            update_episode_with_file(&mut new_episode, src_path, &album_uuid, &episode_uuid);
        }

        new_episode.updated_at = Some(Utc::now().naive_utc());

        match Repository::update_episode(pool, new_episode.clone()).await {
            Ok(size) if size > 0 => {
                handle_old_file_cleanup(
                    &update_episode,
                    &new_episode,
                    &episode.url,
                );
                Ok(models::EpisodeResponse::from_episode(new_episode))
            }
            Ok(_) => Err("Album not found"),
            Err(_) => Err("Failed to update album"),
        }
    }
}

fn build_updated_episode(
    episode: &models::Episode,
    update_episode: &models::UpdateEpisodeRequest,
    file_url_exists: bool,
) -> models::Episode {
    let should_reset_fields = file_url_exists || update_episode.remove_old_file;

    models::Episode {
        id: episode.id,
        album_id: episode.album_id,
        uuid: episode.uuid.clone(),
        title: update_episode.title.clone().unwrap_or_else(|| episode.title.clone()),
        url: if should_reset_fields {
            Some(String::new())
        } else {
            episode.url.clone()
        },
        file_url: update_episode.file_url.clone(),
        content_type: if should_reset_fields {
            Some(String::new())
        } else {
            episode.content_type.clone()
        },
        width: if should_reset_fields { 0 } else { episode.width },
        height: if should_reset_fields { 0 } else { episode.height },
        bytes: if should_reset_fields { 0 } else { episode.bytes },
        broken_at: episode.broken_at,
        created_at: episode.created_at,
        updated_at: episode.updated_at,
    }
}

fn update_episode_with_file(
    new_episode: &mut models::Episode,
    src_path: &str,
    album_uuid: &str,
    episode_uuid: &str,
) {
    let new_uuid = Uuid::new_v4().to_string();
    let format = src_path.split('.').last().unwrap_or_default();
    let metadata = get_file_metadata(src_path);

    new_episode.content_type = Some(metadata.content_type);
    new_episode.width = metadata.image_data.clone().unwrap_or(ImageMetadata::default()).width as i32;
    new_episode.height = metadata.image_data.clone().unwrap_or(ImageMetadata::default()).height as i32;
    new_episode.bytes = metadata.size as i32;

    new_episode.url = Some(format!(
        "{}/{}/{}/{}.{}",
        get_data_directory(),
        album_uuid,
        episode_uuid,
        new_uuid,
        format
    ));
}

fn handle_old_file_cleanup(
    update_episode: &models::UpdateEpisodeRequest,
    new_episode: &models::Episode,
    old_url: &Option<String>,
) {
    if old_url != &new_episode.url || update_episode.remove_old_file {
        if let Some(old_url) = old_url {
            if !old_url.trim().is_empty() {
                let old_file_path = format!("{}/{}", get_project_directory(), old_url);
                if let Some(old_directory_path) = get_directory_from_file_path(&old_file_path) {
                    delete_directory_if_exists(&old_directory_path);
                }
            }
        }

        if let (Some(src_file_path), Some(new_ep_url)) = (&update_episode.file, &new_episode.url) {
            let dest_file_path = format!("{}/{}", get_project_directory(), new_ep_url);
            move_file_and_replace(src_file_path, &dest_file_path);
        }
    }
}

