// #include <string.h>
// #include "crypto_sign.h"
// #include "crypto_verify_32.h"
// #include "crypto_hash_sha512.h"
// #include "ge25519.h"

use crate::crypto_hash::sha512::openssl::hash::crypto_hash_sha512;
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519::ge25519;
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519_double_scalarmult::ge25519_double_scalarmult_vartime;
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519_pack::ge25519_pack;
use crate::crypto_sign::ed25519::amd64_51_30k::ge25519_unpackneg::ge25519_unpackneg_vartime;
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519::sc25519;
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519_from32bytes::sc25519_from32bytes;
use crate::crypto_sign::ed25519::amd64_51_30k::sc25519_from64bytes::sc25519_from64bytes;
use crate::crypto_verify::ref_32::verify_32::crypto_verify_32;
#[inline(always)]
pub fn crypto_sign_open(m: &mut Vec<u8>, mlen: &mut u64, sm: &[u8], smlen: u64, pk: &[u8]) -> i32 {
    crypto_sign_ed25519_amd64_51_30k_open(m, mlen, sm, smlen, pk.try_into().unwrap())
}
pub fn crypto_sign_ed25519_amd64_51_30k_open(
    m: &mut Vec<u8>,
    mlen: &mut u64,
    sm: &[u8],
    smlen: u64,
    pk: &[u8; 32],
) -> i32 {
    let mut pkcopy = [0u8; 32];
    let mut rcopy = [0u8; 32];
    let mut hram = [0u8; 64];
    let mut rcheck = [0u8; 32];
    let (mut get1, mut get2) = (ge25519::default(), ge25519::default());
    let (mut schram, mut scs) = (sc25519::default(), sc25519::default());

    if smlen < 64 {
        *mlen = u64::MAX;
        *m = vec![0; smlen as usize];
        return -1;
    }
    if (sm[63] & 224) != 0 {
        *mlen = u64::MAX;
        *m = vec![0; smlen as usize];
        return -1;
    }
    if ge25519_unpackneg_vartime(&mut get1, pk) != 0 {
        *mlen = u64::MAX;
        *m = vec![0; smlen as usize];
        return -1;
    }
    pkcopy[..32].clone_from_slice(pk);
    rcopy[..32].clone_from_slice(sm);

    sc25519_from32bytes(&mut scs, (&sm[32..]).try_into().unwrap());

    m[..smlen as usize].clone_from_slice(&sm);
    m[32..].clone_from_slice(&pkcopy);
    crypto_hash_sha512(&mut hram, m, smlen);

    sc25519_from64bytes(&mut schram, &hram);

    ge25519_double_scalarmult_vartime(&mut get2, &get1, &schram, &scs);
    ge25519_pack(&mut rcheck, &get2);

    if crypto_verify_32(&rcopy, &rcheck) == 0 {
        m.rotate_left(64);
        m[smlen as usize - 64..].fill(0);
        *mlen = smlen - 64;
    }

    0
}

// use crate::crypto_sign::ed25519::amd64_51_30k::{ge25519::Ge25519P3, sc25519::Sc25519};
// use sha2::{Digest, Sha512};
// use subtle::ConstantTimeEq; // 用於恆定時間比對

// pub fn crypto_sign_verify(
//     sig_and_msg: &[u8],
//     public_key: &[u8; 32],
// ) -> Result<Vec<u8>, &'static str> {
//     let smlen = sig_and_msg.len();
//     if smlen < 64 {
//         return Err("badsig");
//     }

//     // 1. 標量範圍檢查 (sm[63] & 224 必須為 0)
//     if (sig_and_msg[63] & 0b1110_0000) != 0 {
//         return Err("badsig");
//     }

//     // 2. 解壓公鑰並取負 (對應 ge25519_unpackneg_vartime)
//     let get1 = Ge25519P3::from_bytes_neg_vartime(public_key);
//     // .ok_or("badsig")?;

//     // 3. 準備 HRAM 計算: H(R || A || M)
//     let mut hasher = Sha512::new();
//     hasher.update(&sig_and_msg[..32]); // R
//     hasher.update(public_key); // A
//     hasher.update(&sig_and_msg[64..]); // M
//     let hram_hash = hasher.finalize();

//     // 4. 轉換標量
//     let scs = Sc25519::from_32bytes(&sig_and_msg[32..64].try_into().unwrap());
//     let schram = Sc25519::from_64bytes(&hram_hash.into());

//     // 5. 執行雙標量乘法: get2 = [s]B + [k](-A)
//     // 對應 ge25519_double_scalarmult_vartime
//     let get2 = Ge25519P3::double_scalarmult_vartime(&get1, &schram, &scs);

//     // 6. 序列化並比對 R
//     let r_check = get2.to_bytes();
//     let r_original = &sig_and_msg[..32];

//     // 使用恆定時間比對 (對應 crypto_verify_32)
//     if r_check.ct_eq(r_original).into() {
//         // 驗證成功，返回原始消息
//         Ok(sig_and_msg[64..].to_vec())
//     } else {
//         Err("badsig")
//     }
// }
