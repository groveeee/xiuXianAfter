mod dao;

use std::process::id;
use actix_web::{HttpResponse, Responder, web};
use actix_web::guard::fn_guard;
use actix_web::web::route;
use crypto::digest::Digest;
use crypto::md5::Md5;
use serde::Deserialize;
use sqlx::{Error, Pool, Postgres, Row};
use uuid::Uuid;
use crate::header_handler::{check_token, create_token};
use crate::r#pub::{failed, success};

/// <h2>user模块的接口配置<h2>
pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("register",web::post().to(register))
            .route("login",web::post().to(login))
            // .guard(fn_guard(|ctx| {
            //     let option = ctx.head().headers().get("X-Auth-Token");
            //     match option {
            //         None => { false }
            //         Some(h) => {
            //             let path = ctx.head().uri.path();
            //             println!("{}", path);
            //             if path == "/login" || path=="/register" {
            //                 return true;
            //             }
            //             check_token(h.to_str().unwrap()).is_some()
            //         }
            //     }
            // }))
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
                    Ok(_) => {
                        println!("ok");
                        let string = create_token(id);
                        return success(string).await;
                    }
                    Err(e) => { println!("{}", e); }
                }
            }
            Some(_) => {
                let response = failed(String::from("注册失败!该账号已经被注册!")).await;
                return response;
            }
        }
    }
    HttpResponse::BadRequest().finish()
}

/// <h2>账号登录</h2>
async fn login(info: web::Json<RegisterInfo>, pool: web::Data<Pool<Postgres>>) -> HttpResponse {
    // 对密码进行MD5加密
    let mut hasher = Md5::new();
    hasher.input_str(&info.passwd);
    let md5_passwd = hasher.result_str();

    let one = sqlx::query!(r#"select id from xiuxian.friar where account = $1 and passwd = $2"#,info.account,md5_passwd).fetch_one(&**pool).await;
    return match one {
        Ok(record) => {
            let string = create_token(record.id);
            success(string).await
        }
        Err(_) => {
            failed(String::from("账号或密码错误!")).await
        }
    }

}



