// #include "sc25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519::{sc25519, sc25519_barrett};
// #define ull4_mul        crypto_sign_ed25519_amd64_51_30k_batch_ull4_mul

// extern void ull4_mul(unsigned long long r[8], const unsigned long long x[4], const unsigned long long y[4]);
unsafe extern "C" {
    /// 256位 x 256位 -> 512位 乘法
    /// r: [u64; 8], x: [u64; 4], y: [u64; 4]
    pub fn crypto_sign_ed25519_amd64_51_30k_batch_ull4_mul(
        r: *mut u64,
        x: *const u64,
        y: *const u64,
    );
}
pub fn ull4_mul(r: &mut u64, x: &[u64; 4], y: &[u64; 4]) {}
pub fn sc25519_mul(r: &mut sc25519, x: &sc25519, y: &sc25519) {
    let mut t = [0u64; 8];
    ull4_mul(&mut t[0], &x.v, &y.v);
    sc25519_barrett(r, &t);
}

// impl Sc25519 {
//     /// 對應 sc25519_mul
//     pub fn mul(&self, other: &Self) -> Self {
//         // 1. 執行 256x256 -> 512 位乘法 (對應 ull4_mul)
//         let t = self.mul_wide(other);

//         // 2. 執行 Barrett 歸約 (對應 sc25519_barrett)
//         Self::barrett_reduce(t)
//     }

//     /// 256位寬乘法 (4 limbs * 4 limbs -> 8 limbs)
//     fn mul_wide(&self, other: &Self) -> [u64; 8] {
//         let mut r = [0u64; 8];
//         for i in 0..4 {
//             let mut carry = 0u128;
//             for j in 0..4 {
//                 // 使用 u128 處理 u64 * u64 + 原有值 + 進位
//                 let product =
//                     (self.v[i] as u128) * (other.v[j] as u128) + (r[i + j] as u128) + carry;
//                 r[i + j] = product as u64;
//                 carry = product >> 64;
//             }
//             r[i + 4] = carry as u64;
//         }
//         r
//     }
// }
