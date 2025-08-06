#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget::Gadget;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::math::long_integer_floor_div_gadget;
use crate::examples::gadgets::math::long_integer_mod_gadget;
use crate::examples::gadgets::math::long_integer_mod_pow_gadget;
use crate::zkay::zkay_baby_jub_jub_gadget::JubJubPoint;
use crate::zkay::zkay_baby_jub_jub_gadget::ZkayBabyJubJubGadget;
use crate::zkay::zkay_paillier_dec_gadget::long_element::LongElement;
use crate::zkay::zkay_paillier_dec_gadget::long_integer_floor_div_gadget::LongIntegerFloorDivGadget;
use crate::zkay::zkay_paillier_dec_gadget::long_integer_mod_gadget::LongIntegerModGadget;
use crate::zkay::zkay_paillier_dec_gadget::long_integer_mod_pow_gadget::LongIntegerModPowGadget;
use rccell::RcCell;

pub struct ZkayPaillierDecGadget {
    n: LongElement,
    nSquare: LongElement,
    nBits: i32,
    lambda: LongElement,
    mu: LongElement,
    cipher: LongElement,
    plain: Option<LongElement>,
}

impl ZkayPaillierDecGadget {
    pub fn new(
        n: LongElement,
        nBits: i32,
        lambda: LongElement,
        mu: LongElement,
        cipher: LongElement,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let nSquareMaxBits = 2 * nBits;
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
                lambda,
                mu,
                cipher,
                plain: None,
            },
        };
        _self.buildCircuit();
        _self
    }
}
impl Gadget<ZkayPaillierDecGadget> {
    fn buildCircuit(&mut self) {
        let nSquareMinBits = 2 * self.nBits - 1; // Minimum bit length of n^2

        // plain = L(cipher^lambda mod n^2) * mu mod n
        let cPowLambda = LongIntegerModPowGadget::new(
            self.cipher,
            self.lambda,
            self.nSquare,
            self.nSquareMinBits,
            -1,
            &Some("c^lambda".to_owned()),
            self.cg(),
        )
        .getResult();
        let lOutput =
            LongIntegerFloorDivGadget::new(cPowLambda.sub(1), self.n, "(c^lambda - 1) / n")
                .getQuotient();
        let timesMu = lOutput.mul(self.mu);
        self.t.plain = LongIntegerModGadget::new(timesMu, self.n, self.nBits, true).getRemainder();
    }

    pub fn getPlaintext(&self) -> &LongElement {
        &self.t.plain
    }
}
impl GadgetConfig for Gadget<ZkayPaillierDecGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        self.plain.getArray()
    }
}
