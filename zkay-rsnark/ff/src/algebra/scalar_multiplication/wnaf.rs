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
 Declaration of interfaces for wNAF ("width-w Non-Adjacent Form") exponentiation routines.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
// //#ifndef WNAF_HPP_
// // #define WNAF_HPP_

// //#include <vector>

use crate::algebra::field_utils::bigint::bigint;

// // namespace libff {

// /**
//  * Find the wNAF representation of the given scalar relative to the given window size.
//  */
// template<mp_size_t N>
// std::vector<long> find_wnaf(window_size:std::usize, scalar:&bigint<N>);

// /**
//  * In additive notation, use wNAF exponentiation (with the given window size) to compute scalar * base.
//  */
// template<typename T, mp_size_t N>
// T fixed_window_wnaf_exp(window_size:std::usize, base:&T, scalar:&bigint<N>);

// /**
//  * In additive notation, use wNAF exponentiation (with the window size determined by T) to compute scalar * base.
//  */
// template<typename T, mp_size_t N>
// T opt_window_wnaf_exp(base:&T, scalar:&bigint<N>, const std::usize scalar_bits);

// } // namespace libff

// use libff::algebra::scalar_multiplication::wnaf.tcc;

//#endif // WNAF_HPP_
/** @file
 *****************************************************************************
 Implementation of interfaces for wNAF ("weighted Non-Adjacent Form") exponentiation routines.

 See wnaf.hpp .
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef WNAF_TCC_
// #define WNAF_TCC_

//#include <gmp.h>

// namespace libff {

// using std::usize;

pub fn find_wnaf<const N:usize>(window_size:usize, scalar:&bigint<N>)->Vec<i64>
{
    let   length = scalar.max_bits(); // upper bound
    let mut  res=Vec::with_capacity(length+1);
    let  mut c = scalar;
    let mut  j = 0;
    while !c.is_zero()
    {
        let mut  u;
        if (c.data[0] & 1) == 1
        {
            u = c.data[0] % (1u32 << (window_size+1));
            if u > (1 << window_size)
            {
                u = u - (1 << (window_size+1));
            }

            if u > 0
            {
                // mpn_sub_1(c.data, c.data, N, u);
            }
            else
            {
                // mpn_add_1(c.data, c.data, N, -u);
            }
        }
        else
        {
            u = 0;
        }
        res[j] = u;
        j+=1;

        // mpn_rshift(c.data, c.data, N, 1); // c = c/2
    }

    return res;
}

pub fn fixed_window_wnaf_exp<T,const N:usize>(window_size:usize, base:&T, scalar:&bigint<N>)->T
{
    let  naf = find_wnaf(window_size, scalar);
    let mut  table=Vec::with_capacity(1u64<<(window_size-1));
    let mut  tmp = base;
    let mut  dbl = base.dbl();
    for i in 0..1u64<<(window_size-1)
    {
        table[i] = tmp;
        tmp = tmp + dbl;
    }

    let mut  res = T::zero();
    let mut  found_nonzero = false;
    for  i in (0.. naf.len()).rev()
    {
        if found_nonzero
        {
            res = res.dbl();
        }

        if naf[i] != 0
        {
            found_nonzero = true;
            if naf[i] > 0
            {
                res = res + table[naf[i]/2];
            }
            else
            {
                res = res - table[(-naf[i])/2];
            }
        }
    }

    return res;
}

pub fn  opt_window_wnaf_exp<T,const N:usize>(base:&T, scalar:&bigint<N>, scalar_bits:usize)->T
{
    let mut  best = 0;
    for  i in (0.. T::wnaf_window_table.len()).rev()
    {
        if scalar_bits >= T::wnaf_window_table[i]
        {
            best = i+1;
            break;
        }
    }

    if best > 0
    {
        return fixed_window_wnaf_exp(best, base, scalar);
    }
    else
    {
        return scalar * base;
    }
}

// } // namespace libff

//#endif // WNAF_TCC_
