/** @file
 *****************************************************************************

 Declaration of functionality that is shared among MNT curves.

 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MNT46_COMMON_HPP_
// #define MNT46_COMMON_HPP_

use crate::algebra::field_utils::bigint;

// namespace libff {

const mp_size_t mnt46_A_bitcount = 298;
const mp_size_t mnt46_B_bitcount = 298;

const mp_size_t mnt46_A_limbs = (mnt46_A_bitcount+GMP_NUMB_BITS-1)/GMP_NUMB_BITS;
const mp_size_t mnt46_B_limbs = (mnt46_B_bitcount+GMP_NUMB_BITS-1)/GMP_NUMB_BITS;

extern bigint<mnt46_A_limbs> mnt46_modulus_A;
extern bigint<mnt46_B_limbs> mnt46_modulus_B;

// } // namespace libff

//#endif
/** @file
 *****************************************************************************

 Implementation of functionality that is shared among MNT curves.

 See mnt46_common.hpp .

 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use libff/algebra/curves/mnt/mnt46_common;

// namespace libff {

bigint<mnt46_A_limbs> mnt46_modulus_A;
bigint<mnt46_B_limbs> mnt46_modulus_B;

// } // namespace libff
