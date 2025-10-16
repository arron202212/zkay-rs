
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
 Declaration of miscellaneous math, serialization, and other common utility
 functions.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
// //#ifndef UTILS_HPP_
// // #define UTILS_HPP_

//#include <cassert>
//#include <iostream>
//#include <sstream>
//#include <string>
//#include <vector>

// namespace libff {

pub type bit_vector=Vec<bool> ;

// template<bool B, class T = void>
// struct enable_if { typedef void* type; };

// template<class T>
// struct enable_if<true, T> { typedef T type; };

// std::usize get_power_of_two(std::usize n);

// std::usize round_to_next_power_of_2(n:std::usize);
// bool is_power_of_2(n:std::usize);

// /// returns ceil(log2(n)), so 1ul<<log2(n) is the smallest power of 2, that is not less than n
// std::usize log2(std::usize n);

// inline std::usize exp2(std::usize k) { return std::usize(1) << k; }

// std::usize to_twos_complement(i32 i, std::usize w);
// i32 from_twos_complement(std::usize i, std::usize w);

// std::usize bitreverse(std::usize n, const std::usize l);
// bit_vector int_list_to_bits(l:&const std::initializer_list<unsigned long>, const std::usize wordsize);
// /* throws error if y = 0 */
// long long div_ceil(long long x, long long y);

// bool is_little_endian();

// std::string FORMAT(prefix:&const std::string, const char* format, ...);

/* A variadic template to suppress unused argument warnings */
// template<typename ... Types>
// void UNUSED(Types&&...) {}

// #ifdef DEBUG
// #define FMT FORMAT
// #else
// #define FMT(...) (UNUSED(__VA_ARGS__), "")
//#endif

// void serialize_bit_vector(out:&String,v:& const bit_vector);
// void deserialize_bit_vector(in:&mut String,v:& bit_vector);

// /** Should not be used for fields, because the field function is named ceil_size_in_bits instead. */
// template<typename CurveT>
// std::usize curve_size_in_bits(const Vec <CurveT> &v);

/* Print a vector in the form { elem0 elem1 elem2 ... }, with a newline at the end
template<typename T>
void print_vector(Vec <T> &vec);
template<typename T>
void print_vector(Vec <T> vec);*/
 use std::fmt::Write;
pub fn  print_vector<T: std::fmt::Display>(vec:&Vec <T>)
{
    print!("{{ ");
    for  elem in  vec
    {
        print!("{elem} ");
    }
    print!("}}\n");
}


// /**
//  * Returns a random element of T that is not zero or one.
//  * T can be a field or elliptic curve group.
//  * Used for testing to generate a test example that doesn't error.
//  */
// template<typename T>
// T random_element_non_zero_one();
// /**
//  * Returns a random element of T that is not zero.
//  * T can be a field or elliptic curve group.
//  * Used for testing to generate a test example that doesn't error.
//  */
// template<typename T>
// T random_element_non_zero();
// /**
//  * Returns a random element of T that is not equal to y.
//  * T can be a field or elliptic curve group.
//  * Used for testing to generate a test example that doesn't error.
//  */
// template<typename T>
// T random_element_exclude(T y);

// #define ARRAY_SIZE(arr) (sizeof(arr)/sizeof(arr[0]))

// } // namespace libff

// use crate::common::utils.tcc; /* note that utils has a templatized part (utils.tcc) and non-templatized part (utils.cpp) */
//#endif // UTILS_HPP_
/** @file
 *****************************************************************************
 Implementation of misc math and serialization utility functions.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#include <algorithm>
//#include <cassert>
//#include <cstdarg>
//#include <cstdint>

// use crate::common::utils;

// namespace libff {

// using std::usize;

/**
 * Round n to the next power of two.
 * If n is a power of two, return n
 */
pub fn get_power_of_two(mut n:usize)->usize{
    n-=1;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n |= n >> 32;
    n+=1;

    return n;
}

/* If n is a power of 2, returns n */
pub fn round_to_next_power_of_2(n:usize)->usize{
    1usize << log2(n)
}

pub fn is_power_of_2(n:usize)->bool{
    return (n != 0) && ((n & (n-1)) == 0);
}

/* returns ceil(log2(n)), so 1ul<<log2(n) is the smallest power of 2,
   that is not less than n. */
pub fn log2(mut n:usize)->usize
{
    let mut  r = if n & (n-1) == 0  {0 }else {1}; // add 1 if n is not power of 2

    while n > 1
    {
        n >>= 1;
        r+=1;
    }

    return r;
}

pub fn to_twos_complement(i:i64, w:usize)->usize{
    assert!(i  >= -(1i64<<(w-1)));
    assert!(i  < (1i64<<(w-1)));
    (if i >= 0 {i} else {i + (1i64<<w)}) as _
}

pub fn from_twos_complement(i:usize, w:usize)->usize{
    assert!(i < (1usize<<w));
    if i < (1usize<<(w-1)) {i }else {i - (1usize<<w)}
}

pub fn bitreverse(mut n:usize, l:usize)->usize{
    let mut  r = 0;
    for k in 0..l
    {
        r = (r << 1) | (n & 1);
        n >>= 1;
    }
    return r;
}

pub fn int_list_to_bits(l:&Vec<u64>, wordsize:usize)->bit_vector{
     let mut res=Vec::with_capacity(wordsize*l.len());
    for i in 0..l.len()
    {
        for j in 0..wordsize
        {
            res[i*wordsize + j] = (l[i] & (1u64<<(wordsize-1-j))) != 0;
        }
    }
    return res;
}

pub fn div_ceil( x:i64,  y:i64)->eyre::Result<i64>{
    if y == 0
    {
        eyre::bail!("div_ceil: division by zero, second argument must be non-zero");
    }
    return Ok((x + (y-1)) / y);
}

pub fn is_little_endian()->bool{
    if cfg!(target_endian = "little") { true } else { false}
    // let  a:u64 = 0x12345678;
    // a.to_le()==a.to_ne()
}

// pub fn FORMAT(prefix:&const std::string, const char* format, ...)->string{
//     const static usize MAX_FMT = 256;
//     char buf[MAX_FMT];
//     va_list args;
//     va_start(args, format);
//     vsnprintf(buf, MAX_FMT, format, args);
//     va_end(args);

//     return prefix + std::string(buf);
// }

pub fn serialize_bit_vector(out:&mut String,v:& bit_vector){
    write!(out,"{}\n",v.len());
    for  b in  v
    {
         write!(out,"{}\n",b);
    }
}

pub fn deserialize_bit_vector(ins:&String,v:&mut  bit_vector){
    let mut buf_read=ins.split_ascii_whitespace();
    let  size=buf_read.next().unwrap().parse::<usize>().unwrap() ;
    v.resize(size,false);
    for i in 0..size
    {
        v[i] =buf_read.next().unwrap().parse::<bool>().unwrap();
    }
}

// } // namespace libff
/** @file
 *****************************************************************************
 Implementation of templatized utility functions.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef UTILS_TCC_
// #define UTILS_TCC_

// namespace libff {

// using std::usize;

trait CConfig{
    fn size_in_bits()->usize;
    fn random_element<T>()->T;
    fn is_zero(&self)->bool;
}
pub fn curve_size_in_bits<CurveT:CConfig>(v:&Vec<CurveT>)->usize{
    return v.len() * CurveT::size_in_bits();
}


pub fn random_element_non_zero_one<T:CConfig+num_traits::One+ std::cmp::PartialEq>()->T{
    let mut  x :T= T::random_element();
    while x.is_zero() || x == T::one()
     {   x = T::random_element();}
    return x;
}

pub fn random_element_non_zero<T:CConfig>()->T{
    let mut  x:T = T::random_element();
    while x.is_zero()
       { x = T::random_element();}
    return x;
}


pub fn random_element_exclude<T:CConfig+ std::cmp::PartialEq>(y:T)->T{
    let mut  x:T = T::random_element();
    while x == y
       { x = T::random_element();}
    return x;
}

// } // namespace libff

//#endif // UTILS_TCC_
