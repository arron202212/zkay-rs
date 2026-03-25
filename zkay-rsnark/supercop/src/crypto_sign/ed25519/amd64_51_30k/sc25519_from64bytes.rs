// #include "sc25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519::{sc25519, sc25519_barrett};
pub fn sc25519_from64bytes(r: &mut sc25519, x: &[u8; 64]) {
    /* assuming little-endian representation of unsigned long long */
    let mut limbs = [0u64; 8];
    for i in 0..8 {
        limbs[i] = u64::from_le_bytes(x[i * 8..(i + 1) * 8].try_into().unwrap());
    }
    sc25519_barrett(r, &limbs);
}

// impl Sc25519 {
//     /// 對應 sc25519_from64bytes
//     pub fn from_64bytes(x: &[u8; 64]) -> Self {
//         // 1. 將 64 字節解析為 8 個 u64 (512位中間值)
//         let mut limbs = [0u64; 8];
//         for i in 0..8 {
//             limbs[i] = u64::from_le_bytes(x[i * 8..(i + 1) * 8].try_into().unwrap());
//         }

//         // 2. 執行 Barrett 歸約 (對應 sc25519_barrett)
//         Self::barrett_reduce(limbs)
//     }

//     /// Barrett 歸約核心邏輯
//     fn barrett_reduce(limbs: [u64; 8]) -> Self {
//         // 這裡需要用到預計算的常數 μ = floor(2^512 / L)
//         // 以及針對 256 位數據的長乘法 (u128 累積)
//         // 最終返回歸約後的 Sc25519
//         todo!("實現具體的 Barrett 乘法與減法邏輯")
//     }
// }
