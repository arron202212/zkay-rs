// package org.bouncycastle.crypto;

// import java.math.BigInteger;

// /**
//  * An "extended" interface for classes implementing DSA-style algorithms, that provides access to
//  * the group order.
//  */
// public interface DSAExt
//     extends DSA
// {
//     /**
//      * Get the order of the group that the r, s values in signatures belong to.
//      */
//     public BigInteger getOrder();
// }
use num_bigint::BigInt;

/// 對應 Java 的 DSAExt 接口
/// 擴展了 Dsa 特徵，提供獲取簽名值 (r, s) 所屬群階 (Group Order) 的方法
pub trait DsaExt: Dsa {
    /// 返回該 DSA 算法使用的群階 n (Order of the group)
    /// 
    /// # 返回
    /// 大整數 BigInt，通常用於規範化簽名值或進行模運算
    fn get_order(&self) -> &BigInt;
}
pub struct EcdsaSigner {
    order: BigInt,
    // 其他狀態...
}

impl Dsa for EcdsaSigner {
    fn init(&mut self, for_signing: bool, params: &dyn CipherParameters) -> Result<(), CryptError> {
        // 初始化並提取群階 n
        Ok(())
    }

    fn generate_signature(&self, message: &[u8]) -> (BigInt, BigInt) {
        // 執行 ECDSA 簽名邏輯
        (BigInt::from(1), BigInt::from(2))
    }

    fn verify_signature(&self, message: &[u8], r: &BigInt, s: &BigInt) -> bool {
        // 驗證邏輯
        true
    }
}

impl DsaExt for EcdsaSigner {
    fn get_order(&self) -> &BigInt {
        &self.order
    }
}
