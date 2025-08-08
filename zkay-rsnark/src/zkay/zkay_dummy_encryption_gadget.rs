#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::auxiliary::long_element::LongElement;
use crate::circuit::operations::gadget::Gadget;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::CGConfig;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire::WireConfig;
use crate::circuit::structure::wire_type::WireType;
use crate::zkay::crypto::dummy_backend::DummyBackend; //::CIPHER_CHUNK_SIZE;
use crate::zkay::typed_wire::TypedWire;
use crate::zkay::zkay_baby_jub_jub_gadget::JubJubPoint;
use crate::zkay::zkay_baby_jub_jub_gadget::ZkayBabyJubJubGadget;
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
        keyBits: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        // let generators=generator.borrow().clone();
        // assert!(plain.is_some() && pk.is_some() && rnd.is_some());
        let pkarr = pk.getBits().as_ref().unwrap().packBitsIntoWords(256, &None);
        for i in 1..pkarr.len() {
            generator.addZeroAssertion(
                pkarr[i].as_ref().unwrap(),
                &Some("Dummy enc pk valid".to_owned()),
            );
        }

        let mut _self = Gadget::<Self> {
            generator,
            description: desc
                .as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
            t: Self {
                plain: plain.wire.clone(),
                pk: pkarr[0].clone().unwrap(),
                cipher: vec![
                    None;
                    ((keyBits as f64) / DummyBackend::CIPHER_CHUNK_SIZE as f64).ceil()
                        as usize
                ],
            },
        };
        _self.buildCircuit();
        _self
    }
}
impl Gadget<ZkayDummyEncryptionGadget> {
    fn buildCircuit(&mut self) {
        let res = self
            .t
            .plain
            .addw(&self.t.pk, &Some("plain + pk".to_owned()));
        self.t.cipher.fill(Some(res));
    }
}
impl GadgetConfig for Gadget<ZkayDummyEncryptionGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.cipher
    }
}
