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
use crate::zkay::crypto::crypto_backend::CryptoBackend;
use crate::zkay::crypto::crypto_backend::CryptoBackendConfig;
use crate::zkay::crypto::crypto_backend::Symmetric;
use crate::zkay::crypto::crypto_backend::SymmetricConfig;
use crate::zkay::homomorphic_input::HomomorphicInput;
use crate::zkay::typed_wire::TypedWire;
use crate::zkay::zkay_cbc_symmetric_enc_gadget::CipherType;
use crate::zkay::zkay_cbc_symmetric_enc_gadget::ZkayCBCSymmetricEncGadget;
use crate::zkay::zkay_type::zkbool;
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
impl SymmetricConfig for CryptoBackend<Symmetric<ECDHBackend>> {}
impl CryptoBackendConfig for CryptoBackend<Symmetric<ECDHBackend>> {
    fn getKeyChunkSize() -> i32 {
        ECDHBackend::KEY_CHUNK_SIZE
    }
    fn createEncryptionGadget(
        &self,
        plain: &TypedWire,
        key: &String,
        ivArr: &Vec<Option<WireType>>,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Box<dyn GadgetConfig> {
        Box::new(ZkayCBCSymmetricEncGadget::new(
            plain.clone(),
            self.getKey(key, generator.clone()),
            self.extractIV(&Some(ivArr.clone())),
            self.t.t.cipherType.clone(),
            desc,
            generator,
        ))
    }
}
