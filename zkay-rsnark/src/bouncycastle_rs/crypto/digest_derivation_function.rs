// package org.bouncycastle.crypto;

// /**
//  * base interface for general purpose Digest based byte derivation functions.
//  */
// public interface DigestDerivationFunction
//     extends DerivationFunction
// {
//     /**
//      * return the message digest used as the basis for the function
//      */
//     public Digest getDigest();
// }
/// 對應 Java 的 DigestDerivationFunction 接口
/// 擴展了 DerivationFunction，專用於基於哈希摘要的派生函數（如 KDF1, KDF2）
pub trait DigestDerivationFunction: DerivationFunction {
    /// 返回該派生函數所使用的底層哈希摘要實例
    /// 
    /// # 返回
    /// 實現了 Digest 特徵的對象引用
    fn get_digest(&self) -> &dyn Digest;
}
pub trait Digest {
    fn get_algorithm_name(&self) -> &str;
    fn get_digest_size(&self) -> usize;
    fn update(&mut self, input: &[u8]);
    fn do_final(&mut self, out: &mut [u8], out_off: usize);
    fn reset(&mut self);
}
pub struct Kdf2BytesGenerator {
    digest: Box<dyn Digest>,
    // 其他狀態...
}

impl DerivationFunction for Kdf2BytesGenerator {
    fn init(&mut self, params: &dyn DerivationParameters) -> Result<(), CryptError> {
        // 初始化邏輯...
        Ok(())
    }

    fn generate_bytes(&mut self, out: &mut [u8], out_off: usize, len: usize) -> Result<usize, CryptError> {
        // 生成邏輯...
        Ok(len)
    }
}

impl DigestDerivationFunction for Kdf2BytesGenerator {
    fn get_digest(&self) -> &dyn Digest {
        self.digest.as_ref()
    }
}
