//  Declaration of interfaces for pairing operations on MNT6.

use crate::algebra::curves::mnt::mnt6::mnt6_fields::{
    mnt6_Fq, mnt6_Fq3, mnt6_Fq6, mnt6_Fr, mnt6_GT,
};
use crate::algebra::curves::mnt::mnt6::mnt6_g1::mnt6_G1;
use crate::algebra::curves::mnt::mnt6::mnt6_g2::mnt6_G2;
use crate::algebra::curves::mnt::mnt6::mnt6_init::{
    mnt6_ate_is_loop_count_neg, mnt6_ate_loop_count, mnt6_final_exponent_last_chunk_abs_of_w0,
    mnt6_final_exponent_last_chunk_is_w0_neg, mnt6_final_exponent_last_chunk_w1, mnt6_twist,
    mnt6_twist_coeff_a,
};
use crate::{CoeffsConfig, affine_ate_G_precomp_typeConfig};
use ffec::common::profiling::{enter_block, leave_block};
use ffec::field_utils::bigint::bigint;
use ffec::scalar_multiplication::wnaf::find_wnaf;

const num_limbs: usize = 5;

/* affine ate miller loop */
#[derive(Clone, Debug, Default, PartialEq)]
pub struct mnt6_affine_ate_G1_precomputation {
    pub PX: mnt6_Fq,
    pub PY: mnt6_Fq,
    pub PY_twist_squared: mnt6_Fq3,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct mnt6_affine_ate_coeffs {
    // TODO: trim (not all of them are needed)
    pub old_RX: mnt6_Fq3,
    pub old_RY: mnt6_Fq3,
    pub gamma: mnt6_Fq3,
    pub gamma_twist: mnt6_Fq3,
    pub gamma_X: mnt6_Fq3,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct mnt6_affine_ate_G2_precomputation {
    pub QX: mnt6_Fq3,
    pub QY: mnt6_Fq3,
    pub coeffs: Vec<mnt6_affine_ate_coeffs>,
}

/* ate pairing */
#[derive(Clone, Debug, Default, PartialEq)]
pub struct mnt6_ate_G1_precomp {
    pub PX: mnt6_Fq,
    pub PY: mnt6_Fq,
    pub PX_twist: mnt6_Fq3,
    pub PY_twist: mnt6_Fq3,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct mnt6_ate_dbl_coeffs {
    pub c_H: mnt6_Fq3,
    pub c_4C: mnt6_Fq3,
    pub c_J: mnt6_Fq3,
    pub c_L: mnt6_Fq3,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct mnt6_ate_add_coeffs {
    pub c_L1: mnt6_Fq3,
    pub c_RZ: mnt6_Fq3,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct mnt6_ate_G2_precomp {
    pub QX: mnt6_Fq3,
    pub QY: mnt6_Fq3,
    pub QY2: mnt6_Fq3,
    pub QX_over_twist: mnt6_Fq3,
    pub QY_over_twist: mnt6_Fq3,
    pub dbl_coeffs: Vec<mnt6_ate_dbl_coeffs>,
    pub add_coeffs: Vec<mnt6_ate_add_coeffs>,
}

/* choice of pairing */

pub type mnt6_G1_precomp = mnt6_ate_G1_precomp;
pub type mnt6_G2_precomp = mnt6_ate_G2_precomp;

impl affine_ate_G_precomp_typeConfig for mnt6_affine_ate_G1_precomputation {
    type CC = mnt6_ate_add_coeffs;
}
impl CoeffsConfig for mnt6_ate_add_coeffs {}
impl affine_ate_G_precomp_typeConfig for mnt6_affine_ate_G2_precomputation {
    type CC = mnt6_ate_add_coeffs;
}

use std::fmt;
impl fmt::Display for mnt6_affine_ate_G1_precomputation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}
impl fmt::Display for mnt6_affine_ate_coeffs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}

impl fmt::Display for mnt6_affine_ate_G2_precomputation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}
impl fmt::Display for mnt6_ate_G1_precomp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}

impl fmt::Display for mnt6_ate_dbl_coeffs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}
impl fmt::Display for mnt6_ate_add_coeffs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}

impl fmt::Display for mnt6_ate_G2_precomp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}

/* final exponentiations */

pub fn mnt6_final_exponentiation_last_chunk(elt: &mnt6_Fq6, elt_inv: &mnt6_Fq6) -> mnt6_Fq6 {
    enter_block("Call to mnt6_final_exponentiation_last_chunk", false);
    let elt_q = elt.Frobenius_map(1);
    let w1_part = elt_q.cyclotomic_exp(&mnt6_final_exponent_last_chunk_w1);
    let w0_part = if mnt6_final_exponent_last_chunk_is_w0_neg {
        elt_inv.cyclotomic_exp(&mnt6_final_exponent_last_chunk_abs_of_w0)
    } else {
        elt.cyclotomic_exp(&mnt6_final_exponent_last_chunk_abs_of_w0)
    };
    let result = w1_part * w0_part;
    leave_block("Call to mnt6_final_exponentiation_last_chunk", false);

    result
}

pub fn mnt6_final_exponentiation_first_chunk(elt: &mnt6_Fq6, elt_inv: &mnt6_Fq6) -> mnt6_Fq6 {
    enter_block("Call to mnt6_final_exponentiation_first_chunk", false);

    /* (q^3-1)*(q+1) */

    /* elt_q3 = elt^(q^3) */
    let elt_q3 = elt.Frobenius_map(3);
    /* elt_q3_over_elt = elt^(q^3-1) */
    let elt_q3_over_elt = elt_q3 * elt_inv;
    /* alpha = elt^((q^3-1) * q) */
    let alpha = elt_q3_over_elt.Frobenius_map(1);
    /* beta = elt^((q^3-1)*(q+1) */
    let beta = alpha * elt_q3_over_elt;
    leave_block("Call to mnt6_final_exponentiation_first_chunk", false);
    return beta;
}

pub fn mnt6_final_exponentiation(elt: &mnt6_Fq6) -> mnt6_GT {
    enter_block("Call to mnt6_final_exponentiation", false);
    let elt_inv = elt.inverse();
    let elt_to_first_chunk = mnt6_final_exponentiation_first_chunk(elt, &elt_inv);
    let elt_inv_to_first_chunk = mnt6_final_exponentiation_first_chunk(&elt_inv, elt);
    let result = mnt6_final_exponentiation_last_chunk(&elt_to_first_chunk, &elt_inv_to_first_chunk);
    leave_block("Call to mnt6_final_exponentiation", false);

    result
}

/* affine ate miller loop */

pub fn mnt6_affine_ate_precompute_G1(P: &mnt6_G1) -> mnt6_affine_ate_G1_precomputation {
    enter_block("Call to mnt6_affine_ate_precompute_G1", false);

    let mut Pcopy = P.clone();
    Pcopy.to_affine_coordinates();

    let mut result = mnt6_affine_ate_G1_precomputation::default();
    result.PX = Pcopy.X;
    result.PY = Pcopy.Y;
    result.PY_twist_squared = mnt6_twist.squared() * Pcopy.Y.clone();

    leave_block("Call to mnt6_affine_ate_precompute_G1", false);
    result
}

pub fn mnt6_affine_ate_precompute_G2(Q: &mnt6_G2) -> mnt6_affine_ate_G2_precomputation {
    enter_block("Call to mnt6_affine_ate_precompute_G2", false);

    let mut Qcopy = Q.clone();
    Qcopy.to_affine_coordinates();

    let mut result = mnt6_affine_ate_G2_precomputation::default();
    result.QX = Qcopy.X.clone();
    result.QY = Qcopy.Y.clone();

    let mut RX = Qcopy.X.clone();
    let mut RY = Qcopy.Y.clone();

    let loop_count: bigint<num_limbs> = mnt6_ate_loop_count;
    let mut found_nonzero = false;

    let NAF = find_wnaf(1, loop_count);
    for i in (0..=NAF.len() - 1).rev() {
        if !found_nonzero {
            /* this skips the MSB itself */
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        let mut c = mnt6_affine_ate_coeffs::default();
        c.old_RX = RX;
        c.old_RY = RY;
        let old_RX_2 = c.old_RX.squared();
        c.gamma =
            (old_RX_2 + old_RX_2 + old_RX_2 + mnt6_twist_coeff_a) * (c.old_RY + c.old_RY).inverse();
        c.gamma_twist = c.gamma * mnt6_twist;
        c.gamma_X = c.gamma * c.old_RX;
        result.coeffs.push(c.clone());

        RX = c.gamma.squared() - (c.old_RX + c.old_RX);
        RY = c.gamma * (c.old_RX - RX) - c.old_RY;

        if NAF[i] != 0 {
            let mut c = mnt6_affine_ate_coeffs::default();
            c.old_RX = RX;
            c.old_RY = RY;
            if NAF[i] > 0 {
                c.gamma = (c.old_RY - result.QY) * (c.old_RX - result.QX).inverse();
            } else {
                c.gamma = (c.old_RY + result.QY) * (c.old_RX - result.QX).inverse();
            }
            c.gamma_twist = c.gamma * mnt6_twist;
            c.gamma_X = c.gamma * result.QX;
            result.coeffs.push(c.clone());

            RX = c.gamma.squared() - (c.old_RX + result.QX);
            RY = c.gamma * (c.old_RX - RX) - c.old_RY;
        }
    }

    /* TODO: maybe handle neg
    if mnt6_ate_is_loop_count_neg
    {
        let mut ac=mnt6_ate_add_coeffs::default();
        let mut c=mnt6_affine_ate_dbl_coeffs::default();
        c.old_RX = RX;
        c.old_RY = -RY;
        old_RX_2 = c.old_RY.squared();
        c.gamma = (old_RX_2 + old_RX_2 + old_RX_2 + mnt6_coeff_a) * (c.old_RY + c.old_RY).inverse();
        c.gamma_twist = c.gamma * mnt6_twist;
        c.gamma_X = c.gamma * c.old_RX;
        result.coeffs.push(c);
    }
    */

    leave_block("Call to mnt6_affine_ate_precompute_G2", false);
    result
}

pub fn mnt6_affine_ate_miller_loop(
    prec_P: &mnt6_affine_ate_G1_precomputation,
    prec_Q: &mnt6_affine_ate_G2_precomputation,
) -> mnt6_Fq6 {
    enter_block("Call to mnt6_affine_ate_miller_loop", false);

    let mut f = mnt6_Fq6::one();

    let loop_count: bigint<num_limbs> = mnt6_ate_loop_count;
    let mut found_nonzero = false;
    let mut idx = 0;

    let mut NAF = find_wnaf(1, loop_count);
    for i in (0..=NAF.len() - 1).rev() {
        if !found_nonzero {
            /* this skips the MSB itself */
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        /* code below gets executed for all bits (EXCEPT the MSB itself) of
        mnt6_param_p (skipping leading zeros) in MSB to LSB
        order */
        let mut c = prec_Q.coeffs[idx].clone();
        idx += 1;

        let mut g_RR_at_P = mnt6_Fq6::new(
            prec_P.PY_twist_squared,
            c.gamma_twist.clone() * (-prec_P.PX).clone() + c.gamma_X - c.old_RY,
        );
        f = f.squared().mul_by_2345(&g_RR_at_P);

        if NAF[i] != 0 {
            let mut c = prec_Q.coeffs[idx].clone();
            idx += 1;
            let mut g_RQ_at_P = mnt6_Fq6::default();
            if NAF[i] > 0 {
                g_RQ_at_P = mnt6_Fq6::new(
                    prec_P.PY_twist_squared,
                    c.gamma_twist.clone() * -prec_P.PX.clone() + c.gamma_X - prec_Q.QY,
                );
            } else {
                g_RQ_at_P = mnt6_Fq6::new(
                    prec_P.PY_twist_squared,
                    c.gamma_twist.clone() * -prec_P.PX.clone() + c.gamma_X + prec_Q.QY,
                );
            }
            f = f.mul_by_2345(&g_RQ_at_P);
        }
    }

    /* TODO: maybe handle neg
    if mnt6_ate_is_loop_count_neg
    {
        // TODO:
        mnt6_affine_ate_coeffs ac = prec_Q.coeffs[idx];idx+=1;
        mnt6_Fq6 g_RnegR_at_P = mnt6_Fq6::new(prec_P.PY_twist_squared,
                                          - prec_P.PX * c.gamma_twist + c.gamma_X - c.old_RY);
        f = (f * g_RnegR_at_P).inverse();
    }
    */

    leave_block("Call to mnt6_affine_ate_miller_loop", false);

    f
}

/* ate pairing */
#[derive(Clone, Debug, Default, PartialEq)]
pub struct extended_mnt6_G2_projective {
    pub X: mnt6_Fq3,
    pub Y: mnt6_Fq3,
    pub Z: mnt6_Fq3,
    pub T: mnt6_Fq3,
}
impl extended_mnt6_G2_projective {
    pub fn print(&self) {
        print!("extended mnt6_G2 projective X/Y/Z/T:\n");
        self.X.print();
        self.Y.print();
        self.Z.print();
        self.T.print();
    }

    pub fn test_invariant(&self) {
        assert!(self.T == self.Z.squared());
    }
}

pub fn doubling_step_for_flipped_miller_loop(
    current: &mut extended_mnt6_G2_projective,
    dc: &mut mnt6_ate_dbl_coeffs,
) {
    let X = current.X.clone();
    let mut Y = current.Y.clone();
    let mut Z = current.Z.clone();
    let mut T = current.T.clone();

    let A = T.squared(); // A = T1^2
    let B = X.squared(); // B = X1^2
    let C = Y.squared(); // C = Y1^2
    let D = C.squared(); // D = C^2
    let E = (X + C).squared() - B - D; // E = (X1+C)^2-B-D
    let F = (B + B + B) + mnt6_twist_coeff_a * A; // F = 3*B +  a  *A
    let G = F.squared(); // G = F^2

    current.X = -(E + E + E + E) + G; // X3 = -4*E+G
    current.Y = D * -mnt6_Fq::from("8") + F * (E + E - current.X); // Y3 = -8*D+F*(2*E-X3)
    current.Z = (Y + Z).squared() - C - Z.squared(); // Z3 = (Y1+Z1)^2-C-Z1^2
    current.T = current.Z.squared(); // T3 = Z3^2

    dc.c_H = (current.Z + T).squared() - current.T - A; // H = (Z3+T1)^2-T3-A
    dc.c_4C = C + C + C + C; // fourC = 4*C
    dc.c_J = (F + T).squared() - G - A; // J = (F+T1)^2-G-A
    dc.c_L = (F + X).squared() - G - B; // L = (F+X1)^2-G-B

    // #ifdef DEBUG
    current.test_invariant();
}

pub fn mixed_addition_step_for_flipped_miller_loop(
    base_X: mnt6_Fq3,
    base_Y: mnt6_Fq3,
    base_Y_squared: mnt6_Fq3,
    mut current: extended_mnt6_G2_projective,
    mut ac: mnt6_ate_add_coeffs,
) {
    let X1 = current.X.clone();
    let Y1 = current.Y.clone();
    let Z1 = current.Z.clone();
    let T1 = current.T.clone();
    let x2: mnt6_Fq3 = base_X.clone();
    let y2 = base_Y.clone();
    let y2_squared = base_Y_squared;

    let B = x2 * T1; // B = x2 * T1
    let D = ((y2 + Z1).squared() - y2_squared - T1) * T1; // D = ((y2 + Z1)^2 - y2squared - T1) * T1
    let H = B - X1; // H = B - X1
    let I = H.squared(); // I = H^2
    let E = I + I + I + I; // E = 4*I
    let J = H * E; // J = H * E
    let V = X1 * E; // V = X1 * E
    let L1 = D - (Y1 + Y1); // L1 = D - 2 * Y1

    current.X = L1.squared() - J - (V + V); // X3 = L1^2 - J - 2*V
    current.Y = L1 * (V - current.X) - (Y1 + Y1) * J; // Y3 = L1 * (V-X3) - 2*Y1 * J
    current.Z = (Z1 + H).squared() - T1 - I; // Z3 = (Z1 + H)^2 - T1 - I
    current.T = current.Z.squared(); // T3 = Z3^2

    ac.c_L1 = L1;
    ac.c_RZ = current.Z;
    // #ifdef DEBUG
    current.test_invariant();
}

pub fn mnt6_ate_precompute_G1(P: &mnt6_G1) -> mnt6_ate_G1_precomp {
    enter_block("Call to mnt6_ate_precompute_G1", false);

    let mut Pcopy = P.clone();
    Pcopy.to_affine_coordinates();

    let mut result = mnt6_ate_G1_precomp::default();
    result.PX = Pcopy.X;
    result.PY = Pcopy.Y;
    result.PX_twist = mnt6_twist.clone() * Pcopy.X.clone();
    result.PY_twist = mnt6_twist.clone() * Pcopy.Y.clone();

    leave_block("Call to mnt6_ate_precompute_G1", false);
    result
}

pub fn mnt6_ate_precompute_G2(Q: &mnt6_G2) -> mnt6_ate_G2_precomp {
    enter_block("Call to mnt6_ate_precompute_G2", false);

    let mut Qcopy: mnt6_G2 = Q.clone();
    Qcopy.to_affine_coordinates();

    let mnt6_twist_inv = mnt6_twist.inverse(); // could add to global params if needed

    let mut result = mnt6_ate_G2_precomp::default();
    result.QX = Qcopy.X;
    result.QY = Qcopy.Y;
    result.QY2 = Qcopy.Y.squared();
    result.QX_over_twist = Qcopy.X * mnt6_twist_inv;
    result.QY_over_twist = Qcopy.Y * mnt6_twist_inv;

    let mut R = extended_mnt6_G2_projective::default();
    R.X = Qcopy.X;
    R.Y = Qcopy.Y;
    R.Z = mnt6_Fq3::one();
    R.T = mnt6_Fq3::one();

    let loop_count: bigint<num_limbs> = mnt6_ate_loop_count;
    let mut found_one = false;
    for i in (0..=loop_count.max_bits() - 1).rev() {
        let mut bit = loop_count.test_bit(i);

        if !found_one {
            /* this skips the MSB itself */
            found_one |= bit;
            continue;
        }

        let mut dc = mnt6_ate_dbl_coeffs::default();
        doubling_step_for_flipped_miller_loop(&mut R, &mut dc);
        result.dbl_coeffs.push(dc);

        if bit {
            let mut ac = mnt6_ate_add_coeffs::default();
            mixed_addition_step_for_flipped_miller_loop(
                result.QX,
                result.QY,
                result.QY2,
                R.clone(),
                ac.clone(),
            );
            result.add_coeffs.push(ac);
        }
    }

    if mnt6_ate_is_loop_count_neg {
        let mut RZ_inv = R.Z.inverse();
        let mut RZ2_inv = RZ_inv.squared();
        let mut RZ3_inv = RZ2_inv * RZ_inv;
        let mut minus_R_affine_X = R.X.clone() * RZ2_inv;
        let mut minus_R_affine_Y = -R.Y.clone() * RZ3_inv;
        let mut minus_R_affine_Y2 = minus_R_affine_Y.squared();
        let ac = mnt6_ate_add_coeffs::default();
        mixed_addition_step_for_flipped_miller_loop(
            minus_R_affine_X,
            minus_R_affine_Y,
            minus_R_affine_Y2,
            R,
            ac.clone(),
        );
        result.add_coeffs.push(ac);
    }

    leave_block("Call to mnt6_ate_precompute_G2", false);
    result
}

pub fn mnt6_ate_miller_loop(
    prec_P: &mnt6_ate_G1_precomp,
    prec_Q: &mnt6_ate_G2_precomp,
) -> mnt6_Fq6 {
    enter_block("Call to mnt6_ate_miller_loop", false);

    let L1_coeff =
        mnt6_Fq3::new(prec_P.PX, mnt6_Fq::zero(), mnt6_Fq::zero()) - prec_Q.QX_over_twist;

    let mut f = mnt6_Fq6::one();

    let mut found_one = false;
    let mut dbl_idx = 0;
    let mut add_idx = 0;

    let mut loop_count: bigint<num_limbs> = mnt6_ate_loop_count;

    for i in (0..=loop_count.max_bits() - 1).rev() {
        let mut bit = loop_count.test_bit(i);

        if !found_one {
            /* this skips the MSB itself */
            found_one |= bit;
            continue;
        }

        /* code below gets executed for all bits (EXCEPT the MSB itself) of
        mnt6_param_p (skipping leading zeros) in MSB to LSB
        order */
        let mut dc = prec_Q.dbl_coeffs[dbl_idx].clone();
        dbl_idx += 1;

        let mut g_RR_at_P = mnt6_Fq6::new(
            -dc.c_4C - dc.c_J * prec_P.PX_twist + dc.c_L,
            dc.c_H * prec_P.PY_twist,
        );
        f = f.squared() * g_RR_at_P;

        if bit {
            let mut ac = prec_Q.add_coeffs[add_idx].clone();
            add_idx += 1;
            let mut g_RQ_at_P = mnt6_Fq6::new(
                ac.c_RZ * prec_P.PY_twist,
                -(prec_Q.QY_over_twist * ac.c_RZ + L1_coeff * ac.c_L1),
            );
            f = f * g_RQ_at_P;
        }
    }

    if mnt6_ate_is_loop_count_neg {
        let mut ac = prec_Q.add_coeffs[add_idx].clone();
        add_idx += 1;
        let mut g_RnegR_at_P = mnt6_Fq6::new(
            ac.c_RZ * prec_P.PY_twist,
            -(prec_Q.QY_over_twist * ac.c_RZ + L1_coeff * ac.c_L1),
        );
        f = (f * g_RnegR_at_P).inverse();
    }

    leave_block("Call to mnt6_ate_miller_loop", false);

    f
}

pub fn mnt6_ate_double_miller_loop(
    prec_P1: &mnt6_ate_G1_precomp,
    prec_Q1: &mnt6_ate_G2_precomp,
    prec_P2: &mnt6_ate_G1_precomp,
    prec_Q2: &mnt6_ate_G2_precomp,
) -> mnt6_Fq6 {
    enter_block("Call to mnt6_ate_double_miller_loop", false);

    let mut L1_coeff1 =
        mnt6_Fq3::new(prec_P1.PX, mnt6_Fq::zero(), mnt6_Fq::zero()) - prec_Q1.QX_over_twist;
    let mut L1_coeff2 =
        mnt6_Fq3::new(prec_P2.PX, mnt6_Fq::zero(), mnt6_Fq::zero()) - prec_Q2.QX_over_twist;

    let mut f = mnt6_Fq6::one();

    let mut found_one = false;
    let mut dbl_idx = 0;
    let mut add_idx = 0;

    let mut loop_count: bigint<num_limbs> = mnt6_ate_loop_count;

    for i in (0..=loop_count.max_bits() - 1).rev() {
        let mut bit = loop_count.test_bit(i);

        if !found_one {
            /* this skips the MSB itself */
            found_one |= bit;
            continue;
        }

        /* code below gets executed for all bits (EXCEPT the MSB itself) of
        mnt6_param_p (skipping leading zeros) in MSB to LSB
        order */
        let mut dc1 = prec_Q1.dbl_coeffs[dbl_idx].clone();
        let mut dc2 = prec_Q2.dbl_coeffs[dbl_idx].clone();
        dbl_idx += 1;

        let mut g_RR_at_P1 = mnt6_Fq6::new(
            -dc1.c_4C - dc1.c_J * prec_P1.PX_twist + dc1.c_L,
            dc1.c_H * prec_P1.PY_twist,
        );

        let mut g_RR_at_P2 = mnt6_Fq6::new(
            -dc2.c_4C - dc2.c_J * prec_P2.PX_twist + dc2.c_L,
            dc2.c_H * prec_P2.PY_twist,
        );

        f = f.squared() * g_RR_at_P1 * g_RR_at_P2;

        if bit {
            let mut ac1 = prec_Q1.add_coeffs[add_idx].clone();
            let mut ac2 = prec_Q2.add_coeffs[add_idx].clone();
            add_idx += 1;

            let mut g_RQ_at_P1 = mnt6_Fq6::new(
                ac1.c_RZ * prec_P1.PY_twist,
                -(prec_Q1.QY_over_twist * ac1.c_RZ + L1_coeff1 * ac1.c_L1),
            );
            let mut g_RQ_at_P2 = mnt6_Fq6::new(
                ac2.c_RZ * prec_P2.PY_twist,
                -(prec_Q2.QY_over_twist * ac2.c_RZ + L1_coeff2 * ac2.c_L1),
            );

            f = f * g_RQ_at_P1 * g_RQ_at_P2;
        }
    }

    if mnt6_ate_is_loop_count_neg {
        let mut ac1 = prec_Q1.add_coeffs[add_idx].clone();
        let mut ac2 = prec_Q2.add_coeffs[add_idx].clone();
        add_idx += 1;
        let mut g_RnegR_at_P1 = mnt6_Fq6::new(
            ac1.c_RZ * prec_P1.PY_twist,
            -(prec_Q1.QY_over_twist * ac1.c_RZ + L1_coeff1 * ac1.c_L1),
        );
        let mut g_RnegR_at_P2 = mnt6_Fq6::new(
            ac2.c_RZ * prec_P2.PY_twist,
            -(prec_Q2.QY_over_twist * ac2.c_RZ + L1_coeff2 * ac2.c_L1),
        );

        f = (f * g_RnegR_at_P1 * g_RnegR_at_P2).inverse();
    }

    leave_block("Call to mnt6_ate_double_miller_loop", false);

    f
}

pub fn mnt6_ate_pairing(P: &mnt6_G1, Q: &mnt6_G2) -> mnt6_Fq6 {
    enter_block("Call to mnt6_ate_pairing", false);
    let mut prec_P = mnt6_ate_precompute_G1(P);
    let mut prec_Q = mnt6_ate_precompute_G2(Q);
    let mut result = mnt6_ate_miller_loop(&prec_P, &prec_Q);
    leave_block("Call to mnt6_ate_pairing", false);
    result
}

pub fn mnt6_ate_reduced_pairing(P: &mnt6_G1, Q: &mnt6_G2) -> mnt6_GT {
    enter_block("Call to mnt6_ate_reduced_pairing", false);
    let f = mnt6_ate_pairing(P, Q);
    let result = mnt6_final_exponentiation(&f);
    leave_block("Call to mnt6_ate_reduced_pairing", false);
    result
}

pub fn mnt6_precompute_G1(P: &mnt6_G1) -> mnt6_G1_precomp {
    return mnt6_ate_precompute_G1(P);
}

pub fn mnt6_precompute_G2(Q: &mnt6_G2) -> mnt6_G2_precomp {
    return mnt6_ate_precompute_G2(Q);
}

pub fn mnt6_miller_loop(prec_P: &mnt6_G1_precomp, prec_Q: &mnt6_G2_precomp) -> mnt6_Fq6 {
    return mnt6_ate_miller_loop(prec_P, prec_Q);
}

pub fn mnt6_double_miller_loop(
    prec_P1: &mnt6_G1_precomp,
    prec_Q1: &mnt6_G2_precomp,
    prec_P2: &mnt6_G1_precomp,
    prec_Q2: &mnt6_G2_precomp,
) -> mnt6_Fq6 {
    return mnt6_ate_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2);
}

pub fn mnt6_pairing(P: &mnt6_G1, Q: &mnt6_G2) -> mnt6_Fq6 {
    return mnt6_ate_pairing(P, Q);
}

pub fn mnt6_reduced_pairing(P: &mnt6_G1, Q: &mnt6_G2) -> mnt6_GT {
    return mnt6_ate_reduced_pairing(P, Q);
}

pub fn mnt6_affine_reduced_pairing(P: &mnt6_G1, Q: &mnt6_G2) -> mnt6_GT {
    let prec_P = mnt6_affine_ate_precompute_G1(P);
    let prec_Q = mnt6_affine_ate_precompute_G2(Q);
    let f = mnt6_affine_ate_miller_loop(&prec_P, &prec_Q);
    let result = mnt6_final_exponentiation(&f);
    result
}

// use std::io::{self, Read, Write};

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct Mnt6AteG1Precomp {
//     pub px: Fq,
//     pub py: Fq,
//     pub px_twist: Fq3,
//     pub py_twist: Fq3,
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct Mnt6AteDblCoeffs {
//     pub c_h: Fq3,
//     pub c_4c: Fq3,
//     pub c_j: Fq3,
//     pub c_l: Fq3,
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct Mnt6AteAddCoeffs {
//     pub c_l1: Fq3,
//     pub c_rz: Fq3,
// }

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct Mnt6AteG2Precomp {
//     pub qx: Fq3,
//     pub qy: Fq3,
//     pub qy2: Fq3,
//     pub qx_over_twist: Fq3,
//     pub qy_over_twist: Fq3,
//     pub dbl_coeffs: Vec<Mnt6AteDblCoeffs>,
//     pub add_coeffs: Vec<Mnt6AteAddCoeffs>,
// }

// impl Mnt6AteG1Precomp {
//     pub fn serialize<W: Write>(&self, mut writer: W) -> io::Result<()> {
//         writer.write_all(&self.px.to_bytes())?;
//         writer.write_all(b" ")?;
//         writer.write_all(&self.py.to_bytes())?;
//         writer.write_all(b" ")?;
//         writer.write_all(&self.px_twist.to_bytes())?;
//         writer.write_all(b" ")?;
//         writer.write_all(&self.py_twist.to_bytes())?;
//         Ok(())
//     }
// }

// impl Mnt6AteDblCoeffs {
//     pub fn serialize<W: Write>(&self, mut writer: W) -> io::Result<()> {
//         let coords = [&self.c_h, &self.c_4c, &self.c_j, &self.c_l];
//         for (i, c) in coords.iter().enumerate() {
//             writer.write_all(&c.to_bytes())?;
//             if i < 3 {
//                 writer.write_all(b" ")?;
//             }
//         }
//         Ok(())
//     }
// }

// impl Mnt6AteG2Precomp {
//     pub fn serialize<W: Write>(&self, mut writer: W) -> io::Result<()> {
//         let coords = [
//             &self.qx,
//             &self.qy,
//             &self.qy2,
//             &self.qx_over_twist,
//             &self.qy_over_twist,
//         ];
//         for c in coords {
//             writer.write_all(&c.to_bytes())?;
//             writer.write_all(b" ")?;
//         }
//         writer.write_all(b"\n")?;

//         writer.write_all(self.dbl_coeffs.len().to_string().as_bytes())?;
//         writer.write_all(b"\n")?;
//         for dc in &self.dbl_coeffs {
//             dc.serialize(&mut writer)?;
//             writer.write_all(b"\n")?;
//         }

//         writer.write_all(self.add_coeffs.len().to_string().as_bytes())?;
//         writer.write_all(b"\n")?;
//         for ac in &self.add_coeffs {
//             writer.write_all(&ac.c_l1.to_bytes())?;
//             writer.write_all(b" ")?;
//             writer.write_all(&ac.c_rz.to_bytes())?;
//             writer.write_all(b"\n")?;
//         }
//         Ok(())
//     }

//     pub fn deserialize<R: Read>(mut reader: R) -> io::Result<Self> {
//         let qx = Fq3::read(&mut reader)?;
//         let qy = Fq3::read(&mut reader)?;
//         let qy2 = Fq3::read(&mut reader)?;
//         let qx_over_twist = Fq3::read(&mut reader)?;
//         let qy_over_twist = Fq3::read(&mut reader)?;

//         let dbl_s: usize = read_usize(&mut reader)?;
//         let mut dbl_coeffs = Vec::with_capacity(dbl_s);
//         for _ in 0..dbl_s {
//             dbl_coeffs.push(Mnt6AteDblCoeffs {
//                 c_h: Fq3::read(&mut reader)?,
//                 c_4c: Fq3::read(&mut reader)?,
//                 c_j: Fq3::read(&mut reader)?,
//                 c_l: Fq3::read(&mut reader)?,
//             });
//         }

//         let add_s: usize = read_usize(&mut reader)?;
//         let mut add_coeffs = Vec::with_capacity(add_s);
//         for _ in 0..add_s {
//             add_coeffs.push(Mnt6AteAddCoeffs {
//                 c_l1: Fq3::read(&mut reader)?,
//                 c_rz: Fq3::read(&mut reader)?,
//             });
//         }

//         Ok(Self {
//             qx,
//             qy,
//             qy2,
//             qx_over_twist,
//             qy_over_twist,
//             dbl_coeffs,
//             add_coeffs,
//         })
//     }
// }

// fn read_usize<R: Read>(reader: &mut R) -> io::Result<usize> {
//     todo!("Implement integer parsing from reader")
// }
