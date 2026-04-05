// package org.bouncycastle.crypto;

// public class OutputLengthException
//     extends DataLengthException
// {
//     public OutputLengthException(String msg)
//     {
//         super(msg);
//     }
// }
use std::fmt;
use std::error::Error;

/// 對應 Java 的 OutputLengthException
/// 當輸出緩衝區（Output Buffer）空間不足以存放處理結果時拋出
#[derive(Debug, Clone)]
pub struct OutputLengthError {
    message: String,
}

impl OutputLengthError {
    /// 帶有消息的構造函數
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
impl fmt::Display for OutputLengthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OutputLengthError: {}", self.message)
    }
}

impl Error for OutputLengthError {}
#[derive(Debug)]
pub enum CryptoError {
    DataLength(String),
    OutputLength(String), // 對標 OutputLengthException
    InvalidCipherText(String),
}

fn encrypt(input: &[u8], output: &mut [u8]) -> Result<usize, CryptoError> {
    let required_len = input.len() + 16; // 假設需要額外空間
    if output.len() < required_len {
        return Err(CryptoError::OutputLength(
            format!("輸出緩衝區太短: 需要 {}，但只有 {}", required_len, output.len())
        ));
    }
    // 執行加密邏輯...
    Ok(required_len)
}
