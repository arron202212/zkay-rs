// package org.bouncycastle.crypto;

// /**
//  * Generic signer interface for hash based and message recovery signers.
//  */
// public interface Signer 
// {
//     /**
//      * Initialise the signer for signing or verification.
//      * 
//      * @param forSigning true if for signing, false otherwise
//      * @param param necessary parameters.
//      */
//     public void init(boolean forSigning, CipherParameters param);

//     /**
//      * update the internal digest with the byte b
//      */
//     public void update(byte b);

//     /**
//      * update the internal digest with the byte array in
//      */
//     public void update(byte[] in, int off, int len);

//     /**
//      * generate a signature for the message we've been loaded with using
//      * the key we were initialised with.
//      */
//     public byte[] generateSignature()
//         throws CryptoException, DataLengthException;

//     /**
//      * return true if the internal state represents the signature described
//      * in the passed in array.
//      */
//     public boolean verifySignature(byte[] signature);
    
//     /**
//      * reset the internal state
//      */
//     public void reset();
// }
/// 對應 Java 的 Signer 接口
/// 用於基於哈希（Hash-based）或具有訊息恢復能力的簽名算法
pub trait Signer {
    /// 初始化簽名器
    /// 
    /// # 參數
    /// * `for_signing`: true 用於簽名，false 用於驗證
    /// * `params`: 必要的密鑰或算法參數（對應 CipherParameters）
    fn init(&mut self, for_signing: bool, params: &dyn CipherParameters) -> Result<(), CryptError>;

    /// 使用單個位元組更新內部摘要狀態
    fn update_byte(&mut self, input: u8);

    /// 使用位元組塊更新內部摘要狀態
    /// 
    /// # 參數
    /// * `input`: 輸入數據緩衝區
    /// * `off`: 起始偏移量
    /// * `len`: 數據長度
    fn update(&mut self, input: &[u8], off: usize, len: usize);

    /// 生成簽名
    /// 
    /// # 返回
    /// 簽名字節數組 (Vec<u8>)
    /// # 錯誤
    /// 如果數據長度不合法或加密運算失敗，返回 CryptError
    fn generate_signature(&mut self) -> Result<Vec<u8>, CryptError>;

    /// 驗證簽名
    /// 
    /// # 參數
    /// * `signature`: 待驗證的簽名字節
    /// # 返回
    /// 驗證通過返回 true，否則返回 false
    fn verify_signature(&self, signature: &[u8]) -> bool;

    /// 重置內部狀態（通常會回到 init 後的初始狀態）
    fn reset(&mut self);
}
pub struct RsaSigner {
    digest: Box<dyn Digest>,
    // 其他內部狀態
}

impl Signer for RsaSigner {
    fn init(&mut self, _for_signing: bool, _params: &dyn CipherParameters) -> Result<(), CryptError> {
        self.reset();
        Ok(())
    }

    fn update_byte(&mut self, b: u8) {
        self.digest.update_byte(b);
    }

    fn update(&mut self, input: &[u8], off: usize, len: usize) {
        self.digest.update(input, off, len);
    }

    fn generate_signature(&mut self) -> Result<Vec<u8>, CryptError> {
        let mut hash = vec![0u8; self.digest.get_digest_size()];
        self.digest.do_final(&mut hash, 0);
        // 執行私鑰加密 (Signing) 邏輯...
        Ok(hash) 
    }

    fn verify_signature(&self, signature: &[u8]) -> bool {
        // 執行公鑰解密與哈希比對邏輯...
        true
    }

    fn reset(&mut self) {
        self.digest.reset();
    }
}
