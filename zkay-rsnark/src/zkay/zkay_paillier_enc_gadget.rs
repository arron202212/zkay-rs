use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::math::long_integer_mod_gadget;
use crate::examples::gadgets::math::long_integer_mod_inverse_gadget;
use crate::examples::gadgets::math::long_integer_mod_pow_gadget;

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
        g: LongElement,
        plain: LongElement,
        random: LongElement,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let nSquareMaxBits = 2 * nBits; // Maximum bit length of n^2
        let maxNumChunks =
            (nSquareMaxBits + (LongElement::CHUNK_BITWIDTH - 1)) / LongElement::CHUNK_BITWIDTH;
        let nSquare = n.mul(n).align(maxNumChunks);

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
                g,
                plain,
                random,
                cipher: vec![],
            },
        };
        _self.buildCircuit();
        _self
    }

    fn buildCircuit(&mut self) {
        let nSquareMinBits = 2 * nBits - 1; // Minimum bit length of n^2
        // Prove that random is in Z_n* by checking that random has an inverse mod n
        let randInv = LongIntegerModInverseGadget::new(random, n, false).getResult();
        generator.addOneAssertion(randInv.checkNonZero());
        // let c = g^m * r^n mod n^2
        let gPowPlain = LongIntegerModPowGadget::new(
            g,
            plain,
            nBits,
            nSquare,
            nSquareMinBits,
            -1,
            &Some("g^m".to_owned()),
            self.cg(),
        )
        .getResult();
        let randPowN = LongIntegerModPowGadget::new(
            random,
            n,
            nBits,
            nSquare,
            nSquareMinBits,
            -1,
            &Some("r^m".to_owned()),
            self.cg(),
        )
        .getResult();
        let product = gPowPlain.mul(randPowN);
        cipher = LongIntegerModGadget::new(
            product,
            nSquare,
            nSquareMinBits,
            true,
            &Some("g^m * r^n mod n^2").to_owned(),
            self,
            cg(),
        )
        .getRemainder();
    }

    pub fn getCiphertext() -> LongElement {
        cipher
    }
}
impl GadgetConfig for Gadget<ZkayPaillierEncGadget> {
    fn getOutputWires() -> Vec<Option<WireType>> {
        cipher.getArray()
    }
}
