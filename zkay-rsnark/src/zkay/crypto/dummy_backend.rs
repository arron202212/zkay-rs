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
        typed_wire::TypedWire,
        zkay_dummy_encryption_gadget::ZkayDummyEncryptionGadget,
    },
};

use rccell::RcCell;

#[derive(Debug, Clone)]
pub struct DummyBackend;

impl DummyBackend {
    pub const CIPHER_CHUNK_SIZE: i32 = 256;
    pub const KEY_CHUNK_SIZE: i32 = 256;

    pub fn new(
        key_bits: i32,
        generator: RcCell<CircuitGenerator>,
    ) -> CryptoBackend<Asymmetric<Self>> {
        Asymmetric::<Self>::new(key_bits, Self, generator)
    }
}
//impl AsymmetricConfig for CryptoBackend<Asymmetric<DummyBackend>> {}
crate::impl_crypto_backend_configs_for!(DummyBackend);
impl CryptoBackendConfig for CryptoBackend<Asymmetric<DummyBackend>> {
    fn get_key_chunk_size(&self) -> i32 {
        DummyBackend::KEY_CHUNK_SIZE
    }
    fn create_encryption_gadget(
        &mut self,
        plain: &TypedWire,
        key: &String,
        random: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        Box::new(ZkayDummyEncryptionGadget::new(
            plain.clone(),
            self.get_key(key, generator.clone()),
            random.clone(),
            self.key_bits,
            desc,
            generator,
        ))
    }
}
