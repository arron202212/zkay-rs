use circuit::auxiliary::long_element;
use circuit::operations::gadget;
use circuit::structure::wire;

/**
 * This gadget computes the result of the modular exponentiation c = b^e mod m,
 * where c, b, e, and m are LongElements.
 */
pub struct LongIntegerModPowGadget {
    b: LongElement, // base
    e: LongElement, // exponent
    eMaxBits: i32,  // maximum bit length of e
    m: LongElement, // modulus
    mMinBits: i32,  // minimum bit length of m

    c: LongElement, // c = m^e mod m
}
impl LongIntegerModPowGadget {
    pub fn new(
        b: LongElement,
        e: LongElement,
        m: LongElement,
        mMinBitLength: i32,
        desc: Vec<String>,
    ) -> Self {
        this(b, e, -1, m, mMinBitLength, desc);
    }

    pub fn new(
        b: LongElement,
        e: LongElement,
        eMaxBits: i32,
        m: LongElement,
        mMinBits: i32,
        desc: Vec<String>,
    ) -> Self {
        super(desc);
        self.b = b;
        self.e = e;
        self.eMaxBits = eMaxBits;
        self.m = m;
        self.mMinBits = mMinBits;
        buildCircuit();
    }
}
impl Gadget for LongIntegerModPowGadget {
    fn buildCircuit() {
        let one = LongElement::new(vec![BigInteger.ONE]);
        let eBits = e.getBits(eMaxBits).asArray();

        // Start with product = 1
        let product = one;
        // From the most significant to the least significant bit of the exponent, proceed as follow:
        // product = product^2 mod m
        // if eBit == 1) product = (product * base mod m
        for i in (0..=eBits.length - 1).rev() {
            let square = product.mul(product);
            let squareModM =
                LongIntegerModGadget::new(square, m, mMinBits, false, "modPow: prod^2 mod m")
                    .getRemainder();
            let squareTimesBase = squareModM.mul(one.muxBit(b, eBits[i]));
            product = LongIntegerModGadget::new(
                squareTimesBase,
                m,
                mMinBits,
                false,
                "modPow: prod * base mod m",
            )
            .getRemainder();
        }

        c = LongIntegerModGadget::new(product, m, true, "modPow: prod mod m").getRemainder();
    }

    pub fn getResult() -> LongElement {
        return c;
    }

    pub fn getOutputWires() -> Vec<Wire> {
        return c.getArray();
    }
}
