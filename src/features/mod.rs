use actix_web::{HttpMessage, HttpRequest, web};
use utoipa::{Modify, OpenApi};
use utoipa::openapi::Components;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};

use crate::common::enums::Role;
use crate::common::enums::Role::*;
use crate::common::models::response_data::ResponseData;
use crate::common::models::response_message::ResponseMessage;

pub mod album;
pub mod health_check;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    album::configure(cfg);
    health_check::configure(cfg);
}

#[derive(OpenApi)]
#[openapi
(
    paths(
        album::controllers::create_album,
        album::controllers::get_albums,
        album::controllers::update_album,
        album::controllers::delete_album,
        health_check::controllers::get_health,
    ),
    components(
        schemas(
            album::models::Album,
            album::models::CreateAlbumRequest,
            album::models::UpdateAlbumRequest,
            album::models::GetAlbumRequest,
            ResponseMessage,
            ResponseData < album::models::Album >
        )
    ),
    modifiers(& SecurityAddon),
    tags(
        (name = "Album", description = "Album"),
        (name = "HealthCheck", description = "Service Health Checking"),
    ),
)]
pub struct ApiDoc;

struct SecurityAddon;

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
    if "admin" == role { Admin } else if "user" == role { User } else { Unknown }
}
