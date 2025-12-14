pub mod arithmetic_programs;
pub mod circuit_satisfaction_problems;
pub mod constraint_satisfaction_problems;
pub mod ram_computations;
pub mod variable;
use ffec::One;

pub trait FieldTConfig:
    One
    + std::fmt::Display
    + std::ops::Neg<Output = Self>
    + From<i64>
    + From<i32>
    + From<u32>
    + From<usize>
    + ffec::Zero
    + std::cmp::PartialEq
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + ffec::scalar_multiplication::multiexp::AsBigint
    + ffec::scalar_multiplication::wnaf::Config
    + std::ops::Mul<Output = Self>
    + std::ops::Mul<i32, Output = Self>
    + ffec::Zero
    + Clone
    + Default
    + std::ops::BitXor<usize, Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Sub<i32, Output = Self>
    + std::ops::Add<Output = Self>
    + std::ops::Add<i32, Output = Self>
{
    fn random_element() -> Self;
    fn squared(&self) -> Self;
    fn print(&self);
    fn inverse(&self) -> Self;
    fn multiplicative_generator() -> Self;
    fn from_int(i: u64, signed: bool) -> Self;
    fn capacity() -> usize;
    fn as_ulong(&self) -> usize;
    fn size_in_bits() -> usize;
}
