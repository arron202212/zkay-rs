// package org.bouncycastle.crypto;

// public interface ExtendedDigest 
//     extends Digest
// {
//     /**
//      * Return the size in bytes of the internal buffer the digest applies it's compression
//      * function to.
//      * 
//      * @return byte length of the digests internal buffer.
//      */
//     public int getByteLength();
// }
/// 對應 Java 的 ExtendedDigest 接口
/// 擴展了 Digest，提供獲取摘要算法內部壓縮函數處理的緩衝區長度（Block Size）
pub trait ExtendedDigest: Digest {
    /// 返回摘要算法內部緩衝區的字節長度
    /// 
    /// # 返回
    /// 內部塊大小（以字節為單位），例如 SHA-256 返回 64，SHA-512 返回 128
    fn get_byte_length(&self) -> usize;
}
pub struct Sha256Digest {
    // 內部狀態...
}

impl Digest for Sha256Digest {
    fn get_algorithm_name(&self) -> &str { "SHA-256" }
    fn get_digest_size(&self) -> usize { 32 }
    fn update_byte(&mut self, _input: u8) { /* ... */ }
    fn update(&mut self, _input: &[u8], _in_off: usize, _len: usize) { /* ... */ }
    fn do_final(&mut self, _output: &mut [u8], _out_off: usize) -> usize { 32 }
    fn reset(&mut self) { /* ... */ }
}

impl ExtendedDigest for Sha256Digest {
    fn get_byte_length(&self) -> usize {
        // SHA-256 的塊大小是 512 位元 = 64 字節
        64
    }
}
