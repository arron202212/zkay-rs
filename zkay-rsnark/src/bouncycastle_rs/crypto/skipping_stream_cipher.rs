// package org.bouncycastle.crypto;

// /**
//  * General interface for a stream cipher that supports skipping.
//  */
// public interface SkippingStreamCipher
//     extends StreamCipher, SkippingCipher
// {
// }
/// 對應 Java 的 SkippingStreamCipher 介面
/// 繼承了 StreamCipher 和 SkippingCipher，用於支援跳轉功能的流加密器
pub trait SkippingStreamCipher: StreamCipher + SkippingCipher {
    // 這裡通常為空，因為它只是結合了兩個特徵的功能
}
pub trait StreamCipher {
    fn get_algorithm_name(&self) -> &str;
    fn init(&mut self, for_encryption: bool, params: &dyn CipherParameters) -> Result<(), CryptError>;
    fn return_byte(&mut self, input: u8) -> u8;
    fn process_bytes(&mut self, input: &[u8], in_off: usize, len: usize, output: &mut [u8], out_off: usize) -> Result<usize, CryptError>;
    fn reset(&mut self);
}
pub trait SkippingCipher {
    fn skip(&mut self, number_of_bytes: i64) -> i64;
    fn seek_to(&mut self, position: u64) -> u64;
    fn get_position(&self) -> u64;
}
pub struct ChaCha20Engine {
    // 內部狀態、計數器與密鑰流緩衝區
}

// 實作基礎流處理
impl StreamCipher for ChaCha20Engine {
    // ... 實現 init, process_bytes 等 ...
}

// 實作跳轉功能
impl SkippingCipher for ChaCha20Engine {
    fn seek_to(&mut self, position: u64) -> u64 {
        // 重新計算塊計數器 (Block Counter) 並清除當前緩衝區
        todo!("Update counter and reset buffer")
    }
    // ... 實現 skip, get_position ...
}

// 標記為 SkippingStreamCipher
impl SkippingStreamCipher for ChaCha20Engine {}
