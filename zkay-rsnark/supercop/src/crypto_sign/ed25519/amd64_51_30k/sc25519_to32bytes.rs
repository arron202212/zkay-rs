// #include "sc25519.h"
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519::sc25519;
pub fn sc25519_to32bytes(r: &mut [u8; 32], x: &sc25519) {
    /* assuming little-endian */
    for i in 0..4 {
        // 使用 to_le_bytes 確保在任何平台上都是小端序
        let limb_bytes = x.v[i].to_le_bytes();
        // 將 8 位元組拷貝到正確的位置
        r[i * 8..(i + 1) * 8].copy_from_slice(&limb_bytes);
    }
}

// impl Sc25519 {
//     /// 對應 sc25519_to32bytes
//     pub fn to_bytes(&self) -> [u8; 32] {
//         let mut r = [0u8; 32];

//         // 遍歷 4 個 u64  limbs
//         for i in 0..4 {
//             // 使用 to_le_bytes 確保在任何平台上都是小端序
//             let limb_bytes = self.v[i].to_le_bytes();
//             // 將 8 位元組拷貝到正確的位置
//             r[i * 8..(i + 1) * 8].copy_from_slice(&limb_bytes);
//         }

//         r
//     }
// }

// // 模擬 C 風格函數原型
// pub fn sc25519_to32bytes(r: &mut [u8; 32], x: &Sc25519) {
//     *r = x.to_bytes();
// }
