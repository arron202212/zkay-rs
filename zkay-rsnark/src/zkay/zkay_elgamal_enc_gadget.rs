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

//  * Gadget for exponential ElGamal encryption, which is additively homomorphic.
//  * Because the message is in the exponent it is simply a bit string and
//  * does not have to be embedded into the curve.

#[derive(Debug, Clone)]
pub struct ZkayElgamalEncGadget {
    pub randomness_bits: Vec<Option<WireType>>, // little-endian randomness bits
    pub msg_bits: Vec<Option<WireType>>,        // little-endian message bits
    pub pk: JubJubPoint,                        // pub  key
    pub c1: Option<JubJubPoint>,
    pub c2: Option<JubJubPoint>,
    pub outputs: Vec<Option<WireType>>,
}
impl ZkayElgamalEncGadget {
    pub fn new(
        msg_bits: Vec<Option<WireType>>,
        pk: JubJubPoint,
        randomness_bits: Vec<Option<WireType>>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<ZkayBabyJubJubGadget<Self>> {
        let mut _self = ZkayBabyJubJubGadget::<Self>::new(
            &None,
            Self {
                randomness_bits,
                pk,
                c1: None,
                c2: None,
                msg_bits,
                outputs: vec![],
            },
            generator,
        );
        _self.build_circuit();
        _self
    }
}
impl Gadget<ZkayBabyJubJubGadget<ZkayElgamalEncGadget>> {
    fn build_circuit(&mut self) {
        let msg_embedded = self.mul_scalar(&self.get_generator(), &self.t.t.msg_bits);
        let shared_secret = self.mul_scalar(&self.t.t.pk, &self.t.t.randomness_bits);
        let c1 = self.mul_scalar(&self.get_generator(), &self.t.t.randomness_bits);
        let c2 = self.add_points(&msg_embedded, &shared_secret);
        self.t.t.outputs = [&c1.x, &c1.y, &c2.x, &c2.y]
            .iter()
            .map(|&v| Some(v.clone()))
            .collect();
        (self.t.t.c1, self.t.t.c1) = (Some(c1), Some(c2));
    }
}
impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalEncGadget>> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.t.outputs
    }
}
