// package org.bouncycastle.crypto;

// import org.bouncycastle.crypto.params.AsymmetricKeyParameter;

// /**
//  * a holding class for public/private parameter pairs.
//  */
// public class AsymmetricCipherKeyPair
// {
//     private AsymmetricKeyParameter    publicParam;
//     private AsymmetricKeyParameter    privateParam;

//     /**
//      * basic constructor.
//      *
//      * @param publicParam a public key parameters object.
//      * @param privateParam the corresponding private key parameters.
//      */
//     public AsymmetricCipherKeyPair(
//         AsymmetricKeyParameter    publicParam,
//         AsymmetricKeyParameter    privateParam)
//     {
//         this.publicParam = publicParam;
//         this.privateParam = privateParam;
//     }

//     /**
//      * basic constructor.
//      *
//      * @param publicParam a public key parameters object.
//      * @param privateParam the corresponding private key parameters.
//      * @deprecated use AsymmetricKeyParameter
//      */
//     public AsymmetricCipherKeyPair(
//         CipherParameters    publicParam,
//         CipherParameters    privateParam)
//     {
//         this.publicParam = (AsymmetricKeyParameter)publicParam;
//         this.privateParam = (AsymmetricKeyParameter)privateParam;
//     }

//     /**
//      * return the public key parameters.
//      *
//      * @return the public key parameters.
//      */
//     public AsymmetricKeyParameter getPublic()
//     {
//         return publicParam;
//     }

//     /**
//      * return the private key parameters.
//      *
//      * @return the private key parameters.
//      */
//     public AsymmetricKeyParameter getPrivate()
//     {
//         return privateParam;
//     }
// }
/// 對應 Java 的 AsymmetricCipherKeyPair
pub struct AsymmetricCipherKeyPair {
    /// 公鑰參數
    public_param: Box<dyn AsymmetricKeyParameter>,
    /// 私鑰參數
    private_param: Box<dyn AsymmetricKeyParameter>,
}

impl AsymmetricCipherKeyPair {
    /// 基礎構造函數 (對應 Java 的第一個構造函數)
    pub fn new(
        public_param: Box<dyn AsymmetricKeyParameter>,
        private_param: Box<dyn AsymmetricKeyParameter>,
    ) -> Self {
        Self {
            public_param,
            private_param,
        }
    }

    /// 獲取公鑰參數 (對應 getPublic)
    pub fn get_public(&self) -> &dyn AsymmetricKeyParameter {
        self.public_param.as_ref()
    }

    /// 獲取私鑰參數 (對應 getPrivate)
    pub fn get_private(&self) -> &dyn AsymmetricKeyParameter {
        self.private_param.as_ref()
    }
}
/// 使用泛型的密鑰對，T 代表具體的密鑰類型
pub struct GenericAsymmetricCipherKeyPair<P, S> 
where 
    P: AsymmetricKeyParameter, 
    S: AsymmetricKeyParameter 
{
    pub public: P,
    pub private: S,
}
pub trait AsymmetricKeyParameter {
    /// 返回該參數是否為私鑰
    fn is_private(&self) -> bool;
}
