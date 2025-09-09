#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
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
    pub n_square: LongElement,
    pub n_bits: i32,
    pub n_square_max_bits: i32,
    pub plain: LongElement,
    pub random: LongElement,
    pub cipher: Option<LongElement>,
}
impl ZkayPaillierFastEncGadget {
    pub fn new(
        n: LongElement,
        n_bits: i32,
        plain: LongElement,
        random: LongElement,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let n_square_max_bits = 2 * n_bits; // Maximum bit length of n^2
        let max_num_chunks =
            (n_square_max_bits + (LongElement::CHUNK_BITWIDTH - 1)) / LongElement::CHUNK_BITWIDTH;
        let n_square = n.clone().mul(&n).align(max_num_chunks as usize);

        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                n,
                n_square,
                n_bits,
                n_square_max_bits,
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
        let n_square_min_bits = 2 * self.t.n_bits - 1; // Minimum bit length of n^2
        // Prove that random is in Z_n* by checking that random has an inverse mod n
        let rand_inv = LongIntegerModInverseGadget::new(
            self.t.random.clone(),
            self.t.n.clone(),
            false,
            &None,
            self.generator.clone(),
        )
        .get_result()
        .clone();
        let generators = self.generator.clone();
        CircuitGenerator::add_one_assertion(self.generator.clone(), &rand_inv.check_non_zero());
        // Compute c = g^m * r^n mod n^2
        let g_pow_plain = self
            .t
            .n
            .clone()
            .mul(&self.t.plain)
            .add(1)
            .align(self.t.n_square.get_size());
        let rand_pow_n = LongIntegerModPowGadget::new(
            self.t.random.clone(),
            self.t.n.clone(),
            self.t.n_square.clone(),
            n_square_min_bits,
            self.t.n_bits,
            &Some("r^n".to_owned()),
            self.generator.clone(),
        )
        .get_result()
        .clone();
        let product = g_pow_plain.mul(&rand_pow_n);
        self.t.cipher = Some(
            LongIntegerModGadget::new(
                product,
                self.t.n_square.clone(),
                n_square_min_bits,
                true,
                &Some("g^m * r^n mod n^2".to_owned()),
                self.generator.clone(),
            )
            .get_remainder()
            .clone(),
        );
    }

    pub fn get_cipher_text(&self) -> &Option<LongElement> {
        &self.t.cipher
    }
}

impl GadgetConfig for Gadget<ZkayPaillierFastEncGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        self.t.cipher.as_ref().unwrap().get_array()
    }
}
