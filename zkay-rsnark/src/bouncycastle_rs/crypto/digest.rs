// package org.bouncycastle.crypto;

// /**
//  * interface that a message digest conforms to.
//  */
// public interface Digest
// {
//     /**
//      * return the algorithm name
//      *
//      * @return the algorithm name
//      */
//     public String getAlgorithmName();

//     /**
//      * return the size, in bytes, of the digest produced by this message digest.
//      *
//      * @return the size, in bytes, of the digest produced by this message digest.
//      */
//     public int getDigestSize();

//     /**
//      * update the message digest with a single byte.
//      *
//      * @param in the input byte to be entered.
//      */
//     public void update(byte in);

//     /**
//      * update the message digest with a block of bytes.
//      *
//      * @param in the byte array containing the data.
//      * @param inOff the offset into the byte array where the data starts.
//      * @param len the length of the data.
//      */
//     public void update(byte[] in, int inOff, int len);

//     /**
//      * close the digest, producing the final digest value. The doFinal
//      * call leaves the digest reset.
//      *
//      * @param out the array the digest is to be copied into.
//      * @param outOff the offset into the out array the digest is to start at.
//      */
//     public int doFinal(byte[] out, int outOff);

//     /**
//      * reset the digest back to it's initial state.
//      */
//     public void reset();
// }
/// 對應 Java 的 Digest 接口
pub trait Digest {
    /// 返回算法名稱 (如 "SHA-256")
    fn get_algorithm_name(&self) -> &str;

    /// 返回摘要的字節長度 (例如 SHA-256 返回 32)
    fn get_digest_size(&self) -> usize;

    /// 使用單個字節更新摘要
    fn update_byte(&mut self, input: u8);

    /// 使用字節塊更新摘要
    /// 
    /// # 參數
    /// * `input`: 輸入數據緩衝區
    /// * `in_off`: 起始偏移量
    /// * `len`: 數據長度
    fn update(&mut self, input: &[u8], in_off: usize, len: usize);

    /// 完成摘要計算，將結果寫入 output 並重置摘要狀態
    /// 
    /// # 返回
    /// 寫入的摘要字節數
    fn do_final(&mut self, output: &mut [u8], out_off: usize) -> usize;

    /// 將摘要恢復到初始狀態
    fn reset(&mut self);
}
pub struct Sha256Digest {
    // 這裡通常封裝底層的 hash 狀態
    state: Vec<u8>, 
}

impl Digest for Sha256Digest {
    fn get_algorithm_name(&self) -> &str { "SHA-256" }

    fn get_digest_size(&self) -> usize { 32 }

    fn update_byte(&mut self, input: u8) {
        // 調用底層實現更新一個字節
    }

    fn update(&mut self, input: &[u8], in_off: usize, len: usize) {
        let data = &input[in_off..in_off + len];
        // 更新數據塊
    }

    fn do_final(&mut self, output: &mut [u8], out_off: usize) -> usize {
        let size = self.get_digest_size();
        // 將結果寫入 output[out_off..out_off + size]
        self.reset();
        size
    }

    fn reset(&mut self) {
        // 重置內部狀態
    }
}
