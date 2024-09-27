use crate::common::database::DbPool;
use crate::common::utils::{get_project_directory, move_file_and_replace};
use crate::features::episode::models;
use crate::features::episode::repository::*;

pub struct Service;
impl Service {
    pub async fn create_episode(
        pool: &DbPool,
        req_data: models::CreateEpisodeRequest,
    ) -> Result<models::EpisodeResponse, &str> {
        let mut data = req_data.clone().file;
        let new_episode = models::Episode::from_create_request(req_data);
        match Repository::create_episode(pool, new_episode).await {
            Ok(episode) => {
                let response = models::EpisodeResponse::from_episode(episode);
                if !response.url.is_none() {
                    let des_file_path = format!("{}/{}", get_project_directory(), response.url.clone().unwrap());
                    move_file_and_replace(data.unwrap().as_str(), &*des_file_path);
                    Ok(response)
                } else {
                    Err("Some files lost on data exchanging")
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                Err("Failed to create episode")
            }
        }
    }
}