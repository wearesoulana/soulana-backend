use actix_web::web;

pub mod models;
mod handlers;

pub use handlers::{get_project, process_donation, options};

pub fn blink_chain_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_project)
       .service(process_donation)
       .service(options);
} 