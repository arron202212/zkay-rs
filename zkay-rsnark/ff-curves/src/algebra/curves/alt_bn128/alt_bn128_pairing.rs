use crate::algebra::curves::{
    alt_bn128::{
        alt_bn128_fields::{
            alt_bn128_Fq, alt_bn128_Fq2, alt_bn128_Fq12, alt_bn128_Fr, alt_bn128_GT,
        },
        alt_bn128_g1::alt_bn128_G1,
        alt_bn128_g2::alt_bn128_G2,
        alt_bn128_init::{
            alt_bn128_ate_is_loop_count_neg, alt_bn128_ate_loop_count,
            alt_bn128_final_exponent_is_z_neg, alt_bn128_final_exponent_z, alt_bn128_twist,
            alt_bn128_twist_coeff_b,
        },
        curves::{Bn254, Config, G1Affine, G2Affine},
    },
    pairing::{Pairing, prepare_g1, prepare_g2},
};
use ffec::{
    FieldTConfig,
    common::serialization::{
        OUTPUT_NEWLINE, OUTPUT_SEPARATOR, consume_newline, consume_output_newline,
        consume_output_separator, read_line_as_usize,
    },
    {
        One, PpConfig,
        common::profiling::{enter_block, leave_block},
        field_utils::bigint::{BigIntegerT, bigint},
    },
};

//ate pairing

// pub type alt_bn128_ate_G1_precomp = <Bn254 as Pairing>::G1Prepared;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct alt_bn128_ate_G1_precomp {
    pub PX: alt_bn128_Fq,
    pub PY: alt_bn128_Fq,
}
// impl<const N:usize> PpConfig for alt_bn128_ate_G1_precomp{
//     type T=bigint<N>;
// }
use crate::algebra::curves::bn128::bn::g2::EllCoeff;
// pub type alt_bn128_ate_ell_coeffs = EllCoeff<Config>;
#[derive(Clone, Default, PartialEq, Eq)]
pub struct alt_bn128_ate_ell_coeffs {
    pub ell_0: alt_bn128_Fq2,
    pub ell_VW: alt_bn128_Fq2,
    pub ell_VV: alt_bn128_Fq2,
}

use crate::algebra::curves::bn128::bn::g2::G2Prepared;
// pub type alt_bn128_ate_G2_precomp = <Bn254 as Pairing>::G2Prepared;
#[derive(Clone, Default, PartialEq, Eq)]
pub struct alt_bn128_ate_G2_precomp {
    pub QX: alt_bn128_Fq2,
    pub QY: alt_bn128_Fq2,
    pub coeffs: Vec<alt_bn128_ate_ell_coeffs>,
}
// impl<const N:usize> PpConfig for alt_bn128_ate_G2_precomp{
//     type T=bigint<N>;
// }

//choice of pairing

pub type alt_bn128_G1_precomp = alt_bn128_ate_G1_precomp;
pub type alt_bn128_G2_precomp = alt_bn128_ate_G2_precomp;

use std::fmt;
impl fmt::Display for alt_bn128_ate_G1_precomp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{OUTPUT_SEPARATOR}{}", self.PX, self.PY)
    }
}
impl fmt::Display for alt_bn128_ate_ell_coeffs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{OUTPUT_SEPARATOR}{}{OUTPUT_SEPARATOR}{}",
            self.ell_0, self.ell_VW, self.ell_VV
        )
    }
}
impl fmt::Display for alt_bn128_ate_G2_precomp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{OUTPUT_SEPARATOR}{}\n{}\n{}",
            self.QX,
            self.QY,
            self.coeffs.len(),
            self.coeffs
                .iter()
                .map(|c| format!("{c}{OUTPUT_NEWLINE}"))
                .collect::<String>()
        )
    }
}

//final exponentiations

pub fn alt_bn128_final_exponentiation_first_chunk(elt: &alt_bn128_Fq12) -> alt_bn128_Fq12 {
    enter_block("Call to alt_bn128_final_exponentiation_first_chunk", false);

    //   Computes result = elt^((q^6-1)*(q^2+1)).
    //   Follows, e.g., Beuchat et al page 9, by computing result as follows:
    //      elt^((q^6-1)*(q^2+1)) = (conj(elt) * elt^(-1))^(q^2+1)
    //   More precisely:
    //   A = conj(elt)
    //   B = elt.inverse()
    //   C = A * B
    //   D = C.Frobenius_map(2)
    //   result = D * C

    let A = alt_bn128_Fq12::new(elt.c0, -elt.c1);
    let B = elt.inverse();
    let C = A * B;
    let D = C.Frobenius_map(2);
    let result = D * C;

    leave_block("Call to alt_bn128_final_exponentiation_first_chunk", false);

    result
}

pub fn alt_bn128_exp_by_neg_z(elt: &alt_bn128_Fq12) -> alt_bn128_Fq12 {
    enter_block("Call to alt_bn128_exp_by_neg_z", false);

    let mut result = elt.cyclotomic_exp(&alt_bn128_final_exponent_z);
    if !alt_bn128_final_exponent_is_z_neg {
        result = result.unitary_inverse();
    }

    leave_block("Call to alt_bn128_exp_by_neg_z", false);

    result
}

pub fn alt_bn128_final_exponentiation_last_chunk(elt: &alt_bn128_Fq12) -> alt_bn128_Fq12 {
    enter_block("Call to alt_bn128_final_exponentiation_last_chunk", false);

    //   Follows Laura Fuentes-Castaneda et al. "Faster hashing to G2"
    //   by computing:

    //   result = elt^(q^3 * (12*z^3 + 6z^2 + 4z - 1) +
    //                 q^2 * (12*z^3 + 6z^2 + 6z) +
    //                 q   * (12*z^3 + 6z^2 + 4z) +
    //                 1   * (12*z^3 + 12z^2 + 6z + 1))
    //   which equals

    //   result = elt^( 2z * ( 6z^2 + 3z + 1 ) * (q^4 - q^2 + 1)/r ).

    //   Using the following addition chain:

    //   A = exp_by_neg_z(elt)  // = elt^(-z)
    //   B = A^2                // = elt^(-2*z)
    //   C = B^2                // = elt^(-4*z)
    //   D = C * B              // = elt^(-6*z)
    //   E = exp_by_neg_z(D)    // = elt^(6*z^2)
    //   F = E^2                // = elt^(12*z^2)
    //   G = epx_by_neg_z(F)    // = elt^(-12*z^3)
    //   H = conj(D)            // = elt^(6*z)
    //   I = conj(G)            // = elt^(12*z^3)
    //   J = I * E              // = elt^(12*z^3 + 6*z^2)
    //   K = J * H              // = elt^(12*z^3 + 6*z^2 + 6*z)
    //   L = K * B              // = elt^(12*z^3 + 6*z^2 + 4*z)
    //   M = K * E              // = elt^(12*z^3 + 12*z^2 + 6*z)
    //   N = M * elt            // = elt^(12*z^3 + 12*z^2 + 6*z + 1)
    //   O = L.Frobenius_map(1) // = elt^(q*(12*z^3 + 6*z^2 + 4*z))
    //   P = O * N              // = elt^(q*(12*z^3 + 6*z^2 + 4*z) * (12*z^3 + 12*z^2 + 6*z + 1))
    //   Q = K.Frobenius_map(2) // = elt^(q^2 * (12*z^3 + 6*z^2 + 6*z))
    //   R = Q * P              // = elt^(q^2 * (12*z^3 + 6*z^2 + 6*z) + q*(12*z^3 + 6*z^2 + 4*z) * (12*z^3 + 12*z^2 + 6*z + 1))
    //   S = conj(elt)          // = elt^(-1)
    //   T = S * L              // = elt^(12*z^3 + 6*z^2 + 4*z - 1)
    //   U = T.Frobenius_map(3) // = elt^(q^3(12*z^3 + 6*z^2 + 4*z - 1))
    //   V = U * R              // = elt^(q^3(12*z^3 + 6*z^2 + 4*z - 1) + q^2 * (12*z^3 + 6*z^2 + 6*z) + q*(12*z^3 + 6*z^2 + 4*z) * (12*z^3 + 12*z^2 + 6*z + 1))
    //   result = V

    let A = alt_bn128_exp_by_neg_z(elt);
    let B = A.cyclotomic_squared();
    let C = B.cyclotomic_squared();
    let D = C * B;
    let E = alt_bn128_exp_by_neg_z(&D);
    let F = E.cyclotomic_squared();
    let G = alt_bn128_exp_by_neg_z(&F);
    let H = D.unitary_inverse();
    let I = G.unitary_inverse();
    let J = I * E;
    let K = J * H;
    let L = K * B;
    let M = K * E;
    let N = M * elt;
    let O = L.Frobenius_map(1);
    let P = O * N;
    let Q = K.Frobenius_map(2);
    let R = Q * P;
    let S = elt.unitary_inverse();
    let T = S * L;
    let U = T.Frobenius_map(3);
    let V = U * R;

    let result = V;

    leave_block("Call to alt_bn128_final_exponentiation_last_chunk", false);

    result
}

pub fn alt_bn128_final_exponentiation(elt: &alt_bn128_Fq12) -> alt_bn128_GT {
    enter_block("Call to alt_bn128_final_exponentiation", false);
    let A = alt_bn128_final_exponentiation_first_chunk(elt);
    let result = alt_bn128_final_exponentiation_last_chunk(&A);

    leave_block("Call to alt_bn128_final_exponentiation", false);
    result
}

//ate pairing

pub fn doubling_step_for_flipped_miller_loop(
    two_inv: alt_bn128_Fq,
    current: &mut alt_bn128_G2,
    c: &mut alt_bn128_ate_ell_coeffs,
) {
    let (X, Y, Z) = (current.X.clone(), current.Y.clone(), current.Z.clone());

    let A = (X * Y) * two_inv; // A = X1 * Y1 / 2
    let B = Y.squared(); // B = Y1^2
    let C = Z.squared(); // C = Z1^2
    let D = C + C + C; // D = 3 * C
    let E = alt_bn128_twist_coeff_b() * D; // E = twist_b * D
    let F = E + E + E; // F = 3 * E
    let G = (B + F) * two_inv; // G = (B+F)/2
    let H = (Y + Z).squared() - (B + C); // H = (Y1+Z1)^2-(B+C)
    let I = E - B; // I = E-B
    let J = X.squared(); // J = X1^2
    let E_squared = E.squared(); // E_squared = E^2

    current.X = A * (B - F); // X3 = A * (B-F)
    current.Y = G.squared() - (E_squared + E_squared + E_squared); // Y3 = G^2 - 3*E^2
    current.Z = B * H; // Z3 = B * H
    c.ell_0 = alt_bn128_twist * I; // ell_0 = xi * I
    c.ell_VW = -H; // ell_VW = - H (later: * yP)
    c.ell_VV = J + J + J; // ell_VV = 3*J (later: * xP)
}

pub fn mixed_addition_step_for_flipped_miller_loop(
    base: alt_bn128_G2,
    current: &mut alt_bn128_G2,
    c: &mut alt_bn128_ate_ell_coeffs,
) {
    let (X1, Y1, Z1) = (current.X.clone(), current.Y.clone(), current.Z.clone());
    let (x2, y2) = (base.X.clone(), base.Y.clone());

    let D = X1 - x2 * Z1; // D = X1 - X2*Z1
    let E = Y1 - y2 * Z1; // E = Y1 - Y2*Z1
    let F = D.squared(); // F = D^2
    let G = E.squared(); // G = E^2
    let H = D * F; // H = D*F
    let I = X1 * F; // I = X1 * F
    let J = H + Z1 * G - (I + I); // J = H + Z1*G - (I+I)

    current.X = D * J; // X3 = D*J
    current.Y = E * (I - J) - (H * Y1); // Y3 = E*(I-J)-(H*Y1)
    current.Z = Z1 * H; // Z3 = Z1*H
    c.ell_0 = alt_bn128_twist * (E * x2 - D * y2); // ell_0 = xi * (E * X2 - D * Y2)
    c.ell_VV = -E; // ell_VV = - E (later: * xP)
    c.ell_VW = D; // ell_VW = D (later: * yP    )
}

pub fn alt_bn128_ate_precompute_G1(P: &alt_bn128_G1) -> alt_bn128_ate_G1_precomp {
    enter_block("Call to alt_bn128_ate_precompute_G1", false);

    let mut Pcopy = P.clone();
    Pcopy.to_affine_coordinates();

    let mut result = alt_bn128_ate_G1_precomp::default();
    result.PX = Pcopy.X.clone();
    result.PY = Pcopy.Y.clone();

    leave_block("Call to alt_bn128_ate_precompute_G1", false);
    result
}

pub fn alt_bn128_ate_precompute_G2(Q: &alt_bn128_G2) -> alt_bn128_ate_G2_precomp {
    enter_block("Call to alt_bn128_ate_precompute_G2", false);

    let mut Qcopy = Q.clone();
    Qcopy.to_affine_coordinates();

    let two_inv = (alt_bn128_Fq::from("2").inverse()); // could add to global params if needed

    let mut result = alt_bn128_ate_G2_precomp::default();
    result.QX = Qcopy.X;
    result.QY = Qcopy.Y;

    let mut R = alt_bn128_G2::default();
    R.X = Qcopy.X;
    R.Y = Qcopy.Y;
    R.Z = alt_bn128_Fq2::one();

    let loop_count: bigint<{ alt_bn128_Fr::num_limbs }> = alt_bn128_ate_loop_count;
    let mut found_one = false;
    let mut c = alt_bn128_ate_ell_coeffs::default();

    for i in (0..=loop_count.max_bits()).rev() {
        let mut bit = loop_count.test_bit(i);
        if !found_one {
            //this skips the MSB itself
            found_one |= bit;
            continue;
        }

        doubling_step_for_flipped_miller_loop(two_inv, &mut R, &mut c);
        result.coeffs.push(c.clone());

        if bit {
            mixed_addition_step_for_flipped_miller_loop(Qcopy.clone(), &mut R, &mut c);
            result.coeffs.push(c.clone());
        }
    }

    let mut Q1 = Qcopy.mul_by_q();
    assert!(
        Q1.Z == alt_bn128_Fq2::one(),
        "==Qcopy.Z,Q1.Z==={:?},{:?}",
        Qcopy.Z,
        Q1.Z
    );
    let mut Q2 = Q1.mul_by_q();
    assert!(Q2.Z == alt_bn128_Fq2::one());

    if alt_bn128_ate_is_loop_count_neg {
        R.Y = -R.Y;
    }
    Q2.Y = -Q2.Y;

    mixed_addition_step_for_flipped_miller_loop(Q1, &mut R, &mut c);
    result.coeffs.push(c.clone());

    mixed_addition_step_for_flipped_miller_loop(Q2, &mut R, &mut c);
    result.coeffs.push(c);

    leave_block("Call to alt_bn128_ate_precompute_G2", false);
    result
}

pub fn alt_bn128_ate_miller_loop(
    prec_P: &alt_bn128_ate_G1_precomp,
    prec_Q: &alt_bn128_ate_G2_precomp,
) -> alt_bn128_Fq12 {
    enter_block("Call to alt_bn128_ate_miller_loop", false);

    let mut f = alt_bn128_Fq12::one();

    let mut found_one = false;
    let mut idx = 0;

    let loop_count: bigint<{ alt_bn128_Fr::num_limbs }> = alt_bn128_ate_loop_count;
    let mut c = alt_bn128_ate_ell_coeffs::default();

    for i in (0..=loop_count.max_bits() as usize).rev() {
        let mut bit = loop_count.test_bit(i);
        if !found_one {
            //this skips the MSB itself
            found_one |= bit;
            continue;
        }

        /* code below gets executed for all bits (EXCEPT the MSB itself) of
        alt_bn128_param_p (skipping leading zeros) in MSB to LSB
        order */
        c = prec_Q.coeffs[idx].clone();
        idx += 1;
        f = f.squared();
        f = f.mul_by_024(&c.ell_0, &(c.ell_VW * prec_P.PY), &(c.ell_VV * prec_P.PX));

        if bit {
            c = prec_Q.coeffs[idx].clone();
            idx += 1;
            f = f.mul_by_024(&c.ell_0, &(c.ell_VW * prec_P.PY), &(c.ell_VV * prec_P.PX));
        }
    }

    if alt_bn128_ate_is_loop_count_neg {
        f = f.inverse();
    }

    c = prec_Q.coeffs[idx].clone();
    idx += 1;
    f = f.mul_by_024(&c.ell_0, &(c.ell_VW * prec_P.PY), &(c.ell_VV * prec_P.PX));

    c = prec_Q.coeffs[idx].clone();
    idx += 1;
    f = f.mul_by_024(&c.ell_0, &(c.ell_VW * prec_P.PY), &(c.ell_VV * prec_P.PX));

    leave_block("Call to alt_bn128_ate_miller_loop", false);
    f
}

pub fn alt_bn128_ate_double_miller_loop(
    prec_P1: &alt_bn128_ate_G1_precomp,
    prec_Q1: &alt_bn128_ate_G2_precomp,
    prec_P2: &alt_bn128_ate_G1_precomp,
    prec_Q2: &alt_bn128_ate_G2_precomp,
) -> alt_bn128_Fq12 {
    enter_block("Call to alt_bn128_ate_double_miller_loop", false);

    let mut f = alt_bn128_Fq12::one();

    let mut found_one = false;
    let mut idx = 0;

    let loop_count: bigint<{ alt_bn128_Fr::num_limbs }> = alt_bn128_ate_loop_count;
    for i in (0..=loop_count.max_bits()).rev() {
        let mut bit = loop_count.test_bit(i);
        if !found_one {
            //this skips the MSB itself
            found_one |= bit;
            continue;
        }

        /* code below gets executed for all bits (EXCEPT the MSB itself) of
        alt_bn128_param_p (skipping leading zeros) in MSB to LSB
        order */
        let mut c1 = prec_Q1.coeffs[idx].clone();
        let mut c2 = prec_Q2.coeffs[idx].clone();
        idx += 1;

        f = f.squared();

        f = f.mul_by_024(
            &c1.ell_0,
            &(c1.ell_VW * prec_P1.PY),
            &(c1.ell_VV * prec_P1.PX),
        );
        f = f.mul_by_024(
            &c2.ell_0,
            &(c2.ell_VW * prec_P2.PY),
            &(c2.ell_VV * prec_P2.PX),
        );

        if bit {
            let mut c1 = prec_Q1.coeffs[idx].clone();
            let mut c2 = prec_Q2.coeffs[idx].clone();
            idx += 1;

            f = f.mul_by_024(
                &c1.ell_0,
                &(c1.ell_VW * prec_P1.PY),
                &(c1.ell_VV * prec_P1.PX),
            );
            f = f.mul_by_024(
                &c2.ell_0,
                &(c2.ell_VW * prec_P2.PY),
                &(c2.ell_VV * prec_P2.PX),
            );
        }
    }

    if alt_bn128_ate_is_loop_count_neg {
        f = f.inverse();
    }

    let mut c1 = prec_Q1.coeffs[idx].clone();
    let mut c2 = prec_Q2.coeffs[idx].clone();
    idx += 1;
    f = f.mul_by_024(
        &c1.ell_0,
        &(c1.ell_VW * prec_P1.PY),
        &(c1.ell_VV * prec_P1.PX),
    );
    f = f.mul_by_024(
        &c2.ell_0,
        &(c2.ell_VW * prec_P2.PY),
        &(c2.ell_VV * prec_P2.PX),
    );

    c1 = prec_Q1.coeffs[idx].clone();
    c2 = prec_Q2.coeffs[idx].clone();
    idx += 1;
    f = f.mul_by_024(
        &c1.ell_0,
        &(c1.ell_VW * prec_P1.PY),
        &(c1.ell_VV * prec_P1.PX),
    );
    f = f.mul_by_024(
        &c2.ell_0,
        &(c2.ell_VW * prec_P2.PY),
        &(c2.ell_VV * prec_P2.PX),
    );

    leave_block("Call to alt_bn128_ate_double_miller_loop", false);

    f
}

pub fn alt_bn128_ate_pairing(P: &alt_bn128_G1, Q: &alt_bn128_G2) -> alt_bn128_Fq12 {
    enter_block("Call to alt_bn128_ate_pairing", false);
    let mut prec_P = alt_bn128_ate_precompute_G1(P);
    let mut prec_Q = alt_bn128_ate_precompute_G2(Q);
    let mut result = alt_bn128_ate_miller_loop(&prec_P, &prec_Q);
    leave_block("Call to alt_bn128_ate_pairing", false);
    result
}

pub fn alt_bn128_ate_reduced_pairing(P: &alt_bn128_G1, Q: &alt_bn128_G2) -> alt_bn128_GT {
    enter_block("Call to alt_bn128_ate_reduced_pairing", false);
    let f = alt_bn128_ate_pairing(P, Q);
    let result = alt_bn128_final_exponentiation(&f);
    leave_block("Call to alt_bn128_ate_reduced_pairing", false);
    result
}

//choice of pairing

pub fn alt_bn128_precompute_G1(P: &alt_bn128_G1) -> alt_bn128_G1_precomp {
    alt_bn128_ate_precompute_G1(P)
}

pub fn alt_bn128_precompute_G2(Q: &alt_bn128_G2) -> alt_bn128_G2_precomp {
    alt_bn128_ate_precompute_G2(Q)
}

pub fn alt_bn128_miller_loop(
    prec_P: &alt_bn128_G1_precomp,
    prec_Q: &alt_bn128_G2_precomp,
) -> alt_bn128_Fq12 {
    alt_bn128_ate_miller_loop(prec_P, prec_Q)
}

pub fn alt_bn128_double_miller_loop(
    prec_P1: &alt_bn128_G1_precomp,
    prec_Q1: &alt_bn128_G2_precomp,
    prec_P2: &alt_bn128_G1_precomp,
    prec_Q2: &alt_bn128_G2_precomp,
) -> alt_bn128_Fq12 {
    alt_bn128_ate_double_miller_loop(prec_P1, prec_Q1, prec_P2, prec_Q2)
}

pub fn alt_bn128_pairing(P: &alt_bn128_G1, Q: &alt_bn128_G2) -> alt_bn128_Fq12 {
    alt_bn128_ate_pairing(P, Q)
}

pub fn alt_bn128_reduced_pairing(P: &alt_bn128_G1, Q: &alt_bn128_G2) -> alt_bn128_GT {
    alt_bn128_ate_reduced_pairing(P, Q)
}
use std::io::{self, Read, Write};

impl alt_bn128_ate_G1_precomp {
    pub fn write<W: Write>(&self, mut out: W) -> io::Result<()> {
        write!(out, "{}", self.PX)?;
        out.write_all(OUTPUT_SEPARATOR.as_bytes())?;
        write!(out, "{}", self.PY)?;
        Ok(())
    }

    pub fn read<R: Read>(mut input: R) -> io::Result<Self> {
        let PX = alt_bn128_Fq::read(&mut input)?;
        consume_output_separator(&mut input)?;
        let PY = alt_bn128_Fq::read(&mut input)?;
        Ok(Self { PX, PY })
    }
}

impl alt_bn128_ate_ell_coeffs {
    pub fn write<W: Write>(&self, mut out: W) -> io::Result<()> {
        write!(out, "{}", self.ell_0)?;
        out.write_all(OUTPUT_SEPARATOR.as_bytes())?;
        write!(out, "{}", self.ell_VW)?;
        out.write_all(OUTPUT_SEPARATOR.as_bytes())?;
        write!(out, "{}", self.ell_VV)?;
        Ok(())
    }

    pub fn read<R: Read + std::io::BufRead>(mut input: R) -> io::Result<Self> {
        let ell_0 = alt_bn128_Fq2::read(&mut input)?;
        consume_output_separator(&mut input)?;
        let ell_VW = alt_bn128_Fq2::read(&mut input)?;
        consume_output_separator(&mut input)?;
        let ell_VV = alt_bn128_Fq2::read(&mut input)?;
        Ok(Self {
            ell_0,
            ell_VW,
            ell_VV,
        })
    }
}

impl alt_bn128_ate_G2_precomp {
    pub fn write<W: Write>(&self, mut out: W) -> io::Result<()> {
        // out << prec_Q.QX << OUTPUT_SEPARATOR << prec_Q.QY << "\n";
        write!(out, "{}", self.QX)?;
        out.write_all(OUTPUT_SEPARATOR.as_bytes())?;
        write!(out, "{}\n", self.QY)?;

        // out << prec_Q.coeffs.size() << "\n";
        writeln!(out, "{}", self.coeffs.len())?;

        // 遍历输出系数
        for c in &self.coeffs {
            c.write(&mut out)?;
            out.write_all(OUTPUT_NEWLINE.as_bytes())?;
        }
        Ok(())
    }

    pub fn read<R: Read + std::io::BufRead>(mut input: R) -> io::Result<Self> {
        // 读取 QX 和 QY
        let QX = alt_bn128_Fq2::read(&mut input)?;
        consume_output_separator(&mut input)?;
        let QY = alt_bn128_Fq2::read(&mut input)?;
        consume_newline(&mut input)?;

        // 读取向量大小并初始化
        let s: usize = read_line_as_usize(&mut input)?;
        let mut coeffs = Vec::with_capacity(s);

        for _ in 0..s {
            let c = alt_bn128_ate_ell_coeffs::read(&mut input)?;
            consume_output_newline(&mut input)?;
            coeffs.push(c);
        }

        Ok(Self { QX, QY, coeffs })
    }
}
