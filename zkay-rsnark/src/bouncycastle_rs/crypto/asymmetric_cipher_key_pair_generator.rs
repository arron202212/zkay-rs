// package org.bouncycastle.crypto;

// /**
//  * interface that a public/private key pair generator should conform to.
//  */
// public interface AsymmetricCipherKeyPairGenerator
// {
//     /**
//      * intialise the key pair generator.
//      *
//      * @param param the parameters the key pair is to be initialised with.
//      */
//     public void init(KeyGenerationParameters param);

//     /**
//      * return an AsymmetricCipherKeyPair containing the generated keys.
//      *
//      * @return an AsymmetricCipherKeyPair containing the generated keys.
//      */
//     public AsymmetricCipherKeyPair generateKeyPair();
// }

/// 對應 Java 的 AsymmetricCipherKeyPairGenerator 接口
pub trait AsymmetricCipherKeyPairGenerator {
    /// 初始化密鑰對生成器
    /// 
    /// # 參數
    /// * `params`: 密鑰生成的配置參數（如隨機源、密鑰長度等）
    fn init(&mut self, params: Box<dyn KeyGenerationParameters>);

    /// 生成並返回一對非對稱密鑰
    /// 
    /// # 返回
    /// 包含公鑰與私鑰的 AsymmetricCipherKeyPair 結構體
    fn generate_key_pair(&self) -> AsymmetricCipherKeyPair;
}
/// 對應 Java 的 KeyGenerationParameters
pub trait KeyGenerationParameters {
    // 這裡通常包含 SecureRandom 的封裝
}

/// 對應 Java 的 AsymmetricCipherKeyPair
/// 封裝一對公鑰和私鑰
pub struct AsymmetricCipherKeyPair {
    pub public_key: Box<dyn AsymmetricKeyParameter>,
    pub private_key: Box<dyn AsymmetricKeyParameter>,
}

/// 對應 Java 的 AsymmetricKeyParameter
pub trait AsymmetricKeyParameter {
    fn is_private(&self) -> bool;
}
pub struct RsaKeyPairGenerator {
    // 內部存儲初始化後的參數
}

impl AsymmetricCipherKeyPairGenerator for RsaKeyPairGenerator {
    fn init(&mut self, params: Box<dyn KeyGenerationParameters>) {
        // 將 params 轉換為 RsaKeyGenerationParameters 並存儲
    }

    fn generate_key_pair(&self) -> AsymmetricCipherKeyPair {
        // 執行 RSA 密鑰生成算法...
        AsymmetricCipherKeyPair {
            public_key: Box::new(RsaPublicKey { .. }),
            private_key: Box::new(RsaPrivateKey { .. }),
        }
    }
}
