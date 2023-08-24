use std::collections::HashMap;
use lazy_static::lazy_static;

/// <h1>境界</h1>
pub enum Realm {
    FanRen,
    LianQi,
    ZhuJi,
    JinDan,
    // ... 可以继续添加更多境界
}

impl Realm {
    pub fn energy_increase(&self) -> i64 {
        match self {
            Realm::FanRen => 1,
            Realm::LianQi => 10,
            Realm::ZhuJi => 20,
            Realm::JinDan => 30,
            // ... 根据需要添加其他境界的能量增加值
        }
    }
}

// 初始化每个境界对应的值
lazy_static! {
    pub static ref REALM_MAP: HashMap<i64, Realm> = {
        let mut map = HashMap::new();
        map.insert(0, Realm::FanRen);
        map.insert(1, Realm::LianQi);
        map.insert(2, Realm::ZhuJi);
        map.insert(3, Realm::JinDan);
        // ... 可以继续添加其他数字和对应境界的映射
        map
    };
}