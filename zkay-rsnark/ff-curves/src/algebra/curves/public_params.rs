/*
  for every curve the user should define corresponding
  public_params with the following typedefs:

  Fp_type
  G1_type
  G2_type
  G1_precomp_type
  G2_precomp_type
  affine_ate_G1_precomp_type
  affine_ate_G2_precomp_type
  Fq_type
  Fqe_type
  Fqk_type
  GT_type

  one should also define the following static methods:

  pub fn  init_public_params();

  GT<EC_ppT> final_exponentiation(elt:&Fqk<EC_ppT>);

  G1_precomp<EC_ppT> precompute_G1(P:&G1<EC_ppT>);
  G2_precomp<EC_ppT> precompute_G2(Q:&G2<EC_ppT>);

  Fqk<EC_ppT> miller_loop(prec_P:&G1_precomp<EC_ppT>,
                          prec_Q:&G2_precomp<EC_ppT>);

  affine_ate_G1_precomp<EC_ppT> affine_ate_precompute_G1(P:&G1<EC_ppT>);
  affine_ate_G2_precomp<EC_ppT> affine_ate_precompute_G2(Q:&G2<EC_ppT>);


  Fqk<EC_ppT> affine_ate_miller_loop(prec_P:&affine_ate_G1_precomp<EC_ppT>,
                                     prec_Q:&affine_ate_G2_precomp<EC_ppT>);
  Fqk<EC_ppT> affine_ate_e_over_e_miller_loop(prec_P1:&affine_ate_G1_precomp<EC_ppT>,
                                              prec_Q1:&affine_ate_G2_precomp<EC_ppT>,
                                              prec_P2:&affine_ate_G1_precomp<EC_ppT>,
                                              prec_Q2:&affine_ate_G2_precomp<EC_ppT>);
  Fqk<EC_ppT> affine_ate_e_times_e_over_e_miller_loop(prec_P1:&affine_ate_G1_precomp<EC_ppT>,
                                                      prec_Q1:&affine_ate_G2_precomp<EC_ppT>,
                                                      prec_P2:&affine_ate_G1_precomp<EC_ppT>,
                                                      prec_Q2:&affine_ate_G2_precomp<EC_ppT>,
                                                      prec_P3:&affine_ate_G1_precomp<EC_ppT>,
                                                      prec_Q3:&affine_ate_G2_precomp<EC_ppT>);
  Fqk<EC_ppT> double_miller_loop(prec_P1:&G1_precomp<EC_ppT>,
                                 prec_Q1:&G2_precomp<EC_ppT>,
                                 prec_P2:&G1_precomp<EC_ppT>,
                                 prec_Q2:&G2_precomp<EC_ppT>);

  Fqk<EC_ppT> pairing(P:&G1<EC_ppT>,
                      Q:&G2<EC_ppT>);
  GT<EC_ppT> reduced_pairing(P:&G1<EC_ppT>,
                             Q:&G2<EC_ppT>);
  GT<EC_ppT> affine_reduced_pairing(P:&G1<EC_ppT>,
                                    Q:&G2<EC_ppT>);
*/
use ffec::FieldTConfig;
use ffec::One;
use ffec::PpConfig;
use ffec::algebra::scalar_multiplication::multiexp;
use ffec::field_utils::BigInteger;
use ffec::field_utils::bigint::bigint;
use std::ops::Mul;
pub trait PublicParamsType: Default + Clone {
    type Fp_type: FieldTConfig;
    type G1_type: PpConfig;
    type G2_type: PpConfig;
    type G1_precomp_type: std::fmt::Display + Default + Clone + PartialEq;
    type G2_precomp_type: std::fmt::Display + Default + Clone + PartialEq;
    type affine_ate_G1_precomp_type = ();
    type affine_ate_G2_precomp_type = ();
    type Fq_type: PpConfig;
    type Fqe_type: PpConfig;
    type Fqk_type: PpConfig;
    type GT_type: PpConfig;
    const N: usize = 4;
}
//where <Self as PublicParamsType>::G1_type: Mul<<Self as PublicParams>::G2,Output=<Self as PublicParams>::G2>,<Self as PublicParamsType>::G2_type: Mul<<Self as PublicParams>::G1,Output=<Self as PublicParams>::G1>
// +Mul<Self::G2,Output=Self::G1>+Mul<Self::Fr,Output=Self::G1>
// +Mul<Self::G1,Output=Self::G2>
pub trait PublicParams: PublicParamsType {
    type Fr: FieldTConfig = Self::Fp_type;
    type G1: PpConfig = Self::G1_type;
    type G2: PpConfig = Self::G2_type;
    type G1_precomp: std::fmt::Display + Default + Clone + PartialEq = Self::G1_precomp_type;
    type G2_precomp: std::fmt::Display + Default + Clone + PartialEq = Self::G2_precomp_type;
    type affine_ate_G1_precomp = Self::affine_ate_G1_precomp_type;
    type affine_ate_G2_precomp = Self::affine_ate_G2_precomp_type;
    type Fq: PpConfig = Self::Fq_type;
    type Fqe: PpConfig = Self::Fqe_type;
    type Fqk: PpConfig = Self::Fqk_type;
    type GT: PpConfig = Self::GT_type;
    type Fr_vector = Vec<Self::Fr>;
    type G1_vector = Vec<Self::G1>;
    type G2_vector = Vec<Self::G2>;
    const has_affine_pairing: bool = false;

    fn init_public_params();

    fn final_exponentiation(elt: &Fqk<Self>) -> GT<Self>;

    fn precompute_G1(P: &G1<Self>) -> G1_precomp<Self>;
    fn precompute_G2(Q: &G2<Self>) -> G2_precomp<Self>;

    fn miller_loop(prec_P: &G1_precomp<Self>, prec_Q: &G2_precomp<Self>) -> Fqk<Self>;

    fn affine_ate_precompute_G1(P: &G1<Self>) -> affine_ate_G1_precomp<Self>;
    fn affine_ate_precompute_G2(Q: &G2<Self>) -> affine_ate_G2_precomp<Self>;

    fn affine_ate_miller_loop(
        prec_P: &affine_ate_G1_precomp<Self>,
        prec_Q: &affine_ate_G2_precomp<Self>,
    ) -> Fqk<Self>;
    fn affine_ate_e_over_e_miller_loop(
        prec_P1: &affine_ate_G1_precomp<Self>,
        prec_Q1: &affine_ate_G2_precomp<Self>,
        prec_P2: &affine_ate_G1_precomp<Self>,
        prec_Q2: &affine_ate_G2_precomp<Self>,
    ) -> Fqk<Self>;
    fn affine_ate_e_times_e_over_e_miller_loop(
        prec_P1: &affine_ate_G1_precomp<Self>,
        prec_Q1: &affine_ate_G2_precomp<Self>,
        prec_P2: &affine_ate_G1_precomp<Self>,
        prec_Q2: &affine_ate_G2_precomp<Self>,
        prec_P3: &affine_ate_G1_precomp<Self>,
        prec_Q3: &affine_ate_G2_precomp<Self>,
    ) -> Fqk<Self>;
    fn double_miller_loop(
        prec_P1: &G1_precomp<Self>,
        prec_Q1: &G2_precomp<Self>,
        prec_P2: &G1_precomp<Self>,
        prec_Q2: &G2_precomp<Self>,
    ) -> Fqk<Self>;

    fn pairing(P: &G1<Self>, Q: &G2<Self>) -> Fqk<Self>;
    fn reduced_pairing(P: &G1<Self>, Q: &G2<Self>) -> GT<Self>;
    fn affine_reduced_pairing(P: &G1<Self>, Q: &G2<Self>) -> GT<Self>;
}

pub type Fr<Ppt> = <Ppt as PublicParams>::Fr;
pub type G1<Ppt> = <Ppt as PublicParams>::G1;
pub type G2<Ppt> = <Ppt as PublicParams>::G2;
pub type G1_precomp<Ppt> = <Ppt as PublicParams>::G1_precomp;
pub type G2_precomp<Ppt> = <Ppt as PublicParams>::G2_precomp;
pub type affine_ate_G1_precomp<Ppt> = <Ppt as PublicParams>::affine_ate_G1_precomp;
pub type affine_ate_G2_precomp<Ppt> = <Ppt as PublicParams>::affine_ate_G2_precomp;
pub type Fq<Ppt> = <Ppt as PublicParams>::Fq;
pub type Fqe<Ppt> = <Ppt as PublicParams>::Fqe;
pub type Fqk<Ppt> = <Ppt as PublicParams>::Fqk;
pub type GT<Ppt> = <Ppt as PublicParams>::GT;

pub type Fr_vector<Ppt> = Vec<Fr<Ppt>>;
pub type G1_vector<Ppt> = Vec<G1<Ppt>>;
pub type G2_vector<Ppt> = Vec<G2<Ppt>>;
