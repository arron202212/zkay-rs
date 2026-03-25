// #include "crypto_hash_sha256.h"
// #include "sph_sha2.h"

// int crypto_hash_sha256_sphlib(unsigned char *out,const unsigned char *in,unsigned long long inlen)
// {
//   sph_sha256_context mc;
//   sph_sha256_init(&mc);
//   sph_sha256(&mc, in, inlen);
//   sph_sha256_close(&mc,out);
//   return 0;
// }

use sha2::{Digest, Sha256};

/// 对应 crypto_hash_sha256_sphlib 的 Rust 实现
pub fn crypto_hash_sha256(input: &[u8]) -> [u8; 32] {
    // 1. 初始化上下文 (对应 sph_sha256_init)
    let mut hasher = Sha256::new();

    // 2. 处理输入数据 (对应 sph_sha256)
    // Rust 的切片 &[u8] 天然包含了数据指针和长度 inlen
    hasher.update(input);

    // 3. 完成计算并获取结果 (对应 sph_sha256_close)
    // finalize() 返回一个 GenericArray，通过 .into() 转为 [u8; 32]
    hasher.finalize().into()
}

// 如果你需要完全模仿 C 的返回风格（虽然在 Rust 中不常用）：
pub fn crypto_hash_sha256_sphlib_style(out: &mut [u8; 32], input: &[u8]) -> i32 {
    let mut hasher = Sha256::new();
    hasher.update(input);
    out.copy_from_slice(&hasher.finalize());
    0 // 返回 0 表示成功
}
