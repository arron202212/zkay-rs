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
        auxiliary::long_element::LongElement,
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
    examples::gadgets::math::long_integer_division::{
        LongIntegerDivision, LongIntegerDivisionConfig,
    },
    util::{
        util::ARcCell,
        util::{BigInteger, Util},
    },
};

use rccell::RcCell;
use std::{
    fmt::Debug,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
};

//  * This gadget provides floor(a / b), when both operands are represented as long
//  * elements. You can check the RSA gadgets/circuit generators for an example.
//  * Most of the optimizations that reduce the cost of this step are more visible
//  * in the LongElement class methods called by this gadget.

use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct LongIntegerFloorDivGadget;
impl LongIntegerFloorDivGadget {
    pub fn new(
        a: LongElement,
        b: LongElement,
        b_min_bitwidth: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<LongIntegerDivision<Self>> {
        //super(a, b, b_min_bitwidth, true, desc);
        LongIntegerDivision::<Self>::new(a, b, b_min_bitwidth, true, desc, generator)
    }
}
// LongIntegerDivision exteand  GadgetConfig
// impl LongIntegerDivision for LongIntegerFloorDivGadget {
//     fn get_output_wires(&self) -> Vec<Option<WireType>> {
//         get_quotient().get_array()
//     }
// }
// impl GadgetConfig for Gadget<LongIntegerFloorDivGadget> {
//     fn get_output_wires(&self) -> &Vec<Option<WireType>> {
//         self.get_quotient().get_array()
//     }
// }

crate::impl_long_integer_division_config_for!(LongIntegerFloorDivGadget);

impl GadgetConfig for Gadget<LongIntegerDivision<LongIntegerFloorDivGadget>> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        self.get_quotient().get_array()
    }
}
