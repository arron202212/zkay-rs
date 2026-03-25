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

/// 签名与 C 代码基本一致
pub fn crypto_hash_sha256_sphlib(out: &mut [u8; 32], input: &[u8]) -> i32 {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();

    // 将结果拷贝到输出缓冲区
    out.copy_from_slice(&result);
    0 // 返回 0 表示成功
}
