// package org.bouncycastle.crypto;

// /**
//  * this exception is thrown whenever we find something we don't expect in a
//  * message.
//  */
// public class InvalidCipherTextException 
//     extends CryptoException
// {
//     /**
//      * base constructor.
//      */
//     public InvalidCipherTextException()
//     {
//     }

//     /**
//      * create a InvalidCipherTextException with the given message.
//      *
//      * @param message the message to be carried with the exception.
//      */
//     public InvalidCipherTextException(
//         String  message)
//     {
//         super(message);
//     }

//     /**
//      * create a InvalidCipherTextException with the given message.
//      *
//      * @param message the message to be carried with the exception.
//      * @param cause the root cause of the exception.
//      */
//     public InvalidCipherTextException(
//         String  message,
//         Throwable cause)
//     {
//         super(message, cause);
//     }
// }
use std::fmt;
use std::error::Error;

/// 對應 Java 的 InvalidCipherTextException
/// 當解密數據不合法（如 Padding 錯誤或 MAC 驗證失敗）時拋出
#[derive(Debug)]
pub struct InvalidCipherTextError {
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl InvalidCipherTextError {
    /// 基礎構造函數
    pub fn new() -> Self {
        Self {
            message: "Invalid cipher text".to_string(),
            source: None,
        }
    }

    /// 帶有消息的構造函數
    pub fn with_message(message: &str) -> Self {
        Self {
            message: message.to_string(),
            source: None,
        }
    }

    /// 帶有消息和底層原因的構造函數 (對應 Throwable cause)
    pub fn with_cause<E>(message: &str, cause: E) -> Self 
    where 
        E: Error + Send + Sync + 'static 
    {
        Self {
            message: message.to_string(),
            source: Some(Box::new(cause)),
        }
    }
}
impl fmt::Display for InvalidCipherTextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InvalidCipherText: {}", self.message)
    }
}

impl Error for InvalidCipherTextError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}
fn decrypt_block(data: &[u8]) -> Result<Vec<u8>, InvalidCipherTextError> {
    // 模擬 MAC 驗證失敗
    let mac_valid = false; 
    if !mac_valid {
        return Err(InvalidCipherTextError::with_message("MAC 標籤驗證失敗"));
    }
    Ok(vec![0x41, 0x42])
}
