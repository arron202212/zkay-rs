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
        structure::{
            circuit_generator::CGConfig, circuit_generator::CircuitGenerator, wire_type::WireType,
        },
    },
    examples::gadgets::math::{
        long_integer_division::LongIntegerDivisionConfig,
        long_integer_mod_gadget::LongIntegerModGadget,
        long_integer_mod_inverse_gadget::LongIntegerModInverseGadget,
        long_integer_mod_pow_gadget::LongIntegerModPowGadget,
    },
    zkay::zkay_baby_jub_jub_gadget::{JubJubPoint, ZkayBabyJubJubGadget},
};

use rccell::RcCell;
use std::ops::{Add, Mul, Sub};

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
        let nSquare = n.clone().mul(&n).align(maxNumChunks as usize);

        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                n,
                nSquare,
                nBits,
                nSquareMaxBits,
                plain,
                random,
                cipher: None,
            },
        );
        _self.build_circuit();
        _self
    }
}
impl Gadget<ZkayPaillierFastEncGadget> {
    fn build_circuit(&mut self) {
        let nSquareMinBits = 2 * self.t.nBits - 1; // Minimum bit length of n^2
        // Prove that random is in Z_n* by checking that random has an inverse mod n
        let randInv = LongIntegerModInverseGadget::new(
            self.t.random.clone(),
            self.t.n.clone(),
            false,
            &None,
            self.generator.clone(),
        )
        .getResult()
        .clone();
        let generators = self.generator.clone();
        CircuitGenerator::add_one_assertion(
            self.generator.clone(),
            &randInv.check_non_zero(),
            &None,
        );
        // Compute c = g^m * r^n mod n^2
        let gPowPlain = self
            .t
            .n
            .clone()
            .mul(&self.t.plain)
            .add(1)
            .align(self.t.nSquare.get_size());
        let randPowN = LongIntegerModPowGadget::new(
            self.t.random.clone(),
            self.t.n.clone(),
            self.t.nSquare.clone(),
            nSquareMinBits,
            self.t.nBits,
            &Some("r^n".to_owned()),
            self.generator.clone(),
        )
        .getResult()
        .clone();
        let product = gPowPlain.mul(&randPowN);
        self.t.cipher = Some(
            LongIntegerModGadget::new(
                product,
                self.t.nSquare.clone(),
                nSquareMinBits,
                true,
                &Some("g^m * r^n mod n^2".to_owned()),
                self.generator.clone(),
            )
            .getRemainder()
            .clone(),
        );
    }

    pub fn getCiphertext(&self) -> &Option<LongElement> {
        &self.t.cipher
    }
}

impl GadgetConfig for Gadget<ZkayPaillierFastEncGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        self.t.cipher.as_ref().unwrap().get_array()
    }
}
