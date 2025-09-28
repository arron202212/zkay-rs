/** @file
*****************************************************************************
* @author     This file is part of libff, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

#ifndef BLS12_381_PP_HPP_
#define BLS12_381_PP_HPP_
#include <libff/algebra/curves/bls12_381/bls12_381_g1.hpp>
#include <libff/algebra/curves/bls12_381/bls12_381_g2.hpp>
#include <libff/algebra/curves/bls12_381/bls12_381_init.hpp>
#include <libff/algebra/curves/bls12_381/bls12_381_pairing.hpp>
#include <libff/algebra/curves/public_params.hpp>

namespace libff {

class bls12_381_pp {
public:
    typedef bls12_381_Fr Fp_type;
    typedef bls12_381_G1 G1_type;
    typedef bls12_381_G2 G2_type;
    typedef bls12_381_G1_precomp G1_precomp_type;
    typedef bls12_381_G2_precomp G2_precomp_type;
    typedef bls12_381_Fq Fq_type;
    typedef bls12_381_Fq2 Fqe_type;
    typedef bls12_381_Fq12 Fqk_type;
    typedef bls12_381_GT GT_type;

    static const bool has_affine_pairing = false;

    static void init_public_params();
    static bls12_381_GT final_exponentiation(const bls12_381_Fq12 &elt);
    static bls12_381_G1_precomp precompute_G1(const bls12_381_G1 &P);
    static bls12_381_G2_precomp precompute_G2(const bls12_381_G2 &Q);
    static bls12_381_Fq12 miller_loop(const bls12_381_G1_precomp &prec_P,
                                      const bls12_381_G2_precomp &prec_Q);
    static bls12_381_Fq12 double_miller_loop(const bls12_381_G1_precomp &prec_P1,
                                             const bls12_381_G2_precomp &prec_Q1,
                                             const bls12_381_G1_precomp &prec_P2,
                                             const bls12_381_G2_precomp &prec_Q2);
    static bls12_381_Fq12 pairing(const bls12_381_G1 &P,
                                  const bls12_381_G2 &Q);
    static bls12_381_Fq12 reduced_pairing(const bls12_381_G1 &P,
                                          const bls12_381_G2 &Q);
};

} // namespace libff

#endif // BLS12_381_PP_HPP_
#include <libff/algebra/curves/bls12_381/bls12_381_pp.hpp>

namespace libff {

void bls12_381_pp::init_public_params()
{
    init_bls12_381_params();
}

bls12_381_GT bls12_381_pp::final_exponentiation(const bls12_381_Fq12 &elt)
{
    return bls12_381_final_exponentiation(elt);
}

bls12_381_G1_precomp bls12_381_pp::precompute_G1(const bls12_381_G1 &P)
{
    return bls12_381_precompute_G1(P);
}

bls12_381_G2_precomp bls12_381_pp::precompute_G2(const bls12_381_G2 &Q)
{
    return bls12_381_precompute_G2(Q);
}

bls12_381_Fq12 bls12_381_pp::miller_loop(const bls12_381_G1_precomp &prec_P,
                                         const bls12_381_G2_precomp &prec_Q)
{
    return bls12_381_miller_loop(prec_P, prec_Q);
}

bls12_381_Fq12 bls12_381_pp::double_miller_loop(const bls12_381_G1_precomp &prec_P1,
                                                const bls12_381_G2_precomp &prec_Q1,
                                                const bls12_381_G1_precomp &prec_P2,
                                                const bls12_381_G2_precomp &prec_Q2)
{
    return bls12_381_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
}

bls12_381_Fq12 bls12_381_pp::pairing(const bls12_381_G1 &P,
                                     const bls12_381_G2 &Q)
{
    return bls12_381_pairing(P, Q);
}

bls12_381_Fq12 bls12_381_pp::reduced_pairing(const bls12_381_G1 &P,
                                             const bls12_381_G2 &Q)
{
    return bls12_381_reduced_pairing(P, Q);
}

} // namespace libff
