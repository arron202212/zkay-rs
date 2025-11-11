/** @file
*****************************************************************************
* @author     This file is part of libff, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

//#ifndef BLS12_381_PP_HPP_
// #define BLS12_381_PP_HPP_
use crate::algebra::curves::bls12_381/bls12_381_g1;
use crate::algebra::curves::bls12_381/bls12_381_g2;
use crate::algebra::curves::bls12_381/bls12_381_init;
use crate::algebra::curves::bls12_381/bls12_381_pairing;
use crate::algebra::curves::public_params;

// namespace libff {

pub struct bls12_381_pp {

    type Fp_type=bls12_381_Fr;
    type G1_type=bls12_381_G1;
    type G2_type=bls12_381_G2;
    type G1_precomp_type=bls12_381_G1_precomp;
    type G2_precomp_type=bls12_381_G2_precomp;
    type Fq_type=bls12_381_Fq;
    type Fqe_type=bls12_381_Fq2;
    type Fqk_type=bls12_381_Fq12;
    type GT_type=bls12_381_GT;

    static let mut has_affine_pairing = false;

    static pub fn  init_public_params();
    static bls12_381_GT final_exponentiation(elt:&bls12_381_Fq12);
    static bls12_381_G1_precomp precompute_G1(P:&bls12_381_G1);
    static bls12_381_G2_precomp precompute_G2(Q:&bls12_381_G2);
    static bls12_381_Fq12 miller_loop(prec_P:&bls12_381_G1_precomp,
                                      prec_Q:&bls12_381_G2_precomp);
    static bls12_381_Fq12 double_miller_loop(prec_P1:&bls12_381_G1_precomp,
                                             prec_Q1:&bls12_381_G2_precomp,
                                             prec_P2:&bls12_381_G1_precomp,
                                             prec_Q2:&bls12_381_G2_precomp);
    static bls12_381_Fq12 pairing(P:&bls12_381_G1,
                                  Q:&bls12_381_G2);
    static bls12_381_Fq12 reduced_pairing(P:&bls12_381_G1,
                                          Q:&bls12_381_G2);
};

// } // namespace libff

//#endif // BLS12_381_PP_HPP_
use crate::algebra::curves::bls12_381/bls12_381_pp;

// namespace libff {

pub fn init_public_params()
{
    init_bls12_381_params();
}

bls12_381_GT bls12_381_pp::final_exponentiation(elt:&bls12_381_Fq12)
{
    return bls12_381_final_exponentiation(elt);
}

bls12_381_G1_precomp bls12_381_pp::precompute_G1(P:&bls12_381_G1)
{
    return bls12_381_precompute_G1(P);
}

bls12_381_G2_precomp bls12_381_pp::precompute_G2(Q:&bls12_381_G2)
{
    return bls12_381_precompute_G2(Q);
}

bls12_381_Fq12 bls12_381_pp::miller_loop(prec_P:&bls12_381_G1_precomp,
                                         prec_Q:&bls12_381_G2_precomp)
{
    return bls12_381_miller_loop(prec_P, prec_Q);
}

bls12_381_Fq12 bls12_381_pp::double_miller_loop(prec_P1:&bls12_381_G1_precomp,
                                                prec_Q1:&bls12_381_G2_precomp,
                                                prec_P2:&bls12_381_G1_precomp,
                                                prec_Q2:&bls12_381_G2_precomp)
{
    return bls12_381_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
}

bls12_381_Fq12 bls12_381_pp::pairing(P:&bls12_381_G1,
                                     Q:&bls12_381_G2)
{
    return bls12_381_pairing(P, Q);
}

bls12_381_Fq12 bls12_381_pp::reduced_pairing(P:&bls12_381_G1,
                                             Q:&bls12_381_G2)
{
    return bls12_381_reduced_pairing(P, Q);
}

// } // namespace libff
