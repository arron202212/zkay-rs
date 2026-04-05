// package org.bouncycastle.crypto;


// /**
//  * base interface that a public/private key block cipher needs
//  * to conform to.
//  */
// public interface AsymmetricBlockCipher
// {
//     /**
//      * initialise the cipher.
//      *
//      * @param forEncryption if true the cipher is initialised for 
//      *  encryption, if false for decryption.
//      * @param param the key and other data required by the cipher.
//      */
//     public void init(boolean forEncryption, CipherParameters param);

//     /**
//      * returns the largest size an input block can be.
//      *
//      * @return maximum size for an input block.
//      */
//     public int getInputBlockSize();

//     /**
//      * returns the maximum size of the block produced by this cipher.
//      *
//      * @return maximum size of the output block produced by the cipher.
//      */
//     public int getOutputBlockSize();

//     /**
//      * process the block of len bytes stored in in from offset inOff.
//      *
//      * @param in the input data
//      * @param inOff offset into the in array where the data starts
//      * @param len the length of the block to be processed.
//      * @return the resulting byte array of the encryption/decryption process.
//      * @exception InvalidCipherTextException data decrypts improperly.
//      * @exception DataLengthException the input data is too large for the cipher.
//      */
//     public byte[] processBlock(byte[] in, int inOff, int len)
//         throws InvalidCipherTextException;
// }
/// 對應 Java 的 AsymmetricBlockCipher 接口
pub trait AsymmetricBlockCipher {
    /// 初始化密碼器
    /// 
    /// # 參數
    /// * `for_encryption`: 若為 true 則初始化為加密模式，false 則為解密模式
    /// * `params`: 密鑰及其他必要的加密參數（對應 CipherParameters）
    fn init(&mut self, for_encryption: bool, params: Box<dyn CipherParameters>) -> Result<(), CryptError>;

    /// 返回輸入塊的最大長度
    fn get_input_block_size(&self) -> usize;

    /// 返回輸出塊的最大長度
    fn get_output_block_size(&self) -> usize;

    /// 處理數據塊
    /// 
    /// # 參數
    /// * `input`: 輸入數據緩衝區
    /// * `in_off`: 輸入數據的起始偏移量
    /// * `len`: 要處理的數據長度
    /// 
    /// # 返回
    /// 處理後的結果字節數組 (Vec<u8>)
    fn process_block(&self, input: &[u8], in_off: usize, len: usize) -> Result<Vec<u8>, CryptError>;
}
/// 對應 Java 的 CipherParameters 接口
pub trait CipherParameters {}

/// 統一的加密錯誤類型，取代 Java 的各種 Exception
#[derive(Debug)]
pub enum CryptError {
    InvalidCipherText(String),
    DataLength(String),
    InvalidKey(String),
    General(String),
}

impl std::fmt::Display for CryptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for CryptError {}
