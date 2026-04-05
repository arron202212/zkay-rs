// package org.bouncycastle.crypto;

// public interface Wrapper
// {
//     public void init(boolean forWrapping, CipherParameters param);

//     /**
//      * Return the name of the algorithm the wrapper implements.
//      *
//      * @return the name of the algorithm the wrapper implements.
//      */
//     public String getAlgorithmName();

//     public byte[] wrap(byte[] in, int inOff, int inLen);

//     public byte[] unwrap(byte[] in, int inOff, int inLen)
//         throws InvalidCipherTextException;
// }
/// 對應 Java 的 Wrapper 介面
/// 用於實現金鑰包裝（Key Wrapping）算法
pub trait Wrapper {
    /// 初始化包裝器
    /// 
    /// # 參數
    /// * `for_wrapping`: true 用於包裝（加密），false 用於解包（解密）
    /// * `params`: 密鑰及相關參數（對應 CipherParameters）
    fn init(&mut self, for_wrapping: bool, params: &dyn CipherParameters) -> Result<(), CryptError>;

    /// 返回算法名稱（如 "AESWrap"）
    fn get_algorithm_name(&self) -> &str;

    /// 包裝金鑰數據
    /// 
    /// # 參數
    /// * `input`: 待包裝的原始金鑰數據
    /// * `in_off`: 輸入起始偏移量
    /// * `in_len`: 輸入數據長度
    /// 
    /// # 返回
    /// 包裝後的字節數組 (Vec<u8>)
    fn wrap(&self, input: &[u8], in_off: usize, in_len: usize) -> Vec<u8>;

    /// 解包金鑰數據
    /// 
    /// # 返回
    /// 原始金鑰數據 (Vec<u8>)
    /// # 錯誤
    /// 如果校驗失敗（ integrity check failed），返回 InvalidCipherText 錯誤
    fn unwrap(&self, input: &[u8], in_off: usize, in_len: usize) -> Result<Vec<u8>, CryptError>;
}
