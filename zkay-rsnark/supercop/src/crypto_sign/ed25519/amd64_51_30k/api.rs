// #define CRYPTO_SECRETKEYBYTES 64
// #define CRYPTO_PUBLICKEYBYTES 32
// #define CRYPTO_BYTES 64
// #define CRYPTO_DETERMINISTIC 1

/// 对应 #define CRYPTO_SECRETKEYBYTES 64
pub const CRYPTO_SECRETKEYBYTES: usize = 64;

/// 对应 #define CRYPTO_PUBLICKEYBYTES 32
pub const CRYPTO_PUBLICKEYBYTES: usize = 32;

/// 对应 #define CRYPTO_BYTES 64 (通常指签名长度)
pub const CRYPTO_BYTES: usize = 64;

/// 对应 #define CRYPTO_DETERMINISTIC 1 (表示算法是确定性的，如 EdDSA)
pub const CRYPTO_DETERMINISTIC: bool = true;
