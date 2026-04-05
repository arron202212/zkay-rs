// package org.bouncycastle.crypto;

// public interface RawAgreement
// {
//     void init(CipherParameters parameters);

//     int getAgreementSize();

//     void calculateAgreement(CipherParameters publicKey, byte[] buf, int off);
// }
/// 對應 Java 的 RawAgreement 接口
/// 用於直接計算並寫入原始字節格式共享秘密的協商算法
pub trait RawAgreement {
    /// 初始化協商引擎
    /// 
    /// # 參數
    /// * `params`: 包含私鑰及其相關算法參數
    fn init(&mut self, params: Box<dyn CipherParameters>) -> Result<(), CryptError>;

    /// 返回協商結果（共享秘密）的字節長度
    fn get_agreement_size(&self) -> usize;

    /// 根據對方的公鑰計算共享秘密並寫入緩衝區
    /// 
    /// # 參數
    /// * `pub_key`: 來自另一方的公鑰參數
    /// * `output`: 用於接收結果的輸出緩衝區
    /// * `off`: 寫入緩衝區的起始偏移量
    fn calculate_agreement(
        &self, 
        pub_key: Box<dyn CipherParameters>, 
        output: &mut [u8], 
        off: usize
    ) -> Result<(), CryptError>;
}
pub struct X25519Agreement {
    priv_key: Vec<u8>,
}

impl RawAgreement for X25519Agreement {
    fn init(&mut self, params: Box<dyn CipherParameters>) -> Result<(), CryptError> {
        // 從 params 提取 32 字節私鑰...
        Ok(())
    }

    fn get_agreement_size(&self) -> usize { 32 }

    fn calculate_agreement(
        &self, 
        pub_key: Box<dyn CipherParameters>, 
        output: &mut [u8], 
        off: usize
    ) -> Result<(), CryptError> {
        let size = self.get_agreement_size();
        if output.len() < off + size {
            return Err(CryptError::DataLength("輸出緩衝區太短".into()));
        }

        // 執行 X25519 乘法...
        // let secret = x25519(self.priv_key, pub_key_bytes);
        // output[off..off + size].copy_from_slice(&secret);
        
        Ok(())
    }
}
