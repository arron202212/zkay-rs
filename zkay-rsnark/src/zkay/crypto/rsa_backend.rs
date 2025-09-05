#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        operations::gadget::{Gadget, GadgetConfig},
        structure::{
            circuit_generator::CircuitGenerator, wire_array::WireArray, wire_type::WireType,
        },
    },
    zkay::{
        crypto::crypto_backend::{
            Asymmetric, CryptoBackend, CryptoBackendConfig, CryptoBackendConfigs,
        },
        homomorphic_input::HomomorphicInput,
        typed_wire::TypedWire,
        zkay_dummy_encryption_gadget::ZkayDummyEncryptionGadget,
        zkay_rsa_encryption_gadget::PaddingType,
        zkay_rsa_encryption_gadget::ZkayRSAEncryptionGadget,
    },
};

use rccell::RcCell;

#[derive(Debug, Clone)]
pub struct RSABackend {
    pub paddingType: PaddingType,
}
impl RSABackend {
    pub const CIPHER_CHUNK_SIZE: i32 = 232;
    pub const KEY_CHUNK_SIZE: i32 = 232;
    pub const PKCS15_RND_CHUNK_SIZE: i32 = 224;
    pub const OAEP_RND_CHUNK_SIZE: i32 = 128;
    pub fn new(
        keyBits: i32,
        padding: PaddingType,
        generator: RcCell<CircuitGenerator>,
    ) -> CryptoBackend<Asymmetric<Self>> {
        Asymmetric::<Self>::new(
            keyBits,
            Self {
                paddingType: padding,
            },
            generator,
        )
    }
}
//impl AsymmetricConfig for CryptoBackend<Asymmetric<RSABackend>> {}
crate::impl_crypto_backend_configs_for!(RSABackend);
impl CryptoBackendConfig for CryptoBackend<Asymmetric<RSABackend>> {
    fn getKeyChunkSize(&self) -> i32 {
        RSABackend::KEY_CHUNK_SIZE
    }

    fn createEncryptionGadget(
        &mut self,
        plain: &TypedWire,
        key: &String,
        random: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        Box::new(ZkayRSAEncryptionGadget::new(
            plain.clone(),
            self.getKey(key, generator.clone()),
            random.clone(),
            self.keyBits,
            self.t.t.paddingType.clone(),
            desc,
            generator,
        ))
    }
}
