use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::math::long_integer_floor_div_gadget;
use crate::examples::gadgets::math::long_integer_mod_gadget;
use crate::examples::gadgets::math::long_integer_mod_pow_gadget;

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
        desc: &Option<String>,
    ) -> Self {
        //super(desc);
        self.n = n;
        self.nBits = nBits;
        let nSquareMaxBits = 2 * nBits;
        let maxNumChunks =
            (nSquareMaxBits + (LongElement::CHUNK_BITWIDTH - 1)) / LongElement::CHUNK_BITWIDTH;
        self.nSquare = n.mul(n).align(maxNumChunks);
        self.lambda = lambda;
        self.mu = mu;
        self.cipher = cipher;
        _self.buildCircuit();
        _self
    }

    fn buildCircuit(&mut self) {
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
        plain
    }
}
impl GadgetConfig for Gadget<ZkayPaillierDecGadget> {
    fn getOutputWires() -> Vec<Option<WireType>> {
        plain.getArray()
    }
}
