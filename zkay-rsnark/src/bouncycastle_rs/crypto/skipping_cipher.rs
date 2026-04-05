// package org.bouncycastle.crypto;

// /**
//  * Ciphers producing a key stream which can be reset to particular points in the stream implement this.
//  */
// public interface SkippingCipher
// {
//     /**
//      * Skip numberOfBytes forwards, or backwards.
//      *
//      * @param numberOfBytes the number of bytes to skip (positive forward, negative backwards).
//      * @return the number of bytes actually skipped.
//      * @throws java.lang.IllegalArgumentException if numberOfBytes is an invalid value.
//      */
//     long skip(long numberOfBytes);

//     /**
//      * Reset the cipher and then skip forward to a given position.
//      *
//      * @param position the number of bytes in to set the cipher state to.
//      * @return the byte position moved to.
//      */
//     long seekTo(long position);

//     /**
//      * Return the current "position" of the cipher
//      *
//      * @return the current byte position.
//      */
//     long getPosition();
// }
/// 對應 Java 的 SkippingCipher 接口
/// 用於支持在密鑰流中跳轉 (Skip/Seek) 的加密器
pub trait SkippingCipher {
    /// 向前或向後跳轉指定的字節數
    /// 
    /// # 參數
    /// * `number_of_bytes`: 跳轉的字節數（正數向前，負數向後）
    /// 
    /// # 返回
    /// 實際跳轉的字節數
    fn skip(&mut self, number_of_bytes: i64) -> i64;

    /// 重置加密器並跳轉到指定的絕對位置
    /// 
    /// # 參數
    /// * `position`: 目標字節位置
    /// 
    /// # 返回
    /// 移動到的字節位置
    fn seek_to(&mut self, position: u64) -> u64;

    /// 返回加密器當前的字節位置
    fn get_position(&self) -> u64;
}
pub struct AesCtrEngine {
    counter: u64,
    block_size: usize, // 16 bytes for AES
    // ... 其他狀態
}

impl SkippingCipher for AesCtrEngine {
    fn skip(&mut self, number_of_bytes: i64) -> i64 {
        let new_pos = (self.counter as i64 + number_of_bytes) as u64;
        self.seek_to(new_pos);
        number_of_bytes
    }

    fn seek_to(&mut self, position: u64) -> u64 {
        // 在 CTR 模式中，這通常涉及重新計算 Block Counter
        self.counter = position;
        self.counter
    }

    fn get_position(&self) -> u64 {
        self.counter
    }
}
