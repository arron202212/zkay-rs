// #include "crypto_sign.h"

// #include "crypto_verify_32.h"
// #include "crypto_hash_sha512.h"
// #include "randombytes.h"

// #include "ge25519.h"
// #include "hram.h"
use crate::crypto_hash::sha512::openssl::hash::CRYPTO_HASH_SHA512_BYTES;
use crate::crypto_sign::ed25519::amd64_51_30k::open::crypto_sign_open;
use crate::crypto_sign::ed25519::amd64_51_30k::{
    ge25519::ge25519,
    ge25519_base::GE25519_BASE,
    ge25519_isneutral::ge25519_isneutral_vartime,
    ge25519_multi_scalarmult::ge25519_multi_scalarmult_vartime,
    ge25519_unpackneg::ge25519_unpackneg_vartime,
    hram::get_hram,
    sc25519::{sc25519, sc25519_add, shortsc25519},
    sc25519_from_shortsc::sc25519_from_shortsc,
    sc25519_from32bytes::sc25519_from32bytes,
    sc25519_from64bytes::sc25519_from64bytes,
    sc25519_mul_shortsc::sc25519_mul_shortsc,
};
use crate::randombytes::randombytes;
pub const MAXBATCH: usize = 64;

pub fn crypto_sign_ed25519_amd64_51_30k_open_batch(
    m: &mut [u8],
    mlen: &mut [u64],
    sm: &[u8],
    smlen: &[u64],
    pk: &[u8],
    mut num: usize,
) -> i32 {
    let mut ret: i32 = 0;
    let r = [shortsc25519::default(); MAXBATCH];
    let mut scalars = [sc25519::default(); 2 * MAXBATCH + 1];
    let mut points = [ge25519::default(); 2 * MAXBATCH + 1];
    let mut hram = [0u8; CRYPTO_HASH_SHA512_BYTES];
    let mut batchsize: usize;
    mlen[..num].fill(u64::MAX);
    let (mut m_i, mut mlen_i, mut sm_i, mut smlen_i, mut pk_i) = (0, 0, 0, 0, 0);
    while num >= 3 {
        batchsize = num;
        if batchsize > MAXBATCH {
            batchsize = MAXBATCH;
        }

        if (0..batchsize).all(|i| smlen[smlen_i + i] >= 64) {
            let mut r_bytes = vec![0u8];
            randombytes(&mut r_bytes, (size_of::<shortsc25519>() * batchsize) as u64);

            /* Computing scalars[0] = ((r1s1 + r2s2 + ...)) */
            for i in 0..batchsize {
                sc25519_from32bytes(
                    &mut scalars[i],
                    (&sm[sm_i + i..sm_i + i + 32]).try_into().unwrap(),
                );
                let scalarsi = scalars[i].clone();
                sc25519_mul_shortsc(&mut scalars[i], &scalarsi, &r[i]);
            }
            for i in 1..batchsize {
                let scalars0 = scalars[0].clone();
                let scalarsi = scalars[i].clone();
                sc25519_add(&mut scalars[0], &scalars0, &scalarsi);
            }

            /* Computing scalars[1] ... scalars[batchsize] as r[i]*H(R[i],A[i],m[m_i+i]) */
            for i in 0..batchsize {
                get_hram(
                    &mut hram,
                    &sm[sm_i + i..],
                    &pk[pk_i + i..],
                    m[m_i + i..].to_vec(),
                    smlen[smlen_i + i],
                );
                sc25519_from64bytes(&mut scalars[i + 1], &hram);
                let scalarsi1 = scalars[i + 1].clone();
                sc25519_mul_shortsc(&mut scalars[i + 1], &scalarsi1, &r[i]);
            }
            /* Setting scalars[batchsize+1] ... scalars[2*batchsize] to r[i] */
            for i in 0..batchsize {
                sc25519_from_shortsc(&mut scalars[batchsize + i + 1], &r[i]);
            }

            /* Computing points */
            points[0] = GE25519_BASE.clone();

            if !((0..batchsize).any(|i| {
                ge25519_unpackneg_vartime(
                    &mut points[i + 1],
                    (&pk[pk_i + i..pk_i + i + 32]).try_into().unwrap(),
                ) != 0
            }) || (0..batchsize).any(|i| {
                ge25519_unpackneg_vartime(
                    &mut points[batchsize + i + 1],
                    (&sm[sm_i + i..sm_i + i + 32]).try_into().unwrap(),
                ) != 0
            })) {
                let mut pointss = points.clone();
                ge25519_multi_scalarmult_vartime(
                    &mut points[0],
                    &mut pointss,
                    &mut scalars,
                    (2 * batchsize + 1) as u64,
                );

                if ge25519_isneutral_vartime(&points[0]) != 0 {
                    for i in 0..batchsize {
                        for j in 0..smlen[smlen_i + i] as usize - 64 {
                            m[m_i + i..][j] = sm[sm_i + i..][j + 64];
                        }
                        mlen[mlen_i + i] = smlen[smlen_i + i] - 64;
                    }
                }
            }
        }

        for i in 0..batchsize {
            ret |= crypto_sign_open(
                &mut m[m_i + i..].to_vec(),
                &mut mlen[mlen_i + i],
                &sm[sm_i + i..],
                smlen[smlen_i + i],
                &pk[pk_i + i..],
            );
        }

        m_i += batchsize;
        mlen_i += batchsize;
        sm_i += batchsize;
        smlen_i += batchsize;
        pk_i += batchsize;
        num -= batchsize;
    }

    for i in 0..num {
        ret |= crypto_sign_open(
            &mut m[i..].to_vec(),
            &mut mlen[i],
            &sm[i..],
            smlen[i],
            &pk[i..],
        );
    }

    ret
}

// use ed25519_dalek::{Signature, Verifier, VerifyingKey, verify_batch};

// /// 对应 C 代码中的 batch 验证逻辑
// pub fn verify_batches(
//     messages: &[&[u8]],
//     signatures: &[Signature],
//     public_keys: &[VerifyingKey],
// ) -> Result<(), ed25519_dalek::SignatureError> {
//     // ed25519-dalek 内部使用了与 C 代码相同的 MSM 优化技术
//     // 它会自动处理随机标量生成和点乘和计算
//     // batch::allow_weak_parameters_dev_parameters_only_please_do_not_use(); // 仅当需要完全兼容某些老旧实现时

//     verify_batch(messages, signatures, public_keys)
// }
