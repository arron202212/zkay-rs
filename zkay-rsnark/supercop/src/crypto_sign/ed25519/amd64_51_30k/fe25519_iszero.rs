// #include "fe25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::fe25519::{fe25519, fe25519_freeze};
pub fn fe25519_iszero_vartime(x: &fe25519) -> i32 {
    let mut t: fe25519 = x.clone();
    fe25519_freeze(&mut t);
    (0..5).all(|i| t.v[i] == 0) as _
}

// impl Fe25519 {
//     /// 对应 fe25519_iszero_vartime
//     pub fn is_zero_vartime(&self) -> bool {
//         // 1. 拷贝并强制归约（对应 fe25519_freeze）
//         // 确保像 2^255-19 这样的值会被归约为 0
//         let mut t = *self;
//         t.freeze();

//         // 2. 检查所有分量是否为 0
//         // Rust 的数组比较或 iter().all() 在发现非零时会提前返回
//         t.v.iter().all(|&val| val == 0)
//     }
// }

// // 模拟 C 风格函数原型
// pub fn fe25519_iszero_vartime(x: &Fe25519) -> i32 {
//     if x.is_zero_vartime() { 1 } else { 0 }
// }
