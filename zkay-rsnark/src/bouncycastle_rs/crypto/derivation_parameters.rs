// package org.bouncycastle.crypto;

// /**
//  * Parameters for key/byte stream derivation classes
//  */
// public interface DerivationParameters
// {
// }
/// 對應 Java 的 DerivationParameters 介面
/// 是一個標記特徵，所有金鑰/位元流派生參數類（如 KDFParameters, HKDFParameters）都需要實現它
pub trait DerivationParameters {
    // 通常為空，用於類型約束和多態分派
}
/// 基礎的 KDF 參數，包含金鑰種子 (Shared Secret) 和可選的 IV (Context)
pub struct KdfParameters {
    pub shared: Vec<u8>,
    pub iv: Option<Vec<u8>>,
}

impl DerivationParameters for KdfParameters {}

impl KdfParameters {
    pub fn new(shared: &[u8], iv: Option<&[u8]>) -> Self {
        Self {
            shared: shared.to_vec(),
            iv: iv.map(|v| v.to_vec()),
        }
    }
}
impl DerivationFunction for Kdf2BytesGenerator {
    fn init(&mut self, params: &dyn DerivationParameters) -> Result<(), CryptError> {
        // 嘗試將標記特徵轉換為具體的 KdfParameters
        // 在 Rust 中通常會配合 Any trait 或自定義轉換邏輯
        todo!("初始化哈希與種子");
    }
}
