use crate::circuit::structure::wire_type::WireType;

/**
 * Gadget for exponential ElGamal encryption, which is additively homomorphic.
 * Because the message is in the exponent it is simply a bit string and
 * does not have to be embedded into the curve.
 */
pub struct ZkayElgamalEncGadget {
    randomnessBits: Vec<Option<WireType>>, // little-endian randomness bits

    msgBits: Vec<Option<WireType>>, // little-endian message bits

    pk: JubJubPoint, // pub  key

    c1: JubJubPoint,

    c2: JubJubPoint,
}
impl ZkayElgamalEncGadget {
    pub fn new(msgBits: Vec<Option<WireType>>, pk: JubJubPoint, randomnessBits: Vec<Option<WireType>>) -> Self {
        self.randomnessBits = randomnessBits;
        self.msgBits = msgBits;
        self.pk = pk;
        buildCircuit();
    }

    fn buildCircuit(&mut self) {
        let msgEmbedded = mulScalar(getGenerator(), msgBits);
        let sharedSecret = mulScalar(pk, randomnessBits);
        c1 = mulScalar(getGenerator(), randomnessBits);
        c2 = addPoints(msgEmbedded, sharedSecret);
    }
}
impl ZkayBabyJubJubGadget for ZkayElgamalEncGadget {
    fn getOutputWires() -> Vec<Option<WireType>> {
        vec![c1.x, c1.y, c2.x, c2.y]
    }
}
