/** @file
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef EDWARDS_PP_HPP_
// #define EDWARDS_PP_HPP_
use crate::algebra::curves::edwards::edwards_g1;
use crate::algebra::curves::edwards::edwards_g2;
use crate::algebra::curves::edwards::edwards_init;
use crate::algebra::curves::edwards::edwards_pairing;
use crate::algebra::curves::public_params;

// namespace libff {

pub struct edwards_pp {

    type Fp_type=edwards_Fr;
    type G1_type=edwards_G1;
    type G2_type=edwards_G2;
    type G1_precomp_type=edwards_G1_precomp;
    type G2_precomp_type=edwards_G2_precomp;
    type Fq_type=edwards_Fq;
    type Fqe_type=edwards_Fq3;
    type Fqk_type=edwards_Fq6;
    type GT_type=edwards_GT;

    static let mut has_affine_pairing = false;

    static pub fn  init_public_params();
    static edwards_GT final_exponentiation(elt:&edwards_Fq6);
    static edwards_G1_precomp precompute_G1(P:&edwards_G1);
    static edwards_G2_precomp precompute_G2(Q:&edwards_G2);
    static edwards_Fq6 miller_loop(prec_P:&edwards_G1_precomp,
                                   prec_Q:&edwards_G2_precomp);
    static edwards_Fq6 double_miller_loop(prec_P1:&edwards_G1_precomp,
                                          prec_Q1:&edwards_G2_precomp,
                                          prec_P2:&edwards_G1_precomp,
                                          prec_Q2:&edwards_G2_precomp);
    /* the following are used in test files */
    static edwards_Fq6 pairing(P:&edwards_G1,
                               Q:&edwards_G2);
    static edwards_Fq6 reduced_pairing(P:&edwards_G1,
                                       Q:&edwards_G2);
};

// } // namespace libff
//#endif // EDWARDS_PP_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use crate::algebra::curves::edwards::edwards_pp;

// namespace libff {

pub fn init_public_params()
{
    init_edwards_params();
}

edwards_GT edwards_pp::final_exponentiation(elt:&edwards_Fq6)
{
    return edwards_final_exponentiation(elt);
}

edwards_G1_precomp edwards_pp::precompute_G1(P:&edwards_G1)
{
    return edwards_precompute_G1(P);
}

edwards_G2_precomp edwards_pp::precompute_G2(Q:&edwards_G2)
{
    return edwards_precompute_G2(Q);
}

edwards_Fq6 edwards_pp::miller_loop(prec_P:&edwards_G1_precomp,
                                    prec_Q:&edwards_G2_precomp)
{
    return edwards_miller_loop(prec_P, prec_Q);
}

edwards_Fq6 edwards_pp::double_miller_loop(prec_P1:&edwards_G1_precomp,
                                           prec_Q1:&edwards_G2_precomp,
                                           prec_P2:&edwards_G1_precomp,
                                           prec_Q2:&edwards_G2_precomp)
{
    return edwards_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
}

edwards_Fq6 edwards_pp::pairing(P:&edwards_G1,
                                Q:&edwards_G2)
{
    return edwards_pairing(P, Q);
}

edwards_Fq6 edwards_pp::reduced_pairing(P:&edwards_G1,
                                        Q:&edwards_G2)
{
    return edwards_reduced_pairing(P, Q);
}

// } // namespace libff
