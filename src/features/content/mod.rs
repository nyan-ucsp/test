use crate::features::content::controllers::{add_contents, delete_content, get_contents, update_content};
use actix_web::web;

pub mod models;
pub mod repository;
pub mod services;
pub mod controllers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(add_contents);
    cfg.service(get_contents);
    cfg.service(update_content);
    cfg.service(delete_content);
}