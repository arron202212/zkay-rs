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
use crate::zkay::crypto::crypto_backend::AsymmetricConfig;
use crate::zkay::crypto::crypto_backend::CryptoBackend;
use crate::zkay::crypto::crypto_backend::CryptoBackendConfig;
use crate::zkay::homomorphic_input::HomomorphicInput;
use crate::zkay::typed_wire::TypedWire;
use crate::zkay::zkay_dummy_encryption_gadget::ZkayDummyEncryptionGadget;

use crate::zkay::zkay_rsa_encryption_gadget::PaddingType;
use crate::zkay::zkay_rsa_encryption_gadget::ZkayRSAEncryptionGadget;

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
impl AsymmetricConfig for CryptoBackend<Asymmetric<RSABackend>> {}
impl CryptoBackendConfig for CryptoBackend<Asymmetric<RSABackend>> {
    fn getKeyChunkSize(&self) -> i32 {
        RSABackend::KEY_CHUNK_SIZE
    }

    fn createEncryptionGadget(
        &self,
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
