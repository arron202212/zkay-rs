//  This file defines default_ec_pp based on the CURVE=... make flag, which selects
//  which elliptic curve is used to implement group arithmetic and pairings.

// / Pick the elliptic curve /
// // #ifdef CURVE_BLS12_381
// // #define LIBFF_DEFAULT_EC_PP_DEFINED
// use crate::algebra::curves::bls12_381/bls12_381_pp;

// type default_ec_pp=bls12_381_pp;
// 
// 

// // #ifdef CURVE_ALT_BN128
// // #define LIBFF_DEFAULT_EC_PP_DEFINED
use crate::algebra::curves::alt_bn128::alt_bn128_pp::alt_bn128_pp;

pub type default_ec_pp = alt_bn128_pp;
// 
// 

// // #ifdef CURVE_BN128
// // #define LIBFF_DEFAULT_EC_PP_DEFINED
// use crate::algebra::curves::bn128::bn128_pp;

// type default_ec_pp=bn128_pp;
// 
// 

// // #ifdef CURVE_EDWARDS
// // #define LIBFF_DEFAULT_EC_PP_DEFINED
// use crate::algebra::curves::edwards::edwards_pp;

// type default_ec_pp=edwards_pp;
// 
// 

// // #ifdef CURVE_MNT4
// // #define LIBFF_DEFAULT_EC_PP_DEFINED
// use crate::algebra::curves::mnt::mnt4::mnt4_pp;

// type default_ec_pp=mnt4_pp;
// 
// 

// // #ifdef CURVE_MNT6
// // #define LIBFF_DEFAULT_EC_PP_DEFINED
// use crate::algebra::curves::mnt::mnt6::mnt6_pp;

// type default_ec_pp=mnt6_pp;
// 
// 

// 
// // #error You must define one of the CURVE_* symbols to pick a curve for pairings.
// 

// 
