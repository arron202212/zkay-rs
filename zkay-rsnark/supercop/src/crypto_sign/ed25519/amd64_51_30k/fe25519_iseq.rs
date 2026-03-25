// #include "fe25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::fe25519::{fe25519, fe25519_freeze};
pub fn fe25519_iseq_vartime(x: &fe25519, y: &fe25519) -> i32 {
    let mut t1: fe25519 = x.clone();
    let mut t2: fe25519 = y.clone();
    fe25519_freeze(&mut t1);
    fe25519_freeze(&mut t2);
    if t1.v[0] != t2.v[0] {
        return 0;
    }
    if t1.v[1] != t2.v[1] {
        return 0;
    }
    if t1.v[2] != t2.v[2] {
        return 0;
    }
    if t1.v[3] != t2.v[3] {
        return 0;
    }
    if t1.v[4] != t2.v[4] {
        return 0;
    }
    1
}

// impl PartialEq for Fe25519 {
//     /// 对应 fe25519_iseq_vartime
//     /// 注意：Rust 的 == 运算符通常被编译器优化，此处逻辑与 C 版一致
//     fn eq(&self, other: &Self) -> bool {
//         // 1. 拷贝并归约（对应 fe25519_freeze）
//         let mut t1 = *self;
//         let mut t2 = *other;
//         t1.freeze();
//         t2.freeze();

//         // 2. 逐位比较（Rust 数组比较在发现不等时会提前返回，即 vartime）
//         t1.v == t2.v
//     }
// }

// // 如果你需要显式的函数名：
// pub fn fe25519_iseq_vartime(x: &Fe25519, y: &Fe25519) -> i32 {
//     if x == y { 1 } else { 0 }
// }
