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
#[macro_use]
pub mod algebra;
pub use self::algebra::*;
pub mod common;
pub use self::common::utils::*;
const D: &'static [u64] = &[0];
use crate::field_utils::bigint::bigint;
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
    + std::ops::Mul<Self::TT, Output = Self>
    + std::ops::Mul<Output = Self>
    + for<'a> std::ops::Mul<&'a Self, Output = Self>
{
    type TT: AsRef<[u64]>;

    const num_limbs: usize = 1;
    const coeff_a: i64 = 1;
    const coeff_b: i64 = 1;

    fn as_bigint<const N: usize>(&self) -> bigint<N> {
        bigint::<N>::default()
    }
    fn dbl(&self) -> Self {
        self.clone()
    }
    fn random_element() -> Self {
        Default::default()
    }
    fn wnaf_window_table() -> Vec<usize> {
        vec![]
    }
    fn fixed_base_exp_window_table() -> std::vec::Vec<usize> {
        vec![]
    }
    fn batch_to_special_all_non_zeros<T>(_: std::vec::Vec<T>) {}
    fn to_special(&self) {}
    // type T = bigint<1>;
    // fn zero() -> Self {
    //     alt_bn128_G2::default()
    // }
    fn mixed_add(&self, other: &Self) -> Self {
        Default::default()
    }
    fn unitary_inverse(&self) -> Self {
        Default::default()
    }
    fn is_special(&self) -> bool {
        false
    }
    fn print(&self) {}
    fn size_in_bits() -> usize {
        0
    }
    fn is_well_formed(&self) -> bool {
        false
    }
    fn num_bits() -> usize {
        1
    }
    fn to_affine_coordinates(&mut self) {}
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
    + PpConfig+ AsMut<[u64]>
{
    fn as_ref_u64(&self) -> Vec<u64> {
        vec![]
    }
    fn get_bit(&self, i: usize) -> bool {
        false
    }
    fn to_field<FieldTT: Default>(&self) -> FieldTT {
        FieldTT::default()
    }
    fn squared(&self) -> Self {
        Default::default()
    }
    fn inverse(&self) -> Self {
        Default::default()
    }
    fn multiplicative_generator() -> Self {
        Default::default()
    }
    fn from_int(i: u64, signed: bool) -> Self {
        Default::default()
    }
    fn capacity() -> usize {
        0
    }
    fn as_ulong(&self) -> usize {
        0
    }
    fn X(&self) -> Self {
        Default::default()
    }
    fn Y(&self) -> Self {
        Default::default()
    }
}
