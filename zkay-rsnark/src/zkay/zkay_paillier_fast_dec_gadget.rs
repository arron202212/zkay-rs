use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;
use examples::gadgets::math::long_integer_floor_div_gadget;
use examples::gadgets::math::long_integer_mod_gadget;
use examples::gadgets::math::long_integer_mod_inverse_gadget;
use examples::gadgets::math::long_integer_mod_pow_gadget;

pub struct ZkayPaillierFastDecGadget {
    n: LongElement,
    nSquare: LongElement,
    nBits: i32,
    lambda: LongElement,
    cipher: LongElement,
    plain: LongElement,
}
impl ZkayPaillierFastDecGadget {
    pub fn ZkayPaillierFastDecGadget(
        n: LongElement,
        nBits: i32,
        lambda: LongElement,
        cipher: LongElement,
        desc: Vec<String>,
    ) {
        super(desc);
        self.n = n;
        self.nBits = nBits;
        let nSquareMaxBits = 2 * nBits;
        let maxNumChunks =
            (nSquareMaxBits + (LongElement.CHUNK_BITWIDTH - 1)) / LongElement.CHUNK_BITWIDTH;
        self.nSquare = n.mul(n).align(maxNumChunks);
        self.lambda = lambda;
        self.cipher = cipher;
        buildCircuit();
    }
}
impl Gadget for ZkayPaillierFastDecGadget {
    fn buildCircuit() {
        let nSquareMinBits = 2 * nBits - 1; // Minimum bit length of n^2
        let lambdaInverse =
            LongIntegerModInverseGadget::new(lambda, n, false, "lambda^(-1)").getResult();

        // plain = L(cipher^lambda mod n^2) / lambda mod n
        let cPowLambda =
            LongIntegerModPowGadget::new(cipher, lambda, nSquare, nSquareMinBits, "c^lambda")
                .getResult();
        let lOutput =
            LongIntegerFloorDivGadget::new(cPowLambda.subtract(1), n, "(c^lambda - 1) / n")
                .getQuotient();
        let timesLambdaInverse = lOutput.mul(lambdaInverse);
        plain = LongIntegerModGadget::new(timesLambdaInverse, n, nBits, true).getRemainder();
    }

    pub fn getPlaintext() -> LongElement {
        return plain;
    }

    pub fn getOutputWires() -> Vec<WireType> {
        return plain.getArray();
    }
}
