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
    pub cipher_type: CipherType,
}
impl ECDHBackend {
    const KEY_CHUNK_SIZE: i32 = 256;

    pub fn new(
        key_bits: i32,
        cipher_type: CipherType,
        generator: RcCell<CircuitGenerator>,
    ) -> CryptoBackend<Symmetric<Self>> {
        Symmetric::<Self>::new(key_bits, Self { cipher_type }, generator)
    }
}
// impl SymmetricConfig for CryptoBackend<Symmetric<ECDHBackend>> {}
impl CryptoBackendConfig for CryptoBackend<Symmetric<ECDHBackend>> {
    fn get_key_chunk_size(&self) -> i32 {
        ECDHBackend::KEY_CHUNK_SIZE
    }
    fn create_encryption_gadget(
        &mut self,
        plain: &TypedWire,
        key: &String,
        iv_arr: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        Box::new(ZkayCBCSymmetricEncGadget::new_with_option(
            plain.clone(),
            self.get_key(key, generator.clone()),
            Self::extract_iv(&Some(iv_arr.clone())),
            self.t.t.cipher_type.clone(),
            desc,
            generator,
        ))
    }
}
