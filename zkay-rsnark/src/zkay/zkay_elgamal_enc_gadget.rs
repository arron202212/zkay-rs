use crate::circuit::structure::wire_type::WireType;

/**
 * Gadget for exponential ElGamal encryption, which is additively homomorphic.
 * Because the message is in the exponent it is simply a bit string and
 * does not have to be embedded into the curve.
 */
pub struct ZkayElgamalEncGadget {
    randomnessBits: Vec<WireType>, // little-endian randomness bits

    msgBits: Vec<WireType>, // little-endian message bits

    pk: JubJubPoint, // pub  key

    c1: JubJubPoint,

    c2: JubJubPoint,
}
impl ZkayElgamalEncGadget {
    pub fn new(msgBits: Vec<WireType>, pk: JubJubPoint, randomnessBits: Vec<WireType>) -> Self {
        self.randomnessBits = randomnessBits;
        self.msgBits = msgBits;
        self.pk = pk;
        buildCircuit();
    }
}
impl ZkayBabyJubJubGadget for ZkayElgamalEncGadget {
    fn buildCircuit() {
        let msgEmbedded = mulScalar(getGenerator(), msgBits);
        let sharedSecret = mulScalar(pk, randomnessBits);
        c1 = mulScalar(getGenerator(), randomnessBits);
        c2 = addPoints(msgEmbedded, sharedSecret);
    }

    pub fn getOutputWires() -> Vec<WireType> {
        return vec![c1.x, c1.y, c2.x, c2.y];
    }
}
