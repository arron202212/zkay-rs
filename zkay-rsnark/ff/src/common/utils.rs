#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

// Declaration of miscellaneous math, serialization, and other common utility
// functions.
use crate::PpConfig;

pub type bit_vector = Vec<bool>;

use std::fmt::Write;
pub fn print_vector<T: std::fmt::Display>(vec: &Vec<T>) {
    print!("{{ ");
    for elem in vec {
        print!("{elem} ");
    }
    print!("}}\n");
}

//  note that utils has a templatized part (utils.tcc) and non-templatized part (utils.cpp)

// Implementation of misc math and serialization utility functions.

//  * Round n to the next power of two.
//  * If n is a power of two, return n

pub fn get_power_of_two(mut n: usize) -> usize {
    n -= 1;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n |= n >> 32;
    n += 1;

    return n;
}

//If n is a power of 2, returns n
pub fn round_to_next_power_of_2(n: usize) -> usize {
    1usize << log2(n)
}

pub fn is_power_of_2(n: usize) -> bool {
    return (n != 0) && ((n & (n - 1)) == 0);
}

//  returns ceil(log2(n)), so 1u64<<log2(n) is the smallest power of 2,
// that is not less than n.
pub fn log2(mut n: usize) -> usize {
    let mut r = if n & (n - 1) == 0 { 0 } else { 1 }; // add 1 if n is not power of 2

    while n > 1 {
        n >>= 1;
        r += 1;
    }

    return r;
}

pub fn to_twos_complement(i: i64, w: usize) -> usize {
    assert!(i >= -(1i64 << (w - 1)));
    assert!(i < (1i64 << (w - 1)));
    (if i >= 0 { i } else { i + (1i64 << w) }) as _
}

pub fn from_twos_complement(i: usize, w: usize) -> usize {
    assert!(i < (1usize << w));
    if i < (1usize << (w - 1)) {
        i
    } else {
        i - (1usize << w)
    }
}

pub fn bitreverse(mut n: usize, l: usize) -> usize {
    let mut r = 0;
    for k in 0..l {
        r = (r << 1) | (n & 1);
        n >>= 1;
    }
    return r;
}

pub fn int_list_to_bits(l: &[usize], wordsize: usize) -> bit_vector {
    let mut res = Vec::with_capacity(wordsize * l.len());
    for i in 0..l.len() {
        for j in 0..wordsize {
            res[i * wordsize + j] = (l[i] & (1usize << (wordsize - 1 - j))) != 0;
        }
    }
    res
}

pub fn div_ceil(x: usize, y: usize) -> eyre::Result<usize> {
    if y == 0 {
        eyre::bail!("div_ceil: division by zero, second argument must be non-zero");
    }
    return Ok((x + (y - 1)) / y);
}

pub fn is_little_endian() -> bool {
    if cfg!(target_endian = "little") {
        true
    } else {
        false
    }
    // let  a:u64 = 0x12345678;
    // a.to_le()==a.to_ne()
}

pub fn serialize_bit_vector(out: &mut String, v: &bit_vector) {
    write!(out, "{}\n", v.len());
    for b in v {
        write!(out, "{}\n", b);
    }
}

pub fn deserialize_bit_vector(ins: &String, v: &mut bit_vector) {
    let mut buf_read = ins.split_ascii_whitespace();
    let size = buf_read.next().unwrap().parse::<usize>().unwrap();
    v.resize(size, false);
    for i in 0..size {
        v[i] = buf_read.next().unwrap().parse::<bool>().unwrap();
    }
}

pub fn size_in_bits<CurveT: PpConfig>(v: &Vec<CurveT>) -> usize {
    v.len() * CurveT::size_in_bits()
}
//  * Returns a random element of T that is not zero or one.
//  * T can be a field or elliptic curve group.
//  * Used for testing to generate a test example that doesn't error.

pub fn random_element_non_zero_one<T: PpConfig>() -> T {
    let mut x: T = T::random_element();
    while x.is_zero() || x == T::one() {
        x = T::random_element();
    }
    return x;
}
//  * Returns a random element of T that is not zero.
//  * T can be a field or elliptic curve group.
//  * Used for testing to generate a test example that doesn't error.

pub fn random_element_non_zero<T: PpConfig>() -> T {
    let mut x: T = T::random_element();
    while x.is_zero() {
        x = T::random_element();
    }
    return x;
}
//  * Returns a random element of T that is not equal to y.
//  * T can be a field or elliptic curve group.
//  * Used for testing to generate a test example that doesn't error.

pub fn random_element_exclude<T: PpConfig>(y: T) -> T {
    let mut x: T = T::random_element();
    while x == y {
        x = T::random_element();
    }
    return x;
}
