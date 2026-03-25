// #include "fe25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::fe25519::{fe25519, fe25519_freeze};
pub fn fe25519_getparity(x: &fe25519) -> u8 {
    let mut t: fe25519 = x.clone();
    fe25519_freeze(&mut t);
    (t.v[0] & 1) as u8
}

// impl Fe25519 {
//     /// 对应 fe25519_getparity
//     pub fn get_parity(&self) -> u8 {
//         // 1. 先进行强制归约（Freeze）
//         // 在 RustCrypto/curve25519-dalek 中通常称为 `is_negative` 逻辑的一部分
//         let t = self.freeze();

//         // 2. 取最低位
//         (t.v[0] & 1) as u8
//     }
// }

// // 模拟 C 风格函数
// pub fn fe25519_getparity(x: &Fe25519) -> u8 {
//     let mut t = *x;
//     t.freeze(); // 假设你已经实现了 freeze 方法处理进位和减去 2^255-19
//     (t.v[0] & 1) as u8
// }
