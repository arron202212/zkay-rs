/** @file
*****************************************************************************
* @author     This file is part of libff, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

//#ifndef ALT_BN128_PP_HPP_
// #define ALT_BN128_PP_HPP_
use crate::algebra::curves::alt_bn128::alt_bn128_g1;
use crate::algebra::curves::alt_bn128::alt_bn128_g2;
use crate::algebra::curves::alt_bn128::alt_bn128_init;
use crate::algebra::curves::alt_bn128::alt_bn128_pairing;
use crate::algebra::curves::public_params;

// namespace libff {

pub struct alt_bn128_pp {

    type Fp_type=alt_bn128_Fr;
    type G1_type=alt_bn128_G1;
    type G2_type=alt_bn128_G2;
    type G1_precomp_type=alt_bn128_G1_precomp;
    type G2_precomp_type=alt_bn128_G2_precomp;
    type Fq_type=alt_bn128_Fq;
    type Fqe_type=alt_bn128_Fq2;
    type Fqk_type=alt_bn128_Fq12;
    type GT_type=alt_bn128_GT;

    static let mut has_affine_pairing = false;

    static pub fn  init_public_params();
    static alt_bn128_GT final_exponentiation(elt:&alt_bn128_Fq12);
    static alt_bn128_G1_precomp precompute_G1(P:&alt_bn128_G1);
    static alt_bn128_G2_precomp precompute_G2(Q:&alt_bn128_G2);
    static alt_bn128_Fq12 miller_loop(prec_P:&alt_bn128_G1_precomp,
                                      prec_Q:&alt_bn128_G2_precomp);
    static alt_bn128_Fq12 double_miller_loop(prec_P1:&alt_bn128_G1_precomp,
                                             prec_Q1:&alt_bn128_G2_precomp,
                                             prec_P2:&alt_bn128_G1_precomp,
                                             prec_Q2:&alt_bn128_G2_precomp);
    static alt_bn128_Fq12 pairing(P:&alt_bn128_G1,
                                  Q:&alt_bn128_G2);
    static alt_bn128_Fq12 reduced_pairing(P:&alt_bn128_G1,
                                          Q:&alt_bn128_G2);
};

// } // namespace libff

//#endif // ALT_BN128_PP_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use crate::algebra::curves::alt_bn128::alt_bn128_pp;

// namespace libff {

pub fn init_public_params()
{
    init_alt_bn128_params();
}

alt_bn128_GT alt_bn128_pp::final_exponentiation(elt:&alt_bn128_Fq12)
{
    return alt_bn128_final_exponentiation(elt);
}

alt_bn128_G1_precomp alt_bn128_pp::precompute_G1(P:&alt_bn128_G1)
{
    return alt_bn128_precompute_G1(P);
}

alt_bn128_G2_precomp alt_bn128_pp::precompute_G2(Q:&alt_bn128_G2)
{
    return alt_bn128_precompute_G2(Q);
}

alt_bn128_Fq12 alt_bn128_pp::miller_loop(prec_P:&alt_bn128_G1_precomp,
                                         prec_Q:&alt_bn128_G2_precomp)
{
    return alt_bn128_miller_loop(prec_P, prec_Q);
}

alt_bn128_Fq12 alt_bn128_pp::double_miller_loop(prec_P1:&alt_bn128_G1_precomp,
                                                prec_Q1:&alt_bn128_G2_precomp,
                                                prec_P2:&alt_bn128_G1_precomp,
                                                prec_Q2:&alt_bn128_G2_precomp)
{
    return alt_bn128_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
}

alt_bn128_Fq12 alt_bn128_pp::pairing(P:&alt_bn128_G1,
                                     Q:&alt_bn128_G2)
{
    return alt_bn128_pairing(P, Q);
}

alt_bn128_Fq12 alt_bn128_pp::reduced_pairing(P:&alt_bn128_G1,
                                             Q:&alt_bn128_G2)
{
    return alt_bn128_reduced_pairing(P, Q);
}

// } // namespace libff
