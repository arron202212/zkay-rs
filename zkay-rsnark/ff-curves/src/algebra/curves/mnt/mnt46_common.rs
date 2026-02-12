

//  Declaration of functionality that is shared among MNT curves.



// #define MNT46_COMMON_HPP_

use ffec::algebra::field_utils::bigint::GMP_NUMB_BITS;



pub const mnt46_A_bitcount: usize = 298;
pub const mnt46_B_bitcount: usize = 298;

pub const mnt46_A_limbs: usize = (mnt46_A_bitcount + GMP_NUMB_BITS - 1) / GMP_NUMB_BITS;
pub const mnt46_B_limbs: usize = (mnt46_B_bitcount + GMP_NUMB_BITS - 1) / GMP_NUMB_BITS;

// extern bigint<mnt46_A_limbs> mnt46_modulus_A;
// extern bigint<mnt46_B_limbs> mnt46_modulus_B;






//  Implementation of functionality that is shared among MNT curves.

//  See mnt46_common.hpp .


// use crate::algebra::curves::mnt/mnt46_common;



// bigint<mnt46_A_limbs> mnt46_modulus_A;
// bigint<mnt46_B_limbs> mnt46_modulus_B;


