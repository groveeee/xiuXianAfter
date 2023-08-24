use std::env;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::{HttpResponse, web};
use crypto::digest::Digest;
use crypto::md5::Md5;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use crate::r#pub::result::{failed, success};


#[derive(Deserialize)]
pub struct RegisterInfo {
    account: String,
    passwd: String,
}

/// <h2>账号注册</h2>
pub async fn register(info: web::Json<RegisterInfo>, pool: web::Data<Pool<Postgres>>) -> HttpResponse {
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
pub async fn login(info: web::Json<RegisterInfo>, pool: web::Data<Pool<Postgres>>) -> HttpResponse {
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





/// 校验Token
pub fn check_token(token: &str) -> Option<Claims> {
    let secret = env::var("TOKEN_SECRET").unwrap();
    let result = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS512));
    match result {
        Ok(r) => Option::from(r.claims),
        Err(_e) => None,
    }
}



/// 声言结构型, 需要由`Serialize` 或 `Deserialize`派生
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    // 可选。听众
    pub exp: usize,
    // 必须。 (validate_exp 在验证中默认为真值)。截止时间 (UTC 时间戳)
    pub sub: String,         // 可选。 标题 (令牌指向的人)
}

impl Claims {
    pub fn new(aud: String, exp: usize, sub: String) -> Self {
        Self { aud, exp, sub }
    }
}

/// 根据用户ID生成Token
pub fn create_token(uuid: Uuid) -> String {
    let time = env::var("TOKEN_EFFECTIVE_TIME").unwrap();
    let i = u64::from_str(time.as_str()).unwrap();
    let duration: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + i;
    let my_claims = Claims::new(String::from("BP"), duration as usize, uuid.to_string());
    let header = Header::new(Algorithm::HS512);
    let secret = env::var("TOKEN_SECRET").unwrap();
    encode(&header, &my_claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}