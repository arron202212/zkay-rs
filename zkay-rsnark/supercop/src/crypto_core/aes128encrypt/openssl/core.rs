// #include <openssl/aes.h>
// #include "crypto_core_aes128encrypt.h"

// int crypto_core_aes128encrypt_openssl(
//         unsigned char *out,
//   const unsigned char *in,
//   const unsigned char *k,
//   const unsigned char *c
// )
// {
//   AES_KEY kexp;
//   AES_set_encrypt_key(k,128,&kexp);
//   AES_encrypt(in,out,&kexp);
//   return 0;
// }

use aes::Aes128;
use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};

/// 对应 crypto_core_aes128encrypt_openssl 的实现
pub fn crypto_core_aes128encrypt(
    out: &mut [u8; 16], // 输出缓冲区
    input: &[u8; 16],   // 输入明文块
    key: &[u8; 16],     // 128位密钥
) -> i32 {
    // 1. 初始化密钥 (对应 AES_set_encrypt_key)
    let key_ga = GenericArray::from_slice(key);
    let cipher = Aes128::new(key_ga);

    // 2. 加密单个数据块 (对应 AES_encrypt)
    let mut block = GenericArray::clone_from_slice(input);
    cipher.encrypt_block(&mut block);

    // 3. 将结果写回输出
    out.copy_from_slice(&block);

    0 // 返回 0 表示成功
}
