//  Declaration of interfaces for pairing operations on MNT4.

use crate::algebra::curves::mnt::mnt4::mnt4_fields::{
    mnt4_Fq, mnt4_Fq2, mnt4_Fq4, mnt4_Fr, mnt4_GT,
};
use crate::algebra::curves::mnt::mnt4::mnt4_g1::mnt4_G1;
use crate::algebra::curves::mnt::mnt4::mnt4_g2::mnt4_G2;
use crate::algebra::curves::mnt::mnt4::mnt4_init::{
    mnt4_ate_is_loop_count_neg, mnt4_ate_loop_count, mnt4_final_exponent_last_chunk_abs_of_w0,
    mnt4_final_exponent_last_chunk_is_w0_neg, mnt4_final_exponent_last_chunk_w1, mnt4_twist,
    mnt4_twist_coeff_a,
};
use crate::{CoeffsConfig, affine_ate_G_precomp_typeConfig};
use ffec::PpConfig;
use ffec::common::profiling::{enter_block, leave_block};
use ffec::field_utils::bigint::bigint;
use ffec::scalar_multiplication::wnaf::find_wnaf;

//affine ate miller loop
#[derive(Clone, Debug, Default, PartialEq)]
pub struct mnt4_affine_ate_G1_precomputation {
    pub PX: mnt4_Fq,
    pub PY: mnt4_Fq,
    pub PY_twist_squared: mnt4_Fq2,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct mnt4_affine_ate_coeffs {
    // TODO: trim (not all of them are needed)
    pub old_RX: mnt4_Fq2,
    pub old_RY: mnt4_Fq2,
    pub gamma: mnt4_Fq2,
    pub gamma_twist: mnt4_Fq2,
    pub gamma_X: mnt4_Fq2,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct mnt4_affine_ate_G2_precomputation {
    pub QX: mnt4_Fq2,
    pub QY: mnt4_Fq2,
    pub coeffs: Vec<mnt4_affine_ate_coeffs>,
}

//ate pairing
#[derive(Clone, Debug, Default, PartialEq)]
pub struct mnt4_ate_G1_precomp {
    pub PX: mnt4_Fq,
    pub PY: mnt4_Fq,
    pub PX_twist: mnt4_Fq2,
    pub PY_twist: mnt4_Fq2,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct mnt4_ate_dbl_coeffs {
    pub c_H: mnt4_Fq2,
    pub c_4C: mnt4_Fq2,
    pub c_J: mnt4_Fq2,
    pub c_L: mnt4_Fq2,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct mnt4_ate_add_coeffs {
    pub c_L1: mnt4_Fq2,
    pub c_RZ: mnt4_Fq2,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct mnt4_ate_G2_precomp {
    pub QX: mnt4_Fq2,
    pub QY: mnt4_Fq2,
    pub QY2: mnt4_Fq2,
    pub QX_over_twist: mnt4_Fq2,
    pub QY_over_twist: mnt4_Fq2,
    pub dbl_coeffs: Vec<mnt4_ate_dbl_coeffs>,
    pub add_coeffs: Vec<mnt4_ate_add_coeffs>,
}

//choice of pairing

pub type mnt4_G1_precomp = mnt4_ate_G1_precomp;
pub type mnt4_G2_precomp = mnt4_ate_G2_precomp;

impl affine_ate_G_precomp_typeConfig for mnt4_affine_ate_G1_precomputation {
    type CC = mnt4_ate_add_coeffs;
}
impl CoeffsConfig for mnt4_ate_add_coeffs {}
impl affine_ate_G_precomp_typeConfig for mnt4_affine_ate_G2_precomputation {
    type CC = mnt4_ate_add_coeffs;
}

use std::fmt;
impl fmt::Display for mnt4_affine_ate_G1_precomputation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}
impl fmt::Display for mnt4_affine_ate_coeffs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}

impl fmt::Display for mnt4_affine_ate_G2_precomputation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}
impl fmt::Display for mnt4_ate_G1_precomp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}

impl fmt::Display for mnt4_ate_dbl_coeffs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}
impl fmt::Display for mnt4_ate_add_coeffs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}

impl fmt::Display for mnt4_ate_G2_precomp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1)
    }
}
//final exponentiations

pub fn mnt4_final_exponentiation_last_chunk(elt: &mnt4_Fq4, elt_inv: &mnt4_Fq4) -> mnt4_Fq4 {
    enter_block("Call to mnt4_final_exponentiation_last_chunk", false);
    let elt_q = elt.Frobenius_map(1);
    let w1_part = elt_q.cyclotomic_exp(&mnt4_final_exponent_last_chunk_w1);
    let mut w0_part = if mnt4_final_exponent_last_chunk_is_w0_neg {
        elt_inv.cyclotomic_exp(&mnt4_final_exponent_last_chunk_abs_of_w0)
    } else {
        elt.cyclotomic_exp(&mnt4_final_exponent_last_chunk_abs_of_w0)
    };
    let result = w1_part * w0_part;
    leave_block("Call to mnt4_final_exponentiation_last_chunk", false);

    result
}

pub fn mnt4_final_exponentiation_first_chunk(elt: &mnt4_Fq4, elt_inv: &mnt4_Fq4) -> mnt4_Fq4 {
    enter_block("Call to mnt4_final_exponentiation_first_chunk", false);

    //(q^2-1)

    //elt_q2 = elt^(q^2)
    let elt_q2 = elt.Frobenius_map(2);
    //elt_q3_over_elt = elt^(q^2-1)
    let elt_q2_over_elt = elt_q2 * elt_inv;

    leave_block("Call to mnt4_final_exponentiation_first_chunk", false);
    elt_q2_over_elt
}

pub fn mnt4_final_exponentiation(elt: &mnt4_Fq4) -> mnt4_GT {
    enter_block("Call to mnt4_final_exponentiation", false);
    let elt_inv = elt.inverse();
    let elt_to_first_chunk = mnt4_final_exponentiation_first_chunk(elt, &elt_inv);
    let elt_inv_to_first_chunk = mnt4_final_exponentiation_first_chunk(&elt_inv, elt);
    let result = mnt4_final_exponentiation_last_chunk(&elt_to_first_chunk, &elt_inv_to_first_chunk);
    leave_block("Call to mnt4_final_exponentiation", false);

    result
}

//affine ate miller loop

pub fn mnt4_affine_ate_precompute_G1(P: &mnt4_G1) -> mnt4_affine_ate_G1_precomputation {
    enter_block("Call to mnt4_affine_ate_precompute_G1", false);

    let mut Pcopy = P.clone();
    Pcopy.to_affine_coordinates();

    let mut result = mnt4_affine_ate_G1_precomputation::default();
    result.PX = Pcopy.X;
    result.PY = Pcopy.Y;
    result.PY_twist_squared = mnt4_twist.squared() * Pcopy.Y;

    leave_block("Call to mnt4_affine_ate_precompute_G1", false);
    result
}

pub fn mnt4_affine_ate_precompute_G2(Q: &mnt4_G2) -> mnt4_affine_ate_G2_precomputation {
    enter_block("Call to mnt4_affine_ate_precompute_G2", false);

    let mut Qcopy = Q.clone();
    Qcopy.to_affine_coordinates();

    let mut result = mnt4_affine_ate_G2_precomputation::default();
    result.QX = Qcopy.X;
    result.QY = Qcopy.Y;

    let mut RX = Qcopy.X;
    let mut RY = Qcopy.Y;

    let loop_count: bigint<{ mnt4_Fr::num_limbs }> = mnt4_ate_loop_count;
    let mut found_nonzero = false;

    let NAF = find_wnaf(1, loop_count);
    for i in (0..=NAF.len() - 1).rev() {
        if !found_nonzero {
            //this skips the MSB itself
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        let mut c = mnt4_affine_ate_coeffs::default();
        c.old_RX = RX;
        c.old_RY = RY;
        let old_RX_2 = c.old_RX.squared();
        c.gamma =
            (old_RX_2 + old_RX_2 + old_RX_2 + mnt4_twist_coeff_a) * (c.old_RY + c.old_RY).inverse();
        c.gamma_twist = c.gamma * mnt4_twist;
        c.gamma_X = c.gamma * c.old_RX;
        result.coeffs.push(c.clone());

        RX = c.gamma.squared() - (c.old_RX + c.old_RX);
        RY = c.gamma * (c.old_RX - RX) - c.old_RY;

        if NAF[i] != 0 {
            let mut c = mnt4_affine_ate_coeffs::default();
            c.old_RX = RX;
            c.old_RY = RY;
            if NAF[i] > 0 {
                c.gamma = (c.old_RY - result.QY) * (c.old_RX - result.QX).inverse();
            } else {
                c.gamma = (c.old_RY + result.QY) * (c.old_RX - result.QX).inverse();
            }
            c.gamma_twist = c.gamma * mnt4_twist;
            c.gamma_X = c.gamma * result.QX;
            result.coeffs.push(c.clone());

            RX = c.gamma.squared() - (c.old_RX + result.QX);
            RY = c.gamma * (c.old_RX - RX) - c.old_RY;
        }
    }

    /* TODO: maybe handle neg
       if mnt4_ate_is_loop_count_neg
       {
       ac:mnt4_ate_add_coeffs,
       c:mnt4_affine_ate_dbl_coeffs,
       c.old_RX = RX;
       c.old_RY = -RY;
       old_RX_2 = c.old_RY.squared();
       c.gamma = (old_RX_2 + old_RX_2 + old_RX_2 + mnt4_coeff_a) * (c.old_RY + c.old_RY).inverse();
       c.gamma_twist = c.gamma * mnt4_twist;
       c.gamma_X = c.gamma * c.old_RX;
       result.coeffs.push(c);
       }
    */

    leave_block("Call to mnt4_affine_ate_precompute_G2", false);
    result
}

pub fn mnt4_affine_ate_miller_loop(
    prec_P: &mnt4_affine_ate_G1_precomputation,
    prec_Q: &mnt4_affine_ate_G2_precomputation,
) -> mnt4_Fq4 {
    enter_block("Call to mnt4_affine_ate_miller_loop", false);

    let mut f = mnt4_Fq4::one();

    let mut found_nonzero = false;
    let mut idx = 0;
    let loop_count: bigint<{ mnt4_Fr::num_limbs }> = mnt4_ate_loop_count;

    let NAF = find_wnaf(1, loop_count);
    for i in (0..=NAF.len() - 1).rev() {
        if !found_nonzero {
            //this skips the MSB itself
            found_nonzero |= (NAF[i] != 0);
            continue;
        }

        /* code below gets executed for all bits (EXCEPT the MSB itself) of
        mnt4_param_p (skipping leading zeros) in MSB to LSB
        order */
        let mut c = prec_Q.coeffs[idx].clone();
        idx += 1;
        let g_RR_at_P = mnt4_Fq4::new(
            prec_P.PY_twist_squared,
            c.gamma_twist * (-prec_P.PX) + c.gamma_X - c.old_RY,
        );
        f = f.squared().mul_by_023(&g_RR_at_P);

        if NAF[i] != 0 {
            let c = prec_Q.coeffs[idx].clone();
            idx += 1;
            let g_RQ_at_P = if NAF[i] > 0 {
                mnt4_Fq4::new(
                    prec_P.PY_twist_squared,
                    c.gamma_twist * (-prec_P.PX) + c.gamma_X - prec_Q.QY,
                )
            } else {
                mnt4_Fq4::new(
                    prec_P.PY_twist_squared,
                    c.gamma_twist * (-prec_P.PX) + c.gamma_X + prec_Q.QY,
                )
            };
            f = f.mul_by_023(&g_RQ_at_P);
        }
    }

    /* TODO: maybe handle neg
       if mnt4_ate_is_loop_count_neg
       {
       // TODO:
       mnt4_affine_ate_coeffs ac = prec_Q.coeffs[idx];idx+=1;
       mnt4_Fq4 g_RnegR_at_P = mnt4_Fq4(prec_P.PY_twist_squared,
       - prec_P.PX * c.gamma_twist + c.gamma_X - c.old_RY);
       f = (f * g_RnegR_at_P).inverse();
       }
    */

    leave_block("Call to mnt4_affine_ate_miller_loop", false);

    f
}

//ate pairing
#[derive(Clone, Debug, Default, PartialEq)]
pub struct extended_mnt4_G2_projective {
    pub X: mnt4_Fq2,
    pub Y: mnt4_Fq2,
    pub Z: mnt4_Fq2,
    pub T: mnt4_Fq2,
}
impl extended_mnt4_G2_projective {
    pub fn print(&self) {
        print!("extended mnt4_G2 projective X/Y/Z/T:\n");
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
    current: &mut extended_mnt4_G2_projective,
    dc: &mut mnt4_ate_dbl_coeffs,
) {
    let X = current.X;
    let Y = current.Y;
    let Z = current.Z;
    let T = current.T;

    let A = T.squared(); // A = T1^2
    let B = X.squared(); // B = X1^2
    let C = Y.squared(); // C = Y1^2
    let D = C.squared(); // D = C^2
    let E = (X + C).squared() - B - D; // E = (X1+C)^2-B-D
    let F = (B + B + B) + mnt4_twist_coeff_a * A; // F = 3*B +  a  *A
    let G = F.squared(); // G = F^2

    current.X = -(E + E + E + E) + G; // X3 = -4*E+G
    current.Y = D * (-mnt4_Fq::from("8")) + F * (E + E - current.X); // Y3 = -8*D+F*(2*E-X3)
    current.Z = (Y + Z).squared() - C - Z.squared(); // Z3 = (Y1+Z1)^2-C-Z1^2
    current.T = current.Z.squared(); // T3 = Z3^2

    dc.c_H = (current.Z + T).squared() - current.T - A; // H = (Z3+T1)^2-T3-A
    dc.c_4C = C + C + C + C; // fourC = 4*C
    dc.c_J = (F + T).squared() - G - A; // J = (F+T1)^2-G-A
    dc.c_L = (F + X).squared() - G - B; // L = (F+X1)^2-G-B

    current.test_invariant();
}

pub fn mixed_addition_step_for_flipped_miller_loop(
    base_X: mnt4_Fq2,
    base_Y: mnt4_Fq2,
    base_Y_squared: mnt4_Fq2,
    current: &mut extended_mnt4_G2_projective,
    ac: &mut mnt4_ate_add_coeffs,
) {
    let X1 = current.X.clone();
    let Y1 = current.Y.clone();
    let Z1 = current.Z.clone();
    let T1 = current.T;
    let x2: mnt4_Fq2 = base_X.clone();
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

pub fn mnt4_ate_precompute_G1(P: &mnt4_G1) -> mnt4_ate_G1_precomp {
    enter_block("Call to mnt4_ate_precompute_G1", false);

    let mut Pcopy = P.clone();
    Pcopy.to_affine_coordinates();

    let mut result = mnt4_ate_G1_precomp::default();
    result.PX = Pcopy.X;
    result.PY = Pcopy.Y;
    result.PX_twist = mnt4_twist * Pcopy.X;
    result.PY_twist = mnt4_twist * Pcopy.Y;

    leave_block("Call to mnt4_ate_precompute_G1", false);
    result
}

pub fn mnt4_ate_precompute_G2(Q: &mnt4_G2) -> mnt4_ate_G2_precomp {
    enter_block("Call to mnt4_ate_precompute_G2", false);

    let mut Qcopy = (Q.clone());
    Qcopy.to_affine_coordinates();

    let mut result = mnt4_ate_G2_precomp::default();
    result.QX = Qcopy.X;
    result.QY = Qcopy.Y;
    result.QY2 = Qcopy.Y.squared();
    result.QX_over_twist = Qcopy.X * mnt4_twist.inverse();
    result.QY_over_twist = Qcopy.Y * mnt4_twist.inverse();

    let mut R = extended_mnt4_G2_projective::default();
    R.X = Qcopy.X;
    R.Y = Qcopy.Y;
    R.Z = mnt4_Fq2::one();
    R.T = mnt4_Fq2::one();

    let loop_count: bigint<{ mnt4_Fr::num_limbs }> = mnt4_ate_loop_count;
    let mut found_one = false;

    for i in (0..=loop_count.max_bits() as usize - 1).rev() {
        let mut bit = loop_count.test_bit(i);
        if !found_one {
            //this skips the MSB itself
            found_one |= bit;
            continue;
        }

        let mut dc = mnt4_ate_dbl_coeffs::default();
        doubling_step_for_flipped_miller_loop(&mut R, &mut dc);
        result.dbl_coeffs.push(dc);
        if bit {
            let mut ac = mnt4_ate_add_coeffs::default();
            mixed_addition_step_for_flipped_miller_loop(
                result.QX, result.QY, result.QY2, &mut R, &mut ac,
            );
            result.add_coeffs.push(ac);
        }
    }

    if mnt4_ate_is_loop_count_neg {
        let mut RZ_inv = R.Z.inverse();
        let mut RZ2_inv = RZ_inv.squared();
        let mut RZ3_inv = RZ2_inv * RZ_inv;
        let mut minus_R_affine_X = R.X * RZ2_inv;
        let mut minus_R_affine_Y = -R.Y * RZ3_inv;
        let mut minus_R_affine_Y2 = minus_R_affine_Y.squared();
        let mut ac = mnt4_ate_add_coeffs::default();
        mixed_addition_step_for_flipped_miller_loop(
            minus_R_affine_X,
            minus_R_affine_Y,
            minus_R_affine_Y2,
            &mut R,
            &mut ac,
        );
        result.add_coeffs.push(ac);
    }

    leave_block("Call to mnt4_ate_precompute_G2", false);
    result
}

pub fn mnt4_ate_miller_loop(
    prec_P: &mnt4_ate_G1_precomp,
    prec_Q: &mnt4_ate_G2_precomp,
) -> mnt4_Fq4 {
    enter_block("Call to mnt4_ate_miller_loop", false);

    let L1_coeff = mnt4_Fq2::new(prec_P.PX, mnt4_Fq::zero()) - prec_Q.QX_over_twist;

    let mut f = mnt4_Fq4::one();

    let mut found_one = false;
    let mut dbl_idx = 0;
    let mut add_idx = 0;

    let mut loop_count: bigint<{ mnt4_Fr::num_limbs }> = mnt4_ate_loop_count;
    for i in (0..=loop_count.max_bits() as usize - 1).rev() {
        let mut bit = loop_count.test_bit(i);

        if !found_one {
            //this skips the MSB itself
            found_one |= bit;
            continue;
        }

        /* code below gets executed for all bits (EXCEPT the MSB itself) of
        mnt4_param_p (skipping leading zeros) in MSB to LSB
        order */
        let dc = prec_Q.dbl_coeffs[dbl_idx].clone();
        dbl_idx += 1;
        let g_RR_at_P = mnt4_Fq4::new(
            -dc.c_4C - dc.c_J * prec_P.PX_twist + dc.c_L,
            dc.c_H * prec_P.PY_twist,
        );
        f = f.squared() * g_RR_at_P;
        if bit {
            let ac = prec_Q.add_coeffs[add_idx].clone();
            add_idx += 1;
            let g_RQ_at_P = mnt4_Fq4::new(
                ac.c_RZ * prec_P.PY_twist,
                -(prec_Q.QY_over_twist * ac.c_RZ + L1_coeff * ac.c_L1),
            );
            f = f * g_RQ_at_P;
        }
    }

    if mnt4_ate_is_loop_count_neg {
        let ac = prec_Q.add_coeffs[add_idx].clone();
        add_idx += 1;
        let g_RnegR_at_P = mnt4_Fq4::new(
            ac.c_RZ * prec_P.PY_twist,
            -(prec_Q.QY_over_twist * ac.c_RZ + L1_coeff * ac.c_L1),
        );
        f = (f * g_RnegR_at_P).inverse();
    }

    leave_block("Call to mnt4_ate_miller_loop", false);

    f
}

pub fn mnt4_ate_double_miller_loop(
    prec_P1: &mnt4_ate_G1_precomp,
    prec_Q1: &mnt4_ate_G2_precomp,
    prec_P2: &mnt4_ate_G1_precomp,
    prec_Q2: &mnt4_ate_G2_precomp,
) -> mnt4_Fq4 {
    enter_block("Call to mnt4_ate_double_miller_loop", false);

    let L1_coeff1 = mnt4_Fq2::new(prec_P1.PX, mnt4_Fq::zero()) - prec_Q1.QX_over_twist;
    let L1_coeff2 = mnt4_Fq2::new(prec_P2.PX, mnt4_Fq::zero()) - prec_Q2.QX_over_twist;

    let mut f = mnt4_Fq4::one();

    let mut found_one = false;
    let mut dbl_idx = 0;
    let mut add_idx = 0;

    let mut loop_count: bigint<{ mnt4_Fr::num_limbs }> = mnt4_ate_loop_count;
    for i in (0..=loop_count.max_bits() - 1).rev() {
        let mut bit = loop_count.test_bit(i);

        if !found_one {
            //this skips the MSB itself
            found_one |= bit;
            continue;
        }

        /* code below gets executed for all bits (EXCEPT the MSB itself) of
        mnt4_param_p (skipping leading zeros) in MSB to LSB
        order */
        let mut dc1 = prec_Q1.dbl_coeffs[dbl_idx].clone();
        let mut dc2 = prec_Q2.dbl_coeffs[dbl_idx].clone();
        dbl_idx += 1;

        let mut g_RR_at_P1 = mnt4_Fq4::new(
            -dc1.c_4C - dc1.c_J * prec_P1.PX_twist + dc1.c_L,
            dc1.c_H * prec_P1.PY_twist,
        );

        let mut g_RR_at_P2 = mnt4_Fq4::new(
            -dc2.c_4C - dc2.c_J * prec_P2.PX_twist + dc2.c_L,
            dc2.c_H * prec_P2.PY_twist,
        );

        f = f.squared() * g_RR_at_P1 * g_RR_at_P2;

        if bit {
            let mut ac1 = prec_Q1.add_coeffs[add_idx].clone();
            let mut ac2 = prec_Q2.add_coeffs[add_idx].clone();
            add_idx += 1;

            let mut g_RQ_at_P1 = mnt4_Fq4::new(
                ac1.c_RZ * prec_P1.PY_twist,
                -(prec_Q1.QY_over_twist * ac1.c_RZ + L1_coeff1 * ac1.c_L1),
            );
            let mut g_RQ_at_P2 = mnt4_Fq4::new(
                ac2.c_RZ * prec_P2.PY_twist,
                -(prec_Q2.QY_over_twist * ac2.c_RZ + L1_coeff2 * ac2.c_L1),
            );

            f = f * g_RQ_at_P1 * g_RQ_at_P2;
        }
    }

    if mnt4_ate_is_loop_count_neg {
        let mut ac1 = prec_Q1.add_coeffs[add_idx].clone();
        let mut ac2 = prec_Q2.add_coeffs[add_idx].clone();
        add_idx += 1;
        let mut g_RnegR_at_P1 = mnt4_Fq4::new(
            ac1.c_RZ * prec_P1.PY_twist,
            -(prec_Q1.QY_over_twist * ac1.c_RZ + L1_coeff1 * ac1.c_L1),
        );
        let mut g_RnegR_at_P2 = mnt4_Fq4::new(
            ac2.c_RZ * prec_P2.PY_twist,
            -(prec_Q2.QY_over_twist * ac2.c_RZ + L1_coeff2 * ac2.c_L1),
        );

        f = (f * g_RnegR_at_P1 * g_RnegR_at_P2).inverse();
    }

    leave_block("Call to mnt4_ate_double_miller_loop", false);

    f
}

pub fn mnt4_ate_pairing(P: &mnt4_G1, Q: &mnt4_G2) -> mnt4_Fq4 {
    enter_block("Call to mnt4_ate_pairing", false);
    let mut prec_P = mnt4_ate_precompute_G1(P);
    let mut prec_Q = mnt4_ate_precompute_G2(Q);
    let mut result = mnt4_ate_miller_loop(&prec_P, &prec_Q);
    leave_block("Call to mnt4_ate_pairing", false);
    result
}

pub fn mnt4_ate_reduced_pairing(P: &mnt4_G1, Q: &mnt4_G2) -> mnt4_GT {
    enter_block("Call to mnt4_ate_reduced_pairing", false);
    let f = mnt4_ate_pairing(P, Q);
    let result = mnt4_final_exponentiation(&f);
    leave_block("Call to mnt4_ate_reduced_pairing", false);
    result
}

pub fn mnt4_precompute_G1(P: &mnt4_G1) -> mnt4_G1_precomp {
    mnt4_ate_precompute_G1(P)
}

pub fn mnt4_precompute_G2(Q: &mnt4_G2) -> mnt4_G2_precomp {
    mnt4_ate_precompute_G2(Q)
}

pub fn mnt4_miller_loop(prec_P: &mnt4_G1_precomp, prec_Q: &mnt4_G2_precomp) -> mnt4_Fq4 {
    mnt4_ate_miller_loop(prec_P, prec_Q)
}

pub fn mnt4_double_miller_loop(
    prec_P1: &mnt4_G1_precomp,
    prec_Q1: &mnt4_G2_precomp,
    prec_P2: &mnt4_G1_precomp,
    prec_Q2: &mnt4_G2_precomp,
) -> mnt4_Fq4 {
    mnt4_ate_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2)
}

pub fn mnt4_pairing(P: &mnt4_G1, Q: &mnt4_G2) -> mnt4_Fq4 {
    mnt4_ate_pairing(P, Q)
}

pub fn mnt4_reduced_pairing(P: &mnt4_G1, Q: &mnt4_G2) -> mnt4_GT {
    mnt4_ate_reduced_pairing(P, Q)
}

pub fn mnt4_affine_reduced_pairing(P: &mnt4_G1, Q: &mnt4_G2) -> mnt4_GT {
    let prec_P = mnt4_affine_ate_precompute_G1(P);
    let prec_Q = mnt4_affine_ate_precompute_G2(Q);
    let f = mnt4_affine_ate_miller_loop(&prec_P, &prec_Q);
    let result = mnt4_final_exponentiation(&f);
    result
}

// use std::fmt;
// use std::io::{self, Read, Write};

// #[derive(Clone, Debug, PartialEq)]
// pub struct Mnt4AteG1Precomp {
//     pub px: Fq,
//     pub py: Fq,
//     pub px_twist: Fq,
//     pub py_twist: Fq,
// }

// impl fmt::Display for Mnt4AteG1Precomp {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{}{}{}{}{}{}{}",
//             self.px,
//             OUTPUT_SEPARATOR,
//             self.py,
//             OUTPUT_SEPARATOR,
//             self.px_twist,
//             OUTPUT_SEPARATOR,
//             self.py_twist
//         )
//     }
// }

// #[derive(Clone, Debug, PartialEq)]
// pub struct Mnt4AteDblCoeffs {
//     pub c_h: Fq,
//     pub c_4c: Fq,
//     pub c_j: Fq,
//     pub c_l: Fq,
// }

// impl fmt::Display for Mnt4AteDblCoeffs {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{}{}{}{}{}{}{}",
//             self.c_h,
//             OUTPUT_SEPARATOR,
//             self.c_4c,
//             OUTPUT_SEPARATOR,
//             self.c_j,
//             OUTPUT_SEPARATOR,
//             self.c_l
//         )
//     }
// }

// #[derive(Clone, Debug, PartialEq)]
// pub struct Mnt4AteAddCoeffs {
//     pub c_l1: Fq,
//     pub c_rz: Fq,
// }

// impl fmt::Display for Mnt4AteAddCoeffs {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}{}{}", self.c_l1, OUTPUT_SEPARATOR, self.c_rz)
//     }
// }

// #[derive(Clone, Debug, PartialEq)]
// pub struct Mnt4AteG2Precomp {
//     pub qx: Fq,
//     pub qy: Fq,
//     pub qy2: Fq,
//     pub qx_over_twist: Fq,
//     pub qy_over_twist: Fq,
//     pub dbl_coeffs: Vec<Mnt4AteDblCoeffs>,
//     pub add_coeffs: Vec<Mnt4AteAddCoeffs>,
// }

// impl Mnt4AteG2Precomp {
//     pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
//         writeln!(
//             writer,
//             "{}{}{}{}{}{}{}{}{}",
//             self.qx,
//             OUTPUT_SEPARATOR,
//             self.qy,
//             OUTPUT_SEPARATOR,
//             self.qy2,
//             OUTPUT_SEPARATOR,
//             self.qx_over_twist,
//             OUTPUT_SEPARATOR,
//             self.qy_over_twist
//         )?;

//         writeln!(writer, "{}", self.dbl_coeffs.len())?;
//         for dc in &self.dbl_coeffs {
//             write!(writer, "{}{}", dc, OUTPUT_NEWLINE)?;
//         }

//         writeln!(writer, "{}", self.add_coeffs.len())?;
//         for ac in &self.add_coeffs {
//             write!(writer, "{}{}", ac, OUTPUT_NEWLINE)?;
//         }
//         Ok(())
//     }

//     pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
//         let qx = read_fq(&mut reader)?;
//         consume_output_separator(&mut reader)?;
//         let qy = read_fq(&mut reader)?;
//         consume_output_separator(&mut reader)?;
//         let qy2 = read_fq(&mut reader)?;
//         consume_output_separator(&mut reader)?;
//         let qx_over_twist = read_fq(&mut reader)?;
//         consume_output_separator(&mut reader)?;
//         let qy_over_twist = read_fq(&mut reader)?;
//         consume_newline(&mut reader)?;

//         let dbl_s: usize = read_usize(&mut reader)?;
//         consume_newline(&mut reader)?;
//         let mut dbl_coeffs = Vec::with_capacity(dbl_s);
//         for _ in 0..dbl_s {
//             let dc = Mnt4AteDblCoeffs::read(&mut reader)?;
//             consume_output_newline(&mut reader)?;
//             dbl_coeffs.push(dc);
//         }

//         let add_s: usize = read_usize(&mut reader)?;
//         consume_newline(&mut reader)?;
//         let mut add_coeffs = Vec::with_capacity(add_s);
//         for _ in 0..add_s {
//             let ac = Mnt4AteAddCoeffs::read(&mut reader)?;
//             consume_output_newline(&mut reader)?;
//             add_coeffs.push(ac);
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
