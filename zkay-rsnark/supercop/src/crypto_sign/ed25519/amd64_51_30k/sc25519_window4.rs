// #include "sc25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519::sc25519;
pub fn sc25519_window4(r: &mut [i8; 64], s: &sc25519) {
    let mut carry: i8;
    // let mut i: i32;
    for i in 0..16 {
        r[i] = ((s.v[0] >> (4 * i)) & 15) as i8;
    }
    for i in 0..16 {
        r[i + 16] = ((s.v[1] >> (4 * i)) & 15) as i8;
    }
    for i in 0..16 {
        r[i + 32] = ((s.v[2] >> (4 * i)) & 15) as i8;
    }
    for i in 0..16 {
        r[i + 48] = ((s.v[3] >> (4 * i)) & 15) as i8;
    }

    /* Making it signed */
    carry = 0;
    for i in 0..63 {
        r[i] += carry;
        r[i + 1] += r[i] >> 4;
        r[i] &= 15;
        carry = r[i] >> 3;
        r[i] -= carry << 4;
    }
    r[63] += carry;
}

// impl Sc25519 {
//     /// 对应 sc25519_window4
//     /// 将标量分解为 64 个 4-bit 有符号窗口
//     pub fn to_window4(&self) -> [i8; 64] {
//         let mut r = [0i8; 64];

//         // 1. 提取原始 4-bit 块 (0-15)
//         for i in 0..16 {
//             r[i] = ((self.v[0] >> (4 * i)) & 15) as i8;
//             r[i + 16] = ((self.v[1] >> (4 * i)) & 15) as i8;
//             r[i + 32] = ((self.v[2] >> (4 * i)) & 15) as i8;
//             r[i + 48] = ((self.v[3] >> (4 * i)) & 15) as i8;
//         }

//         // 2. 转换为有符号表示 (Signed Digit Representation)
//         // 确保每个 window 在 [-8, 8] 范围内，用于恒定时间查表
//         let mut carry: i8 = 0;
//         for i in 0..63 {
//             r[i] += carry;
//             // 进位逻辑
//             carry = (r[i] + 8) >> 4;
//             r[i] -= carry << 4;
//         }
//         r[63] += carry;

//         r
//     }
// }
