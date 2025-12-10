// // /** @file
// //  *****************************************************************************

// //  Declaration of interfaces for public parameters of MNT4.

// //  *****************************************************************************
// //  * @author     This file is part of libff, developed by SCIPR Lab
// //  *             and contributors (see AUTHORS).
// //  * @copyright  MIT license (see LICENSE file)
// //  *****************************************************************************/
// // //#ifndef MNT4_PP_HPP_
// // // #define MNT4_PP_HPP_

// // use crate::algebra::curves::mnt::mnt4::mnt4_g1;
// // use crate::algebra::curves::mnt::mnt4::mnt4_g2;
// // use crate::algebra::curves::mnt::mnt4::mnt4_init;
// // use crate::algebra::curves::mnt::mnt4::mnt4_pairing;
// // use crate::algebra::curves::public_params;

// pub struct mnt4_pp;
// impl PublicParamsType for mnt4_pp {
//     type Fp_type = mnt4_Fr;
//     type G1_type = mnt4_G1;
//     type G2_type = mnt4_G2;
//     type G1_precomp_type = mnt4_G1_precomp;
//     type G2_precomp_type = mnt4_G2_precomp;
//     type affine_ate_G1_precomp_type = mnt4_affine_ate_G1_precomputation;
//     type affine_ate_G2_precomp_type = mnt4_affine_ate_G2_precomputation;
//     type Fq_type = mnt4_Fq;
//     type Fqe_type = mnt4_Fq2;
//     type Fqk_type = mnt4_Fq4;
//     type GT_type = mnt4_GT;
// }

// //     static let mut has_affine_pairing = true;

// //     staticfn   init_public_params();
// //     static mnt4_GT final_exponentiation(elt:&mnt4_Fq4);

// //     static mnt4_G1_precomp precompute_G1(P:&mnt4_G1);
// //     static mnt4_G2_precomp precompute_G2(Q:&mnt4_G2);

// //     static mnt4_Fq4 miller_loop(prec_P:&mnt4_G1_precomp,
// //                                 prec_Q:&mnt4_G2_precomp);

// //     static mnt4_affine_ate_G1_precomputation affine_ate_precompute_G1(P:&mnt4_G1);
// //     static mnt4_affine_ate_G2_precomputation affine_ate_precompute_G2(Q:&mnt4_G2);
// //     static mnt4_Fq4 affine_ate_miller_loop(prec_P:&mnt4_affine_ate_G1_precomputation,
// //                                            prec_Q:&mnt4_affine_ate_G2_precomputation);

// //     static mnt4_Fq4 affine_ate_e_over_e_miller_loop(prec_P1:&mnt4_affine_ate_G1_precomputation,
// //                                                     prec_Q1:&mnt4_affine_ate_G2_precomputation,
// //                                                     prec_P2:&mnt4_affine_ate_G1_precomputation,
// //                                                     prec_Q2:&mnt4_affine_ate_G2_precomputation);
// //     static mnt4_Fq4 affine_ate_e_times_e_over_e_miller_loop(prec_P1:&mnt4_affine_ate_G1_precomputation,
// //                                                             prec_Q1:&mnt4_affine_ate_G2_precomputation,
// //                                                             prec_P2:&mnt4_affine_ate_G1_precomputation,
// //                                                             prec_Q2:&mnt4_affine_ate_G2_precomputation,
// //                                                             prec_P3:&mnt4_affine_ate_G1_precomputation,
// //                                                             prec_Q3:&mnt4_affine_ate_G2_precomputation);

// //     static mnt4_Fq4 double_miller_loop(prec_P1:&mnt4_G1_precomp,
// //                                        prec_Q1:&mnt4_G2_precomp,
// //                                        prec_P2:&mnt4_G1_precomp,
// //                                        prec_Q2:&mnt4_G2_precomp);

// //     /* the following are used in test files */
// //     static mnt4_Fq4 pairing(P:&mnt4_G1,
// //                             Q:&mnt4_G2);
// //     static mnt4_Fq4 reduced_pairing(P:&mnt4_G1,
// //                                     Q:&mnt4_G2);
// //     static mnt4_Fq4 affine_reduced_pairing(P:&mnt4_G1,
// //                                            Q:&mnt4_G2);
// // };

// impl PublicParams for mnt4_pp {
//    fn  init_public_params() {
//         init_mnt4_params();
//     }

//     fn final_exponentiation(elt: &mnt4_Fq4) -> mnt4_GT {
//         return mnt4_final_exponentiation(elt);
//     }

//     fn precompute_G1(P: &mnt4_G1) -> mnt4_G1_precomp {
//         return mnt4_precompute_G1(P);
//     }

//     fn precompute_G2(Q: &mnt4_G2) -> mnt4_G2_precomp {
//         return mnt4_precompute_G2(Q);
//     }

//     fn miller_loop(prec_P: &mnt4_G1_precomp, prec_Q: &mnt4_G2_precomp) -> mnt4_Fq4 {
//         return mnt4_miller_loop(prec_P, prec_Q);
//     }

//     fn affine_ate_precompute_G1(P: &mnt4_G1) -> mnt4_affine_ate_G1_precomputation {
//         return mnt4_affine_ate_precompute_G1(P);
//     }

//     fn affine_ate_precompute_G2(Q: &mnt4_G2) -> mnt4_affine_ate_G2_precomputation {
//         return mnt4_affine_ate_precompute_G2(Q);
//     }

//     fn affine_ate_miller_loop(
//         prec_P: &mnt4_affine_ate_G1_precomputation,
//         prec_Q: &mnt4_affine_ate_G2_precomputation,
//     ) -> mnt4_Fq4 {
//         return mnt4_affine_ate_miller_loop(prec_P, prec_Q);
//     }

//     fn affine_ate_e_over_e_miller_loop(
//         prec_P1: &mnt4_affine_ate_G1_precomputation,
//         prec_Q1: &mnt4_affine_ate_G2_precomputation,
//         prec_P2: &mnt4_affine_ate_G1_precomputation,
//         prec_Q2: &mnt4_affine_ate_G2_precomputation,
//     ) -> mnt4_Fq4 {
//         return mnt4_affine_ate_miller_loop(prec_P1, prec_Q1)
//             * mnt4_affine_ate_miller_loop(prec_P2, prec_Q2).unitary_inverse();
//     }

//     fn affine_ate_e_times_e_over_e_miller_loop(
//         prec_P1: &mnt4_affine_ate_G1_precomputation,
//         prec_Q1: &mnt4_affine_ate_G2_precomputation,
//         prec_P2: &mnt4_affine_ate_G1_precomputation,
//         prec_Q2: &mnt4_affine_ate_G2_precomputation,
//         prec_P3: &mnt4_affine_ate_G1_precomputation,
//         prec_Q3: &mnt4_affine_ate_G2_precomputation,
//     ) -> mnt4_Fq4 {
//         return ((mnt4_affine_ate_miller_loop(prec_P1, prec_Q1)
//             * mnt4_affine_ate_miller_loop(prec_P2, prec_Q2))
//             * mnt4_affine_ate_miller_loop(prec_P3, prec_Q3).unitary_inverse());
//     }

//     fn double_miller_loop(
//         prec_P1: &mnt4_G1_precomp,
//         prec_Q1: &mnt4_G2_precomp,
//         prec_P2: &mnt4_G1_precomp,
//         prec_Q2: &mnt4_G2_precomp,
//     ) -> mnt4_Fq4 {
//         return mnt4_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
//     }

//     fn pairing(P: &mnt4_G1, Q: &mnt4_G2) -> mnt4_Fq4 {
//         return mnt4_pairing(P, Q);
//     }

//     fn reduced_pairing(P: &mnt4_G1, Q: &mnt4_G2) -> mnt4_Fq4 {
//         return mnt4_reduced_pairing(P, Q);
//     }

//     fn affine_reduced_pairing(P: &mnt4_G1, Q: &mnt4_G2) -> mnt4_Fq4 {
//         return mnt4_affine_reduced_pairing(P, Q);
//     }
// }
