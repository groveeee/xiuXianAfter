use std::env;
use std::future::{ready, Ready};

use actix_web::{dev::{self, Service, ServiceRequest, ServiceResponse, Transform}, Error, error, HttpMessage, HttpRequest, HttpResponse};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::services::handler::user::Claims;

/// 前端请求头携带参数
const XAUTH: &str = "X-Auth-Token";


// 中间件处理有两个步骤。
// 1. 中间件初始化，中间件工厂被调用，链中的下一个服务作为参数。
// 2. 中间件的调用方式以正常请求调用。
pub struct Auth;

pub fn validate_token(token: &str) -> Option<Claims> {
    let secret = env::var("TOKEN_SECRET").unwrap();
    let result = decode::<Claims>(token,
                                  &DecodingKey::from_secret(secret.as_ref()),
                                  &Validation::new(Algorithm::HS512));
    match result {
        Ok(r) => Option::from(r.claims),
        Err(_e) => None,
    }
}

// 中间件工厂是来自 actix 服务箱的“转换”特征
// “S” - 下一个服务的类型
// “B” - 响应主体的类型
impl<S, B> Transform<S, ServiceRequest> for Auth
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    // 上面的代码都是套模板 核心在于call这个方法 是中间件的核心 控制请求是否继续

    /// <h2>Token校验</h2>
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = String::from(req.path());
        // token是否有效
        let mut is_token = false;
        // 因为所有权的原因 提前校验token
        let option = req.headers().get("X-Auth-Token").and_then(|value| value.to_str().ok());
        if let Some(token) = option {
            let claims = validate_token(token);
            if claims.is_some() {
                is_token = true;
                // 将用户信息存入到请求上下文中
                // req.extensions_mut()获取扩展数据
                // 扩展数据允许您在请求的上下文中存储和访问自定义数据，这些数据可以在整个请求的生命周期内共享
                let claims_unwrap = claims.unwrap();
                let id:Uuid = claims_unwrap.sub.parse().unwrap();
                let realm = claims_unwrap.realm;
                req.extensions_mut().insert(ContextUser{id,realm});
            }
        }
        let fut = self.service.call(req);
        if path == "/user/login" || path == "/user/register" {
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else if is_token {
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Err(error::ErrorUnauthorized("Not logged in!"))
            })
        }
    }
}

/// <h2>当前登录用户的信息</h2>
#[derive(Debug)]
pub struct ContextUser {
    pub id: Uuid,
    pub realm:i64,
}

/// <h2>获取当前登录用户</h2>
pub fn get_current_user(req: HttpRequest) ->  ContextUser {
    let extensions = req.extensions();
    let context_user = extensions.get::<ContextUser>().unwrap();
    ContextUser{id:context_user.id,realm:context_user.realm}
}