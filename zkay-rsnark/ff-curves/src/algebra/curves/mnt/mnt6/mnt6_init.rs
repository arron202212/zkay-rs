//  Declaration of interfaces for initializing MNT6.

use crate::algebra::curves::mnt::mnt6::mnt6_fields::{mnt6_Fq, mnt6_Fq3, mnt6_q_limbs};
use ffec::field_utils::bigint::bigint;
//bigint<mnt6_r_limbs> mnt6_modulus_r = mnt46_modulus_B;
//bigint<mnt6_q_limbs> mnt6_modulus_q = mnt46_modulus_A;

pub const mnt6_twist: mnt6_Fq3 = mnt6_Fq3::const_default();
pub const mnt6_twist_coeff_a: mnt6_Fq3 = mnt6_Fq3::const_default();
pub const mnt6_twist_coeff_b: mnt6_Fq3 = mnt6_Fq3::const_default();
pub const mnt6_twist_mul_by_a_c0: mnt6_Fq = mnt6_Fq::const_default();
pub const mnt6_twist_mul_by_a_c1: mnt6_Fq = mnt6_Fq::const_default();
pub const mnt6_twist_mul_by_a_c2: mnt6_Fq = mnt6_Fq::const_default();
pub const mnt6_twist_mul_by_b_c0: mnt6_Fq = mnt6_Fq::const_default();
pub const mnt6_twist_mul_by_b_c1: mnt6_Fq = mnt6_Fq::const_default();
pub const mnt6_twist_mul_by_b_c2: mnt6_Fq = mnt6_Fq::const_default();
pub const mnt6_twist_mul_by_q_X: mnt6_Fq = mnt6_Fq::const_default();
pub const mnt6_twist_mul_by_q_Y: mnt6_Fq = mnt6_Fq::const_default();

pub const mnt6_q_limbs6: usize = 6 * mnt6_q_limbs;
pub const mnt6_ate_loop_count: bigint<mnt6_q_limbs> = bigint::<mnt6_q_limbs>::zero();
pub const mnt6_ate_is_loop_count_neg: bool = false;
pub const mnt6_final_exponent: bigint<mnt6_q_limbs6> = bigint::<mnt6_q_limbs6>::zero();
pub const mnt6_final_exponent_last_chunk_abs_of_w0: bigint<mnt6_q_limbs> =
    bigint::<mnt6_q_limbs>::zero();
pub const mnt6_final_exponent_last_chunk_is_w0_neg: bool = false;
pub const mnt6_final_exponent_last_chunk_w1: bigint<mnt6_q_limbs> = bigint::<mnt6_q_limbs>::zero();

// parameters for twisted short Weierstrass curve E'/Fq3 : y^2 = x^3 + (a * twist^2) * x + (b * twist^3)

pub fn init_mnt6_params() {}

// use ark_ec::mnt6::MNT6Config;
// use ark_ff::{BigInteger, Field, Fp3, PrimeField};
// use ark_mnt6_298::{Fq, Fq3, Fr, G1Projective, G2Projective};

// pub struct Mnt6_298Config;

// impl MNT6Config for Mnt6_298Config {
//     type BaseField = Fq;
//     type ScalarField = Fr;

//     const COEFF_A: Fq = field_new!(Fq, "11");
//     const COEFF_B: Fq = field_new!(
//         Fq,
//         "106700080510851735677967319632585352256454251201367587890185989362936000262606668469523074"
//     );

//     const G1_GENERATOR: G1Projective = G1Projective::new(
//         field_new!(
//             Fq,
//             "336685752883082228109289846353937104185698209371404178342968838739115829740084426881123453"
//         ),
//         field_new!(
//             Fq,
//             "402596290139780989709332707716568920777622032073762749862342374583908837063963736098549800"
//         ),
//         Fq::ONE,
//     );

//     const G2_COEFF_A: Fq3 = Fq3::new(Fq::ZERO, Fq::ZERO, field_new!(Fq, "11"));
//     const G2_COEFF_B: Fq3 = Fq3::new(
//         field_new!(
//             Fq,
//             "106700080510851735677967319632585352256454251201367587890185989362936000262606668469523074"
//         ) * &Fq3::NONRESIDUE,
//         Fq::ZERO,
//         Fq::ZERO,
//     );
// }

// pub const G1_WNAF_WINDOW_TABLE: [usize; 4] = [11, 24, 60, 127];

// pub const G1_FIXED_BASE_WINDOW_TABLE: [usize; 22] = [
//     1, 4, 10, 25, 60, 146, 350, 845, 1840, 3904, 11309, 24016, 72289, 138413, 156390, 562560,
//     1036742, 2053819, 4370224, 8215704, 0, 42682375,
// ];

// pub const G2_GENERATOR: G2Projective = G2Projective::new(
//     Fq3::new(
//         field_new!(
//             Fq,
//             "421456435772811846256826561593908322288509115489119907560382401870203318738334702321297427"
//         ),
//         field_new!(
//             Fq,
//             "103072927438548502463527009961344915021167584706439945404959058962657261178393635706405114"
//         ),
//         field_new!(
//             Fq,
//             "143029172143731852627002926324735183809768363301149009204849580478324784395590388826052558"
//         ),
//     ),
//     Fq3::new(
//         field_new!(
//             Fq,
//             "464673596668689463130099227575639512541218133445388869383893594087634649237515554342751377"
//         ),
//         field_new!(
//             Fq,
//             "100642907501977375184575075967118071807821117960152743335603284583254620685343989304941678"
//         ),
//         field_new!(
//             Fq,
//             "123019855502969896026940545715841181300275180157288044663051565390506010149881373807142903"
//         ),
//     ),
//     Fq3::ONE,
// );
