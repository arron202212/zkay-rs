// package org.bouncycastle.crypto;

// /**
//  * Signer with message recovery.
//  */
// public interface SignerWithRecovery 
//     extends Signer
// {
//     /**
//      * Returns true if the signer has recovered the full message as
//      * part of signature verification.
//      * 
//      * @return true if full message recovered.
//      */
//     public boolean hasFullMessage();
    
//     /**
//      * Returns a reference to what message was recovered (if any).
//      * 
//      * @return full/partial message, null if nothing.
//      */
//     public byte[] getRecoveredMessage();

//     /**
//      * Perform an update with the recovered message before adding any other data. This must
//      * be the first update method called, and calling it will result in the signer assuming
//      * that further calls to update will include message content past what is recoverable.
//      *
//      * @param signature the signature that we are in the process of verifying.
//      * @throws IllegalStateException
//      */
//     public void updateWithRecoveredMessage(byte[] signature)
//         throws InvalidCipherTextException;
// }
/// 對應 Java 的 SignerWithRecovery 接口
/// 擴展了 Signer，支持在驗證過程中恢復原始消息
pub trait SignerWithRecovery: Signer {
    /// 檢查簽名者是否已從簽名驗證中恢復了完整消息
    fn has_full_message(&self) -> bool;

    /// 獲取已恢復的消息（如果有）
    /// 
    /// # 返回
    /// 返回 Option<Vec<u8>>，如果沒有恢復的消息則為 None
    fn get_recovered_message(&self) -> Option<&[u8]>;

    /// 使用已恢復的消息進行初始化更新
    /// 這必須是第一個被調用的 update 方法
    /// 
    /// # 參數
    /// * `signature`: 正在驗證的簽名字節
    fn update_with_recovered_message(&mut self, signature: &[u8]) -> Result<(), CryptError>;
}
pub trait Signer {
    fn init(&mut self, for_signing: bool, params: &dyn CipherParameters) -> Result<(), CryptError>;
    fn update(&mut self, input: &[u8]);
    fn generate_signature(&mut self) -> Result<Vec<u8>, CryptError>;
    fn verify_signature(&self, signature: &[u8]) -> bool;
    fn reset(&mut self);
}
pub struct Iso9796Signer {
    recovered_message: Vec<u8>,
    is_full: bool,
    // ... 其他狀態
}

impl Signer for Iso9796Signer {
    // 實現 Signer 的基本方法 ...
    fn init(&mut self, _for_signing: bool, _params: &dyn CipherParameters) -> Result<(), CryptError> { todo!() }
    fn update(&mut self, _input: &[u8]) { todo!() }
    fn generate_signature(&mut self) -> Result<Vec<u8>, CryptError> { todo!() }
    fn verify_signature(&self, _signature: &[u8]) -> bool { todo!() }
    fn reset(&mut self) { todo!() }
}

impl SignerWithRecovery for Iso9796Signer {
    fn has_full_message(&self) -> bool {
        self.is_full
    }

    fn get_recovered_message(&self) -> Option<&[u8]> {
        if self.recovered_message.is_empty() {
            None
        } else {
            Some(&self.recovered_message)
        }
    }

    fn update_with_recovered_message(&mut self, signature: &[u8]) -> Result<(), CryptError> {
        // 從簽名中解密並提取訊息塊
        // 如果提取失敗，返回 CryptError::InvalidCipherText
        Ok(())
    }
}
