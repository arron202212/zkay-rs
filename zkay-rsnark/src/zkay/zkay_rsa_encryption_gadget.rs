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
    Pkcs_1_5,
    Oaep,
}

#[derive(Debug, Clone)]
pub struct ZkayRSAEncryptionGadget {
    pub padding_type: PaddingType,
    pub pk: LongElement,
    pub plain: WireType,
    pub rnd: Vec<Option<WireType>>,
    pub key_bits: i32,
    pub cipher: Vec<Option<WireType>>,
}
impl ZkayRSAEncryptionGadget {
    pub fn new(
        plain: TypedWire,
        pk: LongElement,
        rnd: Vec<Option<WireType>>,
        key_bits: i32,
        padding_type: PaddingType,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        // assert!(plain, "plain");
        // assert!(pk, "pk");
        assert!(!rnd.is_empty(), "rnd");
        // assert!(padding_type, "padding_type");

        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                padding_type,
                plain: plain.wire.clone(),
                pk,
                rnd,
                key_bits,
                cipher: vec![],
            },
        );
        _self.build_circuit();
        _self
    }
}
impl Gadget<ZkayRSAEncryptionGadget> {
    fn build_circuit(&mut self) {
        let plain_bytes = ZkayUtil::reverse_bytes(
            self.t.plain.get_bit_wiresi(256, &None),
            8,
            self.generator.clone(),
        );

        let mut enc: Box<dyn GadgetConfig>;
        match self.t.padding_type {
            PaddingType::Oaep => {
                let rnd_bytes = ZkayUtil::reverse_bytes(
                    WireArray::new(self.t.rnd.clone(), self.generator.clone().downgrade())
                        .get_bits(RSABackend::OAEP_RND_CHUNK_SIZE as usize, &None),
                    8,
                    self.generator.clone(),
                );
                let e = RSAEncryptionOAEPGadget::new(
                    self.t.pk.clone(),
                    plain_bytes,
                    rnd_bytes,
                    self.t.key_bits.clone(),
                    &Some(self.description.clone()),
                    self.generator.clone(),
                );
                e.check_seed_compliance();
                enc = Box::new(e);
            }
            PaddingType::Pkcs_1_5 => {
                let rnd_len = self.t.key_bits as usize / 8 - 3 - plain_bytes.len();
                let rnd_bytes = ZkayUtil::reverse_bytes(
                    WireArray::new(self.t.rnd.clone(), self.generator.clone().downgrade())
                        .get_bits(RSABackend::PKCS15_RND_CHUNK_SIZE as usize, &None)
                        .adjust_length(None, rnd_len * 8),
                    8,
                    self.generator.clone(),
                );
                enc = Box::new(RSAEncryptionV1_5_Gadget::new(
                    self.t.pk.clone(),
                    plain_bytes,
                    rnd_bytes,
                    self.t.key_bits.clone(),
                    &Some(self.description.clone()),
                    self.generator.clone(),
                ));
            }
            _ => panic!("Unexpected padding type: {:?}", self.t.padding_type),
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
