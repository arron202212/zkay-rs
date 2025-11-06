#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
/** @file
 *****************************************************************************
 Declaration of bigint wrapper pub struct around GMP's MPZ long integers.

 Notice that this pub struct has no arithmetic operators. This is deliberate. All
 bigints should either be hardcoded or operated on the bit level to ensure
 high performance.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
use ark_std::{
    borrow::Borrow,
    // convert::TryFrom,
    fmt::{Debug, Display, UpperHex},
    io::{Read, Write},
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
        ShrAssign,
    },
    rand::{
        distributions::{Distribution, Standard},
        Rng,
    },
    str::FromStr,
    vec::*,
    Zero,
    UniformRand
};
use num_bigint::BigUint;
use zeroize::Zeroize;

 use ark_ff::{BigInteger,BigInt, PrimeField};
// //#ifndef BIGINT_HPP_
// // #define BIGINT_HPP_
// //#include <cstddef>
// //#include <iostream>

// //#include <gmp.h>

// use crate::common::serialization;

// // // namespace libff {


// // /**
// //  * Wrapper pub struct around GMP's MPZ long integers. It supports arithmetic operations,
// //  * serialization and randomization. Serialization is fragile, see common/serialization.hpp.
// //  */
pub const GMP_NUMB_BITS:usize =64;
// // 

#[derive(Copy, Clone, PartialEq, Eq, Hash, Zeroize)]
pub struct bigint<const N: usize>(pub BigInt<N>);
// // impl<const N:usize> bigint<N>{
//     // n: N:usize =,

    

//     // bigint() = default;
//     // bigint(const u64 x); /// Initalize from a small integer
//     // bigint(const char* s); /// Initialize from a string containing an integer in decimal notation
//     // bigint(const mpz_t r); /// Initialize from MPZ element

//     // static bigint one();

//     // pub fn  print() const;
//     // pub fn  print_hex() const;
//     // bool operator==(const:bigint<n>& other),
//     // bool operator!=(const:bigint<n>& other),
//     // bool operator<(const:bigint<n>& other),
//     // pub fn  clear();
//     // bool is_zero() const;
//     // bool is_even() const;
//     // usize max_bits() GMP_NUMB_BITS:{ return n *, } /// Returns the number of bits representable by this bigint type
//     // usize num_bits() const; /// Returns the number of bits in this specific bigint value, i.e., position of the most-significant 1

//     // u64 as_ulong() const; /// Return the last limb of the integer
//     // pub fn  to_mpz(mpz_t r) const;
//     // bool test_bit(bitno:usize) const;

//     // bigint& randomize();

//     // friend std::ostream& operator<< <n>(std::ostream &out, b:&bigint<n>);
//     // friend std::istream& operator>> <n>(std::istream &in, bigint<n> &b);
// // }

// // // } // namespace libff
// // use ffec::algebra::field_utils::/bigint.tcc;
// // //#endif


// /** @file
//  *****************************************************************************
//  Implementation of bigint wrapper pub struct around GMP's MPZ long integers.
//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// // //#ifndef BIGINT_TCC_
// // // #define BIGINT_TCC_
// // //#include <cassert>
// // //#include <cstring>
// // //#include <random>

// // // namespace libff {

impl<const N: usize> Default for bigint<N> {
    fn default() -> Self {
        Self(BigInt::default())
    }
}

// // using usize;
use std::mem;
impl<const N:usize> bigint<N>{
#[inline]
 pub const fn max_bits(&self)->usize { return N * GMP_NUMB_BITS; }
 /// Initalize from a small integer
pub fn new(x:u64)->Self
{
   Self(BigInt::<N>::from(x))
}
/// Initialize from a string containing an integer in decimal notation
pub fn new_with_str(s:&str) ->eyre::Result<Self>
{
   BigInt::<N>::from_str(s).map(|v|Self(v)).map_err(|v|eyre::eyre!(format!("{v:?}")))
}

pub fn one()->Self
{
    Self(BigInt::<N>::one())
}

pub fn print(&self) 
{
    print!("{:N$?}\n", self.0);
}

pub fn print_hex(&self) 
{
    print!("{:N$x?}\n", self.0);
}

pub fn clear(&mut self)
{
    self.0.0.zeroize();
}
  
pub fn is_zero(&self) ->bool
{
    self.0.is_zero()
}

pub fn is_even(&self) ->bool
{
    self.0.is_even()
}
 
pub fn num_bits(&self) ->usize
{
    self.0.num_bits() as _
}

pub fn as_ulong(&self) ->u64
{
    self.0.0[0]
}


pub fn  test_bit(&self,bitno:usize) ->bool
{
    self.0.get_bit(bitno)
}

pub fn randomize(&mut self)->&Self
{
    let mut rng = ark_std::test_rng();

    self.0 = BigInt::<N>::rand(&mut rng);
    self
}

}


// // // } // namespace libff
// //#endif // BIGINT_TCC_
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
impl<const N:usize> fmt::Display for bigint<N> {
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
        write!(f, "{}",  self.0 )
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
// //#endif
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
// //#endif
//     return in;
// }
