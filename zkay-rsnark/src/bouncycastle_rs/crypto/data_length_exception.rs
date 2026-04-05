// package org.bouncycastle.crypto;

// /**
//  * this exception is thrown if a buffer that is meant to have output
//  * copied into it turns out to be too short, or if we've been given 
//  * insufficient input. In general this exception will get thrown rather
//  * than an ArrayOutOfBounds exception.
//  */
// public class DataLengthException 
//     extends RuntimeCryptoException
// {
//     /**
//      * base constructor.
//      */
//     public DataLengthException()
//     {
//     }

//     /**
//      * create a DataLengthException with the given message.
//      *
//      * @param message the message to be carried with the exception.
//      */
//     public DataLengthException(
//         String  message)
//     {
//         super(message);
//     }
// }
use std::fmt;
use std::error::Error;

/// 對應 Java 的 DataLengthException
/// 當緩衝區空間不足或輸入數據長度不正確時拋出
#[derive(Debug, Clone)]
pub struct DataLengthError {
    message: String,
}

impl DataLengthError {
    /// 基礎構造函數 (Base constructor)
    pub fn new() -> Self {
        Self {
            message: String::new(),
        }
    }

    /// 帶有消息的構造函數
    pub fn with_message(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
impl fmt::Display for DataLengthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.message.is_empty() {
            write!(f, "Data length exception")
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl Error for DataLengthError {}
fn process_block(input: &[u8], output: &mut [u8]) -> Result<(), DataLengthError> {
    if output.len() < input.len() {
        return Err(DataLengthError::with_message("輸出緩衝區太短"));
    }
    // 執行處理...
    Ok(())
}
