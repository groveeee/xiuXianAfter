use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonSuccessResult<T> {
    code: i32,
    data: Option<T>,
}
#[derive(Serialize)]
pub struct JsonFailedResult {
    code: i32,
    msg: String,
}


pub async fn success<T: serde::Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(JsonSuccessResult{code:200,data:Some(data)})
}

pub async fn failed(msg: String) -> HttpResponse {
    HttpResponse::Ok().json(JsonFailedResult{code:500,msg })
}