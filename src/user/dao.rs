/*
定义数据库结构体
 */
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug)]
struct Friar {
    id: Uuid,
    // 账户名称
    account: String,
    // 密码
    passwd: String,
    // 修士名
    name: String,
    // 灵气
    reiki: u64,
    // 账户创建日期
    birth: Option<NaiveDateTime>,
    // 生存时间
    life: i64,
    // 死亡时间
    die: Option<NaiveDateTime>,
    // 境界
    realm: i32,
}