// package org.bouncycastle.crypto;

// /**
//  * With FIPS PUB 202 a new kind of message digest was announced which supported extendable output, or variable digest sizes.
//  * This interface provides the extra method required to support variable output on an extended digest implementation.
//  */
// public interface Xof
//     extends ExtendedDigest
// {
//     /**
//      * Output the results of the final calculation for this digest to outLen number of bytes.
//      *
//      * @param out output array to write the output bytes to.
//      * @param outOff offset to start writing the bytes at.
//      * @param outLen the number of output bytes requested.
//      * @return the number of bytes written
//      */
//     int doFinal(byte[] out, int outOff, int outLen);

//     /**
//      * Start outputting the results of the final calculation for this digest. Unlike doFinal, this method
//      * will continue producing output until the Xof is explicitly reset, or signals otherwise.
//      *
//      * @param out output array to write the output bytes to.
//      * @param outOff offset to start writing the bytes at.
//      * @param outLen the number of output bytes requested.
//      * @return the number of bytes written
//      */
//     int doOutput(byte[] out, int outOff, int outLen);
// }
/// 對應 Java 的 Xof 接口
/// 支持可變長度輸出的摘要算法（如 SHAKE, BLAKE2X）
pub trait Xof: ExtendedDigest {
    /// 完成計算並輸出指定長度的字節，調用後會重置狀態
    /// 
    /// # 參數
    /// * `output`: 輸出緩衝區
    /// * `out_off`: 起始偏移量
    /// * `out_len`: 請求輸出的字節數
    fn do_final(&mut self, output: &mut [u8], out_off: usize, out_len: usize) -> usize;

    /// 開始持續輸出結果。與 do_final 不同，此方法不會自動重置，
    /// 可以多次調用以獲取連續的密鑰流，直到手動調用 reset。
    fn do_output(&mut self, output: &mut [u8], out_off: usize, out_len: usize) -> usize;
}
pub struct Shake256 {
    // 內部海綿狀態 (Sponge State)
    state: [u64; 25],
    is_squeezing: bool,
}

impl Digest for Shake256 {
    fn get_algorithm_name(&self) -> &str { "SHAKE256" }
    fn get_digest_size(&self) -> usize { 32 } // 默認大小
    fn update(&mut self, input: &[u8], _off: usize, _len: usize) {
        // 吸入數據到海綿狀態
    }
    // ...
}

impl ExtendedDigest for Shake256 {
    fn get_byte_length(&self) -> usize { 136 } // SHAKE256 的 Rate
}

impl Xof for Shake256 {
    fn do_output(&mut self, output: &mut [u8], out_off: usize, out_len: usize) -> usize {
        self.is_squeezing = true;
        // 從海綿狀態中「擠出」指定長度的數據
        // output[out_off..out_off + out_len].copy_from_slice(...)
        out_len
    }

    fn do_final(&mut self, output: &mut [u8], out_off: usize, out_len: usize) -> usize {
        let read = self.do_output(output, out_off, out_len);
        self.reset();
        read
    }
}
