#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::operations::gadget::Gadget;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::wire_type::WireType;
use crate::zkay::crypto::crypto_backend::CryptoBackend;
use crate::zkay::crypto::crypto_backend::CryptoBackendConfig;
use crate::zkay::crypto::crypto_backend::Symmetric;
use crate::zkay::crypto::crypto_backend::SymmetricConfig;
use crate::zkay::crypto::elgamal_backend::wire_array::WireArray;
use crate::zkay::homomorphic_input::HomomorphicInput;
use crate::zkay::typed_wire;
use crate::zkay::typed_wire::TypedWire;
use crate::zkay::zkay_cbc_symmetric_enc_gadget;
use crate::zkay::zkay_cbc_symmetric_enc_gadget::CipherType;
use crate::zkay::zkay_cbc_symmetric_enc_gadget::ZkayCBCSymmetricEncGadget;
pub struct ECDHBackend {
    cipherType: CipherType,
}
impl ECDHBackend {
    const KEY_CHUNK_SIZE: i32 = 256;

    pub fn new(keyBits: i32, cipherType: CipherType) -> CryptoBackend<Symmetric<Self>> {
        Symmetric::<Self>::new(keyBits, Self { cipherType })
    }

    pub fn getKeyChunkSize() -> i32 {
        Self::KEY_CHUNK_SIZE
    }
}
impl CryptoBackendConfig for CryptoBackend<Symmetric<ECDHBackend>> {
    fn createEncryptionGadget(
        &self,
        plain: &TypedWire,
        key: &String,
        ivArr: Vec<Option<WireType>>,
        desc: &Option<String>,
    ) -> Gadget {
        ZkayCBCSymmetricEncGadget::new(
            plain,
            self.getKey(key),
            self.extractIV(ivArr),
            self.t.t.cipherType.clone(),
            desc,
        )
    }
}
