use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;
use examples::gadgets::math::long_integer_mod_gadget;
use examples::gadgets::math::long_integer_mod_inverse_gadget;
use examples::gadgets::math::long_integer_mod_pow_gadget;

pub struct ZkayPaillierEncGadget {
    n: LongElement,
    nSquare: LongElement,
    nBits: i32,
    nSquareMaxBits: i32,
    g: LongElement,
    plain: LongElement,
    random: LongElement,
    cipher: LongElement,
}
impl ZkayPaillierEncGadget {
    pub fn new(
        n: LongElement,
        nBits: i32,
        generator: LongElement,
        plain: LongElement,
        random: LongElement,
        desc: &Option<String>,
    ) -> Self {
        super(desc);
        self.n = n;
        self.nBits = nBits;
        self.nSquareMaxBits = 2 * nBits; // Maximum bit length of n^2
        let maxNumChunks =
            (nSquareMaxBits + (LongElement.CHUNK_BITWIDTH - 1)) / LongElement.CHUNK_BITWIDTH;
        self.nSquare = n.mul(n).align(maxNumChunks);
        self.g = generator;
        self.plain = plain;
        self.random = random;
        buildCircuit();
    }
}
impl Gadget for ZkayPaillierEncGadget {
    fn buildCircuit() {
        let nSquareMinBits = 2 * nBits - 1; // Minimum bit length of n^2
        // Prove that random is in Z_n* by checking that random has an inverse mod n
        let randInv = LongIntegerModInverseGadget::new(random, n, false).getResult();
        generator.addOneAssertion(randInv.checkNonZero());
        // let c = g^m * r^n mod n^2
        let gPowPlain =
            LongIntegerModPowGadget::new(g, plain, nBits, nSquare, nSquareMinBits, "g^m")
                .getResult();
        let randPowN =
            LongIntegerModPowGadget::new(random, n, nBits, nSquare, nSquareMinBits, "r^n")
                .getResult();
        let product = gPowPlain.mul(randPowN);
        cipher =
            LongIntegerModGadget::new(product, nSquare, nSquareMinBits, true, "g^m * r^n mod n^2")
                .getRemainder();
    }

    pub fn getCiphertext() -> LongElement {
        return cipher;
    }

    pub fn getOutputWires() -> Vec<Option<WireType>> {
        return cipher.getArray();
    }
}
