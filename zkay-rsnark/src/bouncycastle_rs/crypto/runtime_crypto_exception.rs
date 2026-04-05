// package org.bouncycastle.crypto;

// /**
//  * the foundation class for the exceptions thrown by the crypto packages.
//  */
// public class RuntimeCryptoException 
//     extends RuntimeException
// {
//     /**
//      * base constructor.
//      */
//     public RuntimeCryptoException()
//     {
//     }

//     /**
//      * create a RuntimeCryptoException with the given message.
//      *
//      * @param message the message to be carried with the exception.
//      */
//     public RuntimeCryptoException(
//         String  message)
//     {
//         super(message);
//     }
// }
use std::fmt;
use std::error::Error;

/// 對應 Java 的 RuntimeCryptoException
/// 在 Rust 中，所有錯誤都應透過 Result 顯式處理，不存在「非受檢」異常
#[derive(Debug, Clone)]
pub struct RuntimeCryptoError {
    message: String,
}

impl RuntimeCryptoError {
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
impl fmt::Display for RuntimeCryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.message.is_empty() {
            write!(f, "Runtime crypto exception")
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl Error for RuntimeCryptoError {}
fn get_cipher_name(id: u32) -> Result<&'static str, RuntimeCryptoError> {
    match id {
        1 => Ok("AES"),
        2 => Ok("DES"),
        _ => Err(RuntimeCryptoError::with_message("不支持的加密算法 ID")),
    }
}
