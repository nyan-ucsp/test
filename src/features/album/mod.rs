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

    cfg.service(
        web::scope("/album")
            .service(update_album)
            .service(delete_album)
    );
}
