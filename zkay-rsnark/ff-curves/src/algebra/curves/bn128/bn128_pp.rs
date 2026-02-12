



// #define BN128_PP_HPP_
use crate::algebra::curves::bn128::bn128_g1;
use crate::algebra::curves::bn128::bn128_g2;
use crate::algebra::curves::bn128::bn128_gt;
use crate::algebra::curves::bn128::bn128_init;
use crate::algebra::curves::bn128::bn128_pairing;
use crate::algebra::curves::public_params;



// pub struct bn128_pp {

//     type Fp_type=bn128_Fr;
//     type G1_type=bn128_G1;
//     type G2_type=bn128_G2;
//     type G1_precomp_type=bn128_ate_G1_precomp;
//     type G2_precomp_type=bn128_ate_G2_precomp;
//     type Fq_type=bn128_Fq;
//     type Fqk_type=bn128_Fq12;
//     type GT_type=bn128_GT;

//     static let mut has_affine_pairing = false;

//     static pub fn  init_public_params();
//     static bn128_GT final_exponentiation(elt:&bn128_Fq12);
//     static bn128_ate_G1_precomp precompute_G1(P:&bn128_G1);
//     static bn128_ate_G2_precomp precompute_G2(Q:&bn128_G2);
//     static bn128_Fq12 miller_loop(prec_P:&bn128_ate_G1_precomp,
//                                   prec_Q:&bn128_ate_G2_precomp);
//     static bn128_Fq12 double_miller_loop(prec_P1:&bn128_ate_G1_precomp,
//                                          prec_Q1:&bn128_ate_G2_precomp,
//                                          prec_P2:&bn128_ate_G1_precomp,
//                                          prec_Q2:&bn128_ate_G2_precomp);

//     /* the following are used in test files */
//     static bn128_GT pairing(P:&bn128_G1,
//                             Q:&bn128_G2);
//     static bn128_GT reduced_pairing(P:&bn128_G1,
//                                     Q:&bn128_G2);
// };

// 
// 



// use crate::algebra::curves::bn128::bn128_pp;
// use crate::common::profiling;



// pub fn init_public_params()
// {
//     init_bn128_params();
// }

// bn128_GT bn128_pp::final_exponentiation(elt:&bn128_GT)
// {
//     return bn128_final_exponentiation(elt);
// }

// bn128_ate_G1_precomp bn128_pp::precompute_G1(P:&bn128_G1)
// {
//     return bn128_ate_precompute_G1(P);
// }

// bn128_ate_G2_precomp bn128_pp::precompute_G2(Q:&bn128_G2)
// {
//     return bn128_ate_precompute_G2(Q);
// }

// bn128_Fq12 bn128_pp::miller_loop(prec_P:&bn128_ate_G1_precomp,
//                                  prec_Q:&bn128_ate_G2_precomp)
// {
//     enter_block("Call to miller_loop<bn128_pp>");
//     bn128_Fq12 result = bn128_ate_miller_loop(prec_P, prec_Q);
//     leave_block("Call to miller_loop<bn128_pp>");
//     return result;
// }

// bn128_Fq12 bn128_pp::double_miller_loop(prec_P1:&bn128_ate_G1_precomp,
//                                         prec_Q1:&bn128_ate_G2_precomp,
//                                         prec_P2:&bn128_ate_G1_precomp,
//                                         prec_Q2:&bn128_ate_G2_precomp)
// {
//     enter_block("Call to double_miller_loop<bn128_pp>");
//     bn128_Fq12 result = bn128_double_ate_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
//     leave_block("Call to double_miller_loop<bn128_pp>");
//     return result;
// }

// bn128_Fq12 bn128_pp::pairing(P:&bn128_G1,
//                              Q:&bn128_G2)
// {
//     enter_block("Call to pairing<bn128_pp>");
//     bn128_ate_G1_precomp prec_P = bn128_pp::precompute_G1(P);
//     bn128_ate_G2_precomp prec_Q = bn128_pp::precompute_G2(Q);

//     bn128_Fq12 result = bn128_pp::miller_loop(prec_P, prec_Q);
//     leave_block("Call to pairing<bn128_pp>");
//     return result;
// }

// bn128_GT bn128_pp::reduced_pairing(P:&bn128_G1,
//                                    Q:&bn128_G2)
// {
//     enter_block("Call to reduced_pairing<bn128_pp>");
//     let f= bn128_pp::pairing(P, Q);
//     let result= bn128_pp::final_exponentiation(f);
//     leave_block("Call to reduced_pairing<bn128_pp>");
//     return result;
// }

// 



pub struct bn128_pp;
impl PublicParamsTypefor for bn128_pp{

    type Fp_type=bn128_Fr;
    type G1_type=bn128_G1;
    type G2_type=bn128_G2;
    type G1_precomp_type=bn128_G1_precomp;
    type G2_precomp_type=bn128_G2_precomp;
    type Fq_type=bn128_Fq;
    type Fqe_type=bn128_Fq2;
    type Fqk_type=bn128_Fq12;
    type GT_type=bn128_GT;
}


impl<EC_ppT:PublicParamsType> PublicParams<EC_ppT> for bn128_pp{

    const has_affine_pairing:bool = false;

 fn init_public_params()
{
    init_bn128_params();
}

fn final_exponentiation(elt:&Self::Fqk)->Self::GT
{
     bn128_final_exponentiation(elt)
}

  fn precompute_G1(P:&Self::G1)->Self::G1_precomp

{
     bn128_ate_precompute_G1(P)
}

  fn precompute_G2(Q:&Self::G2)->Self::G2_precomp

{
     bn128_ate_precompute_G2(Q)
}

  fn affine_ate_precompute_G1(P:&Self::G1)->Self::affine_ate_G1_precomp{
        unimplemented!("bn128_affine_ate_precompute_G1");
    }
  fn affine_ate_precompute_G2(Q:&Self::G2)->Self::affine_ate_G2_precomp{
        unimplemented!("bn128_affine_ate_precompute_G2");
    }


  fn affine_ate_miller_loop(prec_P:&Self::affine_ate_G1_precomp,
                                     prec_Q:&Self::affine_ate_G2_precomp)->Self::Fqk{
        unimplemented!("bn128_affine_ate_miller_loop");
    }
  fn affine_ate_e_over_e_miller_loop(prec_P1:&Self::affine_ate_G1_precomp,
                                              prec_Q1:&Self::affine_ate_G2_precomp,
                                              prec_P2:&Self::affine_ate_G1_precomp,
                                              prec_Q2:&Self::affine_ate_G2_precomp)->Self::Fqk{
        unimplemented!("bn128_affine_ate_miller_loop");
    }
  fn affine_ate_e_times_e_over_e_miller_loop(prec_P1:&Self::affine_ate_G1_precomp,
                                                      prec_Q1:&Self::affine_ate_G2_precomp,
                                                      prec_P2:&Self::affine_ate_G1_precomp,
                                                      prec_Q2:&Self::affine_ate_G2_precomp,
                                                      prec_P3:&Self::affine_ate_G1_precomp,
                                                      prec_Q3:&Self::affine_ate_G2_precomp)->Self::Fqk{
        unimplemented!("bn128_affine_ate_e_times_e_over_e_miller_loop");
    }

fn miller_loop(prec_P:&Self::G1_precomp,
                          prec_Q:&Self::G2_precomp)->Self::Fqk
{
       enter_block("Call to miller_loop<bn128_pp>");
    bn128_Fq12 result = bn128_ate_miller_loop(prec_P, prec_Q);
    leave_block("Call to miller_loop<bn128_pp>");
    return result;
}

fn double_miller_loop(prec_P1:&Self::G1_precomp,
                                 prec_Q1:&Self::G2_precomp,
                                 prec_P2:&Self::G1_precomp,
                                 prec_Q2:&Self::G2_precomp)->Self::Fqk
{
      enter_block("Call to double_miller_loop<bn128_pp>");
    bn128_Fq12 result = bn128_double_ate_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
    leave_block("Call to double_miller_loop<bn128_pp>");
    return result;
}

fn pairing(P:&Self::G1,
                      Q:&Self::G2)->Self::Fqk
{
      enter_block("Call to pairing<bn128_pp>");
    bn128_ate_G1_precomp prec_P = bn128_pp::precompute_G1(P);
    bn128_ate_G2_precomp prec_Q = bn128_pp::precompute_G2(Q);

    bn128_Fq12 result = bn128_pp::miller_loop(prec_P, prec_Q);
    leave_block("Call to pairing<bn128_pp>");
    return result;
}

fn reduced_pairing(P:&Self::G1,
                             Q:&Self::G2)->Self::GT
{
      enter_block("Call to reduced_pairing<bn128_pp>");
    let f= bn128_pp::pairing(P, Q);
    let result= bn128_pp::final_exponentiation(f);
    leave_block("Call to reduced_pairing<bn128_pp>");
    return result;
}
  fn affine_reduced_pairing(P:&Self::G1,
                                    Q:&Self::G2)->Self::GT{
        unimplemented!("bn128");
    }

}
