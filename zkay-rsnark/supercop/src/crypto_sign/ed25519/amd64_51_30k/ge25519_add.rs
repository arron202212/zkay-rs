// #include "ge25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519::ge25519_add_p1p1;
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519::ge25519_p1p1;
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519::ge25519_p1p1_to_p3;
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519::ge25519_p3;
pub fn ge25519_add(r: &mut ge25519_p3, p: &ge25519_p3, q: &ge25519_p3) {
    let mut grp1p1 = ge25519_p1p1::default();
    ge25519_add_p1p1(&mut grp1p1, p, q);
    ge25519_p1p1_to_p3(r, &grp1p1);
}

// /// 对应 ge25519_p3 (扩展坐标: X, Y, Z, T)
// #[derive(Clone, Copy, Debug)]
// pub struct Ge25519P3 {
//     pub x: Fe25519,
//     pub y: Fe25519,
//     pub z: Fe25519,
//     pub t: Fe25519,
// }

// /// 对应 ge25519_p1p1 (中间坐标: X, Y, Z, T 但分母不同)
// pub struct Ge25519P1P1 {
//     pub x: Fe25519,
//     pub y: Fe25519,
//     pub z: Fe25519,
//     pub t: Fe25519,
// }

// use std::ops::Add;

// impl Add<&Ge25519P3> for &Ge25519P3 {
//     type Output = Ge25519P3;

//     fn add(self, other: &Ge25519P3) -> Ge25519P3 {
//         // 1. 调用底层的加法公式生成 P1P1 坐标 (对应 ge25519_add_p1p1)
//         let p1p1 = self.add_to_p1p1(other);

//         // 2. 转换回 P3 坐标 (对应 ge25519_p1p1_to_p3)
//         p1p1.to_p3()
//     }
// }

// // 模拟 C 风格函数
// pub fn ge25519_add(r: &mut Ge25519P3, p: &Ge25519P3, q: &Ge25519P3) {
//     *r = p + q;
// }
