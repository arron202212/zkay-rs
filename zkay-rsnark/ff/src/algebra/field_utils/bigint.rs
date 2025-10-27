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

// //#ifndef BIGINT_HPP_
// // #define BIGINT_HPP_
// //#include <cstddef>
// //#include <iostream>

// //#include <gmp.h>

use crate::common::serialization;

// // namespace libff {

//  pub struct bigint;
// &);
// &);

// /**
//  * Wrapper pub struct around GMP's MPZ long integers. It supports arithmetic operations,
//  * serialization and randomization. Serialization is fragile, see common/serialization.hpp.
//  */
pub const GMP_NUMB_BITS:usize =64;
// 
#[derive(Clone)]
pub struct bigint<const N:usize> {
    pub data:[u64;N],
}
// impl<const N:usize> bigint<N>{
    // n: N:usize =,

    

    // bigint() = default;
    // bigint(const u64 x); /// Initalize from a small integer
    // bigint(const char* s); /// Initialize from a string containing an integer in decimal notation
    // bigint(const mpz_t r); /// Initialize from MPZ element

    // static bigint one();

    // pub fn  print() const;
    // pub fn  print_hex() const;
    // bool operator==(const:bigint<n>& other),
    // bool operator!=(const:bigint<n>& other),
    // bool operator<(const:bigint<n>& other),
    // pub fn  clear();
    // bool is_zero() const;
    // bool is_even() const;
    // usize max_bits() GMP_NUMB_BITS:{ return n *, } /// Returns the number of bits representable by this bigint type
    // usize num_bits() const; /// Returns the number of bits in this specific bigint value, i.e., position of the most-significant 1

    // u64 as_ulong() const; /// Return the last limb of the integer
    // pub fn  to_mpz(mpz_t r) const;
    // bool test_bit(bitno:usize) const;

    // bigint& randomize();

    // friend std::ostream& operator<< <n>(std::ostream &out, b:&bigint<n>);
    // friend std::istream& operator>> <n>(std::istream &in, bigint<n> &b);
// }

// // } // namespace libff
// use ffec::algebra::field_utils::/bigint.tcc;
// //#endif


/** @file
 *****************************************************************************
 Implementation of bigint wrapper pub struct around GMP's MPZ long integers.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

// //#ifndef BIGINT_TCC_
// // #define BIGINT_TCC_
// //#include <cassert>
// //#include <cstring>
// //#include <random>

// // namespace libff {

// using usize;
use std::mem;
impl<const N:usize> bigint<N>{
#[inline]
 pub fn max_bits(&self)->usize { return N * GMP_NUMB_BITS; }
 /// Initalize from a small integer
pub fn new(x:u64)->Self
{
    assert!(8*mem::size_of_val(&x) <= GMP_NUMB_BITS);
    let mut data=[0;N];
    data[0] = x;
    Self{data}
}
/// Initialize from a string containing an integer in decimal notation
pub fn new_str(s:&str) ->Self
{
    let  l = s.len();
    let  mut s_copy = vec![0;s.len()];
    let bs=s.as_bytes();
    for i in 0..l
    {
        assert!(bs[i] >= b'0' && bs[i] <= b'9');
        s_copy[i] = (bs[i] - b'0') as u64;
    }
    let data:[u64;N]=s_copy.try_into().unwrap();
    // let limbs_written = //mpn_set_str(data, s_copy, l, 10);
// //#ifndef NDEBUG
//     assert!(limbs_written <= n);
// #else
//     UNUSED(limbs_written);
// //#endif

    // delete[] s_copy;
    Self{data}
}

// Initialize from MPZ element
pub fn new_u64(r:u64) ->Self
{//mpz_t
    let mut  k=0;
    // mpz_init_set(k, r);
    let mut data=[0;N];
    for i in 0..N
    {
        data[i] = k;//mpz_get_ui(k);
        // mpz_fdiv_q_2exp(k, k, GMP_NUMB_BITS);
    }

    // assert!(mpz_sgn(k) == 0);
    // mpz_clear(k);
    Self{data}
}


pub fn one()->Self
{
    let mut  one=Self::new(0);
    one.data[0] = 1;
    return one;
}

pub fn print(&self) 
{
    print!("{:N$?}\n", self.data);
}

pub fn print_hex(&self) 
{
    print!("{:N$x?}\n", self.data);
}

pub fn clear(&mut self)
{
    // mpn_zero(self.data, n);
    self.data[..N].fill(0);
}

pub fn is_zero(&self) ->bool
{
    for i in 0..N
    {
        if self.data[i]!=0
        {
            return false;
        }
    }

    return true;
}

pub fn is_even(&self) ->bool
{
    return (self.data[0] & 1) == 0;
}
 
pub fn num_bits(&self) ->usize
{
/*
    for i in ( 0..=max_bits()).rev()
    {
        if self.test_bit(i)
        {
            return i+1;
        }
    }

    return 0;
*/
    for i  in (0.. N).rev()
    {
        let  x = self.data[i];
        if x == 0
        {
            continue;
        }
        else
        {
            return ((i+1) * GMP_NUMB_BITS) - x.leading_zeros() as usize;
        }
    }
    return 0;
}

pub fn as_ulong(&self) ->u64
{
    return self.data[0];
}

pub fn to_mpz(&self, r:&mut u64) 
{//mpz_t
    // mpz_set_ui(r, 0);

    for  i in (0.. N).rev()
    {
        // mpz_mul_2exp(r, r, GMP_NUMB_BITS);
        // mpz_add_ui(r, r, self.data[i]);
    }
}

pub fn  test_bit(&self,bitno:usize) ->bool
{
    if bitno >= N * GMP_NUMB_BITS
    {
        return false;
    }
    else
    {
        let  part = bitno/GMP_NUMB_BITS;
        let bit = bitno - (GMP_NUMB_BITS*part);
        let  one = 1;//mp_limb_t
        return (self.data[part] & (one<<bit)) != 0;
    }
}

pub fn randomize(&mut self)->&Self
{

    use std::mem;
    assert!(GMP_NUMB_BITS == mem::size_of::<u64>() * 8, "Wrong GMP_NUMB_BITS value");
	// let mut  rd=std::random_device;
use rand::Rng;
 let mut rng = rand::thread_rng();
	let  num_random_words = mem::size_of::<u64>()  * N / mem::size_of::<u64>() ;//std::random_device::result_type
	let  mut random_words = &mut self.data;//reinterpret_cast<std::random_device::result_type*>
	for i in 0..num_random_words
	{
		random_words[i] =  rng.r#gen();
	}

     self
}

}


// // } // namespace libff
//#endif // BIGINT_TCC_
use std::ops::Mul;
impl<const N:usize> Mul for bigint<N> {
    type Output =Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(0)
    }
}
impl<T, const N:usize> Mul<&T> for bigint<N> {
    type Output = bigint<N>;

    fn mul(self, rhs: &T) -> bigint<N> {
        bigint::<N>::new(0)
    }
}
// impl<T,const N:usize> Mul<&T> for bigint<N> {
//     type Output =Self;

//     fn mul(self, rhs: &T) -> Self {
//         Self::new(0)
//     }
// }
use std::cmp::Ordering;


impl<const N:usize> PartialEq for bigint<N> {
     #[inline]
    fn eq(&self, other: &Self) -> bool {
       self.data[.. N]== other.data[.. N]
    }
}
impl<const N:usize> PartialOrd for bigint<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.data.cmp(&other.data))
    }
}



// 
// bool bigint<n>::operator==(const bigint<n>& other) const
// {
//     return (mpn_cmp(self.data, other.data, n) == 0);
// }

// 
// bool bigint<n>::operator!=(const bigint<n>& other) const
// {
//     return !(operator==(other));
// }

// 
// bool bigint<n>::operator<(const bigint<n>& other) const
// {
//     return (mpn_cmp(self.data, other.data, n) < 0);
// }

 use std::fmt;
impl<const N:usize> fmt::Display for bigint<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        cfg_if::cfg_if! {
        if #[cfg(feature="binary_output")]
            {write!(f, "{}",  self.data )}else{
            let mut t=0;   
            self.to_mpz(&mut t);
            write!(f, "{}",  t )}
        }
    }
}


// 
// std::ostream& operator<<(std::ostream &out, b:&bigint<n>)
// {
// // #ifdef BINARY_OUTPUT
//     out.write((char*)b.data, sizeof(b.data[0]) * n);
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
//     in.read((char*)b.data, sizeof(b.data[0]) * n);
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

//     mp_size_t limbs_written = mpn_set_str(b.data, s_copy, l, 10);
//     assert!(limbs_written <= n);
//     if limbs_written < n
//     {
//       memset(b.data + limbs_written, 0, (n - limbs_written) * sizeof(mp_limb_t));
//     }

//     delete[] s_copy;
// //#endif
//     return in;
// }