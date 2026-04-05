// package org.bouncycastle.crypto;

// /**
//  * General holding class for a commitment.
//  */
// public class Commitment
// {
//     private final byte[] secret;
//     private final byte[] commitment;

//     /**
//      * Base constructor.
//      *
//      * @param secret  an encoding of the secret required to reveal the commitment.
//      * @param commitment  an encoding of the sealed commitment.
//      */
//     public Commitment(byte[] secret, byte[] commitment)
//     {
//         this.secret = secret;
//         this.commitment = commitment;
//     }

//     /**
//      * The secret required to reveal the commitment.
//      *
//      * @return an encoding of the secret associated with the commitment.
//      */
//     public byte[] getSecret()
//     {
//         return secret;
//     }

//     /**
//      * The sealed commitment.
//      *
//      * @return an encoding of the sealed commitment.
//      */
//     public byte[] getCommitment()
//     {
//         return commitment;
//     }
// }
/// 對應 Java 的 Commitment
/// 用於保存承諾值（Commitment）及其對應的秘密（Secret）
#[derive(Clone, Debug)]
pub struct Commitment {
    secret: Vec<u8>,
    commitment: Vec<u8>,
}

impl Commitment {
    /// 基礎構造函數 (Base constructor)
    pub fn new(secret: Vec<u8>, commitment: Vec<u8>) -> Self {
        Self {
            secret,
            commitment,
        }
    }

    /// 獲取揭示承諾所需的秘密 (The secret required to reveal the commitment)
    pub fn get_secret(&self) -> &[u8] {
        &self.secret
    }

    /// 獲取封存的承諾值 (The sealed commitment)
    pub fn get_commitment(&self) -> &[u8] {
        &self.commitment
    }
}
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecureCommitment {
    secret: Vec<u8>,
    pub commitment: Vec<u8>, // 承諾值通常是公開的，可以直接訪問
}
