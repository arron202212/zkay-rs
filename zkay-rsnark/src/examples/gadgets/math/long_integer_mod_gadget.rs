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
        auxiliary::long_element,
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

//  * This gadget provides a % b, when both operands are represented as long
//  * elements. You can check the RSA gadgets/circuit generators for an example.
//  * Most of the optimizations that reduce the cost of this step are more visible
//  * in the LongElement class methods called by this gadget.

use zkay_derive::ImplStructNameConfig;
#[derive(Debug, Clone, ImplStructNameConfig)]
pub struct LongIntegerModGadget;
impl LongIntegerModGadget {
    //
    // //@param a
    // //@param b
    // //@param restrict_range
    // //		if true, the output will be forced to be less than b,
    // //		otherwise the output remainder will only be guaranteed
    // //		to have the same bitwidth as b, but not necessarily less
    // //		than b. The second case is helpful when the purpose is
    // //		just to reduce the range, while having consistent
    // //		output. As an example (in a short integer case for
    // //		simplicity): assume we are interested in this operation
    // //		3001 % 10. The output should be 1 in normal cases, but
    // //		to save some operations, we might skip checking that the
    // //		result is less than the modulus and just check that it
    // //		has the same bitwidth as the modulus, which we must do
    // //		anyway since the result is provided as a witness. In
    // //		that case, the output of this gadget could be 1 or 11,
    // //		which in some contexts would be ok, e.g. in intermediate
    // //		operations. See the RSA encryption gadget for an
    // //		illustration.
    // //@param desc
    //
    //     // pub  fn new(a:LongElement, b:LongElement, restrict_range:bool, desc:Vec<String>)  ->Self{
    //     // 	//super(a, b, restrict_range, desc);
    //     // }

    //
    // //@param a
    // //@param b
    // //@param b_min_bitwidth
    // //		The minimum bitwidth of the second operand
    // //@param restrict_range
    // //		if true, the output will be forced to be less than b,
    // //		otherwise the output remainder will only be guaranteed
    // //		to have the same bitwidth as b, but not necessarily less
    // //		than b. The second case is helpful when the purpose is
    // //		just to reduce the range, while having consistent
    // //		output. As an example (in a short integer case for
    // //		simplicity): assume we are interested in this operation
    // //		3001 % 10. The output should be 1 in normal cases, but
    // //		to save some operations, we might skip checking that the
    // //		result is less than the modulus and just check that it
    // //		has the same bitwidth as the modulus, which we must do
    // //		anyway since the result is provided as a witness. In
    // //		that case, the output of this gadget could be 1 or 11,
    // //		which in some contexts would be ok, e.g. in intermediate
    // //		operations. See the RSA encryption gadget for an
    // //		illustration.
    // //@param desc
    //
    #[inline]
    pub fn new(
        a: LongElement,
        b: LongElement,
        b_min_bitwidth: i32,
        restrict_range: bool,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<LongIntegerDivision<Self>> {
        Self::new_with_option(a, b, b_min_bitwidth, restrict_range, &None, generator)
    }
    pub fn new_with_option(
        a: LongElement,
        b: LongElement,
        b_min_bitwidth: i32,
        restrict_range: bool,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<LongIntegerDivision<Self>> {
        LongIntegerDivision::<Self>::new_with_option(
            a,
            b,
            b_min_bitwidth,
            restrict_range,
            desc,
            generator,
        )
    }
}

crate::impl_long_integer_division_config_for!(LongIntegerModGadget);

impl GadgetConfig for Gadget<LongIntegerDivision<LongIntegerModGadget>> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        self.get_remainder().get_array()
    }
}
