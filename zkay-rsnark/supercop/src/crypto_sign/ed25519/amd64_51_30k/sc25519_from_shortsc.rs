// #include "sc25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519::{sc25519, shortsc25519};
pub fn sc25519_from_shortsc(r: &mut sc25519, x: &shortsc25519) {
    r.v[0] = x.v[0];
    r.v[1] = x.v[1];
    r.v[2] = 0;
    r.v[3] = 0;
}

// /// 假設 Sc25519 內部使用 4 個 u64 (256位) 表示
// #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
// pub struct Sc25519 {
//     pub v: [u64; 4],
// }

// /// 假設 ShortSc25519 內部使用 2 個 u64 (128位) 表示
// pub struct ShortSc25519 {
//     pub v: [u64; 2],
// }

// impl From<ShortSc25519> for Sc25519 {
//     /// 對應 sc25519_from_shortsc
//     fn from(x: ShortSc25519) -> Self {
//         Sc25519 {
//             v: [x.v[0], x.v[1], 0, 0],
//         }
//     }
// }

// // 模擬 C 風格函數
// pub fn sc25519_from_shortsc(r: &mut Sc25519, x: &ShortSc25519) {
//     *r = Sc25519::from(*x);
// }
