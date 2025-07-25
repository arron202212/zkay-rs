#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
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
                assert_basic_op::{AssertBasicOp, new_assert},
                basic_op::BasicOp,
                mul_basic_op::{MulBasicOp, new_mul},
            },
            wire_label_instruction::LabelType,
            wire_label_instruction::WireLabelInstruction,
        },
        structure::{
            circuit_generator::{CGConfig, CGConfigFields, CircuitGenerator},
            constant_wire::{ConstantWire, new_constant},
            variable_bit_wire::VariableBitWire,
            variable_wire::{VariableWire, new_variable},
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    util::{
        run_command::run_command,
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
use std::ops::{Mul,Add,Sub};
/**
 * This gadget computes the result of the modular exponentiation c = b^e mod m,
 * where c, b, e, and m are LongElements.
 */
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct LongIntegerModPowGadget {
    b: LongElement, // base
    e: LongElement, // exponent
    eMaxBits: i32,  // maximum bit length of e
    m: LongElement, // modulus
    mMinBits: i32,  // minimum bit length of m

    c: LongElement, // c = m^e mod m
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
        eMaxBits: i32,
        m: LongElement,
        mMinBits: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let mut _self = Gadget::<Self> {
            generator,
            description: desc.as_ref().map_or_else(|| String::new(), |d| d.to_owned()),
            t: Self {c:b.clone(),
                b,
                e,
                eMaxBits,
                m,
                mMinBits,
            },
        };
        _self.buildCircuit();
        _self
    }
}
impl Gadget<LongIntegerModPowGadget> {
    fn buildCircuit(&mut self) {
        let one = LongElement::newb(vec![Util::one()],self.generator.clone().downgrade());
        let eBits = self.t.e.getBitsi(self.t.eMaxBits).asArray().clone();

        // Start with product = 1
        let mut product = one.clone();
        // From the most significant to the least significant bit of the exponent, proceed as follow:
        // product = product^2 mod m
        // if eBit == 1) product = (product * base mod m
        for i in (0..=eBits.len() - 1).rev() {
            let square = product.clone().mul(&product);
            let squareModM = LongIntegerModGadget::new(
                square,
                self.t.m.clone(),
                self.t.mMinBits,
                false,
                &Some("modPow: prod^2 mod m".to_owned()),
                self.generator.clone(),
            )
            .getRemainder().clone();
            let squareTimesBase = squareModM.clone().mul(&one.muxBit(&self.t.b, eBits[i].as_ref().unwrap()));
            product = LongIntegerModGadget::new(
                squareTimesBase,
                self.t.m.clone(),
                self.t.mMinBits,
                false,
                &Some("modPow: prod * base mod m".to_owned()),
                self.generator.clone(),
            )
            .getRemainder().clone();
        }

        self.t.c = LongIntegerModGadget::new(product, self.t.m.clone(),0, true, &Some("modPow: prod mod m".to_owned()),self.generator.clone())
            .getRemainder().clone();
    }

    pub fn getResult(&self) -> &LongElement {
        &self.t.c
    }
}
impl GadgetConfig for Gadget<LongIntegerModPowGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        self.t.c.getArray()
    }
}
