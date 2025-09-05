#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        auxiliary::long_element::LongElement,
        operations::gadget::{Gadget, GadgetConfig},
        structure::{circuit_generator::CircuitGenerator, wire_type::WireType},
    },
    examples::gadgets::math::{
        long_integer_division::LongIntegerDivisionConfig,
        long_integer_floor_div_gadget::LongIntegerFloorDivGadget,
        long_integer_mod_gadget::LongIntegerModGadget,
        long_integer_mod_inverse_gadget::LongIntegerModInverseGadget,
        long_integer_mod_pow_gadget::LongIntegerModPowGadget,
    },
    zkay::zkay_baby_jub_jub_gadget::{JubJubPoint, ZkayBabyJubJubGadget},
};

use rccell::RcCell;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct ZkayPaillierFastDecGadget {
    pub n: LongElement,
    pub nSquare: LongElement,
    pub nBits: i32,
    pub lambda: LongElement,
    pub cipher: LongElement,
    pub plain: Option<LongElement>,
}
impl ZkayPaillierFastDecGadget {
    pub fn new(
        n: LongElement,
        nBits: i32,
        lambda: LongElement,
        cipher: LongElement,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let nSquareMaxBits = 2 * nBits;
        let maxNumChunks =
            (nSquareMaxBits + (LongElement::CHUNK_BITWIDTH - 1)) / LongElement::CHUNK_BITWIDTH;
        let nSquare = n.clone().mul(&n).align(maxNumChunks as usize);
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                n,
                nSquare,
                nBits,
                lambda,
                cipher,
                plain: None,
            },
        );
        _self.buildCircuit();
        _self
    }
}
impl Gadget<ZkayPaillierFastDecGadget> {
    fn buildCircuit(&mut self) {
        let nSquareMinBits = 2 * self.t.nBits - 1; // Minimum bit length of n^2
        let lambdaInverse = LongIntegerModInverseGadget::new(
            self.t.lambda.clone(),
            self.t.n.clone(),
            false,
            &Some("lambda^(-1)".to_owned()),
            self.generator.clone(),
        )
        .getResult()
        .clone();

        // plain = L(cipher^lambda mod n^2) / lambda mod n
        let cPowLambda = LongIntegerModPowGadget::new(
            self.t.cipher.clone(),
            self.t.lambda.clone(),
            self.t.nSquare.clone(),
            nSquareMinBits,
            -1,
            &Some("c^lambda".to_owned()),
            self.generator.clone(),
        )
        .getResult()
        .clone();
        let lOutput = LongIntegerFloorDivGadget::new(
            cPowLambda.sub(1),
            self.t.n.clone(),
            0,
            &Some("(c^lambda - 1) / n".to_owned()),
            self.generator.clone(),
        )
        .getQuotient()
        .clone();
        let timesLambdaInverse = lOutput.mul(&lambdaInverse);
        self.t.plain = Some(
            LongIntegerModGadget::new(
                timesLambdaInverse,
                self.t.n.clone(),
                self.t.nBits,
                true,
                &None,
                self.generator.clone(),
            )
            .getRemainder()
            .clone(),
        );
    }

    pub fn getPlaintext(&self) -> &Option<LongElement> {
        &self.t.plain
    }
}
impl GadgetConfig for Gadget<ZkayPaillierFastDecGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        self.t.plain.as_ref().unwrap().getArray()
    }
}
