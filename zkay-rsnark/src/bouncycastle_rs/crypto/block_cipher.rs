// package org.bouncycastle.crypto;


// /**
//  * Block cipher engines are expected to conform to this interface.
//  */
// public interface BlockCipher
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
//      * Return the block size for this cipher (in bytes).
//      *
//      * @return the block size for this cipher in bytes.
//      */
//     public int getBlockSize();

//     /**
//      * Process one block of input from the array in and write it to
//      * the out array.
//      *
//      * @param in the array containing the input data.
//      * @param inOff offset into the in array the data starts at.
//      * @param out the array the output data will be copied into.
//      * @param outOff the offset into the out array the output will start at.
//      * @exception DataLengthException if there isn't enough data in in, or
//      * space in out.
//      * @exception IllegalStateException if the cipher isn't initialised.
//      * @return the number of bytes processed and produced.
//      */
//     public int processBlock(byte[] in, int inOff, byte[] out, int outOff)
//         throws DataLengthException, IllegalStateException;

//     /**
//      * Reset the cipher. After resetting the cipher is in the same state
//      * as it was after the last init (if there was one).
//      */
//     public void reset();
// }
/// 對應 Java 的 BlockCipher 接口
pub trait BlockCipher {
    /// 初始化密碼器
    /// 
    /// # 參數
    /// * `for_encryption`: true 為加密，false 為解密
    /// * `params`: 密鑰及參數（對應 CipherParameters）
    fn init(&mut self, for_encryption: bool, params: &dyn CipherParameters) -> Result<(), CryptError>;

    /// 返回算法名稱
    fn get_algorithm_name(&self) -> &str;

    /// 返回分組大小（字節）
    fn get_block_size(&self) -> usize;

    /// 處理單個分組
    /// 
    /// # 參數
    /// * `input`: 輸入數據緩衝區
    /// * `in_off`: 輸入起始偏移
    /// * `output`: 輸出數據緩衝區
    /// * `out_off`: 輸出起始偏移
    /// 
    /// # 返回
    /// 處理的字節數（通常等於 get_block_size）
    fn process_block(
        &self, 
        input: &[u8], 
        in_off: usize, 
        output: &mut [u8], 
        out_off: usize
    ) -> Result<usize, CryptError>;

    /// 重置密碼器狀態
    fn reset(&mut self);
}
/// 對應 Java 的 CipherParameters
pub trait CipherParameters {}

#[derive(Debug)]
pub enum CryptError {
    /// 對應 IllegalArgumentException
    InvalidParameter(String),
    /// 對應 DataLengthException
    DataLength(String),
    /// 對應 IllegalStateException
    IllegalState(String),
}
pub struct AesEngine {
    key_schedule: Vec<u32>,
    is_initialised: bool,
}

impl BlockCipher for AesEngine {
    fn init(&mut self, _for_encryption: bool, _params: &dyn CipherParameters) -> Result<(), CryptError> {
        self.is_initialised = true;
        // 執行密鑰擴展邏輯...
        Ok(())
    }

    fn get_block_size(&self) -> usize { 16 }

    fn process_block(&self, input: &[u8], in_off: usize, output: &mut [u8], out_off: usize) -> Result<usize, CryptError> {
        if !self.is_initialised {
            return Err(CryptError::IllegalState("Cipher not init".into()));
        }
        let block_size = self.get_block_size();
        // 邊界檢查
        if input.len() < in_off + block_size || output.len() < out_off + block_size {
            return Err(CryptError::DataLength("Buffer too small".into()));
        }
        
        // 執行加密/解密...
        Ok(block_size)
    }

    fn reset(&mut self) { //清除敏感數據或恢復初始向量 }

    fn get_algorithm_name(&self) -> &str { "AES" }
}
