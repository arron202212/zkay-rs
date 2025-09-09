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
    zkay::zkay_baby_jub_jub_gadget::{
        JubJubPoint, ZkayBabyJubJubGadget, ZkayBabyJubJubGadgetConfig,
    },
};

use rccell::RcCell;

//  * Gadget for homomorphically adding two ElGamal ciphertexts (c1, c2) and (d1, d2).

#[derive(Debug, Clone)]
pub struct ZkayElgamalAddGadget {
    pub c1: JubJubPoint,
    pub c2: JubJubPoint,
    pub d1: JubJubPoint,
    pub d2: JubJubPoint,
    pub e1: Option<JubJubPoint>,
    pub e2: Option<JubJubPoint>,
    pub outputs: Vec<Option<WireType>>,
}
impl ZkayElgamalAddGadget {
    pub fn new(
        c1: JubJubPoint,
        c2: JubJubPoint,
        d1: JubJubPoint,
        d2: JubJubPoint,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<ZkayBabyJubJubGadget<Self>> {
        let mut _self = ZkayBabyJubJubGadget::<Self>::new(
            Self {
                c1,
                c2,
                d1,
                d2,
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
impl Gadget<ZkayBabyJubJubGadget<ZkayElgamalAddGadget>> {
    fn build_circuit(&mut self) {
        let e1 = self.add_points(&self.t.t.c1, &self.t.t.d1);
        let e2 = self.add_points(&self.t.t.c2, &self.t.t.d2);
        self.t.t.outputs = vec![
            Some(e1.x.clone()),
            Some(e1.y.clone()),
            Some(e2.x.clone()),
            Some(e2.y.clone()),
        ];
        (self.t.t.e1, self.t.t.e2) = (Some(e1), Some(e2));
    }
}
impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalAddGadget>> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.t.outputs
    }
}
