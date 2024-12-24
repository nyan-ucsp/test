use actix_web::web;
use crate::features::category::controllers::{add_category, delete_category, get_categories, update_category};

pub mod models;
pub mod repository;
pub mod services;
pub mod controllers;

pub fn configure(c: &mut web::ServiceConfig) {
    c.service(get_categories);
    c.service(add_category);
    c.service(update_category);
    c.service(delete_category);
}