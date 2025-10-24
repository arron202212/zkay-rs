/** @file
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef EDWARDS_PP_HPP_
// #define EDWARDS_PP_HPP_
use crate::algebra::curves::edwards/edwards_g1;
use crate::algebra::curves::edwards/edwards_g2;
use crate::algebra::curves::edwards/edwards_init;
use crate::algebra::curves::edwards/edwards_pairing;
use crate::algebra::curves::public_params;

// namespace libff {

class edwards_pp {

    typedef edwards_Fr Fp_type;
    typedef edwards_G1 G1_type;
    typedef edwards_G2 G2_type;
    typedef edwards_G1_precomp G1_precomp_type;
    typedef edwards_G2_precomp G2_precomp_type;
    typedef edwards_Fq Fq_type;
    typedef edwards_Fq3 Fqe_type;
    typedef edwards_Fq6 Fqk_type;
    typedef edwards_GT GT_type;

    static const bool has_affine_pairing = false;

    static void init_public_params();
    static edwards_GT final_exponentiation(const edwards_Fq6 &elt);
    static edwards_G1_precomp precompute_G1(const edwards_G1 &P);
    static edwards_G2_precomp precompute_G2(const edwards_G2 &Q);
    static edwards_Fq6 miller_loop(const edwards_G1_precomp &prec_P,
                                   const edwards_G2_precomp &prec_Q);
    static edwards_Fq6 double_miller_loop(const edwards_G1_precomp &prec_P1,
                                          const edwards_G2_precomp &prec_Q1,
                                          const edwards_G1_precomp &prec_P2,
                                          const edwards_G2_precomp &prec_Q2);
    /* the following are used in test files */
    static edwards_Fq6 pairing(const edwards_G1 &P,
                               const edwards_G2 &Q);
    static edwards_Fq6 reduced_pairing(const edwards_G1 &P,
                                       const edwards_G2 &Q);
};

// } // namespace libff
//#endif // EDWARDS_PP_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use crate::algebra::curves::edwards/edwards_pp;

// namespace libff {

void edwards_pp::init_public_params()
{
    init_edwards_params();
}

edwards_GT edwards_pp::final_exponentiation(const edwards_Fq6 &elt)
{
    return edwards_final_exponentiation(elt);
}

edwards_G1_precomp edwards_pp::precompute_G1(const edwards_G1 &P)
{
    return edwards_precompute_G1(P);
}

edwards_G2_precomp edwards_pp::precompute_G2(const edwards_G2 &Q)
{
    return edwards_precompute_G2(Q);
}

edwards_Fq6 edwards_pp::miller_loop(const edwards_G1_precomp &prec_P,
                                    const edwards_G2_precomp &prec_Q)
{
    return edwards_miller_loop(prec_P, prec_Q);
}

edwards_Fq6 edwards_pp::double_miller_loop(const edwards_G1_precomp &prec_P1,
                                           const edwards_G2_precomp &prec_Q1,
                                           const edwards_G1_precomp &prec_P2,
                                           const edwards_G2_precomp &prec_Q2)
{
    return edwards_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
}

edwards_Fq6 edwards_pp::pairing(const edwards_G1 &P,
                                const edwards_G2 &Q)
{
    return edwards_pairing(P, Q);
}

edwards_Fq6 edwards_pp::reduced_pairing(const edwards_G1 &P,
                                        const edwards_G2 &Q)
{
    return edwards_reduced_pairing(P, Q);
}

// } // namespace libff
