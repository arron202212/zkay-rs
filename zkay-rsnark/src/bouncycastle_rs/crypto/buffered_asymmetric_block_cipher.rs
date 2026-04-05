// package org.bouncycastle.crypto;

// /**
//  * a buffer wrapper for an asymmetric block cipher, allowing input
//  * to be accumulated in a piecemeal fashion until final processing.
//  */
// public class BufferedAsymmetricBlockCipher
// {
//     protected byte[]        buf;
//     protected int           bufOff;

//     private final AsymmetricBlockCipher   cipher;

//     /**
//      * base constructor.
//      *
//      * @param cipher the cipher this buffering object wraps.
//      */
//     public BufferedAsymmetricBlockCipher(
//         AsymmetricBlockCipher     cipher)
//     {
//         this.cipher = cipher;
//     }

//     /**
//      * return the underlying cipher for the buffer.
//      *
//      * @return the underlying cipher for the buffer.
//      */
//     public AsymmetricBlockCipher getUnderlyingCipher()
//     {
//         return cipher;
//     }

//     /**
//      * return the amount of data sitting in the buffer.
//      *
//      * @return the amount of data sitting in the buffer.
//      */
//     public int getBufferPosition()
//     {
//         return bufOff;
//     }

//     /**
//      * initialise the buffer and the underlying cipher.
//      *
//      * @param forEncryption if true the cipher is initialised for
//      *  encryption, if false for decryption.
//      * @param params the key and other data required by the cipher.
//      */
//     public void init(
//         boolean             forEncryption,
//         CipherParameters    params)
//     {
//         reset();

//         cipher.init(forEncryption, params);

//         //
//         // we allow for an extra byte where people are using their own padding
//         // mechanisms on a raw cipher.
//         //
//         buf = new byte[cipher.getInputBlockSize() + (forEncryption ? 1 : 0)];
//         bufOff = 0;
//     }

//     /**
//      * returns the largest size an input block can be.
//      *
//      * @return maximum size for an input block.
//      */
//     public int getInputBlockSize()
//     {
//         return cipher.getInputBlockSize();
//     }

//     /**
//      * returns the maximum size of the block produced by this cipher.
//      *
//      * @return maximum size of the output block produced by the cipher.
//      */
//     public int getOutputBlockSize()
//     {
//         return cipher.getOutputBlockSize();
//     }

//     /**
//      * add another byte for processing.
//      * 
//      * @param in the input byte.
//      */
//     public void processByte(
//         byte        in)
//     {
//         if (bufOff >= buf.length)
//         {
//             throw new DataLengthException("attempt to process message too long for cipher");
//         }

//         buf[bufOff++] = in;
//     }

//     /**
//      * add len bytes to the buffer for processing.
//      *
//      * @param in the input data
//      * @param inOff offset into the in array where the data starts
//      * @param len the length of the block to be processed.
//      */
//     public void processBytes(
//         byte[]      in,
//         int         inOff,
//         int         len)
//     {
//         if (len == 0)
//         {
//             return;
//         }

//         if (len < 0)
//         {
//             throw new IllegalArgumentException("Can't have a negative input length!");
//         }

//         if (bufOff + len > buf.length)
//         {
//             throw new DataLengthException("attempt to process message too long for cipher");
//         }

//         System.arraycopy(in, inOff, buf, bufOff, len);
//         bufOff += len;
//     }

//     /**
//      * process the contents of the buffer using the underlying
//      * cipher.
//      *
//      * @return the result of the encryption/decryption process on the
//      * buffer.
//      * @exception InvalidCipherTextException if we are given a garbage block.
//      */
//     public byte[] doFinal()
//         throws InvalidCipherTextException
//     {
//         byte[] out = cipher.processBlock(buf, 0, bufOff);

//         reset();

//         return out;
//     }

//     /**
//      * Reset the buffer and the underlying cipher.
//      */
//     public void reset()
//     {
//         /*
//          * clean the buffer.
//          */
//         if (buf != null)
//         {
//             for (int i = 0; i < buf.length; i++)
//             {
//                 buf[i] = 0;
//             }
//         }

//         bufOff = 0;
//     }
// }


/// 對應 Java 的 BufferedAsymmetricBlockCipher
pub struct BufferedAsymmetricBlockCipher {
    /// 內部緩衝區
    buf: Vec<u8>,
    /// 當前緩衝區寫入位置
    buf_off: usize,
    /// 底層的非對稱加密器
    cipher: Box<dyn AsymmetricBlockCipher>,
}
impl BufferedAsymmetricBlockCipher {
    /// 基礎構造函數
    pub fn new(cipher: Box<dyn AsymmetricBlockCipher>) -> Self {
        Self {
            buf: Vec::new(),
            buf_off: 0,
            cipher,
        }
    }

    /// 初始化緩衝區與底層加密器
    pub fn init(&mut self, for_encryption: bool, params: Box<dyn CipherParameters>) -> Result<(), CryptError> {
        self.reset();
        self.cipher.init(for_encryption, params)?;

        // 為緩衝區分配空間。加密時多預留 1 字節以支持某些原始填充
        let buf_size = self.cipher.get_input_block_size() + if for_encryption { 1 } else { 0 };
        self.buf = vec![0u8; buf_size];
        self.buf_off = 0;
        Ok(())
    }

    /// 處理單個字節
    pub fn process_byte(&mut self, input: u8) -> Result<(), CryptError> {
        if self.buf_off >= self.buf.len() {
            return Err(CryptError::DataLength("Message too long for cipher".into()));
        }
        self.buf[self.buf_off] = input;
        self.buf_off += 1;
        Ok(())
    }

    /// 處理多個字節
    pub fn process_bytes(&mut self, input: &[u8]) -> Result<(), CryptError> {
        if input.is_empty() {
            return Ok(());
        }
        if self.buf_off + input.len() > self.buf.len() {
            return Err(CryptError::DataLength("Message too long for cipher".into()));
        }

        self.buf[self.buf_off..self.buf_off + input.len()].copy_from_slice(input);
        self.buf_off += input.len();
        Ok(())
    }

    /// 執行最終的加密/解密處理
    pub fn do_final(&mut self) -> Result<Vec<u8>, CryptError> {
        // 調用底層 cipher 處理緩衝區內容
        let out = self.cipher.process_block(&self.buf, 0, self.buf_off)?;
        self.reset();
        Ok(out)
    }

    /// 重置緩衝區與加密器狀態
    pub fn reset(&mut self) {
        // 安全地清除緩衝區內容（防止敏感數據殘留）
        for byte in &mut self.buf {
            *byte = 0;
        }
        self.buf_off = 0;
    }

    // Getter 方法
    pub fn get_underlying_cipher(&self) -> &dyn AsymmetricBlockCipher { self.cipher.as_ref() }
    pub fn get_buffer_position(&self) -> usize { self.buf_off }
}
