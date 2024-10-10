use actix_web::{web, HttpMessage, HttpRequest};
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::openapi::Components;
use utoipa::{Modify, OpenApi};

use crate::common::enums::Role;
use crate::common::enums::Role::*;
use crate::common::models::response_data::*;
use crate::common::models::response_message::*;

pub mod album;
pub mod content;
pub mod episode;
pub mod health_check;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    album::configure(cfg);
    episode::configure(cfg);
    health_check::configure(cfg);
}

struct SecurityAddon;

#[derive(OpenApi)]
#[openapi
(
    paths(
        album::controllers::create_album,
        album::controllers::get_albums,
        album::controllers::get_album_by_uuid,
        album::controllers::update_album,
        album::controllers::delete_album,
        album::controllers::add_album_images,
        album::controllers::remove_album_images,
        episode::controllers::create_episode,
        episode::controllers::update_episode,
        episode::controllers::delete_episode,
        episode::controllers::get_episodes_by_album_id,
        health_check::controllers::get_health,
    ),
    components(
        schemas(
            album::models::Album,
            album::models::AlbumResponse,
            album::models::CreateAlbumRequest,
            album::models::UpdateAlbumRequest,
            album::models::GetAlbumRequest,
            album::models::AddAlbumImagesRequest,
            album::models::RemoveAlbumImagesRequest,
            ResponseDataAlbum,
            episode::models::Episode,
            episode::models::EpisodeResponse,
            episode::models::CreateEpisodeRequest,
            episode::models::UpdateEpisodeRequest,
            episode::models::FilterEpisodeRequest,
            ResponseMessage
        )
    ),
    modifiers(& SecurityAddon),
    tags(
        (name = "Album", description = "Album"),
        (name = "Episode", description = "Episode"),
        (name = "HealthCheck", description = "Service Health Checking"),
    ),
)]
pub struct ApiDoc;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Components::default);
        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-API-KEY"))),
        );
    }
}

fn check_role(http_request: HttpRequest) -> Role {
    let role = http_request
        .extensions()
        .get::<String>()
        .cloned()
        .unwrap_or_else(|| "unknown".to_string());
    if "admin" == role {
        Admin
    } else if "user" == role {
        User
    } else {
        Unknown
    }
}
