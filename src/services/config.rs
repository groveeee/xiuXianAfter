use actix_web::web;
use crate::services::handler::upgrade::*;
use crate::services::handler::user::*;


/// <h2>模块的接口配置<h2>
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("register", web::post().to(register))
            .route("login", web::post().to(login))
    )
        .service(
            web::scope("/upgrade")
                .route("increaseReiki", web::post().to(increase_reiki))
                .route("breakthrough", web::post().to(breakthrough))
        )
    ;
}