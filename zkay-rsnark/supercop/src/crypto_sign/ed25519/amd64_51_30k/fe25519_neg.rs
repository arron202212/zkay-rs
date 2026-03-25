// #include "fe25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::{
    fe25519::fe25519, fe25519_setint::fe25519_setint, fe25519_sub::fe25519_sub,
};
pub fn fe25519_neg(r: &mut fe25519, x: &fe25519) {
    let mut t = fe25519::default();
    fe25519_setint(&mut t, 0);
    fe25519_sub(r, &t, x);
}

// use std::ops::Neg;

// impl Neg for Fe25519 {
//     type Output = Self;

//     fn neg(self) -> Self {
//         // 1. 创建一个值为 0 的元素 (对应 fe25519_setint(&t, 0))
//         let zero = Fe25519::zero();

//         // 2. 执行减法 (对应 fe25519_sub(r, &t, x))
//         // 假设你已经实现了 Fe25519 的减法逻辑
//         zero.sub(&self)
//     }
// }

// // 模拟 C 风格函数原型
// pub fn fe25519_neg(r: &mut Fe25519, x: &Fe25519) {
//     *r = -(*x);
// }
