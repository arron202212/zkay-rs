// package org.bouncycastle.crypto;


// /**
//  * The base interface for implementations of message authentication codes (MACs).
//  */
// public interface Mac
// {
//     /**
//      * Initialise the MAC.
//      *
//      * @param params the key and other data required by the MAC.
//      * @exception IllegalArgumentException if the params argument is
//      * inappropriate.
//      */
//     public void init(CipherParameters params)
//         throws IllegalArgumentException;

//     /**
//      * Return the name of the algorithm the MAC implements.
//      *
//      * @return the name of the algorithm the MAC implements.
//      */
//     public String getAlgorithmName();

//     /**
//      * Return the block size for this MAC (in bytes).
//      *
//      * @return the block size for this MAC in bytes.
//      */
//     public int getMacSize();

//     /**
//      * add a single byte to the mac for processing.
//      *
//      * @param in the byte to be processed.
//      * @exception IllegalStateException if the MAC is not initialised.
//      */
//     public void update(byte in)
//         throws IllegalStateException;

//     /**
//      * @param in the array containing the input.
//      * @param inOff the index in the array the data begins at.
//      * @param len the length of the input starting at inOff.
//      * @exception IllegalStateException if the MAC is not initialised.
//      * @exception DataLengthException if there isn't enough data in in.
//      */
//     public void update(byte[] in, int inOff, int len)
//         throws DataLengthException, IllegalStateException;

//     /**
//      * Compute the final stage of the MAC writing the output to the out
//      * parameter.
//      * <p>
//      * doFinal leaves the MAC in the same state it was after the last init.
//      *
//      * @param out the array the MAC is to be output to.
//      * @param outOff the offset into the out buffer the output is to start at.
//      * @exception DataLengthException if there isn't enough space in out.
//      * @exception IllegalStateException if the MAC is not initialised.
//      */
//     public int doFinal(byte[] out, int outOff)
//         throws DataLengthException, IllegalStateException;

//     /**
//      * Reset the MAC. At the end of resetting the MAC should be in the
//      * in the same state it was after the last init (if there was one).
//      */
//     public void reset();
// }
/// 對應 Java 的 Mac 接口
/// 訊息鑑別碼（Message Authentication Code）的基礎特徵
pub trait Mac {
    /// 初始化 MAC
    /// 
    /// # 參數
    /// * `params`: 密鑰及相關參數（對應 CipherParameters）
    fn init(&mut self, params: &dyn CipherParameters) -> Result<(), CryptError>;

    /// 返回算法名稱（如 "HMAC/SHA-256"）
    fn get_algorithm_name(&self) -> &str;

    /// 返回 MAC 輸出的字節長度
    fn get_mac_size(&self) -> usize;

    /// 處理單個字節
    fn update_byte(&mut self, input: u8) -> Result<(), CryptError>;

    /// 處理字節塊
    /// 
    /// # 參數
    /// * `input`: 輸入數據切片
    /// * `in_off`: 起始偏移量
    /// * `len`: 數據長度
    fn update(&mut self, input: &[u8], in_off: usize, len: usize) -> Result<(), CryptError>;

    /// 計算最終的 MAC 值，寫入 output 並重置狀態
    /// 
    /// # 返回
    /// 寫入的字節數
    fn do_final(&mut self, output: &mut [u8], out_off: usize) -> Result<usize, CryptError>;

    /// 重置 MAC 狀態，使其回到 init 後的初始狀態
    fn reset(&mut self);
}
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
pub struct Hmac {
    digest: Box<dyn Digest>,
    is_initialised: bool,
    // ... 其他內部緩衝區
}

impl Mac for Hmac {
    fn init(&mut self, params: &dyn CipherParameters) -> Result<(), CryptError> {
        // 執行金鑰填充與 XOR 處理
        self.is_initialised = true;
        Ok(())
    }

    fn get_mac_size(&self) -> usize {
        self.digest.get_digest_size()
    }

    fn do_final(&mut self, output: &mut [u8], out_off: usize) -> Result<usize, CryptError> {
        if !self.is_initialised {
            return Err(CryptError::IllegalState("MAC 未初始化".into()));
        }
        // 1. 完成內部哈希 2. 執行外部哈希 3. 寫入結果
        self.reset(); 
        Ok(self.get_mac_size())
    }

    fn reset(&mut self) {
        // 重置為 init 後的狀態
    }

    fn get_algorithm_name(&self) -> &str { "HMAC" }
    // ... 實現其餘方法
}
