use actix_web::web;

pub mod models;
mod handlers;

pub use handlers::{get_users, create_user};

pub fn users_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users)
       .service(create_user);
} 