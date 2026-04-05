// package org.bouncycastle.crypto;

// /**
//  * all parameter classes implement this.
//  */
// public interface CipherParameters
// {
// }
/// 對應 Java 的 CipherParameters 接口
/// 這是一個標記特徵，所有加密參數類（如 KeyParameter, ParametersWithIV）都需要實現它
pub trait CipherParameters {
    // 通常為空，僅用於類型約束
}
pub struct KeyParameter {
    pub key: Vec<u8>,
}

impl CipherParameters for KeyParameter {}

impl KeyParameter {
    pub fn new(key: &[u8]) -> Self {
        Self { key: key.to_vec() }
    }
}
pub struct ParametersWithIV<'a> {
    pub parameters: &'a dyn CipherParameters,
    pub iv: Vec<u8>,
}

impl<'a> CipherParameters for ParametersWithIV<'a> {}
