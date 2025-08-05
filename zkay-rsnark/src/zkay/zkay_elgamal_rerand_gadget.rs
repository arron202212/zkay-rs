use crate::circuit::structure::wire_type::WireType;

/**
 * Gadget homomorphically re-randomizing an ElGamal encrypted ciphertext.
 */
pub struct ZkayElgamalRerandGadget {
    randomnessBits: Vec<Option<WireType>>, // little-endian randomness bits
    pk: JubJubPoint,                       // pub  key
    c1: JubJubPoint,                       // input ciphertext first point
    c2: JubJubPoint,                       // input ciphertext second point
    o1: Option<JubJubPoint>,
    o2: Option<JubJubPoint>,
    outputs: Vec<Option<WireType>>,
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
        let sharedSecret = self.mulScalar(pk, self.t.t.randomnessBits);
        let z1 = self.mulScalar(self.getGenerator(), self.t.t.randomnessBits);
        let z2 = sharedSecret;

        // add encryption of zero to re-randomize
        let o1 = addPoints(c1, z1);
        let o2 = addPoints(c2, z2);
        self.t.t.outputs = vec![o1.x.clone(), o1.y.clone(), o2.x.clone(), o2.y.clone()];
        (self.t.t.o1, self.t.t.o2) = (o1, o2);
    }
}
impl GadgetConfig for Gadget<ZkayBabyJubJubGadget<ZkayElgamalRerandGadget>> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.t.outputs
    }
}
