// package org.bouncycastle.crypto;

// /**
//  * this exception is thrown whenever a cipher requires a change of key, iv
//  * or similar after x amount of bytes enciphered
//  */
// public class MaxBytesExceededException
//     extends RuntimeCryptoException
// {
//     /**
//      * base constructor.
//      */
//     public MaxBytesExceededException()
//     {
//     }

//     /**
//      * create an with the given message.
//      *
//      * @param message the message to be carried with the exception.
//      */
//     public MaxBytesExceededException(
//         String  message)
//     {
//         super(message);
//     }
// }
use std::fmt;
use std::error::Error;

/// 對應 Java 的 MaxBytesExceededException
/// 當單個密鑰處理的數據量超過安全限制（需更換密鑰或 IV）時拋出
#[derive(Debug, Clone)]
pub struct MaxBytesExceededError {
    message: String,
}

impl MaxBytesExceededError {
    /// 基礎構造函數
    pub fn new() -> Self {
        Self {
            message: "Maximum bytes exceeded for the current key/IV".to_string(),
        }
    }

    /// 帶有消息的構造函數
    pub fn with_message(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
impl fmt::Display for MaxBytesExceededError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MaxBytesExceeded: {}", self.message)
    }
}

impl Error for MaxBytesExceededError {}
pub struct MyCipher {
    bytes_processed: u64,
    max_limit: u64,
}

impl MyCipher {
    pub fn process(&mut self, data_len: usize) -> Result<(), MaxBytesExceededError> {
        if self.bytes_processed + (data_len as u64) > self.max_limit {
            return Err(MaxBytesExceededError::with_message("已達到密鑰安全上限，請更換密鑰"));
        }
        self.bytes_processed += data_len as u64;
        // 執行加密...
        Ok(())
    }
}
