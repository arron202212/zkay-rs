



// #define EDWARDS_PP_HPP_
use crate::algebra::curves::edwards::edwards_g1;
use crate::algebra::curves::edwards::edwards_g2;
use crate::algebra::curves::edwards::edwards_init;
use crate::algebra::curves::edwards::edwards_pairing;
use crate::algebra::curves::public_params;


// pub struct edwards_pp {

//     type Fp_type=edwards_Fr;
//     type G1_type=edwards_G1;
//     type G2_type=edwards_G2;
//     type G1_precomp_type=edwards_G1_precomp;
//     type G2_precomp_type=edwards_G2_precomp;
//     type Fq_type=edwards_Fq;
//     type Fqe_type=edwards_Fq3;
//     type Fqk_type=edwards_Fq6;
//     type GT_type=edwards_GT;

//     static let mut has_affine_pairing = false;

//     static pub fn  init_public_params();
//     static edwards_GT final_exponentiation(elt:&edwards_Fq6);
//     static edwards_G1_precomp precompute_G1(P:&edwards_G1);
//     static edwards_G2_precomp precompute_G2(Q:&edwards_G2);
//     static edwards_Fq6 miller_loop(prec_P:&edwards_G1_precomp,
//                                    prec_Q:&edwards_G2_precomp);
//     static edwards_Fq6 double_miller_loop(prec_P1:&edwards_G1_precomp,
//                                           prec_Q1:&edwards_G2_precomp,
//                                           prec_P2:&edwards_G1_precomp,
//                                           prec_Q2:&edwards_G2_precomp);
//     /* the following are used in test files */
//     static edwards_Fq6 pairing(P:&edwards_G1,
//                                Q:&edwards_G2);
//     static edwards_Fq6 reduced_pairing(P:&edwards_G1,
//                                        Q:&edwards_G2);
// };

// 
// 



// use crate::algebra::curves::edwards::edwards_pp;



// pub fn init_public_params()
// {
//     init_edwards_params();
// }

// edwards_GT edwards_pp::final_exponentiation(elt:&edwards_Fq6)
// {
//     return edwards_final_exponentiation(elt);
// }

// edwards_G1_precomp edwards_pp::precompute_G1(P:&edwards_G1)
// {
//     return edwards_precompute_G1(P);
// }

// edwards_G2_precomp edwards_pp::precompute_G2(Q:&edwards_G2)
// {
//     return edwards_precompute_G2(Q);
// }

// edwards_Fq6 edwards_pp::miller_loop(prec_P:&edwards_G1_precomp,
//                                     prec_Q:&edwards_G2_precomp)
// {
//     return edwards_miller_loop(prec_P, prec_Q);
// }

// edwards_Fq6 edwards_pp::double_miller_loop(prec_P1:&edwards_G1_precomp,
//                                            prec_Q1:&edwards_G2_precomp,
//                                            prec_P2:&edwards_G1_precomp,
//                                            prec_Q2:&edwards_G2_precomp)
// {
//     return edwards_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
// }

// edwards_Fq6 edwards_pp::pairing(P:&edwards_G1,
//                                 Q:&edwards_G2)
// {
//     return edwards_pairing(P, Q);
// }

// edwards_Fq6 edwards_pp::reduced_pairing(P:&edwards_G1,
//                                         Q:&edwards_G2)
// {
//     return edwards_reduced_pairing(P, Q);
// }

// 



pub struct edwards_pp;
impl PublicParamsTypefor for edwards_pp{

    type Fp_type=edwards_Fr;
    type G1_type=edwards_G1;
    type G2_type=edwards_G2;
    type G1_precomp_type=edwards_G1_precomp;
    type G2_precomp_type=edwards_G2_precomp;
    type Fq_type=edwards_Fq;
    type Fqe_type=edwards_Fq3;
    type Fqk_type=edwards_Fq6;
    type GT_type=edwards_GT;
}


impl<EC_ppT:PublicParamsType> PublicParams<EC_ppT> for edwards_pp{

    const has_affine_pairing:bool = false;

 fn init_public_params()
{
    init_edwards_params();
}

fn final_exponentiation(elt:&Self::Fqk)->Self::GT
{
     edwards_final_exponentiation(elt)
}

  fn precompute_G1(P:&Self::G1)->Self::G1_precomp

{
     edwards_precompute_G1(P)
}

  fn precompute_G2(Q:&Self::G2)->Self::G2_precomp

{
     edwards_precompute_G2(Q)
}

  fn affine_ate_precompute_G1(P:&Self::G1)->Self::affine_ate_G1_precomp{
        unimplemented!("edwards_affine_ate_precompute_G1");
    }
  fn affine_ate_precompute_G2(Q:&Self::G2)->Self::affine_ate_G2_precomp{
        unimplemented!("edwards_affine_ate_precompute_G2");
    }


  fn affine_ate_miller_loop(prec_P:&Self::affine_ate_G1_precomp,
                                     prec_Q:&Self::affine_ate_G2_precomp)->Self::Fqk{
        unimplemented!("edwards_affine_ate_miller_loop");
    }
  fn affine_ate_e_over_e_miller_loop(prec_P1:&Self::affine_ate_G1_precomp,
                                              prec_Q1:&Self::affine_ate_G2_precomp,
                                              prec_P2:&Self::affine_ate_G1_precomp,
                                              prec_Q2:&Self::affine_ate_G2_precomp)->Self::Fqk{
        unimplemented!("edwards_affine_ate_miller_loop");
    }
  fn affine_ate_e_times_e_over_e_miller_loop(prec_P1:&Self::affine_ate_G1_precomp,
                                                      prec_Q1:&Self::affine_ate_G2_precomp,
                                                      prec_P2:&Self::affine_ate_G1_precomp,
                                                      prec_Q2:&Self::affine_ate_G2_precomp,
                                                      prec_P3:&Self::affine_ate_G1_precomp,
                                                      prec_Q3:&Self::affine_ate_G2_precomp)->Self::Fqk{
        unimplemented!("edwards_affine_ate_e_times_e_over_e_miller_loop");
    }

fn miller_loop(prec_P:&Self::G1_precomp,
                          prec_Q:&Self::G2_precomp)->Self::Fqk
{
     edwards_miller_loop(prec_P, prec_Q)
}

fn double_miller_loop(prec_P1:&Self::G1_precomp,
                                 prec_Q1:&Self::G2_precomp,
                                 prec_P2:&Self::G1_precomp,
                                 prec_Q2:&Self::G2_precomp)->Self::Fqk
{
     edwards_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2)
}

fn pairing(P:&Self::G1,
                      Q:&Self::G2)->Self::Fqk
{
     edwards_pairing(P, Q)
}

fn reduced_pairing(P:&Self::G1,
                             Q:&Self::G2)->Self::GT
{
     edwards_reduced_pairing(P, Q)
}
  fn affine_reduced_pairing(P:&Self::G1,
                                    Q:&Self::G2)->Self::GT{
        unimplemented!("edwards");
    }

}
