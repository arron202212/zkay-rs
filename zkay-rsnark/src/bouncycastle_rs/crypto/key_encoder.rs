// package org.bouncycastle.crypto;

// import org.bouncycastle.crypto.params.AsymmetricKeyParameter;

// public interface KeyEncoder
// {
//     byte[] getEncoded(AsymmetricKeyParameter keyParameter);
// }
/// 對應 Java 的 KeyEncoder 接口
/// 用於將非對稱密鑰參數轉換為特定格式的字節編碼（如 DER, PEM 或原始格式）
pub trait KeyEncoder {
    /// 獲取指定密鑰參數的編碼表示
    /// 
    /// # 參數
    /// * `key_parameter`: 實現了 AsymmetricKeyParameter 特徵的密鑰引用
    /// 
    /// # 返回
    /// 編碼後的字節數組 (Vec<u8>)
    fn get_encoded(&self, key_parameter: &dyn AsymmetricKeyParameter) -> Vec<u8>;
}
pub trait AsymmetricKeyParameter {
    /// 標識這是否為私鑰
    fn is_private(&self) -> bool;
}

pub struct RawPointEncoder;

impl KeyEncoder for RawPointEncoder {
    fn get_encoded(&self, key_parameter: &dyn AsymmetricKeyParameter) -> Vec<u8> {
        // 在實際實作中，這裡會進行向下轉型 (Downcast) 
        // 以獲取具體的曲線點數據並進行編碼
        vec![0x04, 0xAA, 0xBB, 0xCC] // 示例編碼數據
    }
}

