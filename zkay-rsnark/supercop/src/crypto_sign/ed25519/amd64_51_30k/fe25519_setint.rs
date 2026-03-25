// #include "fe25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::fe25519::fe25519;
pub fn fe25519_setint(r: &mut fe25519, v: u32) {
    r.v[0] = v as u64;
    r.v[1] = 0;
    r.v[2] = 0;
    r.v[3] = 0;
    r.v[4] = 0;
}

// impl From<u32> for Fe25519 {
//     /// 对应 fe25519_setint
//     fn from(v: u32) -> Self {
//         Fe25519 {
//             v: [v as u64, 0, 0, 0, 0],
//         }
//     }
// }

// // 或者定義為方法
// impl Fe25519 {
//     pub fn set_int(&mut self, v: u32) {
//         self.v[0] = v as u64;
//         self.v[1] = 0;
//         self.v[2] = 0;
//         self.v[3] = 0;
//         self.v[4] = 0;
//     }
// }

// // 模擬 C 風格函數
// pub fn fe25519_setint(r: &mut Fe25519, v: u32) {
//     r.set_int(v);
// }
