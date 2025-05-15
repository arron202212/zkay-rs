use circuit::structure::wire;

/**
 * Gadget for homomorphically multiplying an ElGamal ciphertext (c1, c2) by a plaintext scalar
 */
pub struct ZkayElgamalMulGadget {
    c1: JubJubPoint,

    c2: JubJubPoint,

    scalarBits: Vec<Wire>,

    e1: JubJubPoint,

    e2: JubJubPoint,
}
impl ZkayElgamalMulGadget {
    pub fn new(c1: JubJubPoint, c2: JubJubPoint, scalarBits: Vec<Wire>) -> Self {
        self.c1 = c1;
        self.c2 = c2;
        self.scalarBits = scalarBits;
        buildCircuit();
    }
}
impl ZkayBabyJubJubGadget for ZkayElgamalMulGadget {
    fn buildCircuit() {
        e1 = mulScalar(c1, scalarBits);
        e2 = mulScalar(c2, scalarBits);
    }

    pub fn getOutputWires() -> Vec<Wire> {
        return vec![e1.x, e1.y, e2.x, e2.y];
    }
}
