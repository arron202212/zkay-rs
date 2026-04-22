//  Declaration of interfaces for public parameters of MNT6.

use crate::{
    PublicParams, PublicParamsType,
    algebra::curves::mnt::mnt6::{
        mnt6_fields::{mnt6_Fq, mnt6_Fq3, mnt6_Fq6, mnt6_Fr, mnt6_GT},
        mnt6_g1::mnt6_G1,
        mnt6_g2::mnt6_G2,
        mnt6_init::init_mnt6_params,
        mnt6_pairing::{
            mnt6_G1_precomp, mnt6_G2_precomp, mnt6_affine_ate_G1_precomputation,
            mnt6_affine_ate_G2_precomputation, mnt6_affine_ate_miller_loop,
            mnt6_affine_ate_precompute_G1, mnt6_affine_ate_precompute_G2,
            mnt6_affine_reduced_pairing, mnt6_double_miller_loop, mnt6_final_exponentiation,
            mnt6_miller_loop, mnt6_pairing, mnt6_precompute_G1, mnt6_precompute_G2,
            mnt6_reduced_pairing,
        },
    },
};

#[derive(Default, Clone)]
pub struct mnt6_pp;
impl PublicParamsType for mnt6_pp {
    type Fp_type = mnt6_Fr;
    type G1_type = mnt6_G1;
    type G2_type = mnt6_G2;
    type affine_ate_G1_precomp_type = mnt6_affine_ate_G1_precomputation;
    type affine_ate_G2_precomp_type = mnt6_affine_ate_G2_precomputation;
    type G1_precomp_type = mnt6_G1_precomp;
    type G2_precomp_type = mnt6_G2_precomp;
    type Fq_type = mnt6_Fq;
    type Fqe_type = mnt6_Fq3;
    type Fqk_type = mnt6_Fq6;
    type GT_type = mnt6_GT;
}

impl PublicParams for mnt6_pp {
    type Fr = mnt6_Fr;
    type G1 = mnt6_G1;
    type G2 = mnt6_G2;
    type GT = mnt6_GT;
    type affine_ate_G1_precomp = mnt6_affine_ate_G1_precomputation;
    type affine_ate_G2_precomp = mnt6_affine_ate_G2_precomputation;
    fn init_public_params() {
        init_mnt6_params();
    }

    fn final_exponentiation(elt: &mnt6_Fq6) -> mnt6_GT {
        return mnt6_final_exponentiation(elt);
    }

    fn precompute_G1(P: &mnt6_G1) -> mnt6_G1_precomp {
        return mnt6_precompute_G1(P);
    }

    fn precompute_G2(Q: &mnt6_G2) -> mnt6_G2_precomp {
        return mnt6_precompute_G2(Q);
    }

    fn miller_loop(prec_P: &mnt6_G1_precomp, prec_Q: &mnt6_G2_precomp) -> mnt6_Fq6 {
        return mnt6_miller_loop(prec_P, prec_Q);
    }

    fn affine_ate_precompute_G1(P: &mnt6_G1) -> mnt6_affine_ate_G1_precomputation {
        return mnt6_affine_ate_precompute_G1(P);
    }

    fn affine_ate_precompute_G2(Q: &mnt6_G2) -> mnt6_affine_ate_G2_precomputation {
        return mnt6_affine_ate_precompute_G2(Q);
    }

    fn affine_ate_miller_loop(
        prec_P: &mnt6_affine_ate_G1_precomputation,
        prec_Q: &mnt6_affine_ate_G2_precomputation,
    ) -> mnt6_Fq6 {
        return mnt6_affine_ate_miller_loop(prec_P, prec_Q);
    }

    fn double_miller_loop(
        prec_P1: &mnt6_G1_precomp,
        prec_Q1: &mnt6_G2_precomp,
        prec_P2: &mnt6_G1_precomp,
        prec_Q2: &mnt6_G2_precomp,
    ) -> mnt6_Fq6 {
        return mnt6_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
    }

    fn affine_ate_e_over_e_miller_loop(
        prec_P1: &mnt6_affine_ate_G1_precomputation,
        prec_Q1: &mnt6_affine_ate_G2_precomputation,
        prec_P2: &mnt6_affine_ate_G1_precomputation,
        prec_Q2: &mnt6_affine_ate_G2_precomputation,
    ) -> mnt6_Fq6 {
        return mnt6_affine_ate_miller_loop(prec_P1, prec_Q1)
            * mnt6_affine_ate_miller_loop(prec_P2, prec_Q2).unitary_inverse();
    }

    fn affine_ate_e_times_e_over_e_miller_loop(
        prec_P1: &mnt6_affine_ate_G1_precomputation,
        prec_Q1: &mnt6_affine_ate_G2_precomputation,
        prec_P2: &mnt6_affine_ate_G1_precomputation,
        prec_Q2: &mnt6_affine_ate_G2_precomputation,
        prec_P3: &mnt6_affine_ate_G1_precomputation,
        prec_Q3: &mnt6_affine_ate_G2_precomputation,
    ) -> mnt6_Fq6 {
        return ((mnt6_affine_ate_miller_loop(prec_P1, prec_Q1)
            * mnt6_affine_ate_miller_loop(prec_P2, prec_Q2))
            * mnt6_affine_ate_miller_loop(prec_P3, prec_Q3).unitary_inverse());
    }

    fn pairing(P: &mnt6_G1, Q: &mnt6_G2) -> mnt6_Fq6 {
        return mnt6_pairing(P, Q);
    }

    fn reduced_pairing(P: &mnt6_G1, Q: &mnt6_G2) -> mnt6_Fq6 {
        return mnt6_reduced_pairing(P, Q);
    }

    fn affine_reduced_pairing(P: &mnt6_G1, Q: &mnt6_G2) -> mnt6_Fq6 {
        return mnt6_affine_reduced_pairing(P, Q);
    }
}
