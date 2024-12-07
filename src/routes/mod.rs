use actix_web::web;

pub mod blink_chain;
pub mod health;
pub mod users;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/api/blink-chain")
            .configure(blink_chain::blink_chain_config),
    )
    .service(
        web::scope("/api/users")
            .configure(users::users_config),
    );
} 