// package org.bouncycastle.crypto;

// import org.bouncycastle.crypto.params.AsymmetricKeyParameter;

// public interface StagedAgreement
//     extends BasicAgreement
// {
//     AsymmetricKeyParameter calculateStage(CipherParameters pubKey);
// }
/// 對應 Java 的 StagedAgreement 接口
/// 擴展了 BasicAgreement，支持多階段密鑰協商的中間計算
pub trait StagedAgreement: BasicAgreement {
    /// 根據傳入的公鑰計算當前階段的結果
    /// 
    /// # 參數
    /// * `pub_key`: 來自另一方的公鑰參數
    /// 
    /// # 返回
    /// 一個非對稱密鑰參數（通常是下一個階段的輸入或中間公鑰）
    fn calculate_stage(&self, pub_key: Box<dyn CipherParameters>) -> Result<Box<dyn AsymmetricKeyParameter>, CryptError>;
}
use num_bigint::BigInt;

pub trait BasicAgreement {
    fn init(&mut self, params: Box<dyn CipherParameters>) -> Result<(), CryptError>;
    fn get_field_size(&self) -> usize;
    fn calculate_agreement(&self, pub_key: Box<dyn CipherParameters>) -> Result<BigInt, CryptError>;
}
pub struct MyStagedAgreement {
    // 內部狀態...
}

impl BasicAgreement for MyStagedAgreement {
    fn init(&mut self, _params: Box<dyn CipherParameters>) -> Result<(), CryptError> { todo!() }
    fn get_field_size(&self) -> usize { todo!() }
    fn calculate_agreement(&self, _pub_key: Box<dyn CipherParameters>) -> Result<BigInt, CryptError> { todo!() }
}

impl StagedAgreement for MyStagedAgreement {
    fn calculate_stage(&self, pub_key: Box<dyn CipherParameters>) -> Result<Box<dyn AsymmetricKeyParameter>, CryptError> {
        // 執行階段性計算，例如：g^(ab) 
        // 將結果封裝為下一個階段所需的公鑰格式
        // Ok(Box::new(MyPublicKey::new(stage_result)))
        todo!()
    }
}
