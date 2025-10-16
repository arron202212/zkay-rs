// /** @file
//  *****************************************************************************

//  Declaration of interfaces for public parameters of MNT4.

//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// //#ifndef MNT4_PP_HPP_
// // #define MNT4_PP_HPP_

// use crate::algebra::curves::mnt::mnt4::mnt4_g1;
// use crate::algebra::curves::mnt::mnt4::mnt4_g2;
// use crate::algebra::curves::mnt::mnt4::mnt4_init;
// use crate::algebra::curves::mnt::mnt4::mnt4_pairing;
// use crate::algebra::curves::public_params;

// // namespace libff {

// class mnt4_pp {
// public:
//     typedef mnt4_Fr Fp_type;
//     typedef mnt4_G1 G1_type;
//     typedef mnt4_G2 G2_type;
//     typedef mnt4_G1_precomp G1_precomp_type;
//     typedef mnt4_G2_precomp G2_precomp_type;
//     typedef mnt4_affine_ate_G1_precomputation affine_ate_G1_precomp_type;
//     typedef mnt4_affine_ate_G2_precomputation affine_ate_G2_precomp_type;
//     typedef mnt4_Fq Fq_type;
//     typedef mnt4_Fq2 Fqe_type;
//     typedef mnt4_Fq4 Fqk_type;
//     typedef mnt4_GT GT_type;

//     static const bool has_affine_pairing = true;

//     static void init_public_params();
//     static mnt4_GT final_exponentiation(const mnt4_Fq4 &elt);

//     static mnt4_G1_precomp precompute_G1(const mnt4_G1 &P);
//     static mnt4_G2_precomp precompute_G2(const mnt4_G2 &Q);

//     static mnt4_Fq4 miller_loop(const mnt4_G1_precomp &prec_P,
//                                 const mnt4_G2_precomp &prec_Q);

//     static mnt4_affine_ate_G1_precomputation affine_ate_precompute_G1(const mnt4_G1 &P);
//     static mnt4_affine_ate_G2_precomputation affine_ate_precompute_G2(const mnt4_G2 &Q);
//     static mnt4_Fq4 affine_ate_miller_loop(const mnt4_affine_ate_G1_precomputation &prec_P,
//                                            const mnt4_affine_ate_G2_precomputation &prec_Q);

//     static mnt4_Fq4 affine_ate_e_over_e_miller_loop(const mnt4_affine_ate_G1_precomputation &prec_P1,
//                                                     const mnt4_affine_ate_G2_precomputation &prec_Q1,
//                                                     const mnt4_affine_ate_G1_precomputation &prec_P2,
//                                                     const mnt4_affine_ate_G2_precomputation &prec_Q2);
//     static mnt4_Fq4 affine_ate_e_times_e_over_e_miller_loop(const mnt4_affine_ate_G1_precomputation &prec_P1,
//                                                             const mnt4_affine_ate_G2_precomputation &prec_Q1,
//                                                             const mnt4_affine_ate_G1_precomputation &prec_P2,
//                                                             const mnt4_affine_ate_G2_precomputation &prec_Q2,
//                                                             const mnt4_affine_ate_G1_precomputation &prec_P3,
//                                                             const mnt4_affine_ate_G2_precomputation &prec_Q3);

//     static mnt4_Fq4 double_miller_loop(const mnt4_G1_precomp &prec_P1,
//                                        const mnt4_G2_precomp &prec_Q1,
//                                        const mnt4_G1_precomp &prec_P2,
//                                        const mnt4_G2_precomp &prec_Q2);

//     /* the following are used in test files */
//     static mnt4_Fq4 pairing(const mnt4_G1 &P,
//                             const mnt4_G2 &Q);
//     static mnt4_Fq4 reduced_pairing(const mnt4_G1 &P,
//                                     const mnt4_G2 &Q);
//     static mnt4_Fq4 affine_reduced_pairing(const mnt4_G1 &P,
//                                            const mnt4_G2 &Q);
// };

// // } // namespace libff

// //#endif // MNT4_PP_HPP_
// /** @file
//  *****************************************************************************

//  Implementation of interfaces for public parameters of MNT4.

//  See mnt4_pp.hpp .

//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// use crate::algebra::curves::mnt::mnt4::mnt4_pp;

// // namespace libff {

// void mnt4_pp::init_public_params()
// {
//     init_mnt4_params();
// }

// mnt4_GT mnt4_pp::final_exponentiation(const mnt4_Fq4 &elt)
// {
//     return mnt4_final_exponentiation(elt);
// }

// mnt4_G1_precomp mnt4_pp::precompute_G1(const mnt4_G1 &P)
// {
//     return mnt4_precompute_G1(P);
// }

// mnt4_G2_precomp mnt4_pp::precompute_G2(const mnt4_G2 &Q)
// {
//     return mnt4_precompute_G2(Q);
// }

// mnt4_Fq4 mnt4_pp::miller_loop(const mnt4_G1_precomp &prec_P,
//                               const mnt4_G2_precomp &prec_Q)
// {
//     return mnt4_miller_loop(prec_P, prec_Q);
// }

// mnt4_affine_ate_G1_precomputation mnt4_pp::affine_ate_precompute_G1(const mnt4_G1 &P)
// {
//     return mnt4_affine_ate_precompute_G1(P);
// }

// mnt4_affine_ate_G2_precomputation mnt4_pp::affine_ate_precompute_G2(const mnt4_G2 &Q)
// {
//     return mnt4_affine_ate_precompute_G2(Q);
// }

// mnt4_Fq4 mnt4_pp::affine_ate_miller_loop(const mnt4_affine_ate_G1_precomputation &prec_P,
//                                          const mnt4_affine_ate_G2_precomputation &prec_Q)
// {
//     return mnt4_affine_ate_miller_loop(prec_P, prec_Q);
// }

// mnt4_Fq4 mnt4_pp::affine_ate_e_over_e_miller_loop(const mnt4_affine_ate_G1_precomputation &prec_P1,
//                                                   const mnt4_affine_ate_G2_precomputation &prec_Q1,
//                                                   const mnt4_affine_ate_G1_precomputation &prec_P2,
//                                                   const mnt4_affine_ate_G2_precomputation &prec_Q2)
// {
//     return mnt4_affine_ate_miller_loop(prec_P1, prec_Q1) * mnt4_affine_ate_miller_loop(prec_P2, prec_Q2).unitary_inverse();
// }

// mnt4_Fq4 mnt4_pp::affine_ate_e_times_e_over_e_miller_loop(const mnt4_affine_ate_G1_precomputation &prec_P1,
//                                                           const mnt4_affine_ate_G2_precomputation &prec_Q1,
//                                                           const mnt4_affine_ate_G1_precomputation &prec_P2,
//                                                           const mnt4_affine_ate_G2_precomputation &prec_Q2,
//                                                           const mnt4_affine_ate_G1_precomputation &prec_P3,
//                                                           const mnt4_affine_ate_G2_precomputation &prec_Q3)
// {
//     return ((mnt4_affine_ate_miller_loop(prec_P1, prec_Q1) * mnt4_affine_ate_miller_loop(prec_P2, prec_Q2)) *
//             mnt4_affine_ate_miller_loop(prec_P3, prec_Q3).unitary_inverse());
// }

// mnt4_Fq4 mnt4_pp::double_miller_loop(const mnt4_G1_precomp &prec_P1,
//                                      const mnt4_G2_precomp &prec_Q1,
//                                      const mnt4_G1_precomp &prec_P2,
//                                      const mnt4_G2_precomp &prec_Q2)
// {
//     return mnt4_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
// }

// mnt4_Fq4 mnt4_pp::pairing(const mnt4_G1 &P,
//                           const mnt4_G2 &Q)
// {
//     return mnt4_pairing(P, Q);
// }

// mnt4_Fq4 mnt4_pp::reduced_pairing(const mnt4_G1 &P,
//                                   const mnt4_G2 &Q)
// {
//     return mnt4_reduced_pairing(P, Q);
// }

// mnt4_Fq4 mnt4_pp::affine_reduced_pairing(const mnt4_G1 &P,
//                                          const mnt4_G2 &Q)
// {
//     return mnt4_affine_reduced_pairing(P, Q);
// }

// // } // namespace libff
