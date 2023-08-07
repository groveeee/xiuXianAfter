/*
初始化数据库连接
 */
use std::env;
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub async fn init_db() -> Pool<Postgres> {
    // 在访问环境变量之前检查一下，防止因读取环境变量失败导致程序恐慌
    dotenv().ok();
    let connection_str = env::var("DATABASE_URL")
        .expect("If the database connection string cannot be obtained, check whether the database connection string is configured in the env file");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        // postgres://username:password@address/database
        .connect(connection_str.as_str())
        .await;
    match pool {
        Ok(db) => {
            // log!("数据库连接成功");
            log::info!("The database connection succeeded !");
            db
        }
        Err(e) => panic!("Database connection failed :{e}"),
    }
}