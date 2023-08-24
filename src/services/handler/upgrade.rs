use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::{Pool, Postgres};
use crate::enums::{Realm, REALM_MAP};
use crate::middleware::auth::get_current_user;
use crate::r#pub::result::{failed, success};

/// 增加灵气
pub async fn increase_reiki(req: HttpRequest, pool: web::Data<Pool<Postgres>>) -> HttpResponse {
    let user = get_current_user(req);
    let realm = REALM_MAP.get(&user.realm).unwrap();
    let i = Realm::energy_increase(realm);
    let result = sqlx::query!(r#"update xiuxian.xiuxian.friar set reiki = reiki + $1 where id = $2 returning reiki"#,
        i,user.id)
        .fetch_one(&**pool).await;
    match result {
        Ok(r) => { println!("{}", r.reiki.unwrap()); }
        Err(_) => {return failed("系统异常!").await;}
    }
    success("OK").await
}