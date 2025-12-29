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

use ffec::One;
use ffec::algebra::scalar_multiplication::multiexp;
use ffec::field_utils::BigInteger;
use ffec::field_utils::bigint::bigint;
use ffec::scalar_multiplication::multiexp::AsBigint;

pub trait KCConfig:
    Default
    + Clone
    + One
    + AsBigint
    + std::ops::Add<Output = Self>
    + std::ops::Mul<Self::T, Output = Self>
{
    type T: AsRef<[u64]>;
    fn zero() -> Self;
    fn mixed_add(&self, other: &Self) -> Self;
    fn is_special(&self) -> bool;
    fn print(&self);
    fn size_in_bits() -> usize;
}

pub trait PublicParamsType {
    type Fp_type;
    type G1_type: KCConfig;
    type G2_type: KCConfig;
    type G1_precomp_type;
    type G2_precomp_type;
    type affine_ate_G1_precomp_type = ();
    type affine_ate_G2_precomp_type = ();
    type Fq_type;
    type Fqe_type;
    type Fqk_type;
    type GT_type;
    const N: usize = 4;
}

pub trait PublicParams: PublicParamsType {
    type Fr = Self::Fp_type;
    type G1 = Self::G1_type;
    type G2 = Self::G2_type;
    type G1_precomp = Self::G1_precomp_type;
    type G2_precomp = Self::G2_precomp_type;
    type affine_ate_G1_precomp = Self::affine_ate_G1_precomp_type;
    type affine_ate_G2_precomp = Self::affine_ate_G2_precomp_type;
    type Fq = Self::Fq_type;
    type Fqe = Self::Fqe_type;
    type Fqk = Self::Fqk_type;
    type GT = Self::GT_type;
    type Fr_vector = Vec<Self::Fr>;
    type G1_vector = Vec<Self::G1>;
    type G2_vector = Vec<Self::G2>;
    const has_affine_pairing: bool = false;

    fn init_public_params();

    fn final_exponentiation(elt: &Self::Fqk) -> Self::GT;

    fn precompute_G1(P: &Self::G1) -> Self::G1_precomp;
    fn precompute_G2(Q: &Self::G2) -> Self::G2_precomp;

    fn miller_loop(prec_P: &Self::G1_precomp, prec_Q: &Self::G2_precomp) -> Self::Fqk;

    fn affine_ate_precompute_G1(P: &Self::G1) -> Self::affine_ate_G1_precomp;
    fn affine_ate_precompute_G2(Q: &Self::G2) -> Self::affine_ate_G2_precomp;

    fn affine_ate_miller_loop(
        prec_P: &Self::affine_ate_G1_precomp,
        prec_Q: &Self::affine_ate_G2_precomp,
    ) -> Self::Fqk;
    fn affine_ate_e_over_e_miller_loop(
        prec_P1: &Self::affine_ate_G1_precomp,
        prec_Q1: &Self::affine_ate_G2_precomp,
        prec_P2: &Self::affine_ate_G1_precomp,
        prec_Q2: &Self::affine_ate_G2_precomp,
    ) -> Self::Fqk;
    fn affine_ate_e_times_e_over_e_miller_loop(
        prec_P1: &Self::affine_ate_G1_precomp,
        prec_Q1: &Self::affine_ate_G2_precomp,
        prec_P2: &Self::affine_ate_G1_precomp,
        prec_Q2: &Self::affine_ate_G2_precomp,
        prec_P3: &Self::affine_ate_G1_precomp,
        prec_Q3: &Self::affine_ate_G2_precomp,
    ) -> Self::Fqk;
    fn double_miller_loop(
        prec_P1: &Self::G1_precomp,
        prec_Q1: &Self::G2_precomp,
        prec_P2: &Self::G1_precomp,
        prec_Q2: &Self::G2_precomp,
    ) -> Self::Fqk;

    fn pairing(P: &Self::G1, Q: &Self::G2) -> Self::Fqk;
    fn reduced_pairing(P: &Self::G1, Q: &Self::G2) -> Self::GT;
    fn affine_reduced_pairing(P: &Self::G1, Q: &Self::G2) -> Self::GT;
}

pub type Fr<Ppt> = <Ppt as PublicParamsType>::Fp_type;
pub type G1<Ppt> = <Ppt as PublicParamsType>::G1_type;
pub type G2<Ppt> = <Ppt as PublicParamsType>::G2_type;
pub type G1_precomp<Ppt> = <Ppt as PublicParamsType>::G1_precomp_type;
pub type G2_precomp<Ppt> = <Ppt as PublicParamsType>::G2_precomp_type;
pub type affine_ate_G1_precomp<Ppt> = <Ppt as PublicParamsType>::affine_ate_G1_precomp_type;
pub type affine_ate_G2_precomp<Ppt> = <Ppt as PublicParamsType>::affine_ate_G2_precomp_type;
pub type Fq<Ppt> = <Ppt as PublicParamsType>::Fq_type;
pub type Fqe<Ppt> = <Ppt as PublicParamsType>::Fqe_type;
pub type Fqk<Ppt> = <Ppt as PublicParamsType>::Fqk_type;
pub type GT<Ppt> = <Ppt as PublicParamsType>::GT_type;

pub type Fr_vector<Ppt> = Vec<<Ppt as PublicParams>::Fr>;
pub type G1_vector<Ppt> = Vec<<Ppt as PublicParams>::G1>;
pub type G2_vector<Ppt> = Vec<<Ppt as PublicParams>::G2>;
