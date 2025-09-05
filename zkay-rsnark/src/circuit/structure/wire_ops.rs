#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::structure::wire_type::WireType;

pub trait MulWire<T = Self> {
    fn mul_wire(&self, b: T, desc: &Option<String>) -> WireType;
}
pub trait AddWire<T = Self> {
    fn add_wire(&self, w: T, desc: &Option<String>) -> WireType;
}
pub trait SubWire<T = Self> {
    fn sub_wire(&self, w: T, desc: &Option<String>) -> WireType;
}

// pub trait Neg<T=Self>{
//     fn neg(&self,  desc: &Option<String>) -> WireType;
// }

// pub trait Or<T=Self>{
//      fn or(&self, w: T, desc: &Option<String>) -> WireType;
// }

// pub trait Xor<T=Self>{
//      fn xor(&self, w: T, desc: &Option<String>) -> WireType;
// }

// pub trait And<T=Self>{
//      fn and(&self, w: T, desc: &Option<String>) -> WireType;
// }

pub trait XorBitwise<T = Self> {
    fn xor_bitwise(&self, w: T, numBits: u64, desc: &Option<String>) -> WireType;
}

pub trait AndBitwise<T = Self> {
    fn and_bitwise(&self, w: T, numBits: u64, desc: &Option<String>) -> WireType;
}

pub trait OrBitwise<T = Self> {
    fn or_bitwise(&self, w: T, numBits: u64, desc: &Option<String>) -> WireType;
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
