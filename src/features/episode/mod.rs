use crate::features::episode::controllers::{
    create_episode, delete_episode, get_episodes_by_album_id, update_episode,
};
use actix_web::web;

pub mod controllers;
pub mod models;
mod repository;
pub mod services;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_episode);
    cfg.service(update_episode);
    cfg.service(delete_episode);
    cfg.service(get_episodes_by_album_id);
}
