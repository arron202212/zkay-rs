#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::structure::wire_type::WireType;

pub trait MulWire<T = Self> {
    #[inline]
    fn mul_wire(&self, b: T) -> WireType {
        self.mul_wire_with_option(b, &None)
    }
    #[inline]
    fn mul_wire_with_str(&self, b: T, desc: &str) -> WireType {
        self.mul_wire_with_option(b, &Some(desc.to_owned()))
    }
    fn mul_wire_with_option(&self, b: T, desc: &Option<String>) -> WireType;
}
pub trait AddWire<T = Self> {
    #[inline]
    fn add_wire(&self, w: T) -> WireType {
        self.add_wire_with_option(w, &None)
    }
    #[inline]
    fn add_wire_with_str(&self, w: T, desc: &str) -> WireType {
        self.add_wire_with_option(w, &Some(desc.to_owned()))
    }
    fn add_wire_with_option(&self, w: T, desc: &Option<String>) -> WireType;
}
pub trait SubWire<T = Self> {
    #[inline]
    fn sub_wire(&self, w: T) -> WireType {
        self.sub_wire_with_option(w, &None)
    }
    #[inline]
    fn sub_wire_with_str(&self, w: T, desc: &str) -> WireType {
        self.sub_wire_with_option(w, &Some(desc.to_owned()))
    }
    fn sub_wire_with_option(&self, w: T, desc: &Option<String>) -> WireType;
}

pub trait XorBitwise<T = Self> {
    #[inline]
    fn xor_bitwise(&self, w: T, num_bits: u64) -> WireType {
        self.xor_bitwise_with_option(w, num_bits, &None)
    }
    #[inline]
    fn xor_bitwise_with_str(&self, w: T, num_bits: u64, desc: &str) -> WireType {
        self.xor_bitwise_with_option(w, num_bits, &Some(desc.to_owned()))
    }
    fn xor_bitwise_with_option(&self, w: T, num_bits: u64, desc: &Option<String>) -> WireType;
}

pub trait AndBitwise<T = Self> {
    #[inline]
    fn and_bitwise(&self, w: T, num_bits: u64) -> WireType {
        self.and_bitwise_with_option(w, num_bits, &None)
    }
    #[inline]
    fn and_bitwise_with_str(&self, w: T, num_bits: u64, desc: &str) -> WireType {
        self.and_bitwise_with_option(w, num_bits, &Some(desc.to_owned()))
    }
    fn and_bitwise_with_option(&self, w: T, num_bits: u64, desc: &Option<String>) -> WireType;
}

pub trait OrBitwise<T = Self> {
    #[inline]
    fn or_bitwise(&self, w: T, num_bits: u64) -> WireType {
        self.or_bitwise_with_option(w, num_bits, &None)
    }
    #[inline]
    fn or_bitwise_with_str(&self, w: T, num_bits: u64, desc: &str) -> WireType {
        self.or_bitwise_with_option(w, num_bits, &Some(desc.to_owned()))
    }
    fn or_bitwise_with_option(&self, w: T, num_bits: u64, desc: &Option<String>) -> WireType;
}

pub trait IsEqualTo<T = Self> {
    #[inline]
    fn is_equal_to(&self, w: T) -> WireType {
        self.is_equal_to_with_option(w, &None)
    }
    #[inline]
    fn is_equal_to_with_str(&self, w: T, desc: &str) -> WireType {
        self.is_equal_to_with_option(w, &Some(desc.to_owned()))
    }
    fn is_equal_to_with_option(&self, w: T, desc: &Option<String>) -> WireType;
}

pub trait IsLessThanOrEqual<T = Self> {
    #[inline]
    fn is_less_than_or_equal(&self, w: T, bitwidth: i32) -> WireType {
        self.is_less_than_or_equal_with_option(w, bitwidth, &None)
    }
    #[inline]
    fn is_less_than_or_equal_with_str(&self, w: T, bitwidth: i32, desc: &str) -> WireType {
        self.is_less_than_or_equal_with_option(w, bitwidth, &Some(desc.to_owned()))
    }
    fn is_less_than_or_equal_with_option(
        &self,
        w: T,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType;
}
pub trait IsLessThan<T = Self> {
    #[inline]
    fn is_less_than(&self, w: T, bitwidth: i32) -> WireType {
        self.is_less_than_with_option(w, bitwidth, &None)
    }
    #[inline]
    fn is_less_than_with_str(&self, w: T, bitwidth: i32, desc: &str) -> WireType {
        self.is_less_than_with_option(w, bitwidth, &Some(desc.to_owned()))
    }
    fn is_less_than_with_option(&self, w: T, bitwidth: i32, desc: &Option<String>) -> WireType;
}
pub trait IsGreaterThanOrEqual<T = Self> {
    #[inline]
    fn is_greater_than_or_equal(&self, w: T, bitwidth: i32) -> WireType {
        self.is_greater_than_or_equal_with_option(w, bitwidth, &None)
    }
    #[inline]
    fn is_greater_than_or_equal_with_str(&self, w: T, bitwidth: i32, desc: &str) -> WireType {
        self.is_greater_than_or_equal_with_option(w, bitwidth, &Some(desc.to_owned()))
    }
    fn is_greater_than_or_equal_with_option(
        &self,
        w: T,
        bitwidth: i32,
        desc: &Option<String>,
    ) -> WireType;
}
pub trait IsGreaterThan<T = Self> {
    #[inline]
    fn is_greater_than(&self, w: T, bitwidth: i32) -> WireType {
        self.is_greater_than_with_option(w, bitwidth, &None)
    }
    #[inline]
    fn is_greater_than_with_str(&self, w: T, bitwidth: i32, desc: &str) -> WireType {
        self.is_greater_than_with_option(w, bitwidth, &Some(desc.to_owned()))
    }
    fn is_greater_than_with_option(&self, w: T, bitwidth: i32, desc: &Option<String>) -> WireType;
}
