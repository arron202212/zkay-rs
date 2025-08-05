use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::math::long_integer_mod_gadget;
use crate::examples::gadgets::math::long_integer_mod_inverse_gadget;
use crate::examples::gadgets::math::long_integer_mod_pow_gadget;

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
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let nSquareMaxBits = 2 * nBits; // Maximum bit length of n^2
        let maxNumChunks =
            (nSquareMaxBits + (LongElement::CHUNK_BITWIDTH - 1)) / LongElement::CHUNK_BITWIDTH;
        let nSquare = n.mul(&n).align(maxNumChunks);

        let mut _self = Gadget::<Self> {
            generator,
            description: desc
                .as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
            t: Self {
                n,
                nSquare,
                nBits,
                nSquareMaxBits,
                plain,
                random,
                cipher: vec![],
            },
        };
        _self.buildCircuit();
        _self
    }
}
impl Gadget<ZkayPaillierFastEncGadget> {
    fn buildCircuit(&mut self) {
        let nSquareMinBits = 2 * nBits - 1; // Minimum bit length of n^2
        // Prove that random is in Z_n* by checking that random has an inverse mod n
        let randInv = LongIntegerModInverseGadget::new(random, n, false).getResult();
        generator.addOneAssertion(randInv.checkNonZero());
        // Compute c = g^m * r^n mod n^2
        let gPowPlain = n.mul(plain).add(1).align(nSquare.getSize());
        let randPowN = LongIntegerModPowGadget::new(
            random,
            n,
            nBits,
            nSquare,
            nSquareMinBits,
            -1,
            &Some("r^n".to_owned()),
            self.cg(),
        )
        .getResult();
        let product = gPowPlain.mul(randPowN);
        cipher =
            LongIntegerModGadget::new(product, nSquare, nSquareMinBits, true, "g^m * r^n mod n^2")
                .getRemainder();
    }

    pub fn getCiphertext(&self) -> &LongElement {
        self.t.cipher
    }
}

impl GadgetConfig for Gadget<ZkayPaillierFastEncGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        self.t.cipher.getArray()
    }
}
