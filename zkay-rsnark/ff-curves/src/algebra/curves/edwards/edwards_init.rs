use crate::algebra::curves::edwards::edwards_fields::{
    edwards_Fq, edwards_Fq3, init_edwards_fields,
};
use crate::algebra::curves::edwards::edwards_fields::{edwards_q_limbs, edwards_r_limbs};
use ffec::field_utils::bigint::bigint;

pub const edwards_modulus_r: bigint<edwards_r_limbs> = bigint::<edwards_r_limbs>::zero();
pub const edwards_modulus_q: bigint<edwards_q_limbs> = bigint::<edwards_q_limbs>::zero();

pub const edwards_coeff_a: edwards_Fq = edwards_Fq::const_default();
pub const edwards_coeff_d: edwards_Fq = edwards_Fq::const_default();
pub const edwards_twist: edwards_Fq3 = edwards_Fq::const_default();
pub const edwards_twist_coeff_a: edwards_Fq3 = edwards_Fq::const_default();
pub const edwards_twist_coeff_d: edwards_Fq3 = edwards_Fq::const_default();
pub const edwards_twist_mul_by_a_c0: edwards_Fq = edwards_Fq::const_default();
pub const edwards_twist_mul_by_a_c1: edwards_Fq = edwards_Fq::const_default();
pub const edwards_twist_mul_by_a_c2: edwards_Fq = edwards_Fq::const_default();
pub const edwards_twist_mul_by_d_c0: edwards_Fq = edwards_Fq::const_default();
pub const edwards_twist_mul_by_d_c1: edwards_Fq = edwards_Fq::const_default();
pub const edwards_twist_mul_by_d_c2: edwards_Fq = edwards_Fq::const_default();
pub const edwards_twist_mul_by_q_Y: edwards_Fq = edwards_Fq::const_default();
pub const edwards_twist_mul_by_q_Z: edwards_Fq = edwards_Fq::const_default();

pub const edwards_q_limbs6: usize = edwards_q_limbs * 6;
pub const edwards_ate_loop_count: bigint<edwards_q_limbs> = bigint::<edwards_q_limbs>::zero();
pub const edwards_final_exponent: bigint<edwards_q_limbs6> = bigint::<edwards_q_limbs6>::zero();
pub const edwards_final_exponent_last_chunk_abs_of_w0: bigint<edwards_q_limbs> =
    bigint::<edwards_q_limbs>::zero();
pub const edwards_final_exponent_last_chunk_is_w0_neg: bool = false;
pub const edwards_final_exponent_last_chunk_w1: bigint<edwards_q_limbs> =
    bigint::<edwards_q_limbs>::zero();

pub fn init_edwards_params() {
    init_edwards_fields();

    /* choice of Edwards curve and its twist */
}

// use ark_ec::edwards::{EdwardsConfig, Projective};
// use ark_ff::{BigInteger, Field, Fp3, Fp3Config, PrimeField};

// pub struct EdwardsG1Config;

// impl EdwardsConfig for EdwardsG1Config {
//     type BaseField = Fq;
//     type ScalarField = Fr;

//     const COEFF_A: Self::BaseField = field_new!(Fq, "1");
//     const COEFF_D: Self::BaseField =
//         field_new!(Fq, "600581931845324488256649384912508268813600056237543024");

//     const GENERATOR: Projective<Self> = Projective::new(
//         field_new!(
//             Fq,
//             "3713709671941291996998665608188072510389821008693530490"
//         ),
//         field_new!(
//             Fq,
//             "4869953702976555123067178261685365085639705297852816679"
//         ),
//     );
// }

// pub const G1_WNAF_WINDOW_TABLE: [usize; 4] = [9, 14, 24, 117];
// pub const G1_FIXED_BASE_WINDOW_TABLE: [usize; 22] = [
//     1, 4, 10, 25, 60, 149, 370, 849, 1765, 4430, 13389, 15368, 74912, 0, 438107, 0, 1045626,
//     1577434, 0, 0, 17350594, 0,
// ];

// pub const EDWARDS_TWIST: Fq3 = Fq3::new(Fq::ZERO, Fq::ONE, Fq::ZERO);

// pub const ATE_LOOP_COUNT: [u64; 2] = [0x1, 0x1];

// pub struct EdwardsG2Config;
