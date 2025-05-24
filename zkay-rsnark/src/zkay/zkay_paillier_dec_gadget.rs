use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;
use examples::gadgets::math::long_integer_floor_div_gadget;
use examples::gadgets::math::long_integer_mod_gadget;
use examples::gadgets::math::long_integer_mod_pow_gadget;

pub struct ZkayPaillierDecGadget {
    n: LongElement,
    nSquare: LongElement,
    nBits: i32,
    lambda: LongElement,
    mu: LongElement,
    cipher: LongElement,
    plain: LongElement,
}

impl ZkayPaillierDecGadget {
    pub fn new(
        n: LongElement,
        nBits: i32,
        lambda: LongElement,
        mu: LongElement,
        cipher: LongElement,
        desc: &String,
    ) -> Self {
        super(desc);
        self.n = n;
        self.nBits = nBits;
        let nSquareMaxBits = 2 * nBits;
        let maxNumChunks =
            (nSquareMaxBits + (LongElement.CHUNK_BITWIDTH - 1)) / LongElement.CHUNK_BITWIDTH;
        self.nSquare = n.mul(n).align(maxNumChunks);
        self.lambda = lambda;
        self.mu = mu;
        self.cipher = cipher;
        buildCircuit();
    }
}
impl Gadget for ZkayPaillierDecGadget {
    fn buildCircuit() {
        let nSquareMinBits = 2 * nBits - 1; // Minimum bit length of n^2

        // plain = L(cipher^lambda mod n^2) * mu mod n
        let cPowLambda =
            LongIntegerModPowGadget::new(cipher, lambda, nSquare, nSquareMinBits, "c^lambda")
                .getResult();
        let lOutput =
            LongIntegerFloorDivGadget::new(cPowLambda.sub(1), n, "(c^lambda - 1) / n")
                .getQuotient();
        let timesMu = lOutput.mul(mu);
        plain = LongIntegerModGadget::new(timesMu, n, nBits, true).getRemainder();
    }

    pub fn getPlaintext() -> LongElement {
        return plain;
    }

    pub fn getOutputWires() -> Vec<Option<WireType>> {
        return plain.getArray();
    }
}
