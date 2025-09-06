#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::auxiliary::long_element;
use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::long_element::LongElement,
        config::config::Configs,
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
    util::{
        util::ARcCell,
        util::{BigInteger, Util},
    },
};
// use crate::circuit::operations::gadget::GadgetConfig;
// use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::math::long_integer_division::LongIntegerDivision;
use crate::examples::gadgets::math::long_integer_division::LongIntegerDivisionConfig;
use crate::examples::gadgets::math::long_integer_mod_gadget::LongIntegerModGadget;

use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Sub};

//  * This gadget computes the result of the modular exponentiation c = b^e mod m,
//  * where c, b, e, and m are LongElements.

use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct LongIntegerModPowGadget {
    pub b: LongElement, // base
    pub e: LongElement, // exponent
    pub eMaxBits: i32,  // maximum bit length of e
    pub m: LongElement, // modulus
    pub mMinBits: i32,  // minimum bit length of m

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
        mMinBits: i32,
        eMaxBits: i32,
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
                eMaxBits,
                m,
                mMinBits,
            },
        );
        _self.build_circuit();
        _self
    }
}
impl Gadget<LongIntegerModPowGadget> {
    fn build_circuit(&mut self) {
        let one = LongElement::newb(vec![Util::one()], self.generator.clone().downgrade());
        let eBits = self.t.e.get_bitsi(self.t.eMaxBits).as_array().clone();

        // Start with product = 1
        let mut product = one.clone();
        // From the most significant to the least significant bit of the exponent, proceed as follow:
        // product = product^2 mod m
        // if eBit == 1) product = (product * base mod m

        let start = std::time::Instant::now();
        println!("========eBits.len()====={}", eBits.len());
        for i in (0..eBits.len()).rev() {
            println!("=={i}======eBits.len()====={:?}", start.elapsed());
            let square = product.clone().mul(&product);
            let squareModM = LongIntegerModGadget::new(
                square,
                self.t.m.clone(),
                self.t.mMinBits,
                false,
                &Some("modPow: prod^2 mod m".to_owned()),
                self.generator.clone(),
            )
            .getRemainder()
            .clone();
            let squareTimesBase = squareModM
                .clone()
                .mul(&one.mux_bit(&self.t.b, eBits[i].as_ref().unwrap()));
            product = LongIntegerModGadget::new(
                squareTimesBase,
                self.t.m.clone(),
                self.t.mMinBits,
                false,
                &Some("modPow: prod * base mod m".to_owned()),
                self.generator.clone(),
            )
            .getRemainder()
            .clone();
        }

        self.t.c = LongIntegerModGadget::new(
            product,
            self.t.m.clone(),
            0,
            true,
            &Some("modPow: prod mod m".to_owned()),
            self.generator.clone(),
        )
        .getRemainder()
        .clone();
    }

    pub fn getResult(&self) -> &LongElement {
        &self.t.c
    }
}
impl GadgetConfig for Gadget<LongIntegerModPowGadget> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        self.t.c.get_array()
    }
}
