use actix_web::{App, error, get, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, web};
use crate::db::init_db;
use crate::middleware::{auth};
use crate::middleware::auth::ContextUser;
use crate::services::config::config;

mod db;
mod model;
mod r#pub;
mod middleware;
mod services;

#[get("/")]
async fn hello(req:HttpRequest) -> impl Responder {
    let extensions = req.extensions_mut();
    let context_user = extensions.get::<ContextUser>().unwrap();
    println!("{:?}", context_user);
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 配置log4rs日志
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let pool = init_db().await;
    log::info!("starting HTTP server at http://localhost:10086");
    HttpServer::new(move || {
        let json_config = web::JsonConfig::default()
            .limit(4096)// 限制请求负载最大4kb
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });
        App::new()
            .wrap(auth::Auth)
            .configure(config)
            .app_data(json_config)
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
    })
        .bind(("127.0.0.1", 10086))?
        .run()
        .await
}
