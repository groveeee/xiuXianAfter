mod dao;

use actix_web::{HttpResponse, Responder, web};
use crypto::digest::Digest;
use crypto::md5::Md5;
use serde::Deserialize;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use crate::r#pub::{failed, success};

/// <h2>user模块的接口配置<h2>
pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/register")
            .route(web::post().to(register))
    );
}

#[derive(Deserialize)]
struct RegisterInfo {
    account: String,
    passwd: String,
}

/// <h2>账号注册</h2>
async fn register(info: web::Json<RegisterInfo>, pool: web::Data<Pool<Postgres>>) -> HttpResponse {
    let id = Uuid::new_v4();
    // 对密码进行MD5加密
    let mut hasher = Md5::new();
    hasher.input_str(&info.passwd);
    let md5_passwd = hasher.result_str();

    let one = sqlx::query!(r#"select account from xiuxian.friar where account = $1"#,info.account).fetch_one(&**pool).await;
    if let Ok(record) = one {
        match record.account {
            None => {
                let x = sqlx::query!(r#"INSERT INTO xiuxian.friar(id,account,passwd) VALUES($1,$2,$3)"#,id,info.account,md5_passwd)
                    .fetch_one(&**pool).await;
                match x {
                    Ok(_) => { println!("ok"); }
                    Err(e) => { println!("{}", e); }
                }
            }
            Some(_) => {
                let response = success(String::from("注册成功")).await;
                return response;
            }
        }
    }

    failed(String::from("注册失败!")).await

}



