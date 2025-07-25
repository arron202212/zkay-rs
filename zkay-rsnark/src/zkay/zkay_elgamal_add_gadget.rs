use crate::circuit::structure::wire_type::WireType;

/**
 * Gadget for homomorphically adding two ElGamal ciphertexts (c1, c2) and (d1, d2).
 */
pub struct ZkayElgamalAddGadget {
    c1: JubJubPoint,

    c2: JubJubPoint,

    d1: JubJubPoint,

    d2: JubJubPoint,

    e1: JubJubPoint,

    e2: JubJubPoint,
}
impl ZkayElgamalAddGadget {
    pub fn new(c1: JubJubPoint, c2: JubJubPoint, d1: JubJubPoint, d2: JubJubPoint) -> Self {
        self.c1 = c1;
        self.c2 = c2;
        self.d1 = d1;
        self.d2 = d2;
        buildCircuit();
    }

    fn buildCircuit(&mut self) {
        e1 = addPoints(c1, d1);
        e2 = addPoints(c2, d2);
    }
}
impl ZkayBabyJubJubGadget for ZkayElgamalAddGadget {
    fn getOutputWires() -> Vec<Option<WireType>> {
        vec![e1.x, e1.y, e2.x, e2.y]
    }
}
