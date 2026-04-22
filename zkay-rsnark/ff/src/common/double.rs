//  Declaration of complex domain data type.

use crate::{FieldTConfig, One, PpConfig, Zero, algebra::field_utils::bigint::bigint};
use num_complex::{Complex, Complex64, ComplexFloat};
use std::{
    borrow::Borrow,
    cmp::Ordering,
    ops::{Add, AddAssign, BitXor, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Debug)]
pub struct Double {
    val: Complex64,
    v: Option<Vec<u64>>,
}
impl FieldTConfig for Double {}
impl PpConfig for Double {
    type BigIntT = bigint<1>;
}
impl Eq for Double {}
impl AsMut<[u64]> for Double {
    #[inline]
    fn as_mut(&mut self) -> &mut [u64] {
        self.v.as_mut().unwrap()
    }
}

impl Default for Double {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}
impl From<i32> for Double {
    fn from(real: i32) -> Self {
        Self::new(real as f64, 0.0)
    }
}
impl From<u32> for Double {
    fn from(real: u32) -> Self {
        Self::new(real as f64, 0.0)
    }
}
impl From<usize> for Double {
    fn from(real: usize) -> Self {
        Self::new(real as f64, 0.0)
    }
}
impl From<i64> for Double {
    fn from(real: i64) -> Self {
        Self::new(real as f64, 0.0)
    }
}
impl From<f64> for Double {
    fn from(real: f64) -> Self {
        Self::new(real, 0.0)
    }
}
impl From<Complex64> for Double {
    fn from(val: Complex64) -> Self {
        Self { val, v: None }
    }
}

impl Double {
    pub fn new(real: f64, imag: f64) -> Self {
        Self {
            val: Complex::<f64>::new(real, imag),
            v: None,
        }
    }

    pub fn inverse(&self) -> Self {
        Self::from(Complex::<f64>::new(1.0, 0.0) / self.val.clone())
    }

    pub fn as_bigint(&self) -> bigint<1> {
        bigint::<1>::new(self.val.re() as u64)
    }

    pub fn as_ulong(&self) -> u64 {
        self.val.re().round() as u64
    }

    pub fn squared(&self) -> Self {
        Self::from(self.val.clone() * self.val.clone())
    }

    pub fn one() -> Self {
        Self::from(1.0)
    }

    pub fn zero() -> Self {
        Self::from(0.0)
    }

    pub fn random_element() -> Self {
        // use rand::Rng;
        // let mut rng = rand::thread_rng();
        Self::from((rand::random::<i64>() % 1001) as f64)
    }

    pub fn geometric_generator() -> Self {
        Self::from(2.0)
    }

    pub fn arithmetic_generator() -> Self {
        Self::from(1.0)
    }

    pub fn multiplicative_generator() -> Self {
        Self::from(2.0)
    }
}

use std::fmt;
impl fmt::Display for Double {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}

impl One for Double {
    fn one() -> Self {
        Self::one()
    }
}

impl Zero for Double {
    fn zero() -> Self {
        Self::zero()
    }
    fn is_zero(&self) -> bool {
        false
    }
}

impl<O: Borrow<Self>> Add<O> for Double {
    type Output = Self;

    fn add(self, other: O) -> Self::Output {
        let mut r = self;
        r += other.borrow();
        r
    }
}
impl Add<i32> for Double {
    type Output = Self;

    fn add(self, other: i32) -> Self::Output {
        let mut r = self;
        // r += other.borrow();
        r
    }
}
impl Sub for Double {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut r = self;
        r -= other;
        r
    }
}
impl Sub<i32> for Double {
    type Output = Self;

    fn sub(self, other: i32) -> Self::Output {
        let mut r = self;
        // r -= other;
        r
    }
}

impl<O: Borrow<Self>> Mul<O> for Double {
    type Output = Double;

    fn mul(self, rhs: O) -> Self::Output {
        let mut r = self;
        r *= rhs.borrow();
        r
    }
}
impl Mul<bigint<1>> for Double {
    type Output = Self;

    fn mul(self, rhs: bigint<1>) -> Self::Output {
        let mut r = self;
        // r *= rhs.borrow();
        r
    }
}
impl Mul<i32> for Double {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        let mut r = self;
        // r *= rhs.borrow();
        r
    }
}
impl Neg for Double {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}
impl<O: Borrow<Self>> AddAssign<O> for Double {
    fn add_assign(&mut self, other: O) {}
}

impl<O: Borrow<Self>> SubAssign<O> for Double {
    fn sub_assign(&mut self, other: O) {}
}

impl<O: Borrow<Self>> MulAssign<O> for Double {
    fn mul_assign(&mut self, rhs: O) {}
}

impl PartialEq for Double {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl Ord for Double {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        // self.into_bigint().cmp(&other.into_bigint())
        1.cmp(&1)
    }
}

impl PartialOrd for Double {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> BitXor<&bigint<N>> for Double {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: &bigint<N>) -> Self::Output {
        let mut r = self;
        // r ^= rhs;
        r
    }
}
impl BitXor<usize> for Double {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: usize) -> Self::Output {
        let mut r = self;
        // r ^= rhs;
        r
    }
}
