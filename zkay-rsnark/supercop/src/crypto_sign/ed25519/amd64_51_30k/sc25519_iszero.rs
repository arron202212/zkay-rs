// #include "sc25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519::sc25519;
pub fn sc25519_iszero_vartime(x: &sc25519) -> i32 {
    (0..4).all(|i| x.v[i] == 0) as _
}

// impl Sc25519 {
//     /// 對應 sc25519_iszero_vartime
//     /// 判斷標量是否為 0
//     pub fn is_zero_vartime(&self) -> bool {
//         // Rust 的數組比較在發現不等時會自動提前返回 (Vartime)
//         self.v == [0u64; 4]
//     }
// }

// // 模擬 C 風格函數原型
// pub fn sc25519_iszero_vartime(x: &Sc25519) -> i32 {
//     if x.is_zero_vartime() { 1 } else { 0 }
// }
