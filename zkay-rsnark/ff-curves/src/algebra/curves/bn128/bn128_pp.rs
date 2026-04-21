use crate::{
    PublicParams, PublicParamsType,
    algebra::curves::{
        alt_bn128::alt_bn128_pp::affine_ate_precomp,
        bn128::{
            bn128_fields::{bn128_Fq, bn128_Fq2, bn128_Fq12, bn128_Fr},
            bn128_g1::bn128_G1,
            bn128_g2::bn128_G2,
            bn128_gt::bn128_GT,
            bn128_init::init_bn128_params,
            bn128_pairing::{
                bn128_ate_G1_precomp, bn128_ate_G2_precomp, bn128_ate_miller_loop,
                bn128_ate_precompute_G1, bn128_ate_precompute_G2, bn128_double_ate_miller_loop,
                bn128_final_exponentiation,
            },
        },
    },
};
use ffec::common::profiling::{enter_block, leave_block};

#[derive(Default, Clone)]
pub struct bn128_pp;
impl PublicParamsType for bn128_pp {
    type Fp_type = bn128_Fr;
    type G1_type = bn128_G1;
    type G2_type = bn128_G2;
    type G1_precomp_type = bn128_ate_G1_precomp;
    type G2_precomp_type = bn128_ate_G2_precomp;
    type Fq_type = bn128_Fq;
    type Fqe_type = bn128_Fq2;
    type Fqk_type = bn128_Fq12;
    type GT_type = bn128_GT;
    type affine_ate_G1_precomp_type = affine_ate_precomp;
    type affine_ate_G2_precomp_type = affine_ate_precomp;
}

impl PublicParams for bn128_pp {
    type Fr = bn128_Fr;
    type G1 = bn128_G1;
    type G2 = bn128_G2;
    type GT = bn128_GT;
    type affine_ate_G1_precomp = affine_ate_precomp;
    type affine_ate_G2_precomp = affine_ate_precomp;
    const has_affine_pairing: bool = false;

    fn init_public_params() {
        init_bn128_params();
    }

    fn final_exponentiation(elt: &Self::Fqk) -> Self::GT {
        bn128_final_exponentiation(elt)
    }

    fn precompute_G1(P: &Self::G1) -> Self::G1_precomp {
        bn128_ate_precompute_G1(P)
    }

    fn precompute_G2(Q: &Self::G2) -> Self::G2_precomp {
        bn128_ate_precompute_G2(Q)
    }

    fn affine_ate_precompute_G1(P: &Self::G1) -> Self::affine_ate_G1_precomp {
        unimplemented!("bn128_affine_ate_precompute_G1");
    }
    fn affine_ate_precompute_G2(Q: &Self::G2) -> Self::affine_ate_G2_precomp {
        unimplemented!("bn128_affine_ate_precompute_G2");
    }

    fn affine_ate_miller_loop(
        prec_P: &Self::affine_ate_G1_precomp,
        prec_Q: &Self::affine_ate_G2_precomp,
    ) -> Self::Fqk {
        unimplemented!("bn128_affine_ate_miller_loop");
    }
    fn affine_ate_e_over_e_miller_loop(
        prec_P1: &Self::affine_ate_G1_precomp,
        prec_Q1: &Self::affine_ate_G2_precomp,
        prec_P2: &Self::affine_ate_G1_precomp,
        prec_Q2: &Self::affine_ate_G2_precomp,
    ) -> Self::Fqk {
        unimplemented!("bn128_affine_ate_miller_loop");
    }
    fn affine_ate_e_times_e_over_e_miller_loop(
        prec_P1: &Self::affine_ate_G1_precomp,
        prec_Q1: &Self::affine_ate_G2_precomp,
        prec_P2: &Self::affine_ate_G1_precomp,
        prec_Q2: &Self::affine_ate_G2_precomp,
        prec_P3: &Self::affine_ate_G1_precomp,
        prec_Q3: &Self::affine_ate_G2_precomp,
    ) -> Self::Fqk {
        unimplemented!("bn128_affine_ate_e_times_e_over_e_miller_loop");
    }

    fn miller_loop(prec_P: &Self::G1_precomp, prec_Q: &Self::G2_precomp) -> Self::Fqk {
        enter_block("Call to miller_loop<bn128_pp>", false);
        let result = bn128_ate_miller_loop(prec_P, prec_Q);
        leave_block("Call to miller_loop<bn128_pp>", false);
        result
    }

    fn double_miller_loop(
        prec_P1: &Self::G1_precomp,
        prec_Q1: &Self::G2_precomp,
        prec_P2: &Self::G1_precomp,
        prec_Q2: &Self::G2_precomp,
    ) -> Self::Fqk {
        enter_block("Call to double_miller_loop<bn128_pp>", false);
        let result = bn128_double_ate_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
        leave_block("Call to double_miller_loop<bn128_pp>", false);
        result
    }

    fn pairing(P: &Self::G1, Q: &Self::G2) -> Self::Fqk {
        enter_block("Call to pairing<bn128_pp>", false);
        let prec_P = bn128_pp::precompute_G1(P);
        let prec_Q = bn128_pp::precompute_G2(Q);

        let result = bn128_pp::miller_loop(&prec_P, &prec_Q);
        leave_block("Call to pairing<bn128_pp>", false);
        result
    }

    fn reduced_pairing(P: &Self::G1, Q: &Self::G2) -> Self::GT {
        enter_block("Call to reduced_pairing<bn128_pp>", false);
        let f = bn128_pp::pairing(P, Q);
        let result = bn128_pp::final_exponentiation(&f);
        leave_block("Call to reduced_pairing<bn128_pp>", false);
        result
    }
    fn affine_reduced_pairing(P: &Self::G1, Q: &Self::G2) -> Self::GT {
        unimplemented!("bn128");
    }
}
