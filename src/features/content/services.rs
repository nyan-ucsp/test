use crate::common::database::DbPool;
use crate::common::models::file_metadata::ImageMetadata;
use crate::common::utils::{
    get_data_directory, get_file_metadata, get_project_directory, move_file_and_replace,
};
use crate::features::content::models;
use crate::features::content::models::{Content, ContentResponse};
use crate::features::content::repository::Repository;
use uuid::Uuid;

pub struct Service;

impl Service {
    pub async fn create_episode_contents(
        pool: &DbPool,
        req: models::AddEpisodeContentsRequest,
    ) -> Result<Vec<models::ContentResponse>, &str> {
        match Repository::get_album_uuid_by_episode_id(pool, req.episode_id.clone()).await {
            Ok(album_uuid) => {
                if album_uuid.clone().is_none() {
                    Err("Album UUID not found")
                } else {
                    match Repository::get_episode_uuid_by_id(pool, req.episode_id.clone()).await {
                        Ok(episode_uuid) => {
                            if episode_uuid.clone().is_none() {
                                Err("Episode UUID not found")
                            } else {
                                let mut new_contents: Vec<models::Content> = vec![];
                                let mut file_paths = req.files.clone();
                                file_paths.retain(|s| !s.trim().is_empty());
                                for index in 1..req.files.clone().len() {
                                    let file_path = file_paths[index].clone();
                                    let uuid = Uuid::new_v4().to_string();
                                    let new_url = format!(
                                        "{}/{}/{}/{}.{}",
                                        get_data_directory(),
                                        album_uuid.clone().unwrap(),
                                        episode_uuid.clone().unwrap(),
                                        uuid.clone(),
                                        file_path.clone().split(".").last().unwrap()
                                    );
                                    let metadata = get_file_metadata(file_path.as_str());
                                    new_contents.push(Content {
                                        id: None,
                                        episode_id: req.episode_id.clone(),
                                        uuid,
                                        index_no: index as i32,
                                        url: new_url,
                                        content_type: metadata.content_type,
                                        width: metadata
                                            .image_data
                                            .clone()
                                            .unwrap_or(ImageMetadata::default())
                                            .width
                                            as i32,
                                        height: metadata
                                            .image_data
                                            .clone()
                                            .unwrap_or(ImageMetadata::default())
                                            .width
                                            as i32,
                                        bytes: metadata.size as i32,
                                        broken_at: None,
                                        created_at: None,
                                        updated_at: None,
                                    })
                                }
                                match Repository::create_contents(pool, new_contents.clone()).await
                                {
                                    Ok(_) => {
                                        if file_paths.iter().len() == new_contents.len() {
                                            for i in 0..new_contents.len() {
                                                move_file_and_replace(
                                                    file_paths[i].as_str(),
                                                    format!(
                                                        "{}/{}",
                                                        get_project_directory(),
                                                        new_contents[i].url.clone(),
                                                    )
                                                    .as_str(),
                                                )
                                            }
                                            match Repository::get_contents_by_episode_id(
                                                pool,
                                                req.episode_id,
                                            )
                                            .await
                                            {
                                                Ok(contents) => {
                                                    Ok(ContentResponse::from_contents(contents))
                                                }
                                                Err(_) => {
                                                    Err("Failed to get created contents")
                                                }
                                            }
                                        } else {
                                            Err("Some files lost on data exchanging")
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Error: {e}");
                                        Err("Failed to add episode contents")
                                    }
                                }
                            }
                        }
                        Err(_) => Err("Failed to get episode uuid"),
                    }
                }
            }
            Err(_) => Err("Failed to get album uuid"),
        }
    }
}
