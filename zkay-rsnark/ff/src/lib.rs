#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
#![feature(adt_const_params)]
#![feature(generic_const_items)]
#![feature(generic_const_exprs)]
#![feature(associated_type_defaults)]
#![allow(incomplete_features)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_index)]
#[macro_use]
pub mod algebra;
pub use self::algebra::*;
pub mod common;
pub use self::common::utils::*;
const D: &'static [u64] = &[0];
use crate::field_utils::bigint::{BigIntegerT, bigint};

pub trait PpConfig:
    Default
    + std::fmt::Debug
    + std::fmt::Display
    + std::ops::Mul
    + std::cmp::PartialEq
    + Clone
    + One
    + Zero
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Mul<Self, Output = Self>
{
    type BigIntT: BigIntegerT;
    const num_limbs: usize = 254;
    const coeff_a: i64 = 1;
    const coeff_b: i64 = 1;

    fn as_bigint(&self) -> Self::BigIntT {
        panic!("unimplemented");
        Self::BigIntT::default()
    }
    fn dbl(&self) -> Self {
        panic!("unimplemented");
        self.clone()
    }
    fn random_element() -> Self {
        panic!("unimplemented");
        Default::default()
    }
    fn wnaf_window_table() -> Vec<usize> {
        panic!("unimplemented");
        vec![]
    }
    fn fixed_base_exp_window_table() -> std::vec::Vec<usize> {
        panic!("unimplemented");
        vec![]
    }
    fn batch_to_special_all_non_zeros(_: &mut std::vec::Vec<Self>) {
        panic!("unimplemented");
    }
    fn to_special(&mut self) {
        panic!("unimplemented");
    }
    fn mixed_add(&self, other: &Self) -> Self {
        panic!("unimplemented");
        Default::default()
    }
    fn unitary_inverse(&self) -> Self {
        panic!("unimplemented");
        Default::default()
    }
    fn is_special(&self) -> bool {
        panic!("unimplemented");
        false
    }
    fn print(&self) {
        panic!("unimplemented");
    }
    fn size_in_bits() -> usize {
        panic!("unimplemented");
        0
    }
    fn is_well_formed(&self) -> bool {
        panic!("unimplemented");
        false
    }
    fn num_bits() -> usize {
        panic!("unimplemented");
        1
    }
    fn to_affine_coordinates(&mut self) {
        panic!("unimplemented");
    }
}

pub trait FieldTConfig:
    std::ops::Neg<Output = Self>
    + From<i64>
    + From<i32>
    + From<u32>
    + From<usize>
    + std::cmp::PartialEq
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::ops::Mul<Output = Self>
    + std::ops::Mul<i32, Output = Self>
    + std::ops::BitXor<usize, Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Sub<i32, Output = Self>
    + std::ops::Add<Output = Self>
    + std::ops::Add<i32, Output = Self>
    + PpConfig
    + AsMut<[u64]>
{
    fn as_ref_u64(&self) -> Vec<u64> {
        panic!("unimplemented");
        vec![]
    }
    fn test_bit(&self, i: usize) -> bool {
        panic!("unimplemented");
        false
    }
    fn to_field<FieldTT: Default>(&self) -> FieldTT {
        panic!("unimplemented");
        FieldTT::default()
    }
    fn squared(&self) -> Self {
        panic!("unimplemented");
        Default::default()
    }
    fn inverse(&self) -> Self {
        panic!("unimplemented");
        Default::default()
    }
    fn multiplicative_generator() -> Self {
        panic!("unimplemented");
        Default::default()
    }
    fn from_int(i: u64, signed: bool) -> Self {
        panic!("unimplemented");
        Default::default()
    }
    fn capacity() -> usize {
        panic!("unimplemented");
        0
    }
    fn as_ulong(&self) -> usize {
        panic!("unimplemented");
        0
    }
    fn X(&self) -> Self {
        panic!("unimplemented");
        Default::default()
    }
    fn Y(&self) -> Self {
        panic!("unimplemented");
        Default::default()
    }
    fn is_double() -> bool {
        // panic!("unimplemented");
        false
    }
    fn ss() -> usize {
        panic!("unimplemented");
        28
    }
    fn arithmetic_generator() -> Self {
        panic!("unimplemented");
        Default::default()
    }
    fn geometric_generator() -> Self {
        panic!("unimplemented");
        Default::default()
    }
}
