// #include "fe25519.h"
// #include "ge25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::fe25519_iseq::fe25519_iseq_vartime;
use crate::crypto_sign::ed25519::amd64_51_30k::{
    fe25519_iszero::fe25519_iszero_vartime, ge25519::ge25519_p3,
};
pub fn ge25519_isneutral_vartime(p: &ge25519_p3) -> i32 {
    if fe25519_iszero_vartime(&p.x) == 0 {
        return 0;
    }
    if fe25519_iseq_vartime(&p.y, &p.z) == 0 {
        return 0;
    }
    1
}

// impl Ge25519P3 {
//     /// 對應 ge25519_isneutral_vartime
//     pub fn is_neutral_vartime(&self) -> bool {
//         // 1. 檢查 X 是否為 0 (對應 fe25519_iszero_vartime)
//         if !self.x.is_zero_vartime() {
//             return false;
//         }

//         // 2. 檢查 Y 是否等於 Z (對應 fe25519_iseq_vartime)
//         if self.y != self.z {
//             return false;
//         }

//         true
//     }
// }

// // 模擬 C 風格函數
// pub fn ge25519_isneutral_vartime(p: &Ge25519P3) -> i32 {
//     if p.is_neutral_vartime() { 1 } else { 0 }
// }
