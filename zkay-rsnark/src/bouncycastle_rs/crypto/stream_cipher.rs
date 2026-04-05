// package org.bouncycastle.crypto;

// /**
//  * the interface stream ciphers conform to.
//  */
// public interface StreamCipher
// {
//     /**
//      * Initialise the cipher.
//      *
//      * @param forEncryption if true the cipher is initialised for
//      *  encryption, if false for decryption.
//      * @param params the key and other data required by the cipher.
//      * @exception IllegalArgumentException if the params argument is
//      * inappropriate.
//      */
//     public void init(boolean forEncryption, CipherParameters params)
//         throws IllegalArgumentException;

//     /**
//      * Return the name of the algorithm the cipher implements.
//      *
//      * @return the name of the algorithm the cipher implements.
//      */
//     public String getAlgorithmName();

//     /**
//      * encrypt/decrypt a single byte returning the result.
//      *
//      * @param in the byte to be processed.
//      * @return the result of processing the input byte.
//      */
//     public byte returnByte(byte in);

//     /**
//      * process a block of bytes from in putting the result into out.
//      *
//      * @param in the input byte array.
//      * @param inOff the offset into the in array where the data to be processed starts.
//      * @param len the number of bytes to be processed.
//      * @param out the output buffer the processed bytes go into.
//      * @param outOff the offset into the output byte array the processed data starts at.
//      * @return the number of bytes produced - should always be len.
//      * @exception DataLengthException if the output buffer is too small.
//      */
//     public int processBytes(byte[] in, int inOff, int len, byte[] out, int outOff)
//         throws DataLengthException;

//     /**
//      * reset the cipher. This leaves it in the same state
//      * it was at after the last init (if there was one).
//      */
//     public void reset();
// }
/// 對應 Java 的 StreamCipher 接口
/// 流加密器（如 ChaCha20, Salsa20）的基礎特徵
pub trait StreamCipher {
    /// 初始化密碼器
    /// 
    /// # 參數
    /// * `for_encryption`: true 為加密，false 為解密（流加密通常兩者邏輯相同）
    /// * `params`: 密鑰、Nonce/IV 等參數
    fn init(&mut self, for_encryption: bool, params: &dyn CipherParameters) -> Result<(), CryptError>;

    /// 返回算法名稱（如 "ChaCha20"）
    fn get_algorithm_name(&self) -> &str;

    /// 處理單個字節並返回結果
    fn return_byte(&mut self, input: u8) -> u8;

    /// 處理字節塊
    /// 
    /// # 參數
    /// * `input`: 輸入數據緩衝區
    /// * `in_off`: 輸入起始偏移
    /// * `len`: 要處理的長度
    /// * `output`: 輸出數據緩衝區
    /// * `out_off`: 輸出起始偏移
    /// 
    /// # 返回
    /// 處理的字節數（對於流加密，始終等於 len）
    fn process_bytes(
        &mut self,
        input: &[u8],
        in_off: usize,
        len: usize,
        output: &mut [u8],
        out_off: usize,
    ) -> Result<usize, CryptError>;

    /// 重置密碼器狀態
    fn reset(&mut self);
}
pub struct MyStreamCipher {
    key_stream_pos: usize,
    // ... 其他狀態
}

impl StreamCipher for MyStreamCipher {
    fn init(&mut self, _for_encryption: bool, _params: &dyn CipherParameters) -> Result<(), CryptError> {
        self.reset();
        Ok(())
    }

    fn return_byte(&mut self, input: u8) -> u8 {
        let key_byte = 0xAA; // 假設的密鑰流字節
        self.key_stream_pos += 1;
        input ^ key_byte
    }

    fn process_bytes(
        &mut self,
        input: &[u8],
        in_off: usize,
        len: usize,
        output: &mut [u8],
        out_off: usize,
    ) -> Result<usize, CryptError> {
        // 邊界檢查
        if in_off + len > input.len() || out_off + len > output.len() {
            return Err(CryptError::DataLength("緩衝區太短".into()));
        }

        for i in 0..len {
            output[out_off + i] = self.return_byte(input[in_off + i]);
        }

        Ok(len)
    }

    fn reset(&mut self) {
        self.key_stream_pos = 0;
    }

    fn get_algorithm_name(&self) -> &str { "MyStreamCipher" }
}
