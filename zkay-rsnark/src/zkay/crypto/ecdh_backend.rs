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
            CryptoBackend, CryptoBackendConfig, CryptoBackendConfigs, Symmetric,
        },
        homomorphic_input::HomomorphicInput,
        typed_wire::TypedWire,
        zkay_cbc_symmetric_enc_gadget::{CipherType, ZkayCBCSymmetricEncGadget},
        zkay_type::zkbool,
    },
};

use rccell::RcCell;

#[derive(Debug, Clone)]
pub struct ECDHBackend {
    pub cipherType: CipherType,
}
impl ECDHBackend {
    const KEY_CHUNK_SIZE: i32 = 256;

    pub fn new(
        keyBits: i32,
        cipherType: CipherType,
        generator: RcCell<CircuitGenerator>,
    ) -> CryptoBackend<Symmetric<Self>> {
        Symmetric::<Self>::new(keyBits, Self { cipherType }, generator)
    }
}
// impl SymmetricConfig for CryptoBackend<Symmetric<ECDHBackend>> {}
impl CryptoBackendConfig for CryptoBackend<Symmetric<ECDHBackend>> {
    fn getKeyChunkSize(&self) -> i32 {
        ECDHBackend::KEY_CHUNK_SIZE
    }
    fn createEncryptionGadget(
        &mut self,
        plain: &TypedWire,
        key: &String,
        ivArr: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        Box::new(ZkayCBCSymmetricEncGadget::new(
            plain.clone(),
            self.getKey(key, generator.clone()),
            Self::extractIV(&Some(ivArr.clone())),
            self.t.t.cipherType.clone(),
            desc,
            generator,
        ))
    }
}
