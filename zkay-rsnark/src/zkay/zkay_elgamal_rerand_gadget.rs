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
 * Gadget homomorphically re-randomizing an ElGamal encrypted ciphertext.
 */

#[derive(Debug, Clone)]
pub struct ZkayElgamalRerandGadget {
    pub randomnessBits: Vec<Option<WireType>>, // little-endian randomness bits
    pub pk: JubJubPoint,                       // pub  key
    pub c1: JubJubPoint,                       // input ciphertext first point
    pub c2: JubJubPoint,                       // input ciphertext second point
    pub o1: Option<JubJubPoint>,
    pub o2: Option<JubJubPoint>,
    pub outputs: Vec<Option<WireType>>,
}
impl ZkayElgamalRerandGadget {
    pub fn new(
        c1: JubJubPoint,
        c2: JubJubPoint,
        pk: JubJubPoint,
        randomnessBits: Vec<Option<WireType>>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<ZkayBabyJubJubGadget<Self>> {
        let mut _self = ZkayBabyJubJubGadget::<Self>::new(
            &None,
            Self {
                randomnessBits,
                pk,
                c1,
                c2,
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
impl Gadget<ZkayBabyJubJubGadget<ZkayElgamalRerandGadget>> {
    fn buildCircuit(&mut self) {
        // create encryption of zero (z1, z2)
        let sharedSecret = self.mulScalar(&self.t.t.pk, &self.t.t.randomnessBits);
        let z1 = self.mulScalar(&self.getGenerator(), &self.t.t.randomnessBits);
        let z2 = sharedSecret;

        // add encryption of zero to re-randomize
        let o1 = self.addPoints(&self.t.t.c1, &z1);
        let o2 = self.addPoints(&self.t.t.c2, &z2);
        self.t.t.outputs = vec![
            Some(o1.x.clone()),
            Some(o1.y.clone()),
            Some(o2.x.clone()),
            Some(o2.y.clone()),
        ];
        (self.t.t.o1, self.t.t.o2) = (Some(o1), Some(o2));
    }
}
impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalRerandGadget>> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.t.outputs
    }
}
