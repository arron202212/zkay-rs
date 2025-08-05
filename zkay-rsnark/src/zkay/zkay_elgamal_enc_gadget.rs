use crate::circuit::structure::wire_type::WireType;

/**
 * Gadget for exponential ElGamal encryption, which is additively homomorphic.
 * Because the message is in the exponent it is simply a bit string and
 * does not have to be embedded into the curve.
 */
pub struct ZkayElgamalEncGadget {
    randomnessBits: Vec<Option<WireType>>, // little-endian randomness bits
    msgBits: Vec<Option<WireType>>,        // little-endian message bits
    pk: JubJubPoint,                       // pub  key
    c1: Option<JubJubPoint>,
    c2: Option<JubJubPoint>,
    outputs: Vec<Option<WireType>>,
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
impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalEncGadget>> {
    fn buildCircuit(&mut self) {
        let msgEmbedded = self.mulScalar(self.getGenerator(), self.t.t.msgBits);
        let sharedSecret = self.mulScalar(pk, self.t.t.randomnessBits);
        let c1 = self.mulScalar(self.getGenerator(), self.t.t.randomnessBits);
        let c2 = self.addPoints(msgEmbedded, sharedSecret);
        self.t.t.outputs = vec![c1.x.clone(), c1.y.clone(), c2.x.clone(), c2.y.clone()];
        (self.t.t.c1, self.t.t.c1) = (Some(c1), Some(c2));
    }
}
impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalEncGadget>> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.t.outputs
    }
}
