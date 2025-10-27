/** @file
 *****************************************************************************
 Declaration of functions for generating randomness.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef RNG_HPP_
// #define RNG_HPP_

//#include <cstdint>

// namespace libff {


// FieldT SHA512_rng(const uint64_t idx);

// } // namespace libff

// use crate::common::rng.tcc;

//#endif // RNG_HPP_
/** @file
 *****************************************************************************
 Implementation of functions for generating randomness.

 See rng.hpp .
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef RNG_TCC_
// #define RNG_TCC_

//#include <gmp.h>
//#include <openssl/sha.h>

use crate::algebra::field_utils::bigint::GMP_NUMB_BITS;
use crate::common::rng;
use crate::algebra::field_utils::bigint::bigint;
use crate::common::utils::is_little_endian;

// // namespace libff {
// struct A{
// data:[u8;10]}
// // using std::usize;
// pub trait AsBigint{
// const num_limbs:i32=0;
// const mods:A;
// fn ceil_size_in_bits()->usize;
// fn as_bigint<const N:usize>(&self)->bigint<N>;
//     fn dbl<T>(&self)->T;
//     fn fixed_base_exp_window_table()->Vec<usize>;
//     fn batch_to_special_all_non_zeros<T>(t:Vec<T>);
//     fn to_special(&self);
// }
// const SHA512_DIGEST_LENGTH:usize=64;

//  pub fn SHA512_rng<FieldT: AsBigint+From<i32>>(idx:u64)->FieldT
// {
//     assert!(GMP_NUMB_BITS == 64); // current Python code cannot handle larger values, so testing here for some assumptions.
//     assert!(is_little_endian());

//     assert!(FieldT::ceil_size_in_bits() <= SHA512_DIGEST_LENGTH * 8);

//     let mut  rval;
//     let mut  iter = 0;
//     loop
//     {
//         let mut  hash=vec![((SHA512_DIGEST_LENGTH*8) + GMP_NUMB_BITS - 1)/GMP_NUMB_BITS];

//         // SHA512_CTX sha512;
//         // SHA512_Init(&sha512);
//         // SHA512_Update(&sha512, &idx, sizeof(idx));
//         // SHA512_Update(&sha512, &iter, sizeof(iter));
//         // SHA512_Final(hash, &sha512);

//         for i in 0..FieldT::num_limbs
//         {
//             rval.data[i] = hash[i];
//         }

//         /* clear all bits higher than MSB of modulus */
//         let mut  bitno = GMP_NUMB_BITS * FieldT::num_limbs - 1;

//         /* mod is non-zero so the loop will always terminate */
//         while !FieldT::mods.test_bit(bitno)
//         {
//             let  part = bitno/GMP_NUMB_BITS;
//             let bit = bitno - (GMP_NUMB_BITS*part);

//             let one= 1;
//             rval.data[part] &= !(one<<bit);

//             bitno-=1;
//         }

//         iter+=1;
//         let n=FieldT::num_limbs as usize;
//         if rval.data[..n]< FieldT::mods.data[..n]{
//         break
//         }
//     }

//     /* if r.data is still >= modulus -- repeat (rejection sampling) */
//     // while (mpn_cmp(rval.data, FieldT::mod.data, FieldT::num_limbs) >= 0);

//     return FieldT::from(rval);
// }

// } // namespace libff

//#endif // RNG_TCC_
