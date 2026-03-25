// #include "sc25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519::{sc25519, shortsc25519};
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519_from_shortsc::sc25519_from_shortsc;
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519_mul::sc25519_mul;
pub fn sc25519_mul_shortsc(r: &mut sc25519, x: &sc25519, y: &shortsc25519) {
    /* XXX: This wants to be faster */
    let mut t = sc25519::default();
    sc25519_from_shortsc(&mut t, y);
    sc25519_mul(r, x, &t);
}

// impl Sc25519 {
//     /// 對應 sc25519_mul_shortsc
//     /// 計算 self * y (mod L)
//     pub fn mul_shortsc(&self, y: &ShortSc25519) -> Self {
//         // 1. 先將短標量 (128位) 擴展為 256 位標量 (對應 sc25519_from_shortsc)
//         let t = Sc25519::from(*y);

//         // 2. 執行標量域模乘 (對應 sc25519_mul)
//         self.mul(&t)
//     }
// }

// // 模擬 C 風格函數
// pub fn sc25519_mul_shortsc(r: &mut Sc25519, x: &Sc25519, y: &ShortSc25519) {
//     *r = x.mul_shortsc(y);
// }
