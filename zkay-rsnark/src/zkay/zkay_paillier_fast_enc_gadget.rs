#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::auxiliary::long_element::LongElement;
use crate::circuit::operations::gadget::Gadget;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::math::long_integer_mod_gadget::LongIntegerModGadget;
use crate::examples::gadgets::math::long_integer_mod_inverse_gadget::LongIntegerModInverseGadget;
use crate::examples::gadgets::math::long_integer_mod_pow_gadget::LongIntegerModPowGadget;
use crate::zkay::zkay_baby_jub_jub_gadget::JubJubPoint;
use crate::zkay::zkay_baby_jub_jub_gadget::ZkayBabyJubJubGadget;

use rccell::RcCell;

#[derive(Debug, Clone)]
pub struct ZkayPaillierFastEncGadget {
    pub n: LongElement,
    pub nSquare: LongElement,
    pub nBits: i32,
    pub nSquareMaxBits: i32,
    pub plain: LongElement,
    pub random: LongElement,
    pub cipher: Option<LongElement>,
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
                cipher: None,
            },
        };
        _self.buildCircuit();
        _self
    }
}
impl Gadget<ZkayPaillierFastEncGadget> {
    fn buildCircuit(&mut self) {
        let nSquareMinBits = 2 * self.t.nBits - 1; // Minimum bit length of n^2
        // Prove that random is in Z_n* by checking that random has an inverse mod n
        let randInv = LongIntegerModInverseGadget::new(self.t.random, self.t.n, false).getResult();
        self.generator.addOneAssertion(randInv.checkNonZero());
        // Compute c = g^m * r^n mod n^2
        let gPowPlain = self
            .t
            .n
            .clone()
            .mul(self.plain)
            .add(1)
            .align(self.t.nSquare.getSize());
        let randPowN = LongIntegerModPowGadget::new(
            self.t.random,
            self.t.n,
            self.t.nBits,
            self.t.nSquare,
            self.t.nSquareMinBits,
            -1,
            &Some("r^n".to_owned()),
            self.cg(),
        )
        .getResult();
        let product = gPowPlain.mul(randPowN);
        self.t.cipher = Some(
            LongIntegerModGadget::new(
                product,
                self.t.nSquare,
                self.t.nSquareMinBits,
                true,
                "g^m * r^n mod n^2",
            )
            .getRemainder()
            .clone(),
        );
    }

    pub fn getCiphertext(&self) -> &LongElement {
        self.t.cipher.as_ref().unwrap()
    }
}

impl GadgetConfig for Gadget<ZkayPaillierFastEncGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        self.t.cipher.as_ref().unwrap().getArray()
    }
}
