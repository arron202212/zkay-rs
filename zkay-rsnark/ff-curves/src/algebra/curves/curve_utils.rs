

use ffec::algebra::field_utils::bigint::bigint;



//
// GroupT scalar_mul(base:&GroupT, scalar:&bigint<m>);


// use crate::algebra::curves::curve_utils.tcc;






// #define CURVE_UTILS_TCC_



//
pub fn scalar_mul<GroupT: num_traits::Zero, const M: usize>(
    base: &GroupT,
    scalar: &bigint<M>,
) -> GroupT {
    let mut result = GroupT::zero();

    let mut found_one = false;
    for i in (0..=(scalar.max_bits() - 1)).rev() {
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



