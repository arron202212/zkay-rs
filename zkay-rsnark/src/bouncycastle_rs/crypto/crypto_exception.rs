// package org.bouncycastle.crypto;

// /**
//  * the foundation class for the hard exceptions thrown by the crypto packages.
//  */
// public class CryptoException 
//     extends Exception
// {
//     private Throwable cause;

//     /**
//      * base constructor.
//      */
//     public CryptoException()
//     {
//     }

//     /**
//      * create a CryptoException with the given message.
//      *
//      * @param message the message to be carried with the exception.
//      */
//     public CryptoException(
//         String  message)
//     {
//         super(message);
//     }

//     /**
//      * Create a CryptoException with the given message and underlying cause.
//      *
//      * @param message message describing exception.
//      * @param cause the throwable that was the underlying cause.
//      */
//     public CryptoException(
//         String  message,
//         Throwable cause)
//     {
//         super(message);

//         this.cause = cause;
//     }

//     public Throwable getCause()
//     {
//         return cause;
//     }
// }
use std::error::Error;
use std::fmt;

/// 對應 Java 的 CryptoException
#[derive(Debug)]
pub struct CryptoError {
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl CryptoError {
    /// 基礎構造函數 (Base constructor)
    pub fn new() -> Self {
        Self {
            message: String::new(),
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

    /// 帶有消息和底層原因的構造函數 (Message and underlying cause)
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
impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CryptoError: {}", self.message)
    }
}

impl Error for CryptoError {
    /// 對應 Java 的 getCause()
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // 將 Box 轉換為引用返回
        self.source.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}
fn process_crypto_data() -> Result<(), CryptoError> {
    // 模擬一個帶有原因的錯誤拋出
    let underlying_io_error = std::io::Error::new(std::io::ErrorKind::Other, "IO failure");
    
    Err(CryptoError::with_cause("加密處理失敗", underlying_io_error))
}

fn main() {
    if let Err(e) = process_crypto_data() {
        println!("發生錯誤: {}", e);
        if let Some(cause) = e.source() {
            println!("底層原因: {}", cause);
        }
    }
}
