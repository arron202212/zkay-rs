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
        structure::{circuit_generator::CircuitGenerator, wire_type::WireType},
    },
    zkay::zkay_baby_jub_jub_gadget::{
        JubJubPoint, ZkayBabyJubJubGadget, ZkayBabyJubJubGadgetConfig,
    },
};

use rccell::RcCell;

//  * Gadget for homomorphically multiplying an ElGamal ciphertext (c1, c2) by a plaintext scalar

#[derive(Debug, Clone)]
pub struct ZkayElgamalMulGadget {
    pub c1: JubJubPoint,
    pub c2: JubJubPoint,
    pub scalar_bits: Vec<Option<WireType>>,
    pub e1: Option<JubJubPoint>,
    pub e2: Option<JubJubPoint>,
    pub outputs: Vec<Option<WireType>>,
}
impl ZkayElgamalMulGadget {
    pub fn new(
        c1: JubJubPoint,
        c2: JubJubPoint,
        scalar_bits: Vec<Option<WireType>>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<ZkayBabyJubJubGadget<Self>> {
        let mut _self = ZkayBabyJubJubGadget::<Self>::new(
            &None,
            Self {
                scalar_bits,
                c1,
                c2,
                e1: None,
                e2: None,
                outputs: vec![],
            },
            generator,
        );
        _self.build_circuit();
        _self
    }
}
impl Gadget<ZkayBabyJubJubGadget<ZkayElgamalMulGadget>> {
    fn build_circuit(&mut self) {
        let e1 = self.mul_scalar(&self.t.t.c1, &self.t.t.scalar_bits);
        let e2 = self.mul_scalar(&self.t.t.c2, &self.t.t.scalar_bits);
        self.t.t.outputs = vec![&e1.x, &e1.y, &e2.x, &e2.y]
            .iter()
            .map(|&v| Some(v.clone()))
            .collect();
        (self.t.t.e1, self.t.t.e2) = (Some(e1), Some(e2));
    }
}
impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalMulGadget>> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.t.outputs
    }
}
