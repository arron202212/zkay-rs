// #include "crypto_hash_sha512.h"
// #include "hram.h"
use crate::crypto_hash::sha512::openssl::hash::crypto_hash_sha512;
pub fn get_hram(hram: &mut [u8; 64], sm: &[u8], pk: &[u8], mut playground: Vec<u8>, smlen: u64) {
    playground[..32].clone_from_slice(&sm[..32]);
    playground[32..64].clone_from_slice(&pk[..32]);
    playground[64..smlen as usize].clone_from_slice(&sm[64..]);

    crypto_hash_sha512(hram, &playground, smlen);
}

// use sha2::{Digest, Sha512};

// /// 對應 get_hram 的 Rust 實現
// /// signature_r: 簽名的前 32 字節 (R)
// /// public_key: 公鑰 (A)
// /// message: 原始消息 (M)
// pub fn get_hram(signature_r: &[u8; 32], public_key: &[u8; 32], message: &[u8]) -> [u8; 64] {
//     let mut hasher = Sha512::new();

//     // 按照 R, A, M 的順序餵入數據
//     hasher.update(signature_r);
//     hasher.update(public_key);
//     hasher.update(message);

//     // 輸出 64 字節哈希值 (k)
//     hasher.finalize().into()
// }
