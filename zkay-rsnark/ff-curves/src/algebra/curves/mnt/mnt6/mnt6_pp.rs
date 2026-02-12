

//  Declaration of interfaces for public parameters of MNT6.


// 
// // #define MNT6_PP_HPP_

// use crate::algebra::curves::mnt::mnt6::mnt6_g1;
// use crate::algebra::curves::mnt::mnt6::mnt6_g2;
// use crate::algebra::curves::mnt::mnt6::mnt6_init;
// use crate::algebra::curves::mnt::mnt6::mnt6_pairing;
// use crate::algebra::curves::public_params;



// pub struct mnt6_pp {
//
//     type Fp_type=mnt6_Fr;
//     type G1_type=mnt6_G1;
//     type G2_type=mnt6_G2;
//     type affine_ate_G1_precomp_type=mnt6_affine_ate_G1_precomputation;
//     type affine_ate_G2_precomp_type=mnt6_affine_ate_G2_precomputation;
//     type G1_precomp_type=mnt6_G1_precomp;
//     type G2_precomp_type=mnt6_G2_precomp;
//     type Fq_type=mnt6_Fq;
//     type Fqe_type=mnt6_Fq3;
//     type Fqk_type=mnt6_Fq6;
//     type GT_type=mnt6_GT;

//     static let mut has_affine_pairing = true;

//     static pub fn  init_public_params();
//     static mnt6_GT final_exponentiation(elt:&mnt6_Fq6);
//     static mnt6_G1_precomp precompute_G1(P:&mnt6_G1);
//     static mnt6_G2_precomp precompute_G2(Q:&mnt6_G2);
//     static mnt6_Fq6 miller_loop(prec_P:&mnt6_G1_precomp,
//                                 prec_Q:&mnt6_G2_precomp);
//     static mnt6_affine_ate_G1_precomputation affine_ate_precompute_G1(P:&mnt6_G1);
//     static mnt6_affine_ate_G2_precomputation affine_ate_precompute_G2(Q:&mnt6_G2);
//     static mnt6_Fq6 affine_ate_miller_loop(prec_P:&mnt6_affine_ate_G1_precomputation,
//                                            prec_Q:&mnt6_affine_ate_G2_precomputation);
//     static mnt6_Fq6 affine_ate_e_over_e_miller_loop(prec_P1:&mnt6_affine_ate_G1_precomputation,
//                                                     prec_Q1:&mnt6_affine_ate_G2_precomputation,
//                                                     prec_P2:&mnt6_affine_ate_G1_precomputation,
//                                                     prec_Q2:&mnt6_affine_ate_G2_precomputation);
//     static mnt6_Fq6 affine_ate_e_times_e_over_e_miller_loop(prec_P1:&mnt6_affine_ate_G1_precomputation,
//                                                             prec_Q1:&mnt6_affine_ate_G2_precomputation,
//                                                             prec_P2:&mnt6_affine_ate_G1_precomputation,
//                                                             prec_Q2:&mnt6_affine_ate_G2_precomputation,
//                                                             prec_P3:&mnt6_affine_ate_G1_precomputation,
//                                                             prec_Q3:&mnt6_affine_ate_G2_precomputation);
//     static mnt6_Fq6 double_miller_loop(prec_P1:&mnt6_G1_precomp,
//                                        prec_Q1:&mnt6_G2_precomp,
//                                        prec_P2:&mnt6_G1_precomp,
//                                        prec_Q2:&mnt6_G2_precomp);

//     /* the following are used in test files */
//     static mnt6_Fq6 pairing(P:&mnt6_G1,
//                             Q:&mnt6_G2);
//     static mnt6_Fq6 reduced_pairing(P:&mnt6_G1,
//                                     Q:&mnt6_G2);
//     static mnt6_Fq6 affine_reduced_pairing(P:&mnt6_G1,
//                                            Q:&mnt6_G2);
// };

// 

// 


//  Implementation of interfaces for public parameters of MNT6.

//  See mnt6_pp.hpp .


// use crate::algebra::curves::mnt::mnt6::mnt6_pp;



// pub fn init_public_params()
// {
//     init_mnt6_params();
// }

// mnt6_GT mnt6_pp::final_exponentiation(elt:&mnt6_Fq6)
// {
//     return mnt6_final_exponentiation(elt);
// }

// mnt6_G1_precomp mnt6_pp::precompute_G1(P:&mnt6_G1)
// {
//     return mnt6_precompute_G1(P);
// }

// mnt6_G2_precomp mnt6_pp::precompute_G2(Q:&mnt6_G2)
// {
//     return mnt6_precompute_G2(Q);
// }

// mnt6_Fq6 mnt6_pp::miller_loop(prec_P:&mnt6_G1_precomp,
//                               prec_Q:&mnt6_G2_precomp)
// {
//     return mnt6_miller_loop(prec_P, prec_Q);
// }

// mnt6_affine_ate_G1_precomputation mnt6_pp::affine_ate_precompute_G1(P:&mnt6_G1)
// {
//     return mnt6_affine_ate_precompute_G1(P);
// }

// mnt6_affine_ate_G2_precomputation mnt6_pp::affine_ate_precompute_G2(Q:&mnt6_G2)
// {
//     return mnt6_affine_ate_precompute_G2(Q);
// }

// mnt6_Fq6 mnt6_pp::affine_ate_miller_loop(prec_P:&mnt6_affine_ate_G1_precomputation,
//                                          prec_Q:&mnt6_affine_ate_G2_precomputation)
// {
//     return mnt6_affine_ate_miller_loop(prec_P, prec_Q);
// }

// mnt6_Fq6 mnt6_pp::double_miller_loop(prec_P1:&mnt6_G1_precomp,
//                                      prec_Q1:&mnt6_G2_precomp,
//                                      prec_P2:&mnt6_G1_precomp,
//                                      prec_Q2:&mnt6_G2_precomp)
// {
//     return mnt6_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
// }

// mnt6_Fq6 mnt6_pp::affine_ate_e_over_e_miller_loop(prec_P1:&mnt6_affine_ate_G1_precomputation,
//                                                   prec_Q1:&mnt6_affine_ate_G2_precomputation,
//                                                   prec_P2:&mnt6_affine_ate_G1_precomputation,
//                                                   prec_Q2:&mnt6_affine_ate_G2_precomputation)
// {
//     return mnt6_affine_ate_miller_loop(prec_P1, prec_Q1) * mnt6_affine_ate_miller_loop(prec_P2, prec_Q2).unitary_inverse();
// }

// mnt6_Fq6 mnt6_pp::affine_ate_e_times_e_over_e_miller_loop(prec_P1:&mnt6_affine_ate_G1_precomputation,
//                                                           prec_Q1:&mnt6_affine_ate_G2_precomputation,
//                                                           prec_P2:&mnt6_affine_ate_G1_precomputation,
//                                                           prec_Q2:&mnt6_affine_ate_G2_precomputation,
//                                                           prec_P3:&mnt6_affine_ate_G1_precomputation,
//                                                           prec_Q3:&mnt6_affine_ate_G2_precomputation)
// {
//     return ((mnt6_affine_ate_miller_loop(prec_P1, prec_Q1) * mnt6_affine_ate_miller_loop(prec_P2, prec_Q2)) *
//             mnt6_affine_ate_miller_loop(prec_P3, prec_Q3).unitary_inverse());
// }

// mnt6_Fq6 mnt6_pp::pairing(P:&mnt6_G1,
//                           Q:&mnt6_G2)
// {
//     return mnt6_pairing(P, Q);
// }

// mnt6_Fq6 mnt6_pp::reduced_pairing(P:&mnt6_G1,
//                                   Q:&mnt6_G2)
// {
//     return mnt6_reduced_pairing(P, Q);
// }

// mnt6_Fq6 mnt6_pp::affine_reduced_pairing(P:&mnt6_G1,
//                                          Q:&mnt6_G2)
// {
//     return mnt6_affine_reduced_pairing(P, Q);
// }

// 
