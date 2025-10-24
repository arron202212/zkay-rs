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

class alt_bn128_pp {

    typedef alt_bn128_Fr Fp_type;
    typedef alt_bn128_G1 G1_type;
    typedef alt_bn128_G2 G2_type;
    typedef alt_bn128_G1_precomp G1_precomp_type;
    typedef alt_bn128_G2_precomp G2_precomp_type;
    typedef alt_bn128_Fq Fq_type;
    typedef alt_bn128_Fq2 Fqe_type;
    typedef alt_bn128_Fq12 Fqk_type;
    typedef alt_bn128_GT GT_type;

    static const bool has_affine_pairing = false;

    static void init_public_params();
    static alt_bn128_GT final_exponentiation(const alt_bn128_Fq12 &elt);
    static alt_bn128_G1_precomp precompute_G1(const alt_bn128_G1 &P);
    static alt_bn128_G2_precomp precompute_G2(const alt_bn128_G2 &Q);
    static alt_bn128_Fq12 miller_loop(const alt_bn128_G1_precomp &prec_P,
                                      const alt_bn128_G2_precomp &prec_Q);
    static alt_bn128_Fq12 double_miller_loop(const alt_bn128_G1_precomp &prec_P1,
                                             const alt_bn128_G2_precomp &prec_Q1,
                                             const alt_bn128_G1_precomp &prec_P2,
                                             const alt_bn128_G2_precomp &prec_Q2);
    static alt_bn128_Fq12 pairing(const alt_bn128_G1 &P,
                                  const alt_bn128_G2 &Q);
    static alt_bn128_Fq12 reduced_pairing(const alt_bn128_G1 &P,
                                          const alt_bn128_G2 &Q);
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

void alt_bn128_pp::init_public_params()
{
    init_alt_bn128_params();
}

alt_bn128_GT alt_bn128_pp::final_exponentiation(const alt_bn128_Fq12 &elt)
{
    return alt_bn128_final_exponentiation(elt);
}

alt_bn128_G1_precomp alt_bn128_pp::precompute_G1(const alt_bn128_G1 &P)
{
    return alt_bn128_precompute_G1(P);
}

alt_bn128_G2_precomp alt_bn128_pp::precompute_G2(const alt_bn128_G2 &Q)
{
    return alt_bn128_precompute_G2(Q);
}

alt_bn128_Fq12 alt_bn128_pp::miller_loop(const alt_bn128_G1_precomp &prec_P,
                                         const alt_bn128_G2_precomp &prec_Q)
{
    return alt_bn128_miller_loop(prec_P, prec_Q);
}

alt_bn128_Fq12 alt_bn128_pp::double_miller_loop(const alt_bn128_G1_precomp &prec_P1,
                                                const alt_bn128_G2_precomp &prec_Q1,
                                                const alt_bn128_G1_precomp &prec_P2,
                                                const alt_bn128_G2_precomp &prec_Q2)
{
    return alt_bn128_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
}

alt_bn128_Fq12 alt_bn128_pp::pairing(const alt_bn128_G1 &P,
                                     const alt_bn128_G2 &Q)
{
    return alt_bn128_pairing(P, Q);
}

alt_bn128_Fq12 alt_bn128_pp::reduced_pairing(const alt_bn128_G1 &P,
                                             const alt_bn128_G2 &Q)
{
    return alt_bn128_reduced_pairing(P, Q);
}

// } // namespace libff
