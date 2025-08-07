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
use rccell::RcCell;
/**
 * Gadget for homomorphically adding two ElGamal ciphertexts (c1, c2) and (d1, d2).
 */
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
            &None,
            Self {
                c1,
                c2,
                d1,
                d2,
                o1: None,
                o2: None,
                outputs: vec![],
            },
            generator,
        );
        _self.buildCircuit();
        _self
    }
}
impl Gadget<ZkayBabyJubJubGadget<ZkayElgamalAddGadget>> {
    fn buildCircuit(&mut self) {
        let e1 = self.addPoints(&self.t.t.c1, &self.t.t.d1);
        let e2 = self.addPoints(&self.t.t.c2, &self.t.t.d2);
        self.t.t.outputs = vec![e1.x.clone(), e1.y.clone(), e2.x.clone(), e2.y.clone()];
        (self.t.t.e1, self.t.t.e2) = (Some(e1), Some(e2));
    }
}
impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalAddGadget>> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.t.outputs
    }
}
