use circuit::auxiliary::long_element;
use circuit::operations::gadget;
use circuit::structure::wire;
use examples::gadgets::math::long_integer_mod_gadget;
use examples::gadgets::math::long_integer_mod_inverse_gadget;
use examples::gadgets::math::long_integer_mod_pow_gadget;

pub struct ZkayPaillierFastEncGadget {
    n: LongElement,
    nSquare: LongElement,
    nBits: i32,
    nSquareMaxBits: i32,
    plain: LongElement,
    random: LongElement,
    cipher: LongElement,
}
impl ZkayPaillierFastEncGadget {
    pub fn new(
        n: LongElement,
        nBits: i32,
        plain: LongElement,
        random: LongElement,
        desc: Vec<String>,
    ) -> Self {
        super(desc);
        self.n = n;
        self.nBits = nBits;
        self.nSquareMaxBits = 2 * nBits; // Maximum bit length of n^2
        let maxNumChunks =
            (nSquareMaxBits + (LongElement.CHUNK_BITWIDTH - 1)) / LongElement.CHUNK_BITWIDTH;
        self.nSquare = n.mul(n).align(maxNumChunks);
        self.plain = plain;
        self.random = random;
        buildCircuit();
    }
}

impl Gadget for ZkayPaillierFastEncGadget {
    fn buildCircuit() {
        let nSquareMinBits = 2 * nBits - 1; // Minimum bit length of n^2
        // Prove that random is in Z_n* by checking that random has an inverse mod n
        let randInv = LongIntegerModInverseGadget::new(random, n, false).getResult();
        generator.addOneAssertion(randInv.checkNonZero());
        // Compute c = g^m * r^n mod n^2
        let gPowPlain = n.mul(plain).add(1).align(nSquare.getSize());
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

    pub fn getOutputWires() -> Vec<Wire> {
        return cipher.getArray();
    }
}
