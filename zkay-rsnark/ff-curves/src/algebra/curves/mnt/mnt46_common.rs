//  Declaration of functionality that is shared among MNT curves.

use ffec::algebra::field_utils::bigint::GMP_NUMB_BITS;

pub const mnt46_A_bitcount: usize = 298;
pub const mnt46_B_bitcount: usize = 298;

pub const mnt46_A_limbs: usize = (mnt46_A_bitcount + GMP_NUMB_BITS - 1) / GMP_NUMB_BITS;
pub const mnt46_B_limbs: usize = (mnt46_B_bitcount + GMP_NUMB_BITS - 1) / GMP_NUMB_BITS;
