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
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::zkay::crypto::crypto_backend::Asymmetric;

use crate::zkay::crypto::crypto_backend::CryptoBackend;
use crate::zkay::crypto::crypto_backend::{CryptoBackendConfig, CryptoBackendConfigs};
use crate::zkay::typed_wire::TypedWire;
use crate::zkay::zkay_dummy_encryption_gadget::ZkayDummyEncryptionGadget;

use rccell::RcCell;

#[derive(Debug, Clone)]
pub struct DummyBackend;

impl DummyBackend {
    pub const CIPHER_CHUNK_SIZE: i32 = 256;
    pub const KEY_CHUNK_SIZE: i32 = 256;

    pub fn new(
        keyBits: i32,
        generator: RcCell<CircuitGenerator>,
    ) -> CryptoBackend<Asymmetric<Self>> {
        Asymmetric::<Self>::new(keyBits, Self, generator)
    }
}
//impl AsymmetricConfig for CryptoBackend<Asymmetric<DummyBackend>> {}
crate::impl_crypto_backend_configs_for!(DummyBackend);
impl CryptoBackendConfig for CryptoBackend<Asymmetric<DummyBackend>> {
    fn getKeyChunkSize(&self) -> i32 {
        DummyBackend::KEY_CHUNK_SIZE
    }
    fn createEncryptionGadget(
        &self,
        plain: &TypedWire,
        key: &String,
        random: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        Box::new(ZkayDummyEncryptionGadget::new(
            plain.clone(),
            self.getKey(key, generator.clone()),
            random.clone(),
            self.keyBits,
            desc,
            generator,
        ))
    }
}
