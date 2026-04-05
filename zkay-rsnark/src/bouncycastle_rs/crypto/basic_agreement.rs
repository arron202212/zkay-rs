// package org.bouncycastle.crypto;

// import java.math.BigInteger;

// /**
//  * The basic interface that basic Diffie-Hellman implementations
//  * conforms to.
//  */
// public interface BasicAgreement
// {
//     /**
//      * initialise the agreement engine.
//      */
//     void init(CipherParameters param);

//     /**
//      * return the field size for the agreement algorithm in bytes.
//      */
//     int getFieldSize();

//     /**
//      * given a public key from a given party calculate the next
//      * message in the agreement sequence. 
//      */
//     BigInteger calculateAgreement(CipherParameters pubKey);
// }
use num_bigint::BigInt;

/// 對應 Java 的 BasicAgreement 接口
/// 基本的密鑰協商（如 Diffie-Hellman）實現特徵
pub trait BasicAgreement {
    /// 初始化協商引擎
    /// 
    /// # 參數
    /// * `params`: 包含私鑰及其相關算法參數（如 DH 參數）
    fn init(&mut self, params: Box<dyn CipherParameters>) -> Result<(), CryptError>;

    /// 返回協商算法的域大小（以字節為單位）
    fn get_field_size(&self) -> usize;

    /// 根據對方的公鑰計算共享秘密（Shared Secret）
    /// 
    /// # 參數
    /// * `pub_key`: 來自另一方的公鑰參數
    /// 
    /// # 返回
    /// 計算出的共享秘密（BigInt）
    fn calculate_agreement(&self, pub_key: Box<dyn CipherParameters>) -> Result<BigInt, CryptError>;
}
/// 對應 Java 的 CipherParameters
pub trait CipherParameters {}

/// 密鑰協商過程中可能發生的錯誤
#[derive(Debug)]
pub enum CryptError {
    InvalidKey(String),
    IncompatibleParameters(String),
    CalculationFailed(String),
}
pub struct DHBasicAgreement {
    priv_key: Option<DHPrivateKeyParameters>,
}

impl BasicAgreement for DHBasicAgreement {
    fn init(&mut self, params: Box<dyn CipherParameters>) -> Result<(), CryptError> {
        // 將 params 轉換並存儲私鑰...
        Ok(())
    }

    fn get_field_size(&self) -> usize {
        // 返回 p 的字節長度
        1024 / 8 
    }

    fn calculate_agreement(&self, pub_key: Box<dyn CipherParameters>) -> Result<BigInt, CryptError> {
        // 執行 g^ab mod p 計算
        // let shared_secret = pub_key.y.modpow(priv_key.x, p);
        // Ok(shared_secret)
        todo!()
    }
}
