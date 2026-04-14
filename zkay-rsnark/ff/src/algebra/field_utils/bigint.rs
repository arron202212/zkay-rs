#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

// Declaration of bigint wrapper pub struct around GMP's MPZ long integers.

// Notice that this pub struct has no arithmetic operators. This is deliberate. All
// bigints should either be hardcoded or operated on the bit level to ensure
// high performance.

use super::BigInt;
use crate::algebra::{field_utils::BigInteger, fields::fpn_field::PrimeField};
use ark_std::{
    UniformRand,
    Zero,
    borrow::Borrow,
    // convert::TryFrom,
    fmt::{Debug, Display, UpperHex},

    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
        ShrAssign,
    },
    rand::{
        Rng,
        distributions::{Distribution, Standard},
    },
    str::FromStr,
    vec::*,
};
use num_bigint::BigUint;
use num_integer::{ExtendedGcd, Integer};
use std::io::{self, Read, Write};
use std::ops::{Index, IndexMut, MulAssign, Rem, Sub};
use zeroize::Zeroize;

// // /**
// //  * Wrapper pub struct around GMP's MPZ long integers. It supports arithmetic operations,
// //  * serialization and randomization. Serialization is fragile, see common/serialization.hpp.
// //  */
pub const GMP_NUMB_BITS: usize = 64;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Zeroize)]
pub struct bigint<const N: usize>(pub BigInt<N>);

impl<const N: usize> std::iter::ExactSizeIterator for bigint<N> {
    fn len(&self) -> usize {
        self.0.0.len()
    }
}

impl<const N: usize> Iterator for bigint<N> {
    // we will be counting with usize
    type Item = u64;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        // Increment our count. This is why we started at zero.
        // self.count += 1;

        // Check to see if we've finished counting or not.
        // if self.count < 6 {
        //     Some(0)
        // } else {
        None
        // }
    }
}

impl<const N: usize> From<u128> for bigint<N> {
    fn from(rhs: u128) -> Self {
        Self(BigInt::from(rhs as u64))
    }
}

impl<const N: usize> From<bigint<N>> for BigUint {
    #[inline]
    fn from(val: bigint<N>) -> num_bigint::BigUint {
        (BigUint::from_bytes_le(&val.0.to_bytes_le()))
    }
}

impl<const N: usize> From<bigint<N>> for num_bigint::BigInt {
    #[inline]
    fn from(val: bigint<N>) -> num_bigint::BigInt {
        use num_bigint::Sign;
        let sign = if val.0.is_zero() {
            Sign::NoSign
        } else {
            Sign::Plus
        };
        num_bigint::BigInt::from_bytes_le(sign, &val.0.to_bytes_le())
    }
}

impl<const N: usize> Default for bigint<N> {
    fn default() -> Self {
        Self(BigInt::default())
    }
}
impl<const N: usize> AsRef<[u64]> for bigint<N> {
    fn as_ref(&self) -> &[u64] {
        &self.0.0
    }
}
#[macro_export]
macro_rules! BigInte {
    ($c0:expr) => {{ bigint($crate::BigInt!(c0)) }};
}
const SS: bigint<100> = bigint::<100>(BigInt!("1"));
// // using usize;
use std::mem;
impl<const N: usize> bigint<N> {
    #[inline]
    pub const fn max_bits(&self) -> usize {
        return N * GMP_NUMB_BITS;
    }
    /// Initalize from a small integer
    pub fn new(x: u64) -> Self {
        Self(BigInt::<N>::from(x))
    }
    /// Initialize from a string containing an integer in decimal notation
    pub fn new_with_str(s: &str) -> eyre::Result<Self> {
        BigInt::<N>::from_str(s)
            .map(|v| Self(v))
            .map_err(|v| eyre::eyre!(format!("{v:?}")))
    }
    // pub const fn const_str(s:&str)->Self{
    //     Self(BigInt!(s))
    // }
    pub const fn one() -> Self {
        Self(BigInt::<N>::one())
    }
    pub const fn zero() -> Self {
        Self(BigInt::<N>::zero())
    }
    pub fn print(&self) {
        print!("{:N$?}\n", self.0);
    }

    pub fn print_hex(&self) {
        print!("{:N$x?}\n", self.0);
    }

    pub const fn clear(&mut self) {
        self.0.0 = [0; N];
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
    pub fn is_one(&self) -> bool {
        self == &Self::one()
    }
    pub fn is_negative(&self) -> bool {
        false
    }
    pub fn is_even(&self) -> bool {
        self.0.is_even()
    }

    pub fn num_bits(&self) -> usize {
        self.0.num_bits() as _
    }

    pub fn as_ulong(&self) -> u64 {
        self.0.0[0]
    }

    pub fn test_bit(&self, bitno: usize) -> bool {
        self.0.get_bit(bitno)
    }

    pub fn randomize(&mut self) -> &Self {
        let mut rng = ark_std::test_rng();

        self.0 = BigInt::<N>::rand(&mut rng);
        self
    }
    pub fn abs(self) -> Self {
        self
    }
    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let mut repr = [0u64; N];
        for limb in repr.iter_mut() {
            let mut buf = [0u8; 8];
            reader.read_exact(&mut buf)?;
            *limb = u64::from_le_bytes(buf); // 假设电路文件是小端序
        }

        Ok(Self(BigInt::<N>(repr)))
    }
    #[inline]
    pub fn extended_gcd(&self, b: &Self) -> (Self, Self, Self) {
        let (a, b): (BigUint, BigUint) = (self.0.clone().into(), b.0.clone().into());
        let ExtendedGcd::<BigUint> { gcd, x, y } = a.extended_gcd(&b);
        if gcd.is_zero() {
            return (
                Self(a.clone().try_into().unwrap()),
                Self::one(),
                Self::zero(),
            );
        }
        (
            Self(gcd.try_into().unwrap()),
            Self(x.try_into().unwrap()),
            Self(y.try_into().unwrap()),
        )
    }
}

impl<const N: usize> Index<usize> for bigint<N> {
    type Output = u64;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.0.0.get(index).unwrap()
    }
}

impl<const N: usize> IndexMut<usize> for bigint<N> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.0.get_mut(index).unwrap()
    }
}
impl<const N: usize> Rem for bigint<N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        self
    }
}
impl<const N: usize> Sub for bigint<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self
    }
}
impl<const N: usize> MulAssign<&Self> for bigint<N> {
    fn mul_assign(&mut self, rhs: &Self) {}
}
impl<const N: usize> MulAssign for bigint<N> {
    fn mul_assign(&mut self, rhs: Self) {}
}
// use std::ops::Mul;
// impl<const N:usize> Mul for bigint<N> {
//     type Output =Self;

//     fn mul(self, rhs: Self) -> Self {
//         Self::new(0)
//     }
// }
// impl<T, const N:usize> Mul<&T> for bigint<N> {
//     type Output = bigint<N>;

//     fn mul(self, rhs: &T) -> bigint<N> {
//         bigint::<N>::new(0)
//     }
// }
// // impl<T,const N:usize> Mul<&T> for bigint<N> {
// //     type Output =Self;

// //     fn mul(self, rhs: &T) -> Self {
// //         Self::new(0)
// //     }
// // }
// use std::cmp::Ordering;

// impl<const N:usize> PartialEq for bigint<N> {
//      #[inline]
//     fn eq(&self, other: &Self) -> bool {
//        self.0.0[.. N]== other.0.0[.. N]
//     }
// }
// impl<const N:usize> PartialOrd for bigint<N> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.0.0.cmp(&other.0.0))
//     }
// }

// //
// // bool bigint<n>::operator==(const bigint<n>& other) const
// // {
// //     return (mpn_cmp(self.0.0, other.0.0, n) == 0);
// // }

// //
// // bool bigint<n>::operator!=(const bigint<n>& other) const
// // {
// //     return !(operator==(other));
// // }

// //
// // bool bigint<n>::operator<(const bigint<n>& other) const
// // {
// //     return (mpn_cmp(self.0.0, other.0.0, n) < 0);
// // }

use std::fmt;
impl<const N: usize> fmt::Display for bigint<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // cfg_if::cfg_if! {
        // if #[cfg(feature="binary_output")]
        //     {
        //         write!(f, "{}",  self.0.0 )
        //     }
        //     else{
        //     let mut t=0;
        //     self.to_mpz(&mut t);
        //     write!(f, "{}",  t )
        //     }
        // }
        write!(f, "{}", self.0)
    }
}

//
// std::ostream& operator<<(std::ostream &out, b:&bigint<n>)
// {
// // #ifdef BINARY_OUTPUT
//     out.write((char*)b.0.0, sizeof(b.0.0[0]) * n);
// #else
//     mpz_t t;
//     mpz_init(t);
//     b.to_mpz(t);

//     out << t;

//     mpz_clear(t);
//
//     return out;
// }

//
// std::istream& operator>>(std::istream &in, bigint<n> &b)
// {
// // #ifdef BINARY_OUTPUT
//     in.read((char*)b.0.0, sizeof(b.0.0[0]) * n);
// #else
//     String s;
//     in >> s;

//     usize l = s.len();
//     unsigned char* s_copy = new unsigned char[l];

//     for i in 0..l
//     {
//         assert!(s[i] >= '0' && s[i] <= '9');
//         s_copy[i] = s[i] - '0';
//     }

//     mp_size_t limbs_written = mpn_set_str(b.0.0, s_copy, l, 10);
//     assert!(limbs_written <= n);
//     if limbs_written < n
//     {
//       memset(b.0.0 + limbs_written, 0, (n - limbs_written) * sizeof(mp_limb_t));
//     }

//     delete[] s_copy;
//
//     return in;
// }
