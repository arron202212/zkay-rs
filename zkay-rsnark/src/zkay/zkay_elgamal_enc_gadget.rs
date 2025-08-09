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
 * Gadget for exponential ElGamal encryption, which is additively homomorphic.
 * Because the message is in the exponent it is simply a bit string and
 * does not have to be embedded into the curve.
 */

#[derive(Debug, Clone)]
pub struct ZkayElgamalEncGadget {
    pub randomnessBits: Vec<Option<WireType>>, // little-endian randomness bits
    pub msgBits: Vec<Option<WireType>>,        // little-endian message bits
    pub pk: JubJubPoint,                       // pub  key
    pub c1: Option<JubJubPoint>,
    pub c2: Option<JubJubPoint>,
    pub outputs: Vec<Option<WireType>>,
}
impl ZkayElgamalEncGadget {
    pub fn new(
        msgBits: Vec<Option<WireType>>,
        pk: JubJubPoint,
        randomnessBits: Vec<Option<WireType>>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<ZkayBabyJubJubGadget<Self>> {
        let mut _self = ZkayBabyJubJubGadget::<Self>::new(
            &None,
            Self {
                randomnessBits,
                pk,
                c1: None,
                c2: None,
                msgBits,
                outputs: vec![],
            },
            generator,
        );
        _self.buildCircuit();
        _self
    }
}
impl Gadget<ZkayBabyJubJubGadget<ZkayElgamalEncGadget>> {
    fn buildCircuit(&mut self) {
        let msgEmbedded = self.mulScalar(&self.getGenerator(), &self.t.t.msgBits);
        let sharedSecret = self.mulScalar(&self.t.t.pk, &self.t.t.randomnessBits);
        let c1 = self.mulScalar(&self.getGenerator(), &self.t.t.randomnessBits);
        let c2 = self.addPoints(&msgEmbedded, &sharedSecret);
        self.t.t.outputs = [&c1.x, &c1.y, &c2.x, &c2.y]
            .iter()
            .map(|&v| Some(v.clone()))
            .collect();
        (self.t.t.c1, self.t.t.c1) = (Some(c1), Some(c2));
    }
}
impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalEncGadget>> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.t.outputs
    }
}
