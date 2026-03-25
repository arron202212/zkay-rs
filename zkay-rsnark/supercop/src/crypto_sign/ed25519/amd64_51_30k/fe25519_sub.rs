// #include "fe25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::fe25519::fe25519;
pub fn fe25519_sub(r: &mut fe25519, x: &fe25519, y: &fe25519) {
    let mut yt: fe25519 = y.clone();
    /* Not required for reduced input */
    let mut t: u64;
    t = yt.v[0] >> 51;
    yt.v[0] &= 2251799813685247;
    yt.v[1] += t;

    t = yt.v[1] >> 51;
    yt.v[1] &= 2251799813685247;
    yt.v[2] += t;

    t = yt.v[2] >> 51;
    yt.v[2] &= 2251799813685247;
    yt.v[3] += t;

    t = yt.v[3] >> 51;
    yt.v[3] &= 2251799813685247;
    yt.v[4] += t;

    t = yt.v[4] >> 51;
    yt.v[4] &= 2251799813685247;
    yt.v[0] += 19 * t;

    r.v[0] = x.v[0] + 0xFFFFFFFFFFFDA - yt.v[0];
    r.v[1] = x.v[1] + 0xFFFFFFFFFFFFE - yt.v[1];
    r.v[2] = x.v[2] + 0xFFFFFFFFFFFFE - yt.v[2];
    r.v[3] = x.v[3] + 0xFFFFFFFFFFFFE - yt.v[3];
    r.v[4] = x.v[4] + 0xFFFFFFFFFFFFE - yt.v[4];
}

// use std::ops::Sub;

// impl Sub for Fe25519 {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self {
//         let mut yt = other;
//         let mask: u64 = (1 << 51) - 1; // 即 2251799813685247

//         // 1. 对 y 进行进位处理 (Carry propagation)
//         let mut t: u64;

//         t = yt.v[0] >> 51;
//         yt.v[0] &= mask;
//         yt.v[1] += t;

//         t = yt.v[1] >> 51;
//         yt.v[1] &= mask;
//         yt.v[2] += t;

//         t = yt.v[2] >> 51;
//         yt.v[2] &= mask;
//         yt.v[3] += t;

//         t = yt.v[3] >> 51;
//         yt.v[3] &= mask;
//         yt.v[4] += t;

//         t = yt.v[4] >> 51;
//         yt.v[4] &= mask;
//         yt.v[0] += 19 * t;

//         // 2. 相减并加上模数的倍数以防负数
//         // 使用 wrapping 方法确保在任何模式下行为与 C 一致
//         Fe25519 {
//             v: [
//                 self.v[0]
//                     .wrapping_add(0x3FFFFFFFFFFFFDA)
//                     .wrapping_sub(yt.v[0]),
//                 self.v[1]
//                     .wrapping_add(0x3FFFFFFFFFFFFFE)
//                     .wrapping_sub(yt.v[1]),
//                 self.v[2]
//                     .wrapping_add(0x3FFFFFFFFFFFFFE)
//                     .wrapping_sub(yt.v[2]),
//                 self.v[3]
//                     .wrapping_add(0x3FFFFFFFFFFFFFE)
//                     .wrapping_sub(yt.v[3]),
//                 self.v[4]
//                     .wrapping_add(0x3FFFFFFFFFFFFFE)
//                     .wrapping_sub(yt.v[4]),
//             ],
//         }
//     }
// }
