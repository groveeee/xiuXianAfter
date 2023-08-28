use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::{Pool, Postgres};
use crate::enums::{Realm, REALM_MAP};
use crate::middleware::auth::{ContextUser, get_current_user};
use crate::r#pub::result::{failed, success};
use crate::services::handler::user::create_token;

/// 增加灵气
pub async fn increase_reiki(req: HttpRequest, pool: web::Data<Pool<Postgres>>) -> HttpResponse {
    let info = get_user_info(req);
    // 增加灵气 并且不能超过当前境界的最大灵气值
    let result = sqlx::query!(r#"update friar
                    set reiki = case when (reiki + $1)<= $2 then reiki+$3 else $4 end
                    where id = $5
                    returning reiki"#,
        info.1,info.2,info.1,info.2,info.0.id)
        .fetch_one(&**pool).await;
    match result {
        Ok(r) => { println!("{}", r.reiki.unwrap()); }
        Err(_) => { return failed("系统异常!").await; }
    }
    success("OK").await
}


/// 突破
pub async fn breakthrough(req: HttpRequest, pool: web::Data<Pool<Postgres>>) -> HttpResponse {
    let info = get_user_info(req);
    let result = sqlx::query!(r#"update friar set realm = realm+1 where id = $1 and reiki = $2 returning realm"#
        ,info.0.id,info.2)
        .fetch_one(&**pool).await;
    match result {
        Ok(r) => {
            println!("{}", r.realm.unwrap());
            let string = create_token(info.0.id, r.realm.unwrap() as i64);
            success(string).await
        }
        Err(_) => { return failed("系统异常!").await; }
    }
}

/// 获取当前用户的相关信息
pub fn get_user_info(req: HttpRequest) -> (ContextUser, i64, i64) {
    let user = get_current_user(req);
    let realm = REALM_MAP.get(&user.realm).unwrap();
    let speed = Realm::cultivation_speed(realm);
    let max_reiki = Realm::maximum_number_of_reiki(realm);
    (user, speed, max_reiki)
}