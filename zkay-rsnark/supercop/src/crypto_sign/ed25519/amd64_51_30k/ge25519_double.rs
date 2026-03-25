// #include "ge25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519::{
    ge25519_dbl_p1p1, ge25519_p1p1, ge25519_p1p1_to_p3, ge25519_p3,
};
pub fn ge25519_double(r: &mut ge25519_p3, p: &ge25519_p3) {
    let mut grp1p1 = ge25519_p1p1::default();
    ge25519_dbl_p1p1(&mut grp1p1, &[p.into()]); //(ge25519_p2 *)
    ge25519_p1p1_to_p3(r, &grp1p1);
}

// impl Ge25519P3 {
//     /// 对应 ge25519_double
//     pub fn double(&self) -> Self {
//         // 1. 调用倍点专用公式 (对应 ge25519_dbl_p1p1)
//         // 注意：C 代码将 P3 强制转换为 P2 使用，因为倍点公式通常不需要 T 坐标
//         let p1p1 = self.double_to_p1p1();

//         // 2. 转换回 P3 坐标 (对应 ge25519_p1p1_to_p3)
//         p1p1.to_p3()
//     }
// }

// // 模拟 C 风格函数
// pub fn ge25519_double(r: &mut Ge25519P3, p: &Ge25519P3) {
//     *r = p.double();
// }
