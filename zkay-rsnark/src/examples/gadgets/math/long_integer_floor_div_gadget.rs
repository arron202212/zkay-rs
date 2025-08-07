#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

use crate::examples::gadgets::math::long_integer_division::{
    LongIntegerDivision, LongIntegerDivisionConfig,
};
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
use rccell::RcCell;
use std::fmt::Debug;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
/**
 * This gadget provides floor(a / b), when both operands are represented as long
 * elements. You can check the RSA gadgets/circuit generators for an example.
 * Most of the optimizations that reduce the cost of this step are more visible
 * in the LongElement class methods called by this gadget.
 */
use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct LongIntegerFloorDivGadget;
impl LongIntegerFloorDivGadget {
    pub fn new(
        a: LongElement,
        b: LongElement,
        bMinBitwidth: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<LongIntegerDivision<Self>> {
        //super(a, b, bMinBitwidth, true, desc);
        LongIntegerDivision::<Self>::new(a, b, bMinBitwidth, true, desc, generator)
    }
}
// LongIntegerDivision exteand  GadgetConfig
// impl LongIntegerDivision for LongIntegerFloorDivGadget {
//     fn getOutputWires(&self) -> Vec<Option<WireType>> {
//         getQuotient().getArray()
//     }
// }
// impl GadgetConfig for Gadget<LongIntegerFloorDivGadget> {
//     fn getOutputWires(&self) -> &Vec<Option<WireType>> {
//         self.getQuotient().getArray()
//     }
// }

crate::impl_long_integer_division_config_for!(LongIntegerFloorDivGadget);

impl GadgetConfig for Gadget<LongIntegerDivision<LongIntegerFloorDivGadget>> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        self.getQuotient().getArray()
    }
}
