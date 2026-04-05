// package org.bouncycastle.crypto;

// /**
//  * General interface fdr classes that produce and validate commitments.
//  */
// public interface Committer
// {
//     /**
//      * Generate a commitment for the passed in message.
//      *
//      * @param message the message to be committed to,
//      * @return a Commitment
//      */
//     Commitment commit(byte[] message);

//     /**
//      * Return true if the passed in commitment represents a commitment to the passed in maessage.
//      *
//      * @param commitment a commitment previously generated.
//      * @param message the message that was expected to have been committed to.
//      * @return true if commitment matches message, false otherwise.
//      */
//     boolean isRevealed(Commitment commitment, byte[] message);
// }
/// 對應 Java 的 Committer 接口
/// 用於生成和驗證承諾（Commitments）
pub trait Committer {
    /// 為傳入的消息生成承諾
    /// 
    /// # 參數
    /// * `message`: 要承諾的消息字節數組
    /// 
    /// # 返回
    /// 包含秘密（Secret）和承諾值（Commitment）的結構體
    fn commit(&self, message: &[u8]) -> Commitment;

    /// 驗證傳入的承諾是否與消息匹配
    /// 
    /// # 參數
    /// * `commitment`: 先前生成的承諾對象
    /// * `message`: 預期被承諾的消息
    /// 
    /// # 返回
    /// 匹配則返回 true，否則返回 false
    fn is_revealed(&self, commitment: &Commitment, message: &[u8]) -> bool;
}
use sha2::{Sha256, Digest};
use rand::{RngCore, thread_rng};

pub struct HashCommitter;

impl Committer for HashCommitter {
    fn commit(&self, message: &[u8]) -> Commitment {
        // 1. 生成隨機秘密 (Salt)
        let mut salt = vec![0u8; 32];
        thread_rng().fill_bytes(&mut salt);

        // 2. 計算承諾值: H(salt || message)
        let mut hasher = Sha256::new();
        hasher.update(&salt);
        hasher.update(message);
        let commitment_value = hasher.finalize().to_vec();

        // 3. 返回承諾對象
        Commitment::new(salt, commitment_value)
    }

    fn is_revealed(&self, commitment: &Commitment, message: &[u8]) -> bool {
        let salt = commitment.get_secret();
        
        // 重新計算哈希進行比對
        let mut hasher = Sha256::new();
        hasher.update(salt);
        hasher.update(message);
        let expected = hasher.finalize();

        expected.as_slice() == commitment.get_commitment()
    }
}
