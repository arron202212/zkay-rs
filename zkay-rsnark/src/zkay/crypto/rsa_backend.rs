#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
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
    pub padding_type: PaddingType,
}
impl RSABackend {
    pub const CIPHER_CHUNK_SIZE: i32 = 232;
    pub const KEY_CHUNK_SIZE: i32 = 232;
    pub const PKCS15_RND_CHUNK_SIZE: i32 = 224;
    pub const OAEP_RND_CHUNK_SIZE: i32 = 128;
    pub fn new(
        key_bits: i32,
        padding: PaddingType,
        generator: RcCell<CircuitGenerator>,
    ) -> CryptoBackend<Asymmetric<Self>> {
        Asymmetric::<Self>::new(
            key_bits,
            Self {
                padding_type: padding,
            },
            generator,
        )
    }
}
//impl AsymmetricConfig for CryptoBackend<Asymmetric<RSABackend>> {}
crate::impl_crypto_backend_configs_for!(RSABackend);
impl CryptoBackendConfig for CryptoBackend<Asymmetric<RSABackend>> {
    fn get_key_chunk_size(&self) -> i32 {
        RSABackend::KEY_CHUNK_SIZE
    }

    fn create_encryption_gadget(
        &mut self,
        plain: &TypedWire,
        key: &String,
        random: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        Box::new(ZkayRSAEncryptionGadget::new(
            plain.clone(),
            self.get_key(key, generator.clone()),
            random.clone(),
            self.key_bits,
            self.t.t.padding_type.clone(),
            desc,
            generator,
        ))
    }
}
