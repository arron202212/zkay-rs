use crate::algebra::curves::edwards::edwards_fields::{
    edwards_Fq, edwards_Fq3, edwards_Fq6, edwards_Fr, edwards_GT,
};
use ffec::PpConfig;
use crate::algebra::curves::edwards::edwards_g1::edwards_G1;
use crate::algebra::curves::edwards::edwards_g2::edwards_G2;
use crate::algebra::curves::edwards::edwards_init::{
    edwards_ate_loop_count, edwards_final_exponent_last_chunk_abs_of_w0,
    edwards_final_exponent_last_chunk_is_w0_neg, edwards_final_exponent_last_chunk_w1,
    edwards_modulus_r,
};

use ffec::common::profiling::{enter_block, leave_block};
use ffec::field_utils::bigint::bigint;

/* Tate pairing */
#[derive(Clone, Debug, Default, PartialEq)]
struct edwards_Fq_conic_coefficients {
    pub c_ZZ: edwards_Fq,
    pub c_XY: edwards_Fq,
    pub c_XZ: edwards_Fq,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct edwards_tate_G1_precomp(Vec<edwards_Fq_conic_coefficients>);
#[derive(Clone, Debug, Default, PartialEq)]
pub struct edwards_tate_G2_precomp {
    pub y0: edwards_Fq3,
    pub eta: edwards_Fq3,
}

/* ate pairing */
#[derive(Clone, Debug, Default, PartialEq)]
pub struct edwards_Fq3_conic_coefficients {
    pub c_ZZ: edwards_Fq3,
    pub c_XY: edwards_Fq3,
    pub c_XZ: edwards_Fq3,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct edwards_ate_G2_precomp(Vec<edwards_Fq3_conic_coefficients>);
#[derive(Clone, Debug, Default, PartialEq)]
pub struct edwards_ate_G1_precomp {
    pub P_XY: edwards_Fq,
    pub P_XZ: edwards_Fq,
    pub P_ZZplusYZ: edwards_Fq,
}

use std::fmt;
impl fmt::Display for edwards_ate_G2_precomp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}
impl fmt::Display for edwards_ate_G1_precomp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}

impl fmt::Display for edwards_Fq3_conic_coefficients {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}
impl fmt::Display for edwards_Fq_conic_coefficients {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}

impl fmt::Display for edwards_tate_G2_precomp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}

/* choice of pairing */

pub type edwards_G1_precomp = edwards_ate_G1_precomp;
pub type edwards_G2_precomp = edwards_ate_G2_precomp;

/* final exponentiations */
pub fn edwards_final_exponentiation_last_chunk(
    elt: &edwards_Fq6,
    elt_inv: &edwards_Fq6,
) -> edwards_Fq6 {
    enter_block("Call to edwards_final_exponentiation_last_chunk", false);
    let elt_q = elt.Frobenius_map(1);
    let w1_part = elt_q.cyclotomic_exp(&edwards_final_exponent_last_chunk_w1);
    let mut w0_part = if edwards_final_exponent_last_chunk_is_w0_neg {
        elt_inv.cyclotomic_exp(&edwards_final_exponent_last_chunk_abs_of_w0)
    } else {
        elt.cyclotomic_exp(&edwards_final_exponent_last_chunk_abs_of_w0)
    };
    let result = w1_part * w0_part;
    leave_block("Call to edwards_final_exponentiation_last_chunk", false);

    result
}

pub fn edwards_final_exponentiation_first_chunk(
    elt: &edwards_Fq6,
    elt_inv: &edwards_Fq6,
) -> edwards_Fq6 {
    enter_block("Call to edwards_final_exponentiation_first_chunk", false);

    /* (q^3-1)*(q+1) */

    /* elt_q3 = elt^(q^3) */
    let elt_q3 = elt.Frobenius_map(3);
    /* elt_q3_over_elt = elt^(q^3-1) */
    let elt_q3_over_elt = elt_q3 * elt_inv;
    /* alpha = elt^((q^3-1) * q) */
    let alpha = elt_q3_over_elt.Frobenius_map(1);
    /* beta = elt^((q^3-1)*(q+1) */
    let beta = alpha * elt_q3_over_elt;
    leave_block("Call to edwards_final_exponentiation_first_chunk", false);
    return beta;
}

pub fn edwards_final_exponentiation(elt: &edwards_Fq6) -> edwards_GT {
    enter_block("Call to edwards_final_exponentiation", false);
    let elt_inv = elt.inverse();
    let elt_to_first_chunk = edwards_final_exponentiation_first_chunk(elt, &elt_inv);
    let elt_inv_to_first_chunk = edwards_final_exponentiation_first_chunk(&elt_inv, elt);
    let result =
        edwards_final_exponentiation_last_chunk(&elt_to_first_chunk, &elt_inv_to_first_chunk);
    leave_block("Call to edwards_final_exponentiation", false);

    result
}

pub fn edwards_tate_precompute_G2(Q: &edwards_G2) -> edwards_tate_G2_precomp {
    enter_block("Call to edwards_tate_precompute_G2", false);
    let mut Qcopy: edwards_G2  = Q.clone();
    Qcopy.to_affine_coordinates();
    let mut result = edwards_tate_G2_precomp::default();
    result.y0 = Qcopy.Y * Qcopy.Z.inverse(); // Y/Z
    result.eta = (Qcopy.Z + Qcopy.Y) * edwards_Fq6::mul_by_non_residue(&Qcopy.X).inverse(); // (Z+Y)/(nqr*X)
    leave_block("Call to edwards_tate_precompute_G2", false);

    result
}
#[derive(Clone, Debug,Default, PartialEq)]
pub struct extended_edwards_G1_projective {
    pub X: edwards_Fq,
    pub Y: edwards_Fq,
    pub Z: edwards_Fq,
    pub T: edwards_Fq,
}
impl extended_edwards_G1_projective {
    pub fn print(&self) {
        print!("extended edwards_G1 projective X/Y/Z/T:\n");
        self.X.print();
        self.Y.print();
        self.Z.print();
        self.T.print();
    }

    pub fn test_invariant(&self) {
        assert!(self.T * self.Z == self.X * self.Y);
    }
}

pub fn doubling_step_for_miller_loop(
    current: &mut extended_edwards_G1_projective,
    cc: &mut edwards_Fq_conic_coefficients,
) {
    let X: edwards_Fq = current.X.clone();
    let Y = current.Y.clone();
    let Z = current.Z.clone();
    let T = current.T.clone();
    let A = X.squared(); // A    = X1^2
    let B = Y.squared(); // B    = Y1^2
    let C = Z.squared(); // C    = Z1^2
    let D = (X + Y).squared(); // D    = (X1+Y1)^2
    let E = (Y + Z).squared(); // E    = (Y1+Z1)^2
    let F = D - (A + B); // F    = D-(A+B)
    let G = E - (B + C); // G    = E-(B+C)
    let H: edwards_Fq = A; // H    = A (edwards_a=1)
    let I = H + B; // I    = H+B
    let J = C - I; // J    = C-I
    let K = J + C; // K    = J+C

    cc.c_ZZ = Y * (T - X); // c_ZZ = 2*Y1*(T1-X1)
    cc.c_ZZ = cc.c_ZZ + cc.c_ZZ;

    cc.c_XY = J + J + G; // c_XY = 2*J+G
    cc.c_XZ = X * T - B; // c_XZ = 2*(X1*T1-B) (edwards_a=1)
    cc.c_XZ = cc.c_XZ + cc.c_XZ;

    current.X = F * K; // X3 = F*K
    current.Y = I * (B - H); // Y3 = I*(B-H)
    current.Z = I * K; // Z3 = I*K
    current.T = F * (B - H); // T3 = F*(B-H)

    // #ifdef DEBUG
    current.test_invariant();
}

pub fn full_addition_step_for_miller_loop(
    base: &extended_edwards_G1_projective,
    current: &mut extended_edwards_G1_projective,
    cc: &mut edwards_Fq_conic_coefficients,
) {
    let X1: edwards_Fq = current.X.clone();
    let Y1 = current.Y.clone();
    let Z1 = current.Z.clone();
    let T1 = current.T;
    let X2: edwards_Fq = base.X.clone();
    let Y2 = base.Y.clone();
    let Z2 = base.Z.clone();
    let T2 = base.T;

    let A = X1.clone() * X2; // A    = X1*X2
    let B = Y1.clone() * Y2; // B    = Y1*Y2
    let C = Z1.clone() * T2; // C    = Z1*T2
    let D = T1.clone() * Z2; // D    = T1*Z2
    let E = D.clone() + C; // E    = D+C
    let F = (X1.clone() - Y1) * (X2.clone() + Y2) + B - A; // F    = (X1-Y1)*(X2+Y2)+B-A
    let G = B.clone() + A; // G    = B + A (edwards_a=1)
    let H = D.clone() - C; // H    = D-C
    let I = T1 * T2; // I    = T1*T2

    cc.c_ZZ = (T1.clone() - X1) * (T2 + X2) - I + A; // c_ZZ = (T1-X1)*(T2+X2)-I+A
    cc.c_XY = X1.clone() * Z2 - X2 * Z1 + F; // c_XY = X1*Z2-X2*Z1+F
    cc.c_XZ = (Y1.clone() - T1) * (Y2 + T2) - B + I - H; // c_XZ = (Y1-T1)*(Y2+T2)-B+I-H
    current.X = E.clone() * F; // X3   = E*F
    current.Y = G.clone() * H; // Y3   = G*H
    current.Z = F.clone() * G; // Z3   = F*G
    current.T = E.clone() * H; // T3   = E*H

    current.test_invariant();
}

pub fn mixed_addition_step_for_miller_loop(
    base: &extended_edwards_G1_projective,
    current: &mut extended_edwards_G1_projective,
    cc: &mut edwards_Fq_conic_coefficients,
) {
    let X1: edwards_Fq = current.X.clone();
    let Y1 = current.Y.clone();
    let Z1 = current.Z.clone();
    let T1 = current.T;
    let X2: edwards_Fq = base.X.clone();
    let Y2 = base.Y.clone();
    let T2 = base.T;

    let A = X1 * X2; // A    = X1*X2
    let B = Y1 * Y2; // B    = Y1*Y2
    let C = Z1 * T2; // C    = Z1*T2
    let D = T1; // D    = T1*Z2
    let E = D + C; // E    = D+C
    let F = (X1 - Y1) * (X2 + Y2) + B - A; // F    = (X1-Y1)*(X2+Y2)+B-A
    let G = B + A; // G    = B + A (edwards_a=1)
    let H = D - C; // H    = D-C
    let I = T1 * T2; // I    = T1*T2

    cc.c_ZZ = (T1 - X1) * (T2 + X2) - I + A; // c_ZZ = (T1-X1)*(T2+X2)-I+A
    cc.c_XY = X1 - X2 * Z1 + F; // c_XY = X1*Z2-X2*Z1+F
    cc.c_XZ = (Y1 - T1) * (Y2 + T2) - B + I - H; // c_XZ = (Y1-T1)*(Y2+T2)-B+I-H
    current.X = E * F; // X3   = E*F
    current.Y = G * H; // Y3   = G*H
    current.Z = F * G; // Z3   = F*G
    current.T = E * H; // T3   = E*H

    // #ifdef DEBUG
    current.test_invariant();
}

pub fn edwards_tate_precompute_G1(P: &edwards_G1) -> edwards_tate_G1_precomp {
    enter_block("Call to edwards_tate_precompute_G1", false);
    let mut result = edwards_tate_G1_precomp::default();

    let mut Pcopy = P.clone();
    Pcopy.to_affine_coordinates();

    let mut P_ext = extended_edwards_G1_projective::default();
    P_ext.X = Pcopy.X;
    P_ext.Y = Pcopy.Y;
    P_ext.Z = Pcopy.Z;
    P_ext.T = Pcopy.X * Pcopy.Y;

    let mut R = P_ext.clone();

    let mut found_one = false;
    for i in (0..=edwards_modulus_r.max_bits()).rev() {
        let mut bit = edwards_modulus_r.test_bit(i);
        if !found_one {
            /* this skips the MSB itself */
            found_one |= bit;
            continue;
        }

        /* code below gets executed for all bits (EXCEPT the MSB itself) of
        edwards_modulus_r (skipping leading zeros) in MSB to LSB
        order */
        let mut cc = edwards_Fq_conic_coefficients::default();
        doubling_step_for_miller_loop(&mut R, &mut cc);
        result.0.push(cc.clone());

        if bit {
            mixed_addition_step_for_miller_loop(&P_ext, &mut R, &mut cc);
            result.0.push(cc);
        }
    }

    leave_block("Call to edwards_tate_precompute_G1", false);
    result
}

pub fn edwards_tate_miller_loop(
    prec_P: &edwards_tate_G1_precomp,
    prec_Q: &edwards_tate_G2_precomp,
) -> edwards_Fq6 {
    enter_block("Call to edwards_tate_miller_loop", false);

    let mut f = edwards_Fq6::one();

    let mut found_one = false;
    let mut idx = 0;
    for i in (0..=edwards_modulus_r.max_bits() - 1).rev() {
        let mut bit = edwards_modulus_r.test_bit(i);
        if !found_one {
            /* this skips the MSB itself */
            found_one |= bit;
            continue;
        }

        /* code below gets executed for all bits (EXCEPT the MSB itself) of
        edwards_modulus_r (skipping leading zeros) in MSB to LSB
        order */
        let mut cc = prec_P.0[idx].clone();
        idx += 1;
        let g_RR_at_Q = edwards_Fq6::new(
            edwards_Fq3::new(cc.c_XZ, edwards_Fq::from(0), edwards_Fq::from(0))
                + prec_Q.y0.clone()*cc.c_XY.clone(),
prec_Q.eta.clone()*           cc.c_ZZ.clone(),
        );
        f = f.squared() * g_RR_at_Q;
        if bit {
            cc = prec_P.0[idx].clone();
            idx += 1;

            let g_RP_at_Q = edwards_Fq6::new(
                edwards_Fq3::new(cc.c_XZ, edwards_Fq::from(0), edwards_Fq::from(0))
                    + prec_Q.y0.clone()*cc.c_XY.clone(),
 prec_Q.eta.clone()*               cc.c_ZZ.clone(),
            );
            f = f * g_RP_at_Q;
        }
    }
    leave_block("Call to edwards_tate_miller_loop", false);

    f
}

pub fn edwards_tate_pairing(P: &edwards_G1, Q: &edwards_G2) -> edwards_Fq6 {
    enter_block("Call to edwards_tate_pairing", false);
    let prec_P = edwards_tate_precompute_G1(P);
    let prec_Q = edwards_tate_precompute_G2(Q);
    let result = edwards_tate_miller_loop(&prec_P, &prec_Q);
    leave_block("Call to edwards_tate_pairing", false);
    result
}

pub fn edwards_tate_reduced_pairing(P: &edwards_G1, Q: &edwards_G2) -> edwards_GT {
    enter_block("Call to edwards_tate_reduced_pairing", false);
    let f = edwards_tate_pairing(P, Q);
    let result = edwards_final_exponentiation(&f);
    leave_block("Call to edwards_tate_reduce_pairing", false);
    result
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct extended_edwards_G2_projective {
    pub X: edwards_Fq3,
    pub Y: edwards_Fq3,
    pub Z: edwards_Fq3,
    pub T: edwards_Fq3,
}
impl extended_edwards_G2_projective {
    pub fn print(&self) {
        print!("extended edwards_G2 projective X/Y/Z/T:\n");
        self.X.print();
        self.Y.print();
        self.Z.print();
        self.T.print();
    }

    pub fn test_invariant(&self) {
        assert!(self.T * self.Z == self.X * self.Y);
    }
}

pub fn doubling_step_for_flipped_miller_loop(
    current: &mut extended_edwards_G2_projective,
    cc: &mut edwards_Fq3_conic_coefficients,
) {
    let X: edwards_Fq3 = current.X.clone();
    let Y = current.Y.clone();
    let Z = current.Z.clone();
    let T = current.T;
    let A = X.squared(); // A    = X1^2
    let B = Y.squared(); // B    = Y1^2
    let C = Z.squared(); // C    = Z1^2
    let D = (X + Y).squared(); // D    = (X1+Y1)^2
    let E = (Y + Z).squared(); // E    = (Y1+Z1)^2
    let F = D - (A + B); // F    = D-(A+B)
    let G = E - (B + C); // G    = E-(B+C)
    let H = edwards_G2::mul_by_a(&A); // edwards_param_twist_coeff_a is 1 * X for us
    // H    = twisted_a * A
    let I = H + B; // I    = H+B
    let J = C - I; // J    = C-I
    let K = J + C; // K    = J+C

    cc.c_ZZ = Y * (T - X); // c_ZZ = 2*Y1*(T1-X1)
    cc.c_ZZ = cc.c_ZZ + cc.c_ZZ;

    // c_XY = 2*(C-edwards_a * A * delta_3-B)+G (edwards_a = 1 for us)
    cc.c_XY = C - edwards_G2::mul_by_a(&A) - B; // edwards_param_twist_coeff_a is 1 * X for us
    cc.c_XY = cc.c_XY + cc.c_XY + G;

    // c_XZ = 2*(edwards_a*X1*T1*delta_3-B) (edwards_a = 1 for us)
    cc.c_XZ = edwards_G2::mul_by_a(&(X * T)) - B; // edwards_param_twist_coeff_a is 1 * X for us
    cc.c_XZ = cc.c_XZ + cc.c_XZ;

    current.X = F * K; // X3 = F*K
    current.Y = I * (B - H); // Y3 = I*(B-H)
    current.Z = I * K; // Z3 = I*K
    current.T = F * (B - H); // T3 = F*(B-H)
    // #ifdef DEBUG
    current.test_invariant();
}

pub fn full_addition_step_for_flipped_miller_loop(
    base: &extended_edwards_G2_projective,
    current: &mut extended_edwards_G2_projective,
    cc: &mut edwards_Fq3_conic_coefficients,
) {
    let X1: edwards_Fq3 = current.X.clone();
    let Y1 = current.Y.clone();
    let Z1 = current.Z.clone();
    let T1 = current.T;
    let X2: edwards_Fq3 = base.X.clone();
    let Y2 = base.Y.clone();
    let Z2 = base.Z.clone();
    let T2 = base.T;

    let A = X1 * X2; // A    = X1*X2
    let B = Y1 * Y2; // B    = Y1*Y2
    let C = Z1 * T2; // C    = Z1*T2
    let D = T1 * Z2; // D    = T1*Z2
    let E = D + C; // E    = D+C
    let F = (X1 - Y1) * (X2 + Y2) + B - A; // F    = (X1-Y1)*(X2+Y2)+B-A
    // G = B + twisted_edwards_a * A
    let G = B + edwards_G2::mul_by_a(&A); // edwards_param_twist_coeff_a is 1*X for us
    let H = D - C; // H    = D-C
    let I = T1 * T2; // I    = T1*T2

    // c_ZZ = delta_3* ((T1-X1)*(T2+X2)-I+A)
    cc.c_ZZ = edwards_G2::mul_by_a(&((T1 - X1) * (T2 + X2) - I + A)); // edwards_param_twist_coeff_a is 1*X for us

    cc.c_XY = X1 * Z2 - X2 * Z1 + F; // c_XY = X1*Z2-X2*Z1+F
    cc.c_XZ = (Y1 - T1) * (Y2 + T2) - B + I - H; // c_XZ = (Y1-T1)*(Y2+T2)-B+I-H
    current.X = E * F; // X3   = E*F
    current.Y = G * H; // Y3   = G*H
    current.Z = F * G; // Z3   = F*G
    current.T = E * H; // T3   = E*H

    // #ifdef DEBUG
    current.test_invariant();
}

pub fn mixed_addition_step_for_flipped_miller_loop(
    base: &extended_edwards_G2_projective,
    current: &mut extended_edwards_G2_projective,
    cc: &mut edwards_Fq3_conic_coefficients,
) {
    let X1: edwards_Fq3 = current.X.clone();
    let Y1 = current.Y.clone();
    let Z1 = current.Z.clone();
    let T1 = current.T;
    let X2: edwards_Fq3 = base.X.clone();
    let Y2 = base.Y.clone();
    let T2 = base.T;

    let A = X1 * X2; // A    = X1*X2
    let B = Y1 * Y2; // B    = Y1*Y2
    let C = Z1 * T2; // C    = Z1*T2
    let E = T1 + C; // E    = T1+C
    let F = (X1 - Y1) * (X2 + Y2) + B - A; // F    = (X1-Y1)*(X2+Y2)+B-A
    // G = B + twisted_edwards_a * A
    let G = B + edwards_G2::mul_by_a(&A); // edwards_param_twist_coeff_a is 1*X for us
    let H = T1 - C; // H    = T1-C
    let I = T1 * T2; // I    = T1*T2

    // c_ZZ = delta_3* ((T1-X1)*(T2+X2)-I+A)
    cc.c_ZZ = edwards_G2::mul_by_a(&((T1 - X1) * (T2 + X2) - I + A)); // edwards_param_twist_coeff_a is 1*X for us

    cc.c_XY = X1 - X2 * Z1 + F; // c_XY = X1*Z2-X2*Z1+F
    cc.c_XZ = (Y1 - T1) * (Y2 + T2) - B + I - H; // c_XZ = (Y1-T1)*(Y2+T2)-B+I-H
    current.X = E * F; // X3   = E*F
    current.Y = G * H; // Y3   = G*H
    current.Z = F * G; // Z3   = F*G
    current.T = E * H; // T3   = E*H

    // #ifdef DEBUG
    current.test_invariant();
}

pub fn edwards_ate_precompute_G1(P: &edwards_G1) -> edwards_ate_G1_precomp {
    enter_block("Call to edwards_ate_precompute_G1", false);
    let mut Pcopy = P.clone();
    Pcopy.to_affine_coordinates();
    let mut result = edwards_ate_G1_precomp::default();
    result.P_XY = Pcopy.X * Pcopy.Y;
    result.P_XZ = Pcopy.X; // P.X * P.Z but P.Z = 1
    result.P_ZZplusYZ = (edwards_Fq::one() + Pcopy.Y); // (P.Z + P.Y) * P.Z but P.Z = 1
    leave_block("Call to edwards_ate_precompute_G1", false);
    result
}

pub fn edwards_ate_precompute_G2(Q: &edwards_G2) -> edwards_ate_G2_precomp {
    enter_block("Call to edwards_ate_precompute_G2", false);
    let loop_count: bigint<{ edwards_Fr::num_limbs }> = edwards_ate_loop_count;
    let mut result = edwards_ate_G2_precomp::default();

    let mut Qcopy = Q.clone();
    Qcopy.to_affine_coordinates();

    let mut Q_ext = extended_edwards_G2_projective::default();
    Q_ext.X = Qcopy.X;
    Q_ext.Y = Qcopy.Y;
    Q_ext.Z = Qcopy.Z;
    Q_ext.T = Qcopy.X * Qcopy.Y;

    let mut R = Q_ext.clone();

    let mut found_one = false;
    for i in (0..=loop_count.max_bits() - 1).rev() {
        let mut bit = loop_count.test_bit(i);
        if !found_one {
            /* this skips the MSB itself */
            found_one |= bit;
            continue;
        }

        let mut cc = edwards_Fq3_conic_coefficients::default();
        doubling_step_for_flipped_miller_loop(&mut R, &mut cc);
        result.0.push(cc.clone());
        if bit {
            mixed_addition_step_for_flipped_miller_loop(&Q_ext,&mut R, &mut cc);
            result.0.push(cc);
        }
    }

    leave_block("Call to edwards_ate_precompute_G2", false);
    result
}

pub fn edwards_ate_miller_loop(
    prec_P: &edwards_ate_G1_precomp,
    prec_Q: &edwards_ate_G2_precomp,
) -> edwards_Fq6 {
    enter_block("Call to edwards_ate_miller_loop", false);
    let loop_count: bigint<{ edwards_Fr::num_limbs }> = edwards_ate_loop_count;

    let mut f = edwards_Fq6::one();

    let mut found_one = false;
    let mut idx = 0;
    for i in (0..=loop_count.max_bits() - 1).rev() {
        let mut bit = loop_count.test_bit(i);
        if !found_one {
            /* this skips the MSB itself */
            found_one |= bit;
            continue;
        }

        /* code below gets executed for all bits (EXCEPT the MSB itself) of
        edwards_param_p (skipping leading zeros) in MSB to LSB
        order */
        let mut cc = prec_Q.0[idx].clone();
        idx += 1;

        let g_RR_at_P = edwards_Fq6::new(
 cc.c_XY.clone()*           prec_P.P_XY.clone() + cc.c_XZ.clone()*prec_P.P_XZ.clone(),
cc.c_ZZ.clone()*           prec_P.P_ZZplusYZ.clone(),
        );
        f = f.squared() * g_RR_at_P;
        if bit {
            cc = prec_Q.0[idx].clone();
            idx += 1;
            let g_RQ_at_P = edwards_Fq6::new(
cc.c_ZZ.clone()*               prec_P.P_ZZplusYZ.clone(),
 cc.c_XY.clone()*               prec_P.P_XY.clone() + cc.c_XZ*prec_P.P_XZ.clone(),
            );
            f = f * g_RQ_at_P;
        }
    }
    leave_block("Call to edwards_ate_miller_loop", false);

    f
}

pub fn edwards_ate_double_miller_loop(
    prec_P1: &edwards_ate_G1_precomp,
    prec_Q1: &edwards_ate_G2_precomp,
    prec_P2: &edwards_ate_G1_precomp,
    prec_Q2: &edwards_ate_G2_precomp,
) -> edwards_Fq6 {
    enter_block("Call to edwards_ate_double_miller_loop", false);
    let loop_count: bigint<{edwards_Fr::num_limbs}> = edwards_ate_loop_count;

    let mut f = edwards_Fq6::one();

    let mut found_one = false;
    let mut idx = 0;
    for i in (0..=loop_count.max_bits() - 1).rev() {
        let mut bit = loop_count.test_bit(i);
        if !found_one {
            /* this skips the MSB itself */
            found_one |= bit;
            continue;
        }

        /* code below gets executed for all bits (EXCEPT the MSB itself) of
        edwards_param_p (skipping leading zeros) in MSB to LSB
        order */
        let mut cc1 = prec_Q1.0[idx].clone();
        let mut cc2 = prec_Q2.0[idx].clone();
        idx += 1;

        let g_RR_at_P1 = edwards_Fq6::new(
  cc1.c_XY.clone()*           prec_P1.P_XY.clone() +  cc1.c_XZ.clone()*prec_P1.P_XZ.clone() ,
 cc1.c_ZZ.clone()*           prec_P1.P_ZZplusYZ.clone() ,
        );

        let g_RR_at_P2 = edwards_Fq6::new(
 cc2.c_XY.clone()*           prec_P2.P_XY.clone() + cc2.c_XZ.clone()*prec_P2.P_XZ.clone(),
cc2.c_ZZ.clone()*           prec_P2.P_ZZplusYZ.clone(),
        );
        f = f.squared() * g_RR_at_P1 * g_RR_at_P2;

        if bit {
            cc1 = prec_Q1.0[idx].clone();
            cc2 = prec_Q2.0[idx].clone();
            idx += 1;
            let g_RQ_at_P1 = edwards_Fq6::new(
 cc1.c_ZZ.clone()*               prec_P1.P_ZZplusYZ.clone(),
  cc1.c_XY.clone()*               prec_P1.P_XY.clone() +  cc1.c_XZ.clone()*prec_P1.P_XZ.clone() ,
            );
            let g_RQ_at_P2 = edwards_Fq6::new(
cc2.c_ZZ.clone()*               prec_P2.P_ZZplusYZ.clone(),
 cc2.c_XY.clone()*               prec_P2.P_XY.clone() + cc2.c_XZ.clone()*prec_P2.P_XZ.clone(),
            );
            f = f * g_RQ_at_P1 * g_RQ_at_P2;
        }
    }
    leave_block("Call to edwards_ate_double_miller_loop", false);

    f
}

pub fn edwards_ate_pairing(P: &edwards_G1, Q: &edwards_G2) -> edwards_Fq6 {
    enter_block("Call to edwards_ate_pairing", false);
    let prec_P = edwards_ate_precompute_G1(P);
    let prec_Q = edwards_ate_precompute_G2(Q);
    let result = edwards_ate_miller_loop(&prec_P, &prec_Q);
    leave_block("Call to edwards_ate_pairing", false);
    result
}

pub fn edwards_ate_reduced_pairing(P: &edwards_G1, Q: &edwards_G2) -> edwards_GT {
    enter_block("Call to edwards_ate_reduced_pairing", false);
    let f = edwards_ate_pairing(P, Q);
    let result = edwards_final_exponentiation(&f);
    leave_block("Call to edwards_ate_reduced_pairing", false);
    result
}

pub fn edwards_precompute_G1(P: &edwards_G1) -> edwards_G1_precomp {
    return edwards_ate_precompute_G1(P);
}

pub fn edwards_precompute_G2(Q: &edwards_G2) -> edwards_G2_precomp {
    return edwards_ate_precompute_G2(Q);
}

pub fn edwards_miller_loop(
    prec_P: &edwards_G1_precomp,
    prec_Q: &edwards_G2_precomp,
) -> edwards_Fq6 {
    return edwards_ate_miller_loop(prec_P, prec_Q);
}

pub fn edwards_double_miller_loop(
    prec_P1: &edwards_G1_precomp,
    prec_Q1: &edwards_G2_precomp,
    prec_P2: &edwards_G1_precomp,
    prec_Q2: &edwards_G2_precomp,
) -> edwards_Fq6 {
    return edwards_ate_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
}

pub fn edwards_pairing(P: &edwards_G1, Q: &edwards_G2) -> edwards_Fq6 {
    return edwards_ate_pairing(P, Q);
}

pub fn edwards_reduced_pairing(P: &edwards_G1, Q: &edwards_G2) -> edwards_GT {
    return edwards_ate_reduced_pairing(P, Q);
}

// use std::io::{self, Read, Write};

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct EdwardsFqConicCoefficients {
//     pub c_zz: Fq,
//     pub c_xy: Fq,
//     pub c_xz: Fq,
// }

// impl EdwardsFqConicCoefficients {
//     pub fn serialize<W: Write>(&self, mut writer: W) -> io::Result<()> {
//         writer.write_all(&self.c_zz.to_bytes())?;
//         writer.write_all(b" ")?;
//         writer.write_all(&self.c_xy.to_bytes())?;
//         writer.write_all(b" ")?;
//         writer.write_all(&self.c_xz.to_bytes())?;
//         Ok(())
//     }
// }

// pub type EdwardsTateG1Precomp = Vec<EdwardsFqConicCoefficients>;

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct EdwardsTateG2Precomp {
//     pub y0: Fq,
//     pub eta: Fq,
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct EdwardsFq3ConicCoefficients {
//     pub c_zz: Fq3,
//     pub c_xy: Fq3,
//     pub c_xz: Fq3,
// }

// pub type EdwardsAteG2Precomp = Vec<EdwardsFq3ConicCoefficients>;

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct EdwardsAteG1Precomp {
//     pub p_xy: Fq,
//     pub p_xz: Fq,
//     pub p_zz_plus_yz: Fq,
// }

// impl EdwardsAteG1Precomp {
//     pub fn serialize<W: Write>(&self, mut writer: W) -> io::Result<()> {
//         writer.write_all(&self.p_xy.to_bytes())?;
//         writer.write_all(b" ")?;
//         writer.write_all(&self.p_xz.to_bytes())?;
//         writer.write_all(b" ")?;
//         writer.write_all(&self.p_zz_plus_yz.to_bytes())?;
//         Ok(())
//     }

//     pub fn deserialize<R: Read>(mut reader: R) -> io::Result<Self> {

//         let p_xy = Fq::read(&mut reader)?;
//         let p_xz = Fq::read(&mut reader)?;
//         let p_zz_plus_yz = Fq::read(&mut reader)?;
//         Ok(Self {
//             p_xy,
//             p_xz,
//             p_zz_plus_yz,
//         })
//     }
// }

// pub fn serialize_precomp_vec<W: Write, T>(
//     writer: &mut W,
//     vec: &Vec<T>,
//     serialize_item: fn(&T, &mut W) -> io::Result<()>,
// ) -> io::Result<()> {
//     writer.write_all(vec.len().to_string().as_bytes())?;
//     writer.write_all(b"\n")?;
//     for item in vec {
//         serialize_item(item, writer)?;
//         writer.write_all(b"\n")?;
//     }
//     Ok(())
// }
