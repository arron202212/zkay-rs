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
    pub n_square: LongElement,
    pub n_bits: i32,
    pub lambda: LongElement,
    pub cipher: LongElement,
    pub plain: Option<LongElement>,
}
impl ZkayPaillierFastDecGadget {
    pub fn new(
        n: LongElement,
        n_bits: i32,
        lambda: LongElement,
        cipher: LongElement,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let n_square_max_bits = 2 * n_bits;
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
                lambda,
                cipher,
                plain: None,
            },
        );
        _self.build_circuit();
        _self
    }
}
impl Gadget<ZkayPaillierFastDecGadget> {
    fn build_circuit(&mut self) {
        let n_square_min_bits = 2 * self.t.n_bits - 1; // Minimum bit length of n^2
        let lambda_inverse = LongIntegerModInverseGadget::new(
            self.t.lambda.clone(),
            self.t.n.clone(),
            false,
            &Some("lambda^(-1)".to_owned()),
            self.generator.clone(),
        )
        .get_result()
        .clone();

        // plain = L(cipher^lambda mod n^2) / lambda mod n
        let c_pow_lambda = LongIntegerModPowGadget::new(
            self.t.cipher.clone(),
            self.t.lambda.clone(),
            self.t.n_square.clone(),
            n_square_min_bits,
            -1,
            &Some("c^lambda".to_owned()),
            self.generator.clone(),
        )
        .get_result()
        .clone();
        let l_output = LongIntegerFloorDivGadget::new(
            c_pow_lambda.sub(1),
            self.t.n.clone(),
            0,
            &Some("(c^lambda - 1) / n".to_owned()),
            self.generator.clone(),
        )
        .get_quotient()
        .clone();
        let times_lambda_inverse = l_output.mul(&lambda_inverse);
        self.t.plain = Some(
            LongIntegerModGadget::new(
                times_lambda_inverse,
                self.t.n.clone(),
                self.t.n_bits,
                true,
                &None,
                self.generator.clone(),
            )
            .get_remainder()
            .clone(),
        );
    }

    pub fn getPlaintext(&self) -> &Option<LongElement> {
        &self.t.plain
    }
}
impl GadgetConfig for Gadget<ZkayPaillierFastDecGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        self.t.plain.as_ref().unwrap().get_array()
    }
}
