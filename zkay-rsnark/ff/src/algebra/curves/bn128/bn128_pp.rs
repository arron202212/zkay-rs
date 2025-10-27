/** @file
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BN128_PP_HPP_
// #define BN128_PP_HPP_
use crate::algebra::curves::bn128::bn128_g1;
use crate::algebra::curves::bn128::bn128_g2;
use crate::algebra::curves::bn128::bn128_gt;
use crate::algebra::curves::bn128::bn128_init;
use crate::algebra::curves::bn128::bn128_pairing;
use crate::algebra::curves::public_params;

// namespace libff {

pub struct bn128_pp {

    type Fp_type=bn128_Fr;
    type G1_type=bn128_G1;
    type G2_type=bn128_G2;
    type G1_precomp_type=bn128_ate_G1_precomp;
    type G2_precomp_type=bn128_ate_G2_precomp;
    type Fq_type=bn128_Fq;
    type Fqk_type=bn128_Fq12;
    type GT_type=bn128_GT;

    static let mut has_affine_pairing = false;

    static pub fn  init_public_params();
    static bn128_GT final_exponentiation(elt:&bn128_Fq12);
    static bn128_ate_G1_precomp precompute_G1(P:&bn128_G1);
    static bn128_ate_G2_precomp precompute_G2(Q:&bn128_G2);
    static bn128_Fq12 miller_loop(prec_P:&bn128_ate_G1_precomp,
                                  prec_Q:&bn128_ate_G2_precomp);
    static bn128_Fq12 double_miller_loop(prec_P1:&bn128_ate_G1_precomp,
                                         prec_Q1:&bn128_ate_G2_precomp,
                                         prec_P2:&bn128_ate_G1_precomp,
                                         prec_Q2:&bn128_ate_G2_precomp);

    /* the following are used in test files */
    static bn128_GT pairing(P:&bn128_G1,
                            Q:&bn128_G2);
    static bn128_GT reduced_pairing(P:&bn128_G1,
                                    Q:&bn128_G2);
};

// } // namespace libff
//#endif // BN128_PP_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use crate::algebra::curves::bn128::bn128_pp;
use crate::common::profiling;

// namespace libff {

pub fn init_public_params()
{
    init_bn128_params();
}

bn128_GT bn128_pp::final_exponentiation(elt:&bn128_GT)
{
    return bn128_final_exponentiation(elt);
}

bn128_ate_G1_precomp bn128_pp::precompute_G1(P:&bn128_G1)
{
    return bn128_ate_precompute_G1(P);
}

bn128_ate_G2_precomp bn128_pp::precompute_G2(Q:&bn128_G2)
{
    return bn128_ate_precompute_G2(Q);
}

bn128_Fq12 bn128_pp::miller_loop(prec_P:&bn128_ate_G1_precomp,
                                 prec_Q:&bn128_ate_G2_precomp)
{
    enter_block("Call to miller_loop<bn128_pp>");
    bn128_Fq12 result = bn128_ate_miller_loop(prec_P, prec_Q);
    leave_block("Call to miller_loop<bn128_pp>");
    return result;
}

bn128_Fq12 bn128_pp::double_miller_loop(prec_P1:&bn128_ate_G1_precomp,
                                        prec_Q1:&bn128_ate_G2_precomp,
                                        prec_P2:&bn128_ate_G1_precomp,
                                        prec_Q2:&bn128_ate_G2_precomp)
{
    enter_block("Call to double_miller_loop<bn128_pp>");
    bn128_Fq12 result = bn128_double_ate_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
    leave_block("Call to double_miller_loop<bn128_pp>");
    return result;
}

bn128_Fq12 bn128_pp::pairing(P:&bn128_G1,
                             Q:&bn128_G2)
{
    enter_block("Call to pairing<bn128_pp>");
    bn128_ate_G1_precomp prec_P = bn128_pp::precompute_G1(P);
    bn128_ate_G2_precomp prec_Q = bn128_pp::precompute_G2(Q);

    bn128_Fq12 result = bn128_pp::miller_loop(prec_P, prec_Q);
    leave_block("Call to pairing<bn128_pp>");
    return result;
}

bn128_GT bn128_pp::reduced_pairing(P:&bn128_G1,
                                   Q:&bn128_G2)
{
    enter_block("Call to reduced_pairing<bn128_pp>");
    let f= bn128_pp::pairing(P, Q);
    let result= bn128_pp::final_exponentiation(f);
    leave_block("Call to reduced_pairing<bn128_pp>");
    return result;
}

// } // namespace libff
