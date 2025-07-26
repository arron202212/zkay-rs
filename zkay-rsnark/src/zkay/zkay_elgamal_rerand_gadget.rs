use crate::circuit::structure::wire_type::WireType;

/**
 * Gadget homomorphically re-randomizing an ElGamal encrypted ciphertext.
 */
pub struct ZkayElgamalRerandGadget {
    randomnessBits: Vec<Option<WireType>>, // little-endian randomness bits

    pk: JubJubPoint, // pub  key

    c1: JubJubPoint, // input ciphertext first point

    c2: JubJubPoint, // input ciphertext second point

    o1: JubJubPoint,

    o2: JubJubPoint,
}
impl ZkayElgamalRerandGadget {
    pub fn new(
        c1: JubJubPoint,
        c2: JubJubPoint,
        pk: JubJubPoint,
        randomnessBits: Vec<Option<WireType>>,
    ) -> Self {
        self.c1 = c1;
        self.c2 = c2;
        self.randomnessBits = randomnessBits;
        self.pk = pk;
        _self.buildCircuit();
        _self
    }

    fn buildCircuit(&mut self) {
        // create encryption of zero (z1, z2)
        let sharedSecret = mulScalar(pk, randomnessBits);
        let z1 = mulScalar(getGenerator(), randomnessBits);
        let z2 = sharedSecret;

        // add encryption of zero to re-randomize
        o1 = addPoints(c1, z1);
        o2 = addPoints(c2, z2);
    }
}
impl ZkayBabyJubJubGadget for ZkayElgamalRerandGadget {
    fn getOutputWires() -> Vec<Option<WireType>> {
        vec![o1.x, o1.y, o2.x, o2.y]
    }
}
