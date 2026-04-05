// package org.bouncycastle.crypto;

// /**
//  * Interface for a converter that produces a byte encoding for a char array.
//  */
// public interface CharToByteConverter
// {
//     /**
//      * Return the type of the conversion.
//      *
//      * @return a type name for the conversion.
//      */
//     String getType();

//     /**
//      * Return a byte encoded representation of the passed in password.
//      *
//      * @param password the characters to encode.
//      * @return a byte encoding of password.
//      */
//     byte[] convert(char[] password);
// }
/// 對應 Java 的 CharToByteConverter 接口
pub trait CharToByteConverter {
    /// 返回轉換類型的名稱（如 "UTF-8", "PKCS12"）
    fn get_type(&self) -> &str;

    /// 將字符數組轉換為字節數組
    /// 
    /// # 參數
    /// * `password`: 輸入的字符切片
    fn convert(&self, password: &[char]) -> Vec<u8>;
}
pub struct Utf8Converter;

impl CharToByteConverter for Utf8Converter {
    fn get_type(&self) -> &str {
        "UTF-8"
    }

    fn convert(&self, password: &[char]) -> Vec<u8> {
        // 將 char 數組轉為 Rust 的 String (UTF-8)，再轉為 Vec<u8>
        password.iter().collect::<String>().into_bytes()
    }
}
pub struct AsciiConverter;

impl CharToByteConverter for AsciiConverter {
    fn get_type(&self) -> &str {
        "ASCII"
    }

    fn convert(&self, password: &[char]) -> Vec<u8> {
        // 僅保留低 8 位，類似 Java 的 (byte)c 強制轉換
        password.iter().map(|&c| c as u8).collect()
    }
}
