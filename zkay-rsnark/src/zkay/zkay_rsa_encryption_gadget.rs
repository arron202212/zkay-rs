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
        auxiliary::long_element::LongElement,
        operations::gadget::{Gadget, GadgetConfig},
        structure::{
            circuit_generator::CircuitGenerator, wire::WireConfig, wire_array::WireArray,
            wire_type::WireType,
        },
    },
    examples::gadgets::rsa::{
        rsa_encryption_oaep_gadget::RSAEncryptionOAEPGadget,
        rsa_encryption_v1_5_gadget::RSAEncryptionV1_5_Gadget,
    },
    zkay::{crypto::rsa_backend::RSABackend, typed_wire::TypedWire, zkay_util::ZkayUtil},
};

use rccell::RcCell;

#[derive(Debug, Clone)]
pub enum PaddingType {
    PKCS_1_5,
    OAEP,
}

#[derive(Debug, Clone)]
pub struct ZkayRSAEncryptionGadget {
    pub paddingType: PaddingType,
    pub pk: LongElement,
    pub plain: WireType,
    pub rnd: Vec<Option<WireType>>,
    pub keyBits: i32,
    pub cipher: Vec<Option<WireType>>,
}
impl ZkayRSAEncryptionGadget {
    pub fn new(
        plain: TypedWire,
        pk: LongElement,
        rnd: Vec<Option<WireType>>,
        keyBits: i32,
        paddingType: PaddingType,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        // assert!(plain, "plain");
        // assert!(pk, "pk");
        assert!(!rnd.is_empty(), "rnd");
        // assert!(paddingType, "paddingType");

        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                paddingType,
                plain: plain.wire.clone(),
                pk,
                rnd,
                keyBits,
                cipher: vec![],
            },
        );
        _self.build_circuit();
        _self
    }
}
impl Gadget<ZkayRSAEncryptionGadget> {
    fn build_circuit(&mut self) {
        let plainBytes = ZkayUtil::reverseBytes(
            self.t.plain.get_bit_wiresi(256, &None),
            8,
            self.generator.clone(),
        );

        let mut enc: Box<dyn GadgetConfig>;
        match self.t.paddingType {
            PaddingType::OAEP => {
                let rndBytes = ZkayUtil::reverseBytes(
                    WireArray::new(self.t.rnd.clone(), self.generator.clone().downgrade())
                        .get_bits(RSABackend::OAEP_RND_CHUNK_SIZE as usize, &None),
                    8,
                    self.generator.clone(),
                );
                let e = RSAEncryptionOAEPGadget::new(
                    self.t.pk.clone(),
                    plainBytes,
                    rndBytes,
                    self.t.keyBits.clone(),
                    &Some(self.description.clone()),
                    self.generator.clone(),
                );
                e.checkSeedCompliance();
                enc = Box::new(e);
            }
            PaddingType::PKCS_1_5 => {
                let rndLen = self.t.keyBits as usize / 8 - 3 - plainBytes.len();
                let rndBytes = ZkayUtil::reverseBytes(
                    WireArray::new(self.t.rnd.clone(), self.generator.clone().downgrade())
                        .get_bits(RSABackend::PKCS15_RND_CHUNK_SIZE as usize, &None)
                        .adjust_length(None, rndLen * 8),
                    8,
                    self.generator.clone(),
                );
                enc = Box::new(RSAEncryptionV1_5_Gadget::new(
                    self.t.pk.clone(),
                    plainBytes,
                    rndBytes,
                    self.t.keyBits.clone(),
                    &Some(self.description.clone()),
                    self.generator.clone(),
                ));
            }
            _ => panic!("Unexpected padding type: {:?}", self.t.paddingType),
        }

        self.t.cipher = WireArray::new(
            enc.get_output_wires().clone(),
            self.generator.clone().downgrade(),
        )
        .pack_words_into_larger_words(8, RSABackend::CIPHER_CHUNK_SIZE / 8, &None);
    }
}
impl GadgetConfig for Gadget<ZkayRSAEncryptionGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.cipher
    }
}
