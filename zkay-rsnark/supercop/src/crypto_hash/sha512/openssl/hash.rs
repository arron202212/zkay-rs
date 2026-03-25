// #include <stddef.h>
// #include <openssl/sha.h>
// #include "crypto_hash_sha512.h"

// #ifndef crypto_hash_sha512_H
// #define crypto_hash_sha512_H

// #include <openssl/rand.h>
// #include <stddef.h>
// #include <openssl/opensslv.h>
// #define crypto_hash_sha512_openssl_VERSION OPENSSL_VERSION_TEXT
// #define crypto_hash_sha512_openssl_BYTES 64

// #ifdef __cplusplus
// extern "C" {
// #endif
// extern int crypto_hash_sha512_openssl(unsigned char *,const unsigned char *,unsigned long long);
// #ifdef __cplusplus
// }
// #endif

// #define crypto_hash_sha512 crypto_hash_sha512_openssl
// #define CRYPTO_HASH_SHA512_BYTES crypto_hash_sha512_openssl_BYTES
// #define crypto_hash_sha512_IMPLEMENTATION "crypto_hash/sha512/openssl"
// #ifndef crypto_hash_sha512_openssl_VERSION
// #define crypto_hash_sha512_openssl_VERSION "-"
// #endif
// #define crypto_hash_sha512_VERSION crypto_hash_sha512_openssl_VERSION

// #endif
pub const CRYPTO_HASH_SHA512_BYTES: usize = 64;

pub fn crypto_hash_sha512(out: &mut [u8; 64], ins: &[u8], _inlen: u64) -> i32 {
    crypto_hash_sha512_openssl(out, ins)
}

// use sha2::{Sha512, Digest};

// /// 对应 crypto_hash_sha512 的 Rust 实现
// pub fn crypto_hash_sha512(out: &mut [u8; 64], input: &[u8]) -> i32 {
//     // 1. 创建 Hasher 实例并处理数据
//     let mut hasher = Sha512::new();
//     hasher.update(input);

//     // 2. 完成计算并获取结果
//     let result = hasher.finalize();

//     // 3. 将结果复制到输出缓冲区
//     out.copy_from_slice(&result);

//     0 // 返回 0 表示成功
// }

use openssl::hash::{MessageDigest, hash};

pub fn crypto_hash_sha512_openssl(out: &mut [u8; 64], input: &[u8]) -> i32 {
    // 调用 OpenSSL 的 SHA512 接口
    if let Ok(digest) = hash(MessageDigest::sha512(), input) {
        out.copy_from_slice(&digest);
        0
    } else {
        -1 // 处理可能的错误
    }
}
