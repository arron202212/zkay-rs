// /** @file
//  *****************************************************************************

//  Declaration of interfaces for Kronecker substitution.

//  *****************************************************************************
//  * @author     This file is part of libfqfft, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/
//#ifndef KRONECKER_SUBSTITUTION_HPP_
// #define KRONECKER_SUBSTITUTION_HPP_

//#include <vector>

//namespace libfqfft {

use crate::polynomial_arithmetic::basic_operations::_condense;
use ffec::algebra::field_utils::bigint::GMP_NUMB_BITS;
/**
 * Given two polynomial vectors, A and B, the function performs
 * polynomial multiplication and returns the resulting polynomial vector.
 * The implementation makes use of
 * [Harvey 07, Multipoint Kronecker Substitution, Section 2.1] and
 * [Gathen and Gerhard, Modern Computer Algebra 3rd Ed., Section 8.4].
 */
//
// pub fn  kronecker_substitution(v3:&Vec<FieldT>,  v1:&Vec<FieldT>,  v2:&Vec<FieldT>);

//} // libfqfft

// use crate::kronecker_substitution::kronecker_substitution.tcc;

//#endif // KRONECKER_SUBSTITUTION_HPP_
// /** @file
//  *****************************************************************************

//  Implementation of interfaces for Kronecker substitution.

//  See kronecker_substitution.hpp .

//  *****************************************************************************
//  * @author     This file is part of libfqfft, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

//#ifndef KRONECKER_SUBSTITUTION_TCC_
// #define KRONECKER_SUBSTITUTION_TCC_

//#include <algorithm>
//#include <cmath>

//#include <gmp.h>
use ffec::common::utils;
use ffec::common::utils::div_ceil;
const GMP_NAIL_BITS: usize = 0;
const GMP_LIMB_BITS: usize = GMP_NAIL_BITS + GMP_NUMB_BITS;

//namespace libfqfft {

//
pub fn kronecker_substitution<
    FieldT: std::convert::From<usize>
        + std::ops::AddAssign
        + num_traits::Zero
        + std::ops::Add
        + Clone
        + std::cmp::PartialEq
        + std::cmp::Ord
        + std::ops::Mul,
>(
    v3: &mut Vec<FieldT>,
    v1: &Vec<FieldT>,
    v2: &Vec<FieldT>,
) {
    /* Initialize */
    let mut square = if v1 == v2 { 1 } else { 0 };

    /* Polynomial length */
    let n1 = v1.len();
    let n2 = v2.len();
    let n3 = n1 + n2 - 1;

    /* Determine number of bits needed */
    let v1_max = v1.iter().max().unwrap().clone();
    let v2_max = v2.iter().max().unwrap().clone();
    let b = 2; //* (v1_max * v2_max).as_bigint().num_bits() + 1;

    /* Number of limbs needed in total */
    let k1 = div_ceil((n1 * b) as i64, GMP_NUMB_BITS as i64).unwrap();
    let k2 = div_ceil((n2 * b) as i64, GMP_NUMB_BITS as i64).unwrap();

    /* Output polynomial */
    v3.resize(n3, FieldT::zero());

    /*
     * Allocate all MP_LIMB_T space once and store the reference pointer M1
     * to free memory afterwards. P1, P2, and P3 will remain fixed pointers
     * to the start of their respective polynomials as reference.
     */
    let m1 = vec![0; 2 * (k1 + k2) as usize]; //(mp_limb_t*) malloc (sizeof (mp_limb_t) * 2 * (k1 + k2));
    // let p1 = m1;
    //     let p2 = p1 + k1;
    //     let p3 = p2 + k2;

    //     /* Helper variables */
    //     let refs;
    //     let limb;
    //    let val;
    //     let  mask;
    //     let  limb_b;
    //     let  delta;
    //     let  delta_b;

    //     /* Construct P1 limb */
    //     let sref = p1;
    //     limb = 0;
    //     limb_b = 0;
    //     for i in 0..n1
    //     {
    //         // val = v1[i].as_ulong();
    //         // limb += (val << limb_b);

    //         /*
    //          * If the next iteration of LIMB_B is >= to the GMP_LIMB_BITS, then
    //          * write it out to MP_LIMB_T* and reset LIMB. If VAL has remaining
    //          * bits due to GMP_LIMB_BITS boundary, set it in LIMB and proceed.
    //          */
    //         if limb_b + b >= GMP_LIMB_BITS
    //         {
    //             refs = limb;
    //             refs+=1;
    //             limb = if limb_b!=0 { (val >> (GMP_LIMB_BITS - limb_b))} else {0};
    //             limb_b -= GMP_LIMB_BITS;
    //         }
    //         limb_b += b;
    //     }
    //     if limb_b!=0{ refs = limb;refs+=1;}

    //     /* Construct P2 limb. If V2 == V1, then P2 = P1 - square case. */
    //     if square!=0{p2 = p1;}
    //     else
    //     {
    //         refs = p2;
    //         limb = 0;
    //         limb_b = 0;
    //         for i in 0..n2
    //         {
    //             // val = v2[i].as_ulong();
    //             // limb += (val << limb_b);

    //             /*
    //              * If the next iteration of LIMB_B is >= to the GMP_LIMB_BITS, then
    //              * write it out to MP_LIMB_T* and reset LIMB. If VAL has remaining
    //              * bits due to GMP_LIMB_BITS boundary, set it in LIMB and proceed.
    //              */
    //             if limb_b + b >= GMP_LIMB_BITS
    //             {
    //                 refs = limb;
    //                 refs+=1;
    //                 limb = if limb_b!=0  {(val >> (GMP_LIMB_BITS - limb_b)) }else {0};
    //                 limb_b -= GMP_LIMB_BITS;
    //             }
    //             limb_b += b;
    //         }
    //         if limb_b!=0 {refs = limb;refs+=1;}
    //     }

    //     /* Multiply P1 and P2 limbs and store result in P3 limb. */
    //     let  (p3, p1, k1, p2, k2);//mpn_mul

    //     /* Perfect alignment case: bits B is equivalent to GMP_LIMB_BITS */
    //     if b == GMP_LIMB_BITS {
    //         for i in 0..n3 {v3[i] = FieldT::from(*p3);p3+=FieldT::from(1);}
    //     }

    //     else
    //     {/* Non-alignment case */
    //         /* Mask of 2^b - 1 */
    //         mask = (1u64 << b) - 1;

    //         limb = 0;
    //         limb_b = 0;
    //         for i in 0..n3
    //         {
    //             /*
    //              * If the coefficient's bit length is contained in LIMB, then
    //              * write the masked value out to vector V3 and decrement LIMB
    //              * by B bits.
    //              */
    //             if b <= limb_b
    //             {
    //                 v3[i] = FieldT::from((limb & mask) as usize);

    //                 delta = b;
    //                 delta_b = limb_b - delta;
    //             }
    //             /*
    //              * If the remaining coefficient is across two LIMBs, then write
    //              * to vector V3 the current limb's value and add upper bits from
    //              * the second part. Lastly, decrement LIMB by the coefficient's
    //              * upper portion bit length.
    //              */
    //             else
    //             {
    //                 v3[i] = FieldT::from(limb as usize);
    //                 limb=p3;

    //                 v3[i] += FieldT::from(((limb << limb_b) & mask) as usize);
    //                 p3+=1;
    //                 delta = b - limb_b;
    //                 delta_b = GMP_LIMB_BITS - delta;
    //             }

    //             limb >>= delta;
    //             limb_b = delta_b;
    //         }
    //     }

    /* Free memory */
    // free (m1);

    _condense(v3);
}

//} // libfqfft

//#endif // KRONECKER_SUBSTITUTION_TCC_
