// #include <openssl/rand.h>
// #include <stddef.h>
// #include <openssl/opensslv.h>
// #define crypto_hash_sha512_openssl_VERSION OPENSSL_VERSION_TEXT
// #define crypto_hash_sha512_openssl_BYTES 64
/// 对应 #define crypto_hash_sha512_openssl_BYTES 64
pub const CRYPTO_HASH_SHA512_BYTES: usize = 64;

// /// 获取版本信息（Rust 中通常通过 Cargo 环境变量获取自身版本，
// /// 或直接硬编码对应的算法库描述）
// pub const CRYPTO_HASH_SHA512_VERSION: &str = "RustCrypto sha2 v0.10";

// use sha2::{Sha512, Digest};

// pub fn crypto_hash_sha512(data: &[u8]) -> [u8; 64] {
//     let mut hasher = Sha512::new();
//     hasher.update(data);
//     hasher.finalize().into()
// }

// use openssl::hash::{hash, MessageDigest};
// use openssl::version;

// /// 对应 #define crypto_hash_sha512_openssl_BYTES 64
// pub const CRYPTO_HASH_SHA512_BYTES: usize = 64;

// /// 对应 #define crypto_hash_sha512_openssl_VERSION OPENSSL_VERSION_TEXT
// pub fn get_openssl_version() -> &'static str {
//     version::version()
// }

// pub fn crypto_hash_sha512_openssl(data: &[u8]) -> Vec<u8> {
//     hash(MessageDigest::sha512(), data).unwrap().to_vec()
// }
