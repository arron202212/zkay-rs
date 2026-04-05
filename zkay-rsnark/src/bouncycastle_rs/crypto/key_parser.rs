// package org.bouncycastle.crypto;

// import java.io.IOException;
// import java.io.InputStream;

// import org.bouncycastle.crypto.params.AsymmetricKeyParameter;

// public interface KeyParser
// {
//     AsymmetricKeyParameter readKey(InputStream stream)
//         throws IOException;
// }
use std::io::{Read, Result as IoResult};

/// 對應 Java 的 KeyParser 接口
/// 用於從輸入流中解析並讀取非對稱密鑰參數
pub trait KeyParser {
    /// 從實現了 Read 特徵的流中讀取密鑰
    /// 
    /// # 參數
    /// * `stream`: 輸入流（如文件、網路 Socket 或內存緩衝區）
    /// 
    /// # 返回
    /// 解析後的非對稱密鑰參數，或 IO 錯誤
    fn read_key(&self, stream: &mut dyn Read) -> IoResult<Box<dyn AsymmetricKeyParameter>>;
}
/// 對應 Java 的 AsymmetricKeyParameter
pub trait AsymmetricKeyParameter: Send + Sync {
    fn is_private(&self) -> bool;
}
pub struct Pkcs8KeyParser;

impl KeyParser for Pkcs8KeyParser {
    fn read_key(&self, stream: &mut dyn Read) -> IoResult<Box<dyn AsymmetricKeyParameter>> {
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer)?; // 讀取所有字節
        
        // 執行 ASN.1/DER 解析邏輯...
        // 假設解析出一個 RSA 私鑰
        Ok(Box::new(RsaPrivateKey::new(...)))
    }
}
