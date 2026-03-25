// #include <string.h>
// #include "crypto_sign.h"
// #include "crypto_hash_sha512.h"
// #include "randombytes.h"
// #include "ge25519.h"
use crate::crypto_hash::sha512::openssl::hash::crypto_hash_sha512;
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519::ge25519;
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519_pack::ge25519_pack;
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519_scalarmult_base::ge25519_scalarmult_base;
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519::sc25519;
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519_from32bytes::sc25519_from32bytes;
use crate::randombytes::randombytes;

pub fn crypto_sign_ed25519_amd64_51_30k_keypair(pk: &mut [u8; 32], sk: &mut [u8; 32]) -> i32 {
    let mut az = [0u8; 64];
    let mut scsk = sc25519::default();
    let mut gepk = ge25519::default();

    randombytes(sk, 32);
    crypto_hash_sha512(&mut az, sk, 32);
    az[0] &= 248;
    az[31] &= 127;
    az[31] |= 64;

    sc25519_from32bytes(&mut scsk, (&az[..32]).try_into().unwrap());

    ge25519_scalarmult_base(&mut gepk, &scsk);
    ge25519_pack(pk, &gepk);
    sk[32..].copy_from_slice(pk);
    0
}

// use crate::crypto_sign::ed25519::amd64_51_30k::{ge25519::Ge25519P3, sc25519::Sc25519};
// use rand::prelude::*;
// use sha2::{Digest, Sha512}; // 替代 randombytes

// pub fn generate_keypair() -> ([u8; 32], [u8; 64]) {
//     // 1. 生成 32 字節種子 (对应 randombytes)
//     let mut seed = [0u8; 32];
//     rand::thread_rng().fill_bytes(&mut seed);

//     // 2. 派生標量並修整 (Clamping)
//     let mut hasher = Sha512::new();
//     hasher.update(seed);
//     let mut az = hasher.finalize();

//     // 對應 C 代碼的位操作
//     az[0] &= 248;
//     az[31] &= 127;
//     az[31] |= 64;

//     // 3. 計算公鑰 (对应 ge25519_scalarmult_base)
//     // 假設你已經實現了 Sc25519 和 Ge25519P3
//     let scsk = Sc25519::from_32bytes(&az[0..32].try_into().unwrap());
//     let gepk = Ge25519P3::scalarmult_base(&scsk);

//     // 4. 序列化公鑰 (对应 ge25519_pack)
//     let pk = gepk.to_bytes();

//     // 5. 組裝 64 字節私鑰 (seed + pk)
//     let mut sk = [0u8; 64];
//     sk[..32].copy_from_slice(&seed);
//     sk[32..].copy_from_slice(&pk);

//     (pk, sk)
// }
