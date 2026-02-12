
//  Declaration of complex domain data type.

use crate::{One,Zero};
use std::ops::{Add,Sub,Mul,Neg,BitXor,AddAssign,SubAssign,MulAssign};
use std::borrow::Borrow;
 use std::cmp::Ordering;
use crate::{FieldTConfig,PpConfig};
use num_complex::{Complex, ComplexFloat,Complex64};
// #include <complex>
// #include <libff/algebra/fields/bigint.hpp>


#[derive(Clone,Debug)]
pub struct Double {
    val: Complex64,
    v:Option<Vec<u64>>,
}
impl FieldTConfig for Double{
}
impl PpConfig for Double{
    type TT=bigint<1>;
}
impl Eq for Double{}
impl AsMut<[u64]> for Double{
    #[inline]
    fn as_mut(&mut self) -> &mut [u64] {
        self.v.as_mut().unwrap()
    }
}
//       Double();

//       Double(f64 real);

//       Double(f64 real, f64 imag);

//       Double(Complex64 num);

//       static unsigned add_cnt;
//       static unsigned sub_cnt;
//       static unsigned mul_cnt;
//       static unsigned inv_cnt;

//       Double operator+(other:&Double) const;
//       Double operator-(other:&Double) const;
//       Double operator*(other:&Double) const;
//       Double operator-() const;

//       Double& operator+=(other:&Double);
//       Double& operator-=(other:&Double);
//       Double& operator*=(other:&Double);

//       bool operator==(other:&Double) const;
//       bool operator!=(other:&Double) const;

//       bool operator<(other:&Double) const;
//       bool operator>(other:&Double) const;

//       Double operator^(const:bigint<1> power),
//       Double operator^(power:usize) const;

//       bigint<1> as_bigint() const;
//       u64 as_ulong() const;
//       Double inverse() const;
//       Double squared() const;

//       static Double one();
//       static Double zero();
//       static Double random_element();
//       static Double geometric_generator();
//       static Double arithmetic_generator();

//       static Double multiplicative_generator;
//       static Double root_of_unity; // See get_root_of_unity() in field_utils
//       static usize s;
//   };

//#include <cmath>
//#include <complex>

//#include <math.h>
use crate::algebra::field_utils::bigint::bigint;
// use crate::common::f64;




// using std::usize;
impl Default for Double {
     fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}
impl From<i32> for Double{
     fn from(real: i32) -> Self {
        Self::new(real as f64, 0.0)
    }
}
impl From<u32> for Double{
     fn from(real: u32) -> Self {
        Self::new(real as f64, 0.0)
    }
}
impl From<usize> for Double{
     fn from(real: usize) -> Self {
        Self::new(real as f64, 0.0)
    }
}
impl From<i64> for Double{
     fn from(real: i64) -> Self {
        Self::new(real as f64, 0.0)
    }
}
impl From<f64> for Double{
     fn from(real: f64) -> Self {
        Self::new(real, 0.0)
    }
}
impl From<Complex64> for Double{
     fn from(val:Complex64) -> Self {
        Self {
            val,v:None,
        }
    }
}

impl Double {
    pub fn new(real: f64, imag: f64) -> Self {
        Self {
            val: Complex::<f64>::new(real, imag),v:None,
        }
    }


    // unsigned pub fn add_cnt = 0;
    // unsigned pub fn sub_cnt = 0;
    // unsigned pub fn mul_cnt = 0;
    // unsigned pub fn inv_cnt = 0;

    pub fn inverse(&self) -> Self {
        // #ifdef PROFILE_OP_COUNTS
        // ++inv_cnt;
        

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

    pub fn multiplicative_generator()->Self{
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

// pub fn operator+(other:&Double) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     ++add_cnt;
// 

//     return Self::new(val + other.val);
// }
impl< O: Borrow<Self>> Add<O> for Double {
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
// pub fn operator-(other:&Double) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     ++sub_cnt;
// 

//     return Self::new(val - other.val);
// }
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
// pub fn operator*(other:&Double) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     ++mul_cnt;
// 

//     return Self::new(val * other.val);
// }
impl< O: Borrow<Self>> Mul<O> for Double {
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
// pub fn operator-() const
// {
//     if val.imag() == 0 {
//         return Self::new(-val.real());
//     }

//     return Self::new(-val.real(), -val.imag());
// }
impl Neg for Double {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}
// Double& pub fn operator+=(other:&Double)
// {
// // #ifdef PROFILE_OP_COUNTS
//     ++add_cnt;
// 

//     this->val = Complex::<f64>::new(val + other.val);
//     return *this;
// }
impl<O: Borrow<Self>> AddAssign<O>  for Double {
    fn add_assign(&mut self, other: O) {
    }
}
// Double& pub fn operator-=(other:&Double)
// {
// // #ifdef PROFILE_OP_COUNTS
//     ++sub_cnt;
// 

//     this->val = Complex::<f64>::new(val - other.val);
//     return *this;
// }
impl<O: Borrow<Self>> SubAssign<O> for Double {
    fn sub_assign(&mut self, other: O) {}
}

// Double& pub fn operator*=(other:&Double)
// {
// // #ifdef PROFILE_OP_COUNTS
//     ++mul_cnt;
// 

//     this->val *= Complex::<f64>::new(other.val);
//     return *this;
// }
impl< O: Borrow<Self>> MulAssign<O> for Double {
    fn mul_assign(&mut self, rhs: O) {
       
    }
}

// bool pub fn operator==(other:&Double) const
// {
//     return (std::abs(val.real() - other.val.real()) < 0.000001)
//         && (std::abs(val.imag() - other.val.imag()) < 0.000001);
// }
impl PartialEq for Double{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

// bool pub fn operator!=(other:&Double) const
// {
//     return !(*this == other);
// }

// bool pub fn operator<(other:&Double) const
// {
//     return (val.real() < other.val.real());
// }
impl Ord for Double{
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        // self.into_bigint().cmp(&other.into_bigint())
        1.cmp(&1)
    }
}
// bool pub fn operator>(other:&Double) const
// {
//     return (val.real() > other.val.real());
// }
impl PartialOrd for Double{
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
// pub fn operator^(const bigint<1> power) const
// {
//     return Self::new(pow(val, power.as_ulong()));
// }
impl<const N: usize> BitXor<&bigint<N>> for Double {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: &bigint<N>) -> Self::Output {
        let mut r = self;
        // r ^= rhs;
        r
    }
}
// pub fn operator^(power:usize) const
// {
//     return Self::new(pow(val, power));
// }
impl BitXor<usize> for Double {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: usize) -> Self::Output {
        let mut r = self;
        // r ^= rhs;
        r
    }
}