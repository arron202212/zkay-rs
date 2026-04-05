// package org.bouncycastle.crypto;

// public class EphemeralKeyPair
// {
//     private AsymmetricCipherKeyPair keyPair;
//     private KeyEncoder publicKeyEncoder;

//     public EphemeralKeyPair(AsymmetricCipherKeyPair keyPair, KeyEncoder publicKeyEncoder)
//     {
//         this.keyPair = keyPair;
//         this.publicKeyEncoder = publicKeyEncoder;
//     }

//     public AsymmetricCipherKeyPair getKeyPair()
//     {
//         return keyPair;
//     }

//     public byte[] getEncodedPublicKey()
//     {
//         return publicKeyEncoder.getEncoded(keyPair.getPublic());
//     }
// }
/// 對應 Java 的 KeyEncoder 接口
pub trait KeyEncoder {
    /// 將公鑰參數編碼為字節數組
    fn get_encoded(&self, public_key: &dyn AsymmetricKeyParameter) -> Vec<u8>;
}
/// 對應 Java 的 EphemeralKeyPair
/// 用於保存臨時（一次性）密鑰對及其公鑰編碼器
pub struct EphemeralKeyPair {
    key_pair: AsymmetricCipherKeyPair,
    public_key_encoder: Box<dyn KeyEncoder>,
}

impl EphemeralKeyPair {
    /// 基礎構造函數
    pub fn new(
        key_pair: AsymmetricCipherKeyPair, 
        public_key_encoder: Box<dyn KeyEncoder>
    ) -> Self {
        Self {
            key_pair,
            public_key_encoder,
        }
    }

    /// 獲取底層密鑰對 (getKeyPair)
    pub fn get_key_pair(&self) -> &AsymmetricCipherKeyPair {
        &self.key_pair
    }

    /// 獲取編碼後的公鑰 (getEncodedPublicKey)
    pub fn get_encoded_public_key(&self) -> Vec<u8> {
        self.public_key_encoder.get_encoded(self.key_pair.get_public())
    }
}
// 假設我們有一個簡單的點編碼器
pub struct RawPointEncoder;
impl KeyEncoder for RawPointEncoder {
    fn get_encoded(&self, pub_key: &dyn AsymmetricKeyParameter) -> Vec<u8> {
        // 執行具體的編碼邏輯...
        vec![0x04, 0x01, 0x02...] 
    }
}

// 使用方式
let ephemeral = EphemeralKeyPair::new(my_key_pair, Box::new(RawPointEncoder));
let encoded_pub = ephemeral.get_encoded_public_key();
