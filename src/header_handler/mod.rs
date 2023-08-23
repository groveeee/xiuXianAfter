pub(crate) mod auth;

use std::env;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


/// 前端请求头携带参数
const XAUTH: &str = "X-Auth-Token";



/// 校验Token
pub fn check_token(token: &str) -> Option<Claims> {
    let secret = env::var("TOKEN_SECRET").unwrap();
    let result = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS512));
    match result {
        Ok(r) => Option::from(r.claims),
        Err(_e) => None,
    }
}

/// 当前登录用户的信息
pub struct ContextUser {
    pub id: Uuid,
}

impl ContextUser {
    pub fn new(id: Uuid) -> Self {
        Self { id }
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


