#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::config::config::Configs;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::primitive::const_mul_basic_op::{ConstMulBasicOp, new_const_mul};
use crate::circuit::operations::primitive::mul_basic_op::{MulBasicOp, new_mul};
use crate::circuit::operations::primitive::non_zero_check_basic_op::{
    NonZeroCheckBasicOp, new_non_zero_check,
};
use crate::circuit::operations::primitive::or_basic_op::{OrBasicOp, new_or};
use crate::circuit::operations::primitive::pack_basic_op::{PackBasicOp, new_pack};
use crate::circuit::operations::primitive::split_basic_op::{SplitBasicOp, new_split};
use crate::circuit::operations::primitive::xor_basic_op::{XorBasicOp, new_xor};
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::linear_combination_wire::LinearCombinationWire;
use crate::circuit::structure::variable_bit_wire::VariableBitWire;
use crate::circuit::structure::variable_wire::VariableWire;
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::{BigInteger, Util};
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
pub trait MulWire<T = Self> {
    fn mul_wire(self, b: T, desc: &Option<String>) -> WireType;
}
pub trait AddWire<T = Self> {
    fn add_wire(self, w: T, desc: &Option<String>) -> WireType;
}
pub trait SubWire<T = Self> {
    fn sub_wire(self, w: T, desc: &Option<String>) -> WireType;
}

// pub trait Neg<T=Self>{
//     fn neg(self,  desc: &Option<String>) -> WireType;
// }

// pub trait Or<T=Self>{
//      fn or(self, w: T, desc: &Option<String>) -> WireType ;
// }

// pub trait Xor<T=Self>{
//      fn xor(self, w: T, desc: &Option<String>) -> WireType ;
// }

// pub trait And<T=Self>{
//      fn and(self, w: T, desc: &Option<String>) -> WireType ;
// }

pub trait XorBitwise<T = Self> {
    fn xor_bitwise(self, w: T, numBits: i32, desc: &Option<String>) -> WireType;
}

pub trait AndBitwise<T = Self> {
    fn and_bitwise(self, w: T, numBits: i32, desc: &Option<String>) -> WireType;
}

pub trait OrBitwise<T = Self> {
    fn or_bitwise(self, w: T, numBits: i32, desc: &Option<String>) -> WireType;
}

pub trait IsEqualTo<T = Self> {
    fn is_equal_to(&self, w: T, desc: &Option<String>) -> WireType;
}

pub trait IsLessThanOrEqual<T = Self> {
    fn is_less_than_or_equal(&self, w: T, bitwidth: i32, desc: &Option<String>) -> WireType;
}
pub trait IsLessThan<T = Self> {
    fn is_less_than(&self, w: T, bitwidth: i32, desc: &Option<String>) -> WireType;
}
pub trait IsGreaterThanOrEqual<T = Self> {
    fn is_greater_than_or_equal(&self, w: T, bitwidth: i32, desc: &Option<String>) -> WireType;
}
pub trait IsGreaterThan<T = Self> {
    fn is_greater_than(&self, w: T, bitwidth: i32, desc: &Option<String>) -> WireType;
}
