// package org.bouncycastle.crypto;

// /**
//  * A parent class for block cipher modes that do not require block aligned data to be processed, but can function in
//  * a streaming mode.
//  */
// public abstract class StreamBlockCipher
//     implements BlockCipher, StreamCipher
// {
//     private final BlockCipher cipher;

//     protected StreamBlockCipher(BlockCipher cipher)
//     {
//         this.cipher = cipher;
//     }

//     /**
//      * return the underlying block cipher that we are wrapping.
//      *
//      * @return the underlying block cipher that we are wrapping.
//      */
//     public BlockCipher getUnderlyingCipher()
//     {
//         return cipher;
//     }

//     public final byte returnByte(byte in)
//     {
//         return calculateByte(in);
//     }

//     public int processBytes(byte[] in, int inOff, int len, byte[] out, int outOff)
//         throws DataLengthException
//     {
//         if (inOff + len > in.length)
//         {
//             throw new DataLengthException("input buffer too small");
//         }
//         if (outOff + len > out.length)
//         {
//             throw new OutputLengthException("output buffer too short");
//         }

//         int inStart = inOff;
//         int inEnd = inOff + len;
//         int outStart = outOff;

//         while (inStart < inEnd)
//         {
//              out[outStart++] = calculateByte(in[inStart++]);
//         }

//         return len;
//     }

//     protected abstract byte calculateByte(byte b);
// }

/// 對應 Java 的 StreamBlockCipher 抽象類別
/// 專用於將分組加密模式（如 CFB/OFB）轉換為流式操作的基礎特徵
pub trait StreamBlockCipher: BlockCipher + StreamCipher {
    /// 獲取底層封裝的分組加密器
    fn get_underlying_cipher(&self) -> &dyn BlockCipher;

    /// 核心計算邏輯：處理單個字節
    /// 對應 Java 的 protected abstract byte calculateByte(byte b)
    fn calculate_byte(&mut self, input: u8) -> u8;
}
/// 模擬 Java 的 processBytes 邏輯
pub fn process_stream_bytes<T: StreamBlockCipher>(
    cipher: &mut T,
    input: &[u8],
    in_off: usize,
    len: usize,
    output: &mut [u8],
    out_off: usize,
) -> Result<usize, CryptError> {
    // 邊界檢查 (對應 Java 的 DataLengthException/OutputLengthException)
    if in_off + len > input.len() {
        return Err(CryptError::DataLength("輸入緩衝區太短".into()));
    }
    if out_off + len > output.len() {
        return Err(CryptError::OutputLength("輸出緩衝區太短".into()));
    }

    // 逐字節處理循環
    for i in 0..len {
        output[out_off + i] = cipher.calculate_byte(input[in_off + i]);
    }

    Ok(len)
}
pub struct CfbBlockCipher {
    underlying_cipher: Box<dyn BlockCipher>,
    // 其他 CFB 狀態緩衝區...
}

impl StreamBlockCipher for CfbBlockCipher {
    fn get_underlying_cipher(&self) -> &dyn BlockCipher {
        self.underlying_cipher.as_ref()
    }

    fn calculate_byte(&mut self, input: u8) -> u8 {
        // 1. 從底層 cipher 生成密鑰流字節
        // 2. 與 input 進行 XOR
        // 3. 更新反饋緩衝區
        input ^ 0xFF // 示例
    }
}

impl StreamCipher for CfbBlockCipher {
    fn return_byte(&mut self, input: u8) -> u8 {
        self.calculate_byte(input)
    }

    fn process_bytes(&mut self, input: &[u8], in_off: usize, len: usize, output: &mut [u8], out_off: usize) -> Result<usize, CryptError> {
        process_stream_bytes(self, input, in_off, len, output, out_off)
    }
    // ... 其他 StreamCipher 方法
}
