use actix_web::web;
use crate::features::episode::controllers::{create_episode, delete_episode, get_episodes_by_album_id};

pub mod controllers;
pub mod models;
pub mod services;
mod repository;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_episode);
    cfg.service(delete_episode);
    cfg.service(get_episodes_by_album_id);
}