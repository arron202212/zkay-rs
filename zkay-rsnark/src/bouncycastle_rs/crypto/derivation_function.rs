// package org.bouncycastle.crypto;

// /**
//  * base interface for general purpose byte derivation functions.
//  */
// public interface DerivationFunction
// {
//     public void init(DerivationParameters param);

//     public int generateBytes(byte[] out, int outOff, int len)
//         throws DataLengthException, IllegalArgumentException;
// }
/// 對應 Java 的 DerivationFunction 接口
/// 用於通用字節派生函數（如 KDF1, KDF2, HKDF）
pub trait DerivationFunction {
    /// 初始化派生函數
    /// 
    /// # 參數
    /// * `params`: 派生參數（如 Salt, IKM 等，對應 DerivationParameters）
    fn init(&mut self, params: &dyn DerivationParameters) -> Result<(), CryptError>;

    /// 生成派生字節並寫入輸出緩衝區
    /// 
    /// # 參數
    /// * `out`: 輸出緩衝區
    /// * `out_off`: 輸出起始偏移量
    /// * `len`: 要生成的字節長度
    /// 
    /// # 返回
    /// 實際寫入的字節數
    fn generate_bytes(&mut self, out: &mut [u8], out_off: usize, len: usize) -> Result<usize, CryptError>;
}
/// 對應 Java 的 DerivationParameters
pub trait DerivationParameters {}

#[derive(Debug)]
pub enum CryptError {
    /// 對應 IllegalArgumentException
    InvalidParameter(String),
    /// 對應 DataLengthException
    DataLength(String),
    /// 其他錯誤
    General(String),
}
pub struct Kdf2BytesGenerator {
    // 內部存儲哈希、種子等狀態
}

impl DerivationFunction for Kdf2BytesGenerator {
    fn init(&mut self, _params: &dyn DerivationParameters) -> Result<(), CryptError> {
        // 初始化邏輯
        Ok(())
    }

    fn generate_bytes(&mut self, out: &mut [u8], out_off: usize, len: usize) -> Result<usize, CryptError> {
        // 邊界檢查
        if out.len() < out_off + len {
            return Err(CryptError::DataLength("輸出緩衝區太短".into()));
        }

        // 模擬派生邏輯
        let output_slice = &mut out[out_off..out_off + len];
        for i in 0..len {
            output_slice[i] = 0xAA; // 假設的派生數據
        }
        
        Ok(len)
    }
}
