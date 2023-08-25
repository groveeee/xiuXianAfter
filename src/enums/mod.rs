use std::collections::HashMap;
use lazy_static::lazy_static;

/// <h1>境界</h1>
pub enum Realm {
    FanRen,//凡人
    NingQi,//练气
    ZhuJi,//筑基
    JieDan,//结丹
    YuanYin,//元婴
    HuaShen,//化神
    YinBian,//婴变
    WenDing,//问鼎
    YinXu,//阴虚
    YangShi,//阳实
    KuiNie,//窥涅
    JingNie,//净涅
    SuiNie,//碎涅
    TianRenWuShuai_1,//天人五衰 第一衰
    TianRenWuShuai_2,//天人五衰 第二衰
    TianRenWuShuai_3,//天人五衰 第三衰
    TianRenWuShuai_4,//天人五衰 第四衰
    TianRenWuShuai_5,//天人五衰 第五衰
    KongNie,//空涅
    KongLing,//空灵
    KongXuan,//空玄
    KongJie,//空劫
    JinZun,//金尊
    TianZun,//天尊
    YueTianZun,//跃天尊
    DaTianZun,//大天尊
    TaTian,//踏天
    // ... 可以继续添加更多境界
}

impl Realm {
    /// 每个境界对应的灵气修炼速度
    pub fn cultivation_speed(&self) -> i64 {
        match self {
            Realm::FanRen => { 1 }
            Realm::NingQi => { 2 }
            Realm::ZhuJi => { 3 }
            Realm::JieDan => { 4 }
            Realm::YuanYin => { 5 }
            Realm::HuaShen => { 6 }
            Realm::YinBian => { 7 }
            Realm::WenDing => { 8 }
            Realm::YinXu => { 9 }
            Realm::YangShi => { 10 }
            Realm::KuiNie => { 11 }
            Realm::JingNie => { 12 }
            Realm::SuiNie => { 13 }
            Realm::TianRenWuShuai_1 => { 14 }
            Realm::TianRenWuShuai_2 => { 15 }
            Realm::TianRenWuShuai_3 => { 16 }
            Realm::TianRenWuShuai_4 => { 17 }
            Realm::TianRenWuShuai_5 => { 18 }
            Realm::KongNie => { 19 }
            Realm::KongLing => { 20 }
            Realm::KongXuan => { 21 }
            Realm::KongJie => { 22 }
            Realm::JinZun => { 23 }
            Realm::TianZun => { 24 }
            Realm::YueTianZun => { 25 }
            Realm::DaTianZun => { 26 }
            Realm::TaTian => { 27 }
        }
    }

    /// 每个境界对应的最大灵气值
    pub fn maximum_number_of_reiki(&self) -> i64 {
        match self {
            Realm::FanRen => { 10 }
            Realm::NingQi => { 1000 }
            Realm::ZhuJi => { 10240 }
            Realm::JieDan => { 20480 }
            Realm::YuanYin => { 40960 }
            Realm::HuaShen => { 81920 }
            Realm::YinBian => { 163840 }
            Realm::WenDing => { 327680 }
            Realm::YinXu => { 655360 }
            Realm::YangShi => { 1310720 }
            Realm::KuiNie => { 2621440 }
            Realm::JingNie => { 5242880 }
            Realm::SuiNie => { 10485760 }
            Realm::TianRenWuShuai_1 => { 20971520 }
            Realm::TianRenWuShuai_2 => { 41943040 }
            Realm::TianRenWuShuai_3 => { 83886080 }
            Realm::TianRenWuShuai_4 => { 167772160 }
            Realm::TianRenWuShuai_5 => { 335544320 }
            Realm::KongNie => { 671088640 }
            Realm::KongLing => { 1342177280 }
            Realm::KongXuan => { 2684354560 }
            Realm::KongJie => { 5368709120 }
            Realm::JinZun => { 10737418240 }
            Realm::TianZun => { 21474836480 }
            Realm::YueTianZun => { 42949672960 }
            Realm::DaTianZun => { 85899345920 }
            Realm::TaTian => { 171798691840 }
        }
    }
}

// 初始化每个境界对应的值
lazy_static! {
    pub static ref REALM_MAP: HashMap<i64, Realm> = {
        let mut map = HashMap::new();
        map.insert(1, Realm::FanRen);
        map.insert(2, Realm::NingQi);
        map.insert(3, Realm::ZhuJi);
        map.insert(4, Realm::JieDan);
        map.insert(5, Realm::YuanYin);
        map.insert(6, Realm::HuaShen);
        map.insert(7, Realm::YinBian);
        map.insert(8, Realm::WenDing);
        map.insert(9, Realm::YinXu);
        map.insert(10, Realm::YangShi);
        map.insert(11, Realm::KuiNie);
        map.insert(12, Realm::JingNie);
        map.insert(13, Realm::SuiNie);
        map.insert(14, Realm::TianRenWuShuai_1);
        map.insert(15, Realm::TianRenWuShuai_2);
        map.insert(16, Realm::TianRenWuShuai_3);
        map.insert(17, Realm::TianRenWuShuai_4);
        map.insert(18, Realm::TianRenWuShuai_5);
        map.insert(19, Realm::KongNie);
        map.insert(20, Realm::KongLing);
        map.insert(21, Realm::KongXuan);
        map.insert(22, Realm::KongJie);
        map.insert(23, Realm::JinZun);
        map.insert(24, Realm::TianZun);
        map.insert(25, Realm::YueTianZun);
        map.insert(26, Realm::DaTianZun);
        map.insert(27, Realm::TaTian);
        // ... 可以继续添加其他数字和对应境界的映射
        map
    };
}