use crate::circuit::structure::wire_type::WireType;

/**
 * Gadget for homomorphically multiplying an ElGamal ciphertext (c1, c2) by a plaintext scalar
 */
pub struct ZkayElgamalMulGadget {
    c1: JubJubPoint,
    c2: JubJubPoint,
    scalarBits: Vec<Option<WireType>>,
    e1: Option<JubJubPoint>,
    e2: Option<JubJubPoint>,
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
            },
            generator,
        );
        _self.buildCircuit();
        _self
    }
}
impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalMulGadget>> {
    fn buildCircuit(&mut self) {
        let e1 = self.mulScalar(c1, self.t.t.scalarBits);
        let e2 = self.mulScalar(c2, self.t.t.scalarBits);
        self.t.t.outputs = vec![e1.x.clone(), e1.y.clone(), e2.x.clone(), e2.y.clone()];
        (self.t.t.e1, self.t.t.e2) = (Some(e1), Some(e2));
    }
}
impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalMulGadget>> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.t.outputs
    }
}
