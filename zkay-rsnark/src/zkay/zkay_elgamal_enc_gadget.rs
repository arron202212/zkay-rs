

use circuit::structure::wire;

/**
 * Gadget for exponential ElGamal encryption, which is additively homomorphic.
 * Because the message is in the exponent it is simply a bit string and
 * does not have to be embedded into the curve.
 */
pub struct ZkayElgamalEncGadget  {

     randomnessBits:Vec<Wire>,    // little-endian randomness bits

     msgBits:Vec<Wire>,   // little-endian message bits

     pk:JubJubPoint,   // pub  key

     c1:JubJubPoint,

     c2:JubJubPoint,
}
impl ZkayElgamalEncGadget{
    pub  fn new(msgBits:Vec<Wire>, pk:JubJubPoint , randomnessBits:Vec<Wire>)->Self {
        self.randomnessBits = randomnessBits;
        self.msgBits = msgBits;
        self.pk = pk;
        buildCircuit();
    }
}
impl  ZkayBabyJubJubGadget for ZkayElgamalEncGadget{
      fn buildCircuit() {
        let msgEmbedded = mulScalar(getGenerator(), msgBits);
        let sharedSecret = mulScalar(pk, randomnessBits);
        c1 = mulScalar(getGenerator(), randomnessBits);
        c2 = addPoints(msgEmbedded, sharedSecret);
    }

    
    pub  fn getOutputWires()->Vec<Wire>  {
        return vec![Wire::default();]{ c1.x, c1.y, c2.x, c2.y };
    }
}
