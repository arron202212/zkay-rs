// package org.bouncycastle.crypto;

// import java.security.SecureRandom;

// /**
//  * The base class for parameters to key generators.
//  */
// public class KeyGenerationParameters
// {
//     private SecureRandom    random;
//     private int             strength;

//     /**
//      * initialise the generator with a source of randomness
//      * and a strength (in bits).
//      *
//      * @param random the random byte source.
//      * @param strength the size, in bits, of the keys we want to produce.
//      */
//     public KeyGenerationParameters(
//         SecureRandom    random,
//         int             strength)
//     {
//         this.random = random;
//         this.strength = strength;
//     }

//     /**
//      * return the random source associated with this
//      * generator.
//      *
//      * @return the generators random source.
//      */
//     public SecureRandom getRandom()
//     {
//         return random;
//     }

//     /**
//      * return the bit strength for keys produced by this generator,
//      *
//      * @return the strength of the keys this generator produces (in bits).
//      */
//     public int getStrength()
//     {
//         return strength;
//     }
// }
use rand::{CryptoRng, RngCore};

/// 對應 Java 的 KeyGenerationParameters
/// 封裝密鑰生成所需的隨機源和強度（位元數）
pub struct KeyGenerationParameters<R: RngCore + CryptoRng> {
    random: R,
    strength: usize,
}

impl<R: RngCore + CryptoRng> KeyGenerationParameters<R> {
    /// 基礎構造函數 (Base constructor)
    /// 
    /// # 參數
    /// * `random`: 密碼學安全的隨機位元源
    /// * `strength`: 欲生成的密鑰強度（以位元為單位，如 256）
    pub fn new(random: R, strength: usize) -> Self {
        Self {
            random,
            strength,
        }
    }

    /// 獲取隨機源 (getRandom)
    pub fn get_random(&mut self) -> &mut R {
        &mut self.random
    }

    /// 獲取密鑰強度 (getStrength)
    pub fn get_strength(&self) -> usize {
        self.strength
    }
}
use rand::rngs::OsRng;

// 創建 2048 位元密鑰生成的參數
let params = KeyGenerationParameters::new(OsRng, 2048);

println!("Key strength: {} bits", params.get_strength());
