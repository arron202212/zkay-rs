// #include "fe25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::fe25519::fe25519;
// use crate::crypto_sign::ed25519::amd64_51_30k::fe25519_neg::fe25519_neg;
pub fn fe25519_add(r: &mut fe25519, x: &fe25519, y: &fe25519) {
    r.v[0] = x.v[0] + y.v[0];
    r.v[1] = x.v[1] + y.v[1];
    r.v[2] = x.v[2] + y.v[2];
    r.v[3] = x.v[3] + y.v[3];
    r.v[4] = x.v[4] + y.v[4];
}

// /// 假设 fe25519 结构体定义如下（对应 C 的 5 窗口/51位 表示法）
// #[derive(Clone, Copy, Debug)]
// pub struct Fe25519 {
//     pub v: [u64; 5],
// }

// use std::ops::Add;

// impl Add for Fe25519 {
//     type Output = Self;

//     fn add(self, other: Self) -> Self {
//         // 在密码学实现中，加法通常不处理进位（归约），
//         // 而是允许值暂时溢出到更高的位，后续统一进行 reduce。
//         Fe25519 {
//             v: [
//                 self.v[0] + other.v[0],
//                 self.v[1] + other.v[1],
//                 self.v[2] + other.v[2],
//                 self.v[3] + other.v[3],
//                 self.v[4] + other.v[4],
//             ],
//         }
//     }
// }

// // 对应 C 风格的函数写法
// pub fn fe25519_add(r: &mut Fe25519, x: &Fe25519, y: &Fe25519) {
//     for i in 0..5 {
//         r.v[i] = x.v[i] + y.v[i];
//     }
// }
