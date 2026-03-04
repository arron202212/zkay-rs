//  Declaration of interfaces for initializing MNT4.
use crate::algebra::curves::mnt::mnt4::mnt4_fields::{init_mnt4_fields, mnt4_q_limbs};
use crate::algebra::curves::mnt::mnt4::mnt4_fields::{mnt4_Fq, mnt4_Fq2};
use ffec::field_utils::bigint::bigint;

pub const mnt4_twist: mnt4_Fq2 = mnt4_Fq2::zero();
pub const mnt4_twist_coeff_a: mnt4_Fq2 = mnt4_Fq2::zero();
pub const mnt4_twist_coeff_b: mnt4_Fq2 = mnt4_Fq2::zero();
pub const mnt4_twist_mul_by_a_c0: mnt4_Fq = mnt4_Fq::zero();
pub const mnt4_twist_mul_by_a_c1: mnt4_Fq = mnt4_Fq::zero();
pub const mnt4_twist_mul_by_b_c0: mnt4_Fq = mnt4_Fq::zero();
pub const mnt4_twist_mul_by_b_c1: mnt4_Fq = mnt4_Fq::zero();
pub const mnt4_twist_mul_by_q_X: mnt4_Fq = mnt4_Fq::zero();
pub const mnt4_twist_mul_by_q_Y: mnt4_Fq = mnt4_Fq::zero();
pub const mnt4_q_limbs4: usize = mnt4_q_limbs * 4;
pub const mnt4_ate_loop_count: bigint<mnt4_q_limbs> = bigint::<mnt4_q_limbs>::zero();
pub const mnt4_ate_is_loop_count_neg: bool = false;
pub const mnt4_final_exponent: bigint<mnt4_q_limbs4> = bigint::<mnt4_q_limbs4>::zero();
pub const mnt4_final_exponent_last_chunk_abs_of_w0: bigint<mnt4_q_limbs> =
    bigint::<mnt4_q_limbs>::zero();
pub const mnt4_final_exponent_last_chunk_is_w0_neg: bool = false;
pub const mnt4_final_exponent_last_chunk_w1: bigint<mnt4_q_limbs> = bigint::<mnt4_q_limbs>::zero();

pub fn init_mnt4_params() {
    init_mnt4_fields();
}

// use ark_ec::mnt4::MNT4Config;
// use ark_ff::{BigInteger, Field, Fp2, PrimeField};
// use ark_mnt4_298::{Fq, Fq2, Fr, G1Projective, G2Projective};

// pub struct Mnt4_298Config;

// impl MNT4Config for Mnt4_298Config {
//     type BaseField = Fq;
//     type ScalarField = Fr;

//     const COEFF_A: Fq = field_new!(Fq, "2");
//     const COEFF_B: Fq = field_new!(
//         Fq,
//         "423894536526684178289416011533888240029318103673896002803341544124054745019340795360841685"
//     );

//     const G1_GENERATOR: G1Projective = G1Projective::new(
//         field_new!(
//             Fq,
//             "60760244141852568949126569781626075788424196370144486719385562369396875346601926534016838"
//         ),
//         field_new!(
//             Fq,
//             "363732850702582978263902770815145784459747722357071843971107674179038674942891694705904306"
//         ),
//         Fq::ONE,
//     );

//     const G2_COEFF_A: Fq2 = Fq2::new(
//         field_new!(Fq, "34"),
//         Fq::ZERO,
//     );
// }

// pub const G1_WNAF_WINDOW_TABLE: [usize; 4] = [11, 24, 60, 127];

// pub const G1_FIXED_BASE_WINDOW_TABLE: [usize; 22] = [
//     1, 5, 10, 25, 60, 144, 345, 855, 1805, 3912, 11265, 27898, 57597, 145299, 157205, 601601,
//     1107377, 1789647, 4392627, 8221211, 0, 42363731,
// ];

// pub const G2_GENERATOR: G2Projective = G2Projective::new(
//     Fq2::new(
//         field_new!(
//             Fq,
//             "438374926219350099854919100077809681842783509163790991847867546339851681564223481322252708"
//         ),
//         field_new!(
//             Fq,
//             "37620953615500480110935514360923278605464476459712393277679280819942849043649216370485641"
//         ),
//     ),
//     Fq2::new(
//         field_new!(
//             Fq,
//             "37437409008528968268352521034936931842973546441370663118543015118291998305624025037512482"
//         ),
//         field_new!(
//             Fq,
//             "424621479598893882672393190337420680597584695892317197646113820787463109735345923009077489"
//         ),
//     ),
//     Fq2::ONE,
// );
