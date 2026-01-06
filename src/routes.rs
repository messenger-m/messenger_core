use actix_web::web;
use crate::handlers::auth;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(auth::login);
}
