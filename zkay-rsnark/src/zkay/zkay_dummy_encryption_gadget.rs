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
        structure::{circuit_generator::CircuitGenerator, wire::WireConfig, wire_type::WireType},
    },
    zkay::{crypto::dummy_backend::DummyBackend, typed_wire::TypedWire},
};

use rccell::RcCell;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct ZkayDummyEncryptionGadget {
    pub pk: WireType,
    pub plain: WireType,
    pub cipher: Vec<Option<WireType>>,
}
impl ZkayDummyEncryptionGadget {
    pub fn new(
        plain: TypedWire,
        pk: LongElement,
        rnd: Vec<Option<WireType>>,
        key_bits: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        // let generators=generator.borrow().clone();
        // assert!(plain.is_some() && pk.is_some() && rnd.is_some());
        let pkarr = pk
            .get_bits()
            .as_ref()
            .unwrap()
            .pack_bits_into_words(256, &None);
        for i in 1..pkarr.len() {
            CircuitGenerator::add_zero_assertion_with_str(
                generator.clone(),
                pkarr[i].as_ref().unwrap(),
                "Dummy enc pk valid",
            );
        }

        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                plain: plain.wire.clone(),
                pk: pkarr[0].clone().unwrap(),
                cipher: vec![
                    None;
                    ((key_bits as f64) / DummyBackend::CIPHER_CHUNK_SIZE as f64).ceil()
                        as usize
                ],
            },
        );
        _self.build_circuit();
        _self
    }
}
impl Gadget<ZkayDummyEncryptionGadget> {
    fn build_circuit(&mut self) {
        let res = self.t.plain.addw_with_str(&self.t.pk, "plain + pk");
        self.t.cipher.fill(Some(res));
    }
}
impl GadgetConfig for Gadget<ZkayDummyEncryptionGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.cipher
    }
}
