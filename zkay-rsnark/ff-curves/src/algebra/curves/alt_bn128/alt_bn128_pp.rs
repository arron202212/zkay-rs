// use crate::algebra::curves::alt_bn128::alt_bn128_g1;
// use crate::algebra::curves::alt_bn128::alt_bn128_g2;
use crate::CoeffsConfig;
use crate::affine_ate_G_precomp_typeConfig;
use crate::algebra::curves::alt_bn128::alt_bn128_fields::{
    alt_bn128_Fq, alt_bn128_Fq2, alt_bn128_Fq12, alt_bn128_GT,
};
use crate::algebra::curves::alt_bn128::alt_bn128_g2::alt_bn128_G2;
use crate::algebra::curves::alt_bn128::alt_bn128_init::init_alt_bn128_params;
use crate::algebra::curves::alt_bn128::alt_bn128_pairing::alt_bn128_precompute_G1;
use crate::algebra::curves::alt_bn128::alt_bn128_pairing::alt_bn128_precompute_G2;
use crate::algebra::curves::alt_bn128::alt_bn128_pairing::{
    alt_bn128_G1_precomp, alt_bn128_G2_precomp,
};
use crate::algebra::curves::alt_bn128::alt_bn128_pairing::{
    alt_bn128_double_miller_loop, alt_bn128_final_exponentiation, alt_bn128_miller_loop,
    alt_bn128_pairing, alt_bn128_reduced_pairing,
};
use crate::algebra::curves::public_params;

// namespace libff {

// impl<EC_ppT:PublicParamsType> PublicParams<EC_ppT> for alt_bn128_pp{

//     const has_affine_pairing:bool = false;

//     static pub fn  init_public_params();
//     static alt_bn128_GT final_exponentiation(elt:&alt_bn128_Fq12);
//     static alt_bn128_G1_precomp precompute_G1(P:&alt_bn128_G1);
//     static alt_bn128_G2_precomp precompute_G2(Q:&alt_bn128_G2);
//     static alt_bn128_Fq12 miller_loop(prec_P:&alt_bn128_G1_precomp,
//                                       prec_Q:&alt_bn128_G2_precomp);
//     static alt_bn128_Fq12 double_miller_loop(prec_P1:&alt_bn128_G1_precomp,
//                                              prec_Q1:&alt_bn128_G2_precomp,
//                                              prec_P2:&alt_bn128_G1_precomp,
//                                              prec_Q2:&alt_bn128_G2_precomp);
//     static alt_bn128_Fq12 pairing(P:&alt_bn128_G1,
//                                   Q:&alt_bn128_G2);
//     static alt_bn128_Fq12 reduced_pairing(P:&alt_bn128_G1,
//                                           Q:&alt_bn128_G2);
// }

// } // namespace libff

use crate::algebra::curves::alt_bn128::alt_bn128_fields::alt_bn128_Fr;

use crate::algebra::curves::alt_bn128::alt_bn128_g1::alt_bn128_G1;
use crate::{PublicParams, PublicParamsType};
use ffec::PpConfig;
use ffec::field_utils::bigint::bigint;
use ffec::{One, Zero};
use std::fmt;
use std::ops::{Add, Mul, Sub};
#[derive(Default, Clone, Debug, PartialEq)]
pub struct alt_bn128_pp;

impl Add for alt_bn128_pp {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut r = self;
        // r -= other;
        r
    }
}
impl Sub for alt_bn128_pp {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut r = self;
        // r -= other;
        r
    }
}
impl Mul for alt_bn128_pp {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}
impl Mul<&Self> for alt_bn128_pp {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}
impl Mul<bigint<4>> for alt_bn128_pp {
    type Output = Self;

    fn mul(self, rhs: bigint<4>) -> Self::Output {
        let mut r = self;
        // r *= *rhs.borrow();
        r
    }
}
impl One for alt_bn128_pp {
    fn one() -> Self {
        Self::default()
    }
}

impl Zero for alt_bn128_pp {
    fn zero() -> Self {
        Self::default()
    }
    fn is_zero(&self) -> bool {
        false
    }
}
impl std::fmt::Display for alt_bn128_pp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", 1)
    }
}
impl PpConfig for alt_bn128_pp {
    type TT = bigint<4>;
    // type Fr=Self;
}
#[derive(Clone, Default)]
pub struct Coeffs;
impl CoeffsConfig for Coeffs {
    // type FieldT=alt_bn128_Fr;
}
pub struct affine_ate_precomp;
impl affine_ate_G_precomp_typeConfig for affine_ate_precomp {
    type CC = Coeffs;
}
impl PublicParamsType for alt_bn128_pp {
    type Fp_type = alt_bn128_Fr;
    type G1_type = alt_bn128_G1;
    type G2_type = alt_bn128_G2;
    type G1_precomp_type = alt_bn128_G1_precomp;
    type G2_precomp_type = alt_bn128_G2_precomp;
    type Fq_type = alt_bn128_Fq;
    type Fqe_type = alt_bn128_Fq2;
    type Fqk_type = alt_bn128_Fq12;
    type GT_type = alt_bn128_GT;
    type affine_ate_G1_precomp_type = affine_ate_precomp;
    type affine_ate_G2_precomp_type = affine_ate_precomp;
}

impl PublicParams for alt_bn128_pp {
    type Fr = alt_bn128_Fr;
    type G1 = alt_bn128_G1;
    type G2 = alt_bn128_G2;
    type GT = alt_bn128_GT;
    type affine_ate_G1_precomp = affine_ate_precomp;
    type affine_ate_G2_precomp = affine_ate_precomp;
    const has_affine_pairing: bool = false;

    fn init_public_params() {
        init_alt_bn128_params();
    }

    fn final_exponentiation(elt: &Self::Fqk) -> Self::GT {
        alt_bn128_final_exponentiation(elt)
    }

    fn precompute_G1(P: &Self::G1) -> Self::G1_precomp {
        alt_bn128_precompute_G1(P)
    }

    fn precompute_G2(Q: &Self::G2) -> Self::G2_precomp {
        alt_bn128_precompute_G2(Q)
    }

    fn affine_ate_precompute_G1(P: &Self::G1) -> Self::affine_ate_G1_precomp {
        unimplemented!("alt_bn128_affine_ate_precompute_G1");
    }
    fn affine_ate_precompute_G2(Q: &Self::G2) -> Self::affine_ate_G2_precomp {
        unimplemented!("alt_bn128_affine_ate_precompute_G2");
    }

    fn affine_ate_miller_loop(
        prec_P: &Self::affine_ate_G1_precomp,
        prec_Q: &Self::affine_ate_G2_precomp,
    ) -> Self::Fqk {
        unimplemented!("alt_bn128_affine_ate_miller_loop");
    }
    fn affine_ate_e_over_e_miller_loop(
        prec_P1: &Self::affine_ate_G1_precomp,
        prec_Q1: &Self::affine_ate_G2_precomp,
        prec_P2: &Self::affine_ate_G1_precomp,
        prec_Q2: &Self::affine_ate_G2_precomp,
    ) -> Self::Fqk {
        unimplemented!("alt_bn128_affine_ate_miller_loop");
    }
    fn affine_ate_e_times_e_over_e_miller_loop(
        prec_P1: &Self::affine_ate_G1_precomp,
        prec_Q1: &Self::affine_ate_G2_precomp,
        prec_P2: &Self::affine_ate_G1_precomp,
        prec_Q2: &Self::affine_ate_G2_precomp,
        prec_P3: &Self::affine_ate_G1_precomp,
        prec_Q3: &Self::affine_ate_G2_precomp,
    ) -> Self::Fqk {
        unimplemented!("alt_bn128_affine_ate_e_times_e_over_e_miller_loop");
    }

    fn miller_loop(prec_P: &Self::G1_precomp, prec_Q: &Self::G2_precomp) -> Self::Fqk {
        alt_bn128_miller_loop(prec_P, prec_Q)
    }

    fn double_miller_loop(
        prec_P1: &Self::G1_precomp,
        prec_Q1: &Self::G2_precomp,
        prec_P2: &Self::G1_precomp,
        prec_Q2: &Self::G2_precomp,
    ) -> Self::Fqk {
        alt_bn128_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2)
    }

    fn pairing(P: &Self::G1, Q: &Self::G2) -> Self::Fqk {
        alt_bn128_pairing(P, Q)
    }

    fn reduced_pairing(P: &Self::G1, Q: &Self::G2) -> Self::GT {
        alt_bn128_reduced_pairing(P, Q)
    }
    fn affine_reduced_pairing(P: &Self::G1, Q: &Self::G2) -> Self::GT {
        unimplemented!("alt_bn128");
    }
}
