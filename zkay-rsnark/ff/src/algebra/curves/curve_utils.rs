/** @file
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef CURVE_UTILS_HPP_
// #define CURVE_UTILS_HPP_
//#include <cstdint>

use crate::algebra::field_utils::bigint::bigint;

// namespace libff {

// 
// GroupT scalar_mul(base:&GroupT, scalar:&bigint<m>);

// } // namespace libff
// use crate::algebra::curves::curve_utils.tcc;

//#endif // CURVE_UTILS_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef CURVE_UTILS_TCC_
// #define CURVE_UTILS_TCC_

// namespace libff {

// 
 pub fn scalar_mul<GroupT: num_traits::Zero,const M:usize>(base:&GroupT, scalar:&bigint<M>)->GroupT
{
    let mut  result = GroupT::zero();

    let mut  found_one = false;
    for i in ( 0..=(scalar.max_bits() - 1)).rev()
    {
        // if found_one
        // {
        //     result = result.dbl();
        // }

        // if scalar.test_bit(i)
        // {
        //     found_one = true;
        //     result = result + base;
        // }
    }

    return result;
}

// } // namespace libff
//#endif // CURVE_UTILS_TCC_
