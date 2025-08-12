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
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_type::WireType;
use crate::zkay::zkay_baby_jub_jub_gadget::JubJubPoint;
use crate::zkay::zkay_baby_jub_jub_gadget::ZkayBabyJubJubGadget;
use crate::zkay::zkay_baby_jub_jub_gadget::ZkayBabyJubJubGadgetConfig;
use rccell::RcCell;
/**
 * Gadget for homomorphically multiplying an ElGamal ciphertext (c1, c2) by a plaintext scalar
 */

#[derive(Debug, Clone)]
pub struct ZkayElgamalMulGadget {
    pub c1: JubJubPoint,
    pub c2: JubJubPoint,
    pub scalarBits: Vec<Option<WireType>>,
    pub e1: Option<JubJubPoint>,
    pub e2: Option<JubJubPoint>,
    pub outputs: Vec<Option<WireType>>,
}
impl ZkayElgamalMulGadget {
    pub fn new(
        c1: JubJubPoint,
        c2: JubJubPoint,
        scalarBits: Vec<Option<WireType>>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<ZkayBabyJubJubGadget<Self>> {
        let mut _self = ZkayBabyJubJubGadget::<Self>::new(
            &None,
            Self {
                scalarBits,
                c1,
                c2,
                e1: None,
                e2: None,
                outputs: vec![],
            },
            generator,
        );
        _self.buildCircuit();
        _self
    }
}
impl Gadget<ZkayBabyJubJubGadget<ZkayElgamalMulGadget>> {
    fn buildCircuit(&mut self) {
        let e1 = self.mulScalar(&self.t.t.c1, &self.t.t.scalarBits);
        let e2 = self.mulScalar(&self.t.t.c2, &self.t.t.scalarBits);
        self.t.t.outputs = vec![&e1.x, &e1.y, &e2.x, &e2.y]
            .iter()
            .map(|&v| Some(v.clone()))
            .collect();
        (self.t.t.e1, self.t.t.e2) = (Some(e1), Some(e2));
    }
}
impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalMulGadget>> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.t.outputs
    }
}
