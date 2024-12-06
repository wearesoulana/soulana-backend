pub mod health;
pub mod users;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(health::health_check)
        .service(users::get_users)
        .service(users::create_user);
} 