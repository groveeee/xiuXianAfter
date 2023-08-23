use std::env;
use std::future::{ready, Ready};
use std::ptr::eq;

use actix_web::{dev::{self, Service, ServiceRequest, ServiceResponse, Transform}, Error, error, HttpResponse};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
use crate::header_handler::Claims;

// 中间件处理有两个步骤。
// 1. 中间件初始化，中间件工厂被调用，链中的下一个服务作为参数。
// 2. 中间件的调用方式以正常请求调用。
pub struct Auth;

pub fn validate_token(token: &str) -> bool {
    let secret = env::var("TOKEN_SECRET").unwrap();
    let result = decode::<Claims>(token,
                                  &DecodingKey::from_secret(secret.as_ref()),
                                  &Validation::new(Algorithm::HS512));
    match result {
        Ok(_) => { true }
        Err(_) => { false }
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
    type Transform = SayHiMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service }))
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
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
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = String::from(req.path());
        // token是否有效
        let mut is_token = false;
        // 因为所有权的原因 提前校验token
        let option = req.headers().get("X-Auth-Token").and_then(|value| value.to_str().ok());
        if let Some(token) = option {
            is_token = validate_token(token);
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
