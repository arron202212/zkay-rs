// package org.bouncycastle.crypto;

// /**
//  * base interface for general purpose Mac based byte derivation functions.
//  */
// public interface MacDerivationFunction
//     extends DerivationFunction
// {
//     /**
//      * return the MAC used as the basis for the function
//      *
//      * @return the Mac.
//      */
//     public Mac getMac();
// }
/// 對應 Java 的 MacDerivationFunction 接口
/// 擴展了 DerivationFunction，專用於基於 MAC 的派生函數（如 HKDF）
pub trait MacDerivationFunction: DerivationFunction {
    /// 返回該派生函數所使用的底層 MAC 實例
    /// 
    /// # 返回
    /// 實現了 Mac 特徵的對象引用
    fn get_mac(&self) -> &dyn Mac;
}
pub trait Mac {
    fn get_algorithm_name(&self) -> &str;
    fn get_mac_size(&self) -> usize;
    fn init(&mut self, params: &dyn CipherParameters) -> Result<(), CryptError>;
    fn update(&mut self, input: &[u8]);
    fn do_final(&mut self, out: &mut [u8], out_off: usize) -> usize;
    fn reset(&mut self);
}
pub struct HkdfBytesGenerator {
    h_mac: Box<dyn Mac>,
    // 其他狀態...
}

impl DerivationFunction for HkdfBytesGenerator {
    fn init(&mut self, _params: &dyn DerivationParameters) -> Result<(), CryptError> {
        // HKDF 初始化邏輯
        Ok(())
    }

    fn generate_bytes(&mut self, _out: &mut [u8], _out_off: usize, _len: usize) -> Result<usize, CryptError> {
        // HKDF 提取與擴展邏輯
        Ok(0)
    }
}

impl MacDerivationFunction for HkdfBytesGenerator {
    fn get_mac(&self) -> &dyn Mac {
        self.h_mac.as_ref()
    }
}
