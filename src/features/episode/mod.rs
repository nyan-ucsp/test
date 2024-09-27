use actix_web::web;
use crate::features::episode::controllers::create_episode;

pub mod controllers;
pub mod models;
pub mod services;
mod repository;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_episode);
}