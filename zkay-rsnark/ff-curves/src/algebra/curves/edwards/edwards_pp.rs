use crate::{
    algebra::curves::edwards::{
        edwards_fields::{edwards_Fq, edwards_Fq3, edwards_Fq6, edwards_Fr, edwards_GT},
        edwards_g1::edwards_G1,
        edwards_g2::edwards_G2,
        edwards_init::init_edwards_params,
        edwards_pairing::{
            edwards_G1_precomp, edwards_G2_precomp, edwards_double_miller_loop,
            edwards_final_exponentiation, edwards_miller_loop, edwards_pairing,
            edwards_precompute_G1, edwards_precompute_G2, edwards_reduced_pairing,
        },
    },
    {CoeffsConfig, PublicParams, PublicParamsType, affine_ate_G_precomp_typeConfig},
};

#[derive(Default, Clone)]
pub struct edwards_pp;
impl PublicParamsType for edwards_pp {
    type Fp_type = edwards_Fr;
    type G1_type = edwards_G1;
    type G2_type = edwards_G2;
    type G1_precomp_type = edwards_G1_precomp;
    type G2_precomp_type = edwards_G2_precomp;
    type Fq_type = edwards_Fq;
    type Fqe_type = edwards_Fq3;
    type Fqk_type = edwards_Fq6;
    type GT_type = edwards_GT;
    type affine_ate_G1_precomp_type = Dummy;
    type affine_ate_G2_precomp_type = Dummy;
}

#[derive(Default, Clone)]
pub struct Dummy;
impl affine_ate_G_precomp_typeConfig for Dummy {
    type CC = Self;
}
impl CoeffsConfig for Dummy {}

impl PublicParams for edwards_pp {
    type Fr = edwards_Fr;
    type G1 = edwards_G1;
    type G2 = edwards_G2;
    type GT = edwards_GT;
    type affine_ate_G1_precomp = Dummy;
    type affine_ate_G2_precomp = Dummy;
    const has_affine_pairing: bool = false;

    fn init_public_params() {
        init_edwards_params();
    }

    fn final_exponentiation(elt: &Self::Fqk) -> Self::GT {
        edwards_final_exponentiation(elt)
    }

    fn precompute_G1(P: &Self::G1) -> Self::G1_precomp {
        edwards_precompute_G1(P)
    }

    fn precompute_G2(Q: &Self::G2) -> Self::G2_precomp {
        edwards_precompute_G2(Q)
    }

    fn affine_ate_precompute_G1(P: &Self::G1) -> Self::affine_ate_G1_precomp {
        unimplemented!("edwards_affine_ate_precompute_G1");
    }
    fn affine_ate_precompute_G2(Q: &Self::G2) -> Self::affine_ate_G2_precomp {
        unimplemented!("edwards_affine_ate_precompute_G2");
    }

    fn affine_ate_miller_loop(
        prec_P: &Self::affine_ate_G1_precomp,
        prec_Q: &Self::affine_ate_G2_precomp,
    ) -> Self::Fqk {
        unimplemented!("edwards_affine_ate_miller_loop");
    }
    fn affine_ate_e_over_e_miller_loop(
        prec_P1: &Self::affine_ate_G1_precomp,
        prec_Q1: &Self::affine_ate_G2_precomp,
        prec_P2: &Self::affine_ate_G1_precomp,
        prec_Q2: &Self::affine_ate_G2_precomp,
    ) -> Self::Fqk {
        unimplemented!("edwards_affine_ate_miller_loop");
    }
    fn affine_ate_e_times_e_over_e_miller_loop(
        prec_P1: &Self::affine_ate_G1_precomp,
        prec_Q1: &Self::affine_ate_G2_precomp,
        prec_P2: &Self::affine_ate_G1_precomp,
        prec_Q2: &Self::affine_ate_G2_precomp,
        prec_P3: &Self::affine_ate_G1_precomp,
        prec_Q3: &Self::affine_ate_G2_precomp,
    ) -> Self::Fqk {
        unimplemented!("edwards_affine_ate_e_times_e_over_e_miller_loop");
    }

    fn miller_loop(prec_P: &Self::G1_precomp, prec_Q: &Self::G2_precomp) -> Self::Fqk {
        edwards_miller_loop(prec_P, prec_Q)
    }

    fn double_miller_loop(
        prec_P1: &Self::G1_precomp,
        prec_Q1: &Self::G2_precomp,
        prec_P2: &Self::G1_precomp,
        prec_Q2: &Self::G2_precomp,
    ) -> Self::Fqk {
        edwards_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2)
    }

    fn pairing(P: &Self::G1, Q: &Self::G2) -> Self::Fqk {
        edwards_pairing(P, Q)
    }

    fn reduced_pairing(P: &Self::G1, Q: &Self::G2) -> Self::GT {
        edwards_reduced_pairing(P, Q)
    }
    fn affine_reduced_pairing(P: &Self::G1, Q: &Self::G2) -> Self::GT {
        unimplemented!("edwards");
    }
}
