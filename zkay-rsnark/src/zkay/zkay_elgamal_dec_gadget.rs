use crate::circuit::structure::wire_type::WireType;

/**
 * Gadget for checking correct exponential ElGamal decryption.
 * The expected message is provided as an input.
 */
pub struct ZkayElgamalDecGadget {
    skBits: Vec<Option<WireType>>, // little-endian randomness bits

    pk: JubJubPoint,

    c1: JubJubPoint,

    c2: JubJubPoint,

    expectedMsg: WireType,

    msgOk: WireType,
}

impl ZkayElgamalDecGadget {
    pub fn new(
        pk: JubJubPoint,
        skBits: Vec<Option<WireType>>,
        c1: JubJubPoint,
        c2: JubJubPoint,
        expectedMsg: WireType,
    ) -> Self {
        self.pk = pk;
        self.skBits = skBits;
        self.c1 = c1;
        self.c2 = c2;
        self.expectedMsg = expectedMsg;
        buildCircuit();
    }
}
impl ZkayBabyJubJubGadget for ZkayElgamalDecGadget {
    fn buildCircuit() {
        // ensure pk and skBits form a key pair
        let pkExpected = mulScalar(getGenerator(), skBits);
        let keyOk = pkExpected
            .x
            .isEqualTo(pk.x)
            .and(pkExpected.y.isEqualTo(pk.y));

        // decrypt ciphertext (without de-embedding)
        let sharedSecret = mulScalar(c1, skBits);
        let msgEmbedded = addPoints(c2, negatePoint(sharedSecret));

        // embed expected message and assert equality
        let expectedMsgBits = expectedMsg.getBitWires(32).asArray();
        let expectedMsgEmbedded = mulScalar(getGenerator(), expectedMsgBits);
        self.msgOk = expectedMsgEmbedded
            .x
            .isEqualTo(msgEmbedded.x)
            .and(expectedMsgEmbedded.y.isEqualTo(msgEmbedded.y))
            .and(keyOk);
    }

    pub fn getOutputWires() -> Vec<Option<WireType>> {
        vec![self.msgOk]
    }
}
