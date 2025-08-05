use crate::circuit::structure::wire_type::WireType;

/**
 * Gadget for homomorphically adding two ElGamal ciphertexts (c1, c2) and (d1, d2).
 */
pub struct ZkayElgamalAddGadget {
    c1: JubJubPoint,
    c2: JubJubPoint,
    d1: JubJubPoint,
    d2: JubJubPoint,
    e1: Option<JubJubPoint>,
    e2: Option<JubJubPoint>,
    outputs: Vec<Option<WireType>>,
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
impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalAddGadget>> {
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
