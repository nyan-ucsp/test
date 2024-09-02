use actix_web::web;

use crate::features::health_check::controllers::get_health;

pub mod controllers;

// ! Register ever routes in this configure function
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_health);
}