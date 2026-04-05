// package org.bouncycastle.crypto;

// import java.security.SecureRandom;

// /**
//  * The base class for symmetric, or secret, cipher key generators.
//  */
// public class CipherKeyGenerator
// {
//     protected SecureRandom  random;
//     protected int           strength;

//     /**
//      * initialise the key generator.
//      *
//      * @param param the parameters to be used for key generation
//      */
//     public void init(
//         KeyGenerationParameters param)
//     {
//         this.random = param.getRandom();
//         this.strength = (param.getStrength() + 7) / 8;
//     }

//     /**
//      * generate a secret key.
//      *
//      * @return a byte array containing the key value.
//      */
//     public byte[] generateKey()
//     {
//         byte[]  key = new byte[strength];

//         random.nextBytes(key);

//         return key;
//     }
// }
use rand::{RngCore, CryptoRng};

/// 對應 Java 的 CipherKeyGenerator
pub struct CipherKeyGenerator<R: RngCore + CryptoRng> {
    /// 隨機數生成器 (對應 SecureRandom)
    pub random: R,
    /// 密鑰長度（字節數）
    pub strength: usize,
}
impl<R: RngCore + CryptoRng> CipherKeyGenerator<R> {
    /// 構造函數
    pub fn new(random: R) -> Self {
        Self {
            random,
            strength: 0,
        }
    }

    /// 初始化密鑰生成器 (對應 init)
    /// 
    /// # 參數
    /// * `strength_bits`: 密鑰位數（如 128, 256）
    pub fn init(&mut self, strength_bits: usize) {
        // 將位數轉換為字節數，向上取整 (對應 (strength + 7) / 8)
        self.strength = (strength_bits + 7) / 8;
    }

    /// 生成對稱密鑰 (對應 generateKey)
    /// 
    /// # 返回
    /// 包含密鑰值的字節數組 (Vec<u8>)
    pub fn generate_key(&mut self) -> Vec<u8> {
        let mut key = vec![0u8; self.strength];
        // 填充隨機字節 (對應 random.nextBytes)
        self.random.fill_bytes(&mut key);
        key
    }
}
use rand::rngs::OsRng;

fn main() {
    let mut gen = CipherKeyGenerator::new(OsRng);
    gen.init(256); // 初始化 256-bit 密鑰生成
    let key = gen.generate_key();
    println!("Generated key length: {} bytes", key.len());
}
