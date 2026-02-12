
// Declaration of functions for generating randomness.


// FieldT SHA512_rng(const uint64_t idx);


//#include <gmp.h>
//#include <openssl/sha.h>
use crate::algebra::field_utils::bigint::GMP_NUMB_BITS;
use crate::algebra::field_utils::bigint::bigint;
use crate::common::rng;
use crate::common::utils::is_little_endian;


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
//             rval.0.0[i] = hash[i];
//         }

//         /* clear all bits higher than MSB of modulus */
//         let mut  bitno = GMP_NUMB_BITS * FieldT::num_limbs - 1;

//         /* mod is non-zero so the loop will always terminate */
//         while !FieldT::mods.test_bit(bitno)
//         {
//             let  part = bitno/GMP_NUMB_BITS;
//             let bit = bitno - (GMP_NUMB_BITS*part);

//             let one= 1;
//             rval.0.0[part] &= !(one<<bit);

//             bitno-=1;
//         }

//         iter+=1;
//         let n=FieldT::num_limbs as usize;
//         if rval.0.0[..n]< FieldT::mods.0.0[..n]{
//         break
//         }
//     }

//     /* if r.0.0 is still >= modulus -- repeat (rejection sampling) */
//     // while (mpn_cmp(rval.0.0, FieldT::mod.0.0, FieldT::num_limbs) >= 0);

//     return FieldT::from(rval);
// }




