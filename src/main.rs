use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use crate::db::init_db;

mod db;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 配置log4rs日志
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let pool = init_db().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
