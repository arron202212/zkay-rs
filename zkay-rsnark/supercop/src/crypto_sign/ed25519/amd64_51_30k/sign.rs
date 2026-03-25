// #include <string.h>
// #include "crypto_sign.h"
// #include "crypto_hash_sha512.h"
// #include "ge25519.h"
// #ifndef crypto_sign_ed25519_H
// #define crypto_sign_ed25519_H

// #define crypto_sign_ed25519_amd64_51_30k_SECRETKEYBYTES 64
// #define crypto_sign_ed25519_amd64_51_30k_PUBLICKEYBYTES 32
// #define crypto_sign_ed25519_amd64_51_30k_BYTES 64
// #define crypto_sign_ed25519_amd64_51_30k_DETERMINISTIC 1

// #ifdef __cplusplus
// extern "C" {
// #endif
// extern int crypto_sign_ed25519_amd64_51_30k(unsigned char *,unsigned long long *,const unsigned char *,unsigned long long,const unsigned char *);
// extern int crypto_sign_ed25519_amd64_51_30k_open(unsigned char *,unsigned long long *,const unsigned char *,unsigned long long,const unsigned char *);
// extern int crypto_sign_ed25519_amd64_51_30k_keypair(unsigned char *,unsigned char *);
// extern int crypto_sign_ed25519_amd64_51_30k_open_batch(
//     unsigned char* const m[],unsigned long long mlen[],
//     unsigned char* const sm[],const unsigned long long smlen[],
//     unsigned char* const pk[],
//     unsigned long long num
//     );
// #ifdef __cplusplus
// }
// #endif

// #define crypto_sign_ed25519 crypto_sign_ed25519_amd64_51_30k
// #define crypto_sign_ed25519_open crypto_sign_ed25519_amd64_51_30k_open
// #define crypto_sign_ed25519_keypair crypto_sign_ed25519_amd64_51_30k_keypair
// #define crypto_sign_ed25519_BYTES crypto_sign_ed25519_amd64_51_30k_BYTES
// #define crypto_sign_ed25519_SECRETKEYBYTES crypto_sign_ed25519_amd64_51_30k_SECRETKEYBYTES
// #define crypto_sign_ed25519_PUBLICKEYBYTES crypto_sign_ed25519_amd64_51_30k_PUBLICKEYBYTES
// #define crypto_sign_ed25519_DETERMINISTIC crypto_sign_ed25519_amd64_51_30k_DETERMINISTIC
// #define crypto_sign_ed25519_IMPLEMENTATION "crypto_sign/ed25519/amd64-51-30k"
// #ifndef crypto_sign_ed25519_amd64_51_30k_VERSION
// #define crypto_sign_ed25519_amd64_51_30k_VERSION "-"
// #endif
// #define crypto_sign_ed25519_VERSION crypto_sign_ed25519_amd64_51_30k_VERSION

// #endif

// #ifndef crypto_sign_H
// #define crypto_sign_H

// #include "crypto_sign_ed25519.h"

// #define crypto_sign crypto_sign_ed25519
// #define crypto_sign_open crypto_sign_ed25519_open
// #define crypto_sign_keypair crypto_sign_ed25519_keypair
// #define crypto_sign_BYTES crypto_sign_ed25519_BYTES
// #define crypto_sign_SECRETKEYBYTES crypto_sign_ed25519_SECRETKEYBYTES
// #define crypto_sign_PUBLICKEYBYTES crypto_sign_ed25519_PUBLICKEYBYTES
// #define crypto_sign_DETERMINISTIC crypto_sign_ed25519_DETERMINISTIC
// #define crypto_sign_PRIMITIVE "ed25519"
// #define crypto_sign_IMPLEMENTATION crypto_sign_ed25519_IMPLEMENTATION
// #define crypto_sign_VERSION crypto_sign_ed25519_VERSION

// #endif

use crate::crypto_hash::sha512::openssl::hash::crypto_hash_sha512;
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519::ge25519;
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519_pack::ge25519_pack;
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519_scalarmult_base::ge25519_scalarmult_base;
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519::sc25519;
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519::sc25519_add;
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519_from32bytes::sc25519_from32bytes;
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519_from64bytes::sc25519_from64bytes;
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519_mul::sc25519_mul;
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519_to32bytes::sc25519_to32bytes;

pub fn crypto_sign_ed25519_amd64_51_30k(
    sm: &mut Vec<u8>,
    smlen: &mut u64,
    m: &[u8],
    mlen: u64,
    sk: &[u8],
) -> i32 {
    // let mut pk = [0u8; 32];
    let mut az = [0u8; 64];
    let mut nonce = [0u8; 64];
    let mut hram = [0u8; 64];
    let (mut sck, mut scs, mut scsk) = (sc25519::default(), sc25519::default(), sc25519::default());
    let mut ger = ge25519::default();

    let pk = &sk[32..64];
    /* pk: 32-byte public key A */
    crypto_hash_sha512(&mut az, sk, 32);
    az[0] &= 248;
    az[31] &= 127;
    az[31] |= 64;
    /* az: 32-byte scalar a, 32-byte randomizer z */
    *smlen = mlen + 64;
    *sm = Vec::with_capacity(64 + mlen as usize);
    sm[64..64 + mlen as usize].clone_from_slice(m);
    sm[32..64].clone_from_slice(&az[32..]);
    /* sm: 32-byte uninit, 32-byte z, mlen-byte m */
    crypto_hash_sha512(&mut nonce, &mut sm[32..].to_vec(), mlen + 32);
    /* nonce: 64-byte H(z,m) */
    sc25519_from64bytes(&mut sck, &nonce);
    ge25519_scalarmult_base(&mut ger, &sck);
    ge25519_pack((&mut sm[..32]).try_into().unwrap(), &ger);
    /* sm: 32-byte R, 32-byte z, mlen-byte m */
    sm[32..64].clone_from_slice(pk);
    /* sm: 32-byte R, 32-byte A, mlen-byte m */
    crypto_hash_sha512(&mut hram, sm, mlen + 64);
    /* hram: 64-byte H(R,A,m) */
    sc25519_from64bytes(&mut scs, &hram);
    sc25519_from32bytes(&mut scsk, &az[..32].try_into().unwrap());
    let scss = scs.clone();
    sc25519_mul(&mut scs, &scss, &scsk);
    let scss = scs.clone();
    sc25519_add(&mut scs, &scss, &sck);
    /* scs: S = nonce + H(R,A,m)a */
    sc25519_to32bytes((&mut sm[32..]).try_into().unwrap(), &scs);
    /* sm: 32-byte R, 32-byte S, mlen-byte m */
    0
}

// use crate::crypto_sign::ed25519::amd64_51_30k::{ge25519::Ge25519P3, sc25519::Sc25519};
// use sha2::{Digest, Sha512};
// use std::ops::{Add, Mul};

// pub fn crypto_sign_ed25519(
//     message: &[u8],
//     sk: &[u8; 64], // 私鑰 (32B seed + 32B pk)
// ) -> Vec<u8> {
//     // 1. 提取公鑰與擴展私鑰 az
//     let pk = &sk[32..64];
//     let mut hasher = Sha512::new();
//     hasher.update(&sk[..32]);
//     let mut az = hasher.finalize(); // az[0..32]是標量，[32..64]是隨機化因子 z

//     // 標量修整 (Clamping)
//     az[0] &= 248;
//     az[31] &= 127;
//     az[31] |= 64;

//     // 2. 計算 Nonce r = H(z, M)
//     let mut hasher = Sha512::new();
//     hasher.update(&az[32..64]); // z
//     hasher.update(message); // M
//     let nonce_hash = hasher.finalize();
//     let sck = Sc25519::from_64bytes(&nonce_hash.into());

//     // 3. 計算 R = [r]G
//     let ger = Ge25519P3::scalarmult_base(&sck);
//     let r_bytes = ger.to_bytes();

//     // 4. 計算 HRAM k = H(R, A, M)
//     let mut hasher = Sha512::new();
//     hasher.update(&r_bytes);
//     hasher.update(pk);
//     hasher.update(message);
//     let hram_hash = hasher.finalize();
//     let sck_hram = Sc25519::from_64bytes(&hram_hash.into());

//     // 5. 計算 S = (r + k * a) mod L
//     let scsk = Sc25519::from_32bytes(&az[0..32].try_into().unwrap());
//     // S = sck + (sck_hram * scsk)
//     let scs = sck.add(&sck_hram.mul(&scsk));
//     let s_bytes = scs.to_32bytes();

//     // 6. 構造最終簽名 R || S || M
//     let mut sm = Vec::with_capacity(64 + message.len());
//     sm.extend_from_slice(&r_bytes);
//     sm.extend_from_slice(&s_bytes);
//     sm.extend_from_slice(message);
//     sm
// }
