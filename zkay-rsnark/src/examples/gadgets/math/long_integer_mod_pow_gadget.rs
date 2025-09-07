#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::{long_element, long_element::LongElement},
        config::config::CONFIGS,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::{
            gadget::Gadget,
            gadget::GadgetConfig,
            primitive::{
                assert_basic_op::AssertBasicOp, basic_op::BasicOp, mul_basic_op::MulBasicOp,
            },
            wire_label_instruction::LabelType,
            wire_label_instruction::WireLabelInstruction,
        },
        structure::{
            circuit_generator::{CGConfig, CGConfigFields, CircuitGenerator},
            constant_wire::ConstantWire,
            variable_bit_wire::VariableBitWire,
            variable_wire::VariableWire,
            wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    examples::gadgets::math::{
        long_integer_division::LongIntegerDivision,
        long_integer_division::LongIntegerDivisionConfig,
        long_integer_mod_gadget::LongIntegerModGadget,
    },
    util::{
        util::ARcCell,
        util::{BigInteger, Util},
    },
};

use std::{
    fmt::Debug,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Mul, Sub},
};

use rccell::RcCell;

//  * This gadget computes the result of the modular exponentiation c = b^e mod m,
//  * where c, b, e, and m are LongElements.

use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct LongIntegerModPowGadget {
    pub b: LongElement,  // base
    pub e: LongElement,  // exponent
    pub e_max_bits: i32, // maximum bit length of e
    pub m: LongElement,  // modulus
    pub m_min_bits: i32, // minimum bit length of m

    pub c: LongElement, // c = m^e mod m
}
impl LongIntegerModPowGadget {
    // pub fn new(
    //     b: LongElement,
    //     e: LongElement,
    //     m: LongElement,
    //     mMinBitLength: i32,
    //     desc: &Option<String>,
    // ) -> Self {
    //     Self::news(b, e, -1, m, mMinBitLength, desc);
    // }

    pub fn new(
        b: LongElement,
        e: LongElement,
        m: LongElement,
        m_min_bits: i32,
        e_max_bits: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self>::new(
            generator,
            desc,
            Self {
                c: b.clone(),
                b,
                e,
                e_max_bits,
                m,
                m_min_bits,
            },
        );
        _self.build_circuit();
        _self
    }
}
impl Gadget<LongIntegerModPowGadget> {
    fn build_circuit(&mut self) {
        let one = LongElement::newb(vec![Util::one()], self.generator.clone().downgrade());
        let e_bits = self.t.e.get_bitsi(self.t.e_max_bits).as_array().clone();

        // Start with product = 1
        let mut product = one.clone();
        // From the most significant to the least significant bit of the exponent, proceed as follow:
        // product = product^2 mod m
        // if eBit == 1) product = (product * base mod m

        let start = std::time::Instant::now();
        for i in (0..e_bits.len()).rev() {
            let square = product.clone().mul(&product);
            let square_mod_m = LongIntegerModGadget::new(
                square,
                self.t.m.clone(),
                self.t.m_min_bits,
                false,
                &Some("mod_pow: prod^2 mod m".to_owned()),
                self.generator.clone(),
            )
            .get_remainder()
            .clone();
            let square_times_base = square_mod_m
                .clone()
                .mul(&one.mux_bit(&self.t.b, e_bits[i].as_ref().unwrap()));
            product = LongIntegerModGadget::new(
                square_times_base,
                self.t.m.clone(),
                self.t.m_min_bits,
                false,
                &Some("mod_pow: prod * base mod m".to_owned()),
                self.generator.clone(),
            )
            .get_remainder()
            .clone();
        }

        self.t.c = LongIntegerModGadget::new(
            product,
            self.t.m.clone(),
            0,
            true,
            &Some("mod_pow: prod mod m".to_owned()),
            self.generator.clone(),
        )
        .get_remainder()
        .clone();
    }

    pub fn get_result(&self) -> &LongElement {
        &self.t.c
    }
}
impl GadgetConfig for Gadget<LongIntegerModPowGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        self.t.c.get_array()
    }
}
