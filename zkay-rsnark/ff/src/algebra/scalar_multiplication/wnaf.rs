#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

// Declaration of interfaces for wNAF ("width-w Non-Adjacent Form") exponentiation routines.
use crate::FieldTConfig;
use crate::PpConfig;
use crate::Zero;
use crate::algebra::field_utils::bigint::bigint;
use crate::field_utils::BigInteger;
use crate::scalar_multiplication::multiexp::KCConfig;
use std::ops::Mul;

// /**
//  * Find the wNAF representation of the given scalar relative to the given window size.
//  */
//
// Vec<long> find_wnaf(window_size:std::usize, scalar:&bigint<N>);

// /**
//  * In additive notation, use wNAF exponentiation (with the given window size) to compute scalar * base.
//  */
//
// T fixed_window_wnaf_exp(window_size:std::usize, base:&T, scalar:&bigint<N>);

// /**
//  * In additive notation, use wNAF exponentiation (with the window size determined by T) to compute scalar * base.
//  */
//
// T opt_window_wnaf_exp(base:&T, scalar:&bigint<N>, const std::usize scalar_bits);



// pub trait Config {
//     fn wnaf_window_table() -> Vec<usize>;
//     fn dbl<T:Zero>(&self) -> T;
// }
pub fn find_wnaf_u(window_size: usize, scalar: u128) -> Vec<i64> {
    find_wnaf(window_size, &[0u64])
}
pub fn find_wnaf(window_size: usize, scalar: impl AsRef<[u64]>) -> Vec<i64> {
    let length = scalar.as_ref().len() as usize; // upper bound
    let mut res = Vec::with_capacity(length + 1);
    let mut c = scalar;
    let mut j = 0;
    while !c.as_ref().iter().all(|&x| x == 0) {
        let mut u;
        if c.as_ref()[0] & 1 == 1 {
            u = (c.as_ref()[0] as i64) % (1i64 << (window_size + 1));
            if u > (1 << window_size) {
                u = u - (1 << (window_size + 1));
            }

            if u > 0 {
                // mpn_sub_1(c.0.0, c.0.0, N, u);
            } else {
                // mpn_add_1(c.0.0, c.0.0, N, -u);
            }
        } else {
            u = 0;
        }
        res[j] = u;
        j += 1;

        // mpn_rshift(c.0.0, c.0.0, N, 1); // c = c/2
    }

    res
}

pub fn fixed_window_wnaf_exp<T: PpConfig, FieldT: FieldTConfig>(
    window_size: usize,
    base: &T,
    scalar: &FieldT,
) -> T {
    let naf = find_wnaf(window_size, scalar.as_ref_u64());
    let mut table = Vec::with_capacity(1usize << (window_size - 1));
    let mut tmp = base.clone();
    let mut dbl: T = base.dbl();
    for i in 0..1usize << (window_size - 1) {
        table[i] = tmp.clone();
        tmp = tmp + dbl.clone();
    }

    let mut res = T::zero();
    let mut found_nonzero = false;
    for i in (0..naf.len()).rev() {
        if found_nonzero {
            res = res.dbl();
        }

        if naf[i] != 0 {
            found_nonzero = true;
            if naf[i] > 0 {
                res = res + table[naf[i] as usize / 2].clone();
            } else {
                res = res - table[(-naf[i]) as usize / 2].clone();
            }
        }
    }

    res
}

pub fn opt_window_wnaf_exp<T: PpConfig, FieldT: FieldTConfig>(
    base: &T,
    scalar: &FieldT,
    scalar_bits: usize,
) -> T
// where for<'a> &'a T: Mul<&'a bigint<N>, Output = T>
{
    let mut best = 0;
    for i in (0..T::wnaf_window_table().len()).rev() {
        if scalar_bits >= T::wnaf_window_table()[i] {
            best = i + 1;
            break;
        }
    }

    if best > 0 {
        fixed_window_wnaf_exp::<T, FieldT>(best, base, scalar)
    } else {
        T::zero() // base*scalar;
    }
}
