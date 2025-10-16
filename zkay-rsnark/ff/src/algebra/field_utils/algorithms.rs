// #![feature(generic_const_exprs)]
/** @file
 *****************************************************************************
 Declaration of interfaces for (square-and-multiply) exponentiation and
 Tonelli-Shanks square root.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef ALGORITHMS_HPP_
// #define ALGORITHMS_HPP_

//#include <cstdint>
 use num_traits::One;
use crate::algebra::field_utils::bigint::bigint;
// #![feature(generic_const_exprs)]
trait FTConfig{
    const num_limbs:usize;
    const s:usize; // modulus^4 = 2^s * t + 1
    // const  t:bigint<4>; // with t odd
    // static bigint<4*n> t_minus_1_over_2; // (t-1)/2
    // const  nqr:Fp4_model<n, modulus>; // a quadratic nonresidue in Fp4
    // static Fp4_model<n, modulus> nqr_to_t; // nqr^t
}
// // namespace libff {

// /** Repeated squaring. */
// template<typename FieldT, mp_size_t m>
// FieldT power(const FieldT &base, const bigint<m> &exponent);

// /** Repeated squaring. */
// template<typename FieldT>
// FieldT power(const FieldT &base, const unsigned long exponent);

// /**
//  * The unsigned long long versions exist because libiop tends to use size_t instead
//  * of unsigned long, and size_t may be the same size as ul or ull.
//  */
// template<typename FieldT>
// FieldT power(const FieldT &base, const unsigned long long exponent);

// template<typename FieldT>
// FieldT power(const FieldT &base, const std::vector<unsigned long long> exponent);

// /**
//  * Tonelli-Shanks square root with given s, t, and quadratic non-residue.
//  * Only terminates if there is a square root. Only works if required parameters
//  * are set in the field class.
//  */
// template<typename FieldT>
// FieldT tonelli_shanks_sqrt<(const FieldT &value);

// } // namespace libff

// use crate::algebra::field_utils::/algorithms.tcc;

//#endif // ALGORITHMS_HPP_
/** @file
 *****************************************************************************
 Declaration of interfaces for (square-and-multiply) exponentiation and
 Tonelli-Shanks square root.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef ALGORITHMS_TCC_
// #define ALGORITHMS_TCC_

use crate::common::utils;
use crate::common::profiling;

// namespace libff {
pub struct Powers;
// using std::size_t;
pub trait PowerConfig<T=Self>{
   fn power<FieldT:One+Clone+ for<'a> std::ops::MulAssign<&'a FieldT>>(base:&FieldT, exponent:T)->FieldT;
}

impl<const M:usize> PowerConfig<&bigint<M>> for Powers{
// template<typename FieldT, mp_size_t m>
fn power<FieldT:One+Clone>(base:&FieldT, exponent:&bigint<M>)->FieldT
{
    let mut  result = FieldT::one();
    let mut  found_one = false;

    for  i in (0.. exponent.max_bits()).rev()
    {
        // if found_one
        // {
        //     result = result * result;
        // }

        if exponent.test_bit(i)
        {
            found_one = true;
            result = result * base.clone();
        }
    }

    return result;
}
}


// template<typename FieldT>
// FieldT power(const FieldT &base, const unsigned long exponent)
impl PowerConfig<u64> for Powers{
fn power<FieldT:One+Clone+ for<'a> std::ops::MulAssign<&'a FieldT>>(base:&FieldT, exponent:u64)->FieldT
{
    return Self::power::<FieldT>(base, &bigint::<1>::new(exponent));
}
}


// template<typename FieldT>
// FieldT power(const FieldT &base, const unsigned long long exponent)
impl PowerConfig<u128> for Powers{
fn power<FieldT:One+Clone+ for<'a> std::ops::MulAssign<&'a FieldT>>(base:&FieldT, exponent:u128)->FieldT
{
    let mut  result = FieldT::one();

    let mut  found_one = false;

    for  i  in (0..8 * std::mem::size_of_val(&exponent)).rev()
    {
        // if found_one
        // {
        //     result = result.squared();
        // }

        if exponent & (1 << i)!=0
        {
            found_one = true;
            result *= base;
        }
    }

    return result;
}
}

// template<typename FieldT>
// FieldT power(const FieldT &base, const std::vector<unsigned long long> exponent)
impl PowerConfig<Vec<u128>> for Powers{
fn power<FieldT:One + for<'a> std::ops::MulAssign<&'a FieldT>>(base:&FieldT, exponent:Vec<u128>)->FieldT
{
    let mut  result = FieldT::one();

    let mut  found_one = false;

    for j in  0.. exponent.len()
    {
        let mut cur_exp = exponent[j];
        for   i in (0..8 * std::mem::size_of_val(&cur_exp)).rev()
        {
            // if found_one
            // {
            //     result = result.squared();
            // }

            if cur_exp & (1 << i)!=0
            {
                found_one = true;
                result *= base;
            }
        }
    }

    return result;
}
}

// template<typename FieldT>
 pub fn tonelli_shanks_sqrt<FieldT:FTConfig+Clone>(value:&FieldT)->FieldT
{
    // A few assertions to make sure s, t, and nqr are initialized.
    // assert!(FieldT::s != 0);
    // assert!(!FieldT::t.is_even()); // Check that t is odd.
    // assert!(!FieldT::nqr.is_zero());

//     if value.is_zero()
//     {
//         return FieldT::zero();
//     }

//     let  one = FieldT::one();

//     let v = FieldT::s;
//     let z = FieldT::nqr_to_t;
//     let w = value^FieldT::t_minus_1_over_2;
//     let x = value * w;
//     let b = x * w; // b = value^t

// // #if DEBUG
//     // check if square with euler's criterion
//     // FieldT check = b;
//     // for i in 0..v-1
//     // {
//     //     check = check.squared();
//     // }
//     // assert!(check == one);
// //#endif

//     // compute square root with Tonelli--Shanks
//     // (does not terminate if not a square!)

//     while b != one
//     {
//         let mut  m = 0;
//         let mut  b2m = b;
//         while (b2m != one)
//         {
//             /* invariant: b2m = b^(2^m) after entering this loop */
//             b2m = b2m.squared();
//             m += 1;
//         }

//         let mut  j = v-m-1;
//         w = z;
//         while j > 0
//         {
//             w = w.squared();
//             j-=1;
//         } // w = z^2^(v-m-1)

//         z = w.squared();
//         b = b * z;
//         x = x * w;
//         v = m;
//     }

    // return x;
    value.clone()
}

// } // namespace libff

//#endif // ALGORITHMS_TCC_
