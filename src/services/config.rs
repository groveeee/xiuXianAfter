use actix_web::web;
use crate::services::handler::user::{login, register};


/// <h2>模块的接口配置<h2>
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/model")
            .route("register",web::post().to(register))
            .route("login",web::post().to(login))
    );
}