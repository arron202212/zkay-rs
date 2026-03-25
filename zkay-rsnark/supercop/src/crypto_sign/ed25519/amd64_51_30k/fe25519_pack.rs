// #include "fe25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::fe25519::{fe25519, fe25519_freeze};
/* Assumes input x being reduced below 2^255 */
pub fn fe25519_pack(r: &mut [u8; 32], x: &fe25519) {
    let mut t: fe25519 = x.clone();
    fe25519_freeze(&mut t);
    r[0] = (t.v[0] & 0xff) as u8;
    r[1] = ((t.v[0] >> 8) & 0xff) as u8;
    r[2] = ((t.v[0] >> 16) & 0xff) as u8;
    r[3] = ((t.v[0] >> 24) & 0xff) as u8;
    r[4] = ((t.v[0] >> 32) & 0xff) as u8;
    r[5] = ((t.v[0] >> 40) & 0xff) as u8;
    r[6] = (t.v[0] >> 48) as u8;

    r[6] ^= ((t.v[1] << 3) & 0xf8) as u8;
    r[7] = ((t.v[1] >> 5) & 0xff) as u8;
    r[8] = ((t.v[1] >> 13) & 0xff) as u8;
    r[9] = ((t.v[1] >> 21) & 0xff) as u8;
    r[10] = ((t.v[1] >> 29) & 0xff) as u8;
    r[11] = ((t.v[1] >> 37) & 0xff) as u8;
    r[12] = (t.v[1] >> 45) as u8;

    r[12] ^= ((t.v[2] << 6) & 0xc0) as u8;
    r[13] = ((t.v[2] >> 2) & 0xff) as u8;
    r[14] = ((t.v[2] >> 10) & 0xff) as u8;
    r[15] = ((t.v[2] >> 18) & 0xff) as u8;
    r[16] = ((t.v[2] >> 26) & 0xff) as u8;
    r[17] = ((t.v[2] >> 34) & 0xff) as u8;
    r[18] = ((t.v[2] >> 42) & 0xff) as u8;
    r[19] = (t.v[2] >> 50) as u8;

    r[19] ^= ((t.v[3] << 1) & 0xfe) as u8;
    r[20] = ((t.v[3] >> 7) & 0xff) as u8;
    r[21] = ((t.v[3] >> 15) & 0xff) as u8;
    r[22] = ((t.v[3] >> 23) & 0xff) as u8;
    r[23] = ((t.v[3] >> 31) & 0xff) as u8;
    r[24] = ((t.v[3] >> 39) & 0xff) as u8;
    r[25] = (t.v[3] >> 47) as u8;

    r[25] ^= ((t.v[4] << 4) & 0xf0) as u8;
    r[26] = ((t.v[4] >> 4) & 0xff) as u8;
    r[27] = ((t.v[4] >> 12) & 0xff) as u8;
    r[28] = ((t.v[4] >> 20) & 0xff) as u8;
    r[29] = ((t.v[4] >> 28) & 0xff) as u8;
    r[30] = ((t.v[4] >> 36) & 0xff) as u8;
    r[31] = (t.v[4] >> 44) as u8;
}

// impl Fe25519 {
//     /// 对应 fe25519_pack
//     pub fn to_bytes(&self) -> [u8; 32] {
//         let mut t = *self;
//         // 1. 强制归约到 [0, 2^255-20] 范围
//         t.freeze();

//         let mut r = [0u8; 32];

//         // 2. 将 51-bit 的 v[i] 拼接到 8-bit 的字节数组中
//         r[0] = t.v[0] as u8;
//         r[1] = (t.v[0] >> 8) as u8;
//         r[2] = (t.v[0] >> 16) as u8;
//         r[3] = (t.v[0] >> 24) as u8;
//         r[4] = (t.v[0] >> 32) as u8;
//         r[5] = (t.v[0] >> 40) as u8;
//         r[6] = (t.v[0] >> 48) as u8 | (t.v[1] << 3) as u8;

//         r[7] = (t.v[1] >> 5) as u8;
//         r[8] = (t.v[1] >> 13) as u8;
//         r[9] = (t.v[1] >> 21) as u8;
//         r[10] = (t.v[1] >> 29) as u8;
//         r[11] = (t.v[1] >> 37) as u8;
//         r[12] = (t.v[1] >> 45) as u8 | (t.v[2] << 6) as u8;

//         r[13] = (t.v[2] >> 2) as u8;
//         r[14] = (t.v[2] >> 10) as u8;
//         r[15] = (t.v[2] >> 18) as u8;
//         r[16] = (t.v[2] >> 26) as u8;
//         r[17] = (t.v[2] >> 34) as u8;
//         r[18] = (t.v[2] >> 42) as u8;
//         r[19] = (t.v[2] >> 50) as u8 | (t.v[3] << 1) as u8;

//         r[20] = (t.v[3] >> 7) as u8;
//         r[21] = (t.v[3] >> 15) as u8;
//         r[22] = (t.v[3] >> 23) as u8;
//         r[23] = (t.v[3] >> 31) as u8;
//         r[24] = (t.v[3] >> 39) as u8;
//         r[25] = (t.v[3] >> 47) as u8 | (t.v[4] << 4) as u8;

//         r[26] = (t.v[4] >> 4) as u8;
//         r[27] = (t.v[4] >> 12) as u8;
//         r[28] = (t.v[4] >> 20) as u8;
//         r[29] = (t.v[4] >> 28) as u8;
//         r[30] = (t.v[4] >> 36) as u8;
//         r[31] = (t.v[4] >> 44) as u8;

//         r
//     }
// }
