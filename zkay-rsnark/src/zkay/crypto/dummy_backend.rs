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
use crate::zkay::crypto::crypto_backend::Asymmetric;
use crate::zkay::crypto::crypto_backend::AsymmetricConfig;
use crate::zkay::crypto::crypto_backend::CryptoBackend;
use crate::zkay::crypto::crypto_backend::CryptoBackendConfig;
use crate::zkay::typed_wire;
use crate::zkay::typed_wire::TypedWire;
use crate::zkay::zkay_dummy_encryption_gadget;
use crate::zkay::zkay_dummy_encryption_gadget::ZkayDummyEncryptionGadget;
pub struct DummyBackend;
impl DummyBackend {
    const CIPHER_CHUNK_SIZE: i32 = 256;
    const KEY_CHUNK_SIZE: i32 = 256;

    pub fn new(keyBits: i32) -> CryptoBackend<Asymmetric<Self>> {
        Asymmetric::<Self>::new(keyBits, Self)
    }

    pub fn getKeyChunkSize() -> i32 {
        Self::KEY_CHUNK_SIZE
    }
}

impl CryptoBackendConfig for CryptoBackend<Asymmetric<DummyBackend>> {
    fn createEncryptionGadget(
        &self,
        plain: &TypedWire,
        key: &String,
        random: Vec<Option<WireType>>,
        desc: &Option<String>,
    ) -> Gadget {
        ZkayDummyEncryptionGadget::new(plain, getKey(key), random, self.keyBits, desc)
    }
}
