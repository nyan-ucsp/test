use actix_web::web;

use crate::features::album::controllers::*;

pub mod controllers;
pub mod models;
pub mod repository;
pub mod services;

// ! Register ever routes in this configure function
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_album);
    cfg.service(get_albums);
    cfg.service(get_album_by_uuid);
    cfg.service(add_album_images);

    cfg.service(
        web::scope("/album")
            .service(update_album)
            .service(delete_album)
            .service(remove_album_images)
    );
}
