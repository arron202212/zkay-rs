// package org.bouncycastle.crypto;

// /**
//  * The basic interface for key encapsulation mechanisms.
//  */
// public interface KeyEncapsulation
// {
//     /**
//      * Initialise the key encapsulation mechanism.
//      */
//     public void init(CipherParameters param);

//     /**
//      * Encapsulate a randomly generated session key.    
//      */
//     public CipherParameters encrypt(byte[] out, int outOff, int keyLen);
    
//     /**
//      * Decapsulate an encapsulated session key.
//      */
//     public CipherParameters decrypt(byte[] in, int inOff, int inLen, int keyLen);
// }
/// 對應 Java 的 KeyEncapsulation 接口
/// 用於實現金鑰封裝機制 (KEM)
pub trait KeyEncapsulation {
    /// 初始化 KEM 引擎
    /// 
    /// # 參數
    /// * `params`: 初始化參數（如公鑰用於加密，私鑰用於解密）
    fn init(&mut self, params: Box<dyn CipherParameters>) -> Result<(), CryptError>;

    /// 封裝（生成）一個隨機的會話金鑰
    /// 
    /// # 參數
    /// * `output`: 用於存儲封裝後的數據（如 Ciphertext）的緩衝區
    /// * `out_off`: 寫入緩衝區的起始偏移量
    /// * `key_len`: 期望生成的會話金鑰長度
    /// 
    /// # 返回
    /// 包含隨機生成的會話金鑰的參數對象 (通常是 KeyParameter)
    fn encrypt(&self, output: &mut [u8], out_off: usize, key_len: usize) -> Result<Box<dyn CipherParameters>, CryptError>;

    /// 解封裝一個已封裝的會話金鑰
    /// 
    /// # 參數
    /// * `input`: 包含封裝數據的輸入緩衝區
    /// * `in_off`: 輸入起始偏移量
    /// * `in_len`: 輸入數據長度
    /// * `key_len`: 期望解出的會話金鑰長度
    /// 
    /// # 返回
    /// 解封裝後的會話金鑰參數對象
    fn decrypt(&self, input: &[u8], in_off: usize, in_len: usize, key_len: usize) -> Result<Box<dyn CipherParameters>, CryptError>;
}
pub struct RsaKem {
    params: Option<Box<dyn CipherParameters>>,
}

impl KeyEncapsulation for RsaKem {
    fn init(&mut self, params: Box<dyn CipherParameters>) -> Result<(), CryptError> {
        self.params = Some(params);
        Ok(())
    }

    fn encrypt(&self, output: &mut [u8], _out_off: usize, key_len: usize) -> Result<Box<dyn CipherParameters>, CryptError> {
        // 1. 生成隨機數 2. 使用公鑰加密 3. 派生會話金鑰
        // 返回封裝後的 KeyParameter
        todo!()
    }

    fn decrypt(&self, input: &[u8], _in_off: usize, _in_len: usize, key_len: usize) -> Result<Box<dyn CipherParameters>, CryptError> {
        // 1. 使用私鑰解密 2. 恢復原始隨機數 3. 派生會話金鑰
        todo!()
    }
}
