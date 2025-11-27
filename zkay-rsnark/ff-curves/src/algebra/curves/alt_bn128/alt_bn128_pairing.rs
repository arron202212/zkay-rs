/** @file
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef ALT_BN128_PAIRING_HPP_
// #define ALT_BN128_PAIRING_HPP_
//#include <vector>
// use crate::algebra::curves::alt_bn128::alt_bn128_init::{alt_bn128_G2,alt_bn128_G1};
use crate::algebra::curves::alt_bn128::alt_bn128_fields::{
    alt_bn128_Fq, alt_bn128_Fq12, alt_bn128_GT,
};
use crate::algebra::curves::alt_bn128::alt_bn128_g1::alt_bn128_G1;
use crate::algebra::curves::alt_bn128::alt_bn128_g2::alt_bn128_G2;
// namespace libff {

/* final exponentiation */

// alt_bn128_GT alt_bn128_final_exponentiation(elt:&alt_bn128_Fq12);

/* ate pairing */
use crate::algebra::curves::alt_bn128::curves::{Bn254, Config, G1Affine, G2Affine};
use crate::algebra::curves::pairing::{Pairing, prepare_g1, prepare_g2};
pub type alt_bn128_ate_G1_precomp = <Bn254 as Pairing>::G1Prepared;
use ffec::One;
// struct alt_bn128_ate_G1_precomp {
//     alt_bn128_Fq PX;
//     alt_bn128_Fq PY;

//     // bool operator==(other:&alt_bn128_ate_G1_precomp) const;
//     // friend std::ostream& operator<<(std::ostream &out, prec_P:&alt_bn128_ate_G1_precomp);
//     // friend std::istream& operator>>(std::istream &in, alt_bn128_ate_G1_precomp &prec_P);
// }
use crate::algebra::curves::bn128::bn::g2::EllCoeff;
pub type alt_bn128_ate_ell_coeffs = EllCoeff<Config>;
// struct alt_bn128_ate_ell_coeffs {
//     alt_bn128_Fq2 ell_0;
//     alt_bn128_Fq2 ell_VW;
//     alt_bn128_Fq2 ell_VV;

//     // bool operator==(other:&alt_bn128_ate_ell_coeffs) const;
//     // friend std::ostream& operator<<(std::ostream &out, c:&alt_bn128_ate_ell_coeffs);
//     // friend std::istream& operator>>(std::istream &in, c:&alt_bn128_ate_ell_coeffs);
// }

use crate::algebra::curves::bn128::bn::g2::G2Prepared;
pub type alt_bn128_ate_G2_precomp = <Bn254 as Pairing>::G2Prepared;
// struct alt_bn128_ate_G2_precomp {
//     alt_bn128_Fq2 QX;
//     alt_bn128_Fq2 QY;
//     Vec<alt_bn128_ate_ell_coeffs> coeffs;

//     // bool operator==(other:&alt_bn128_ate_G2_precomp) const;
//     // friend std::ostream& operator<<(std::ostream &out, prec_Q:&alt_bn128_ate_G2_precomp);
//     // friend std::istream& operator>>(std::istream &in, alt_bn128_ate_G2_precomp &prec_Q);
// }

// alt_bn128_ate_G1_precomp alt_bn128_ate_precompute_G1(P:&alt_bn128_G1);
// alt_bn128_ate_G2_precomp alt_bn128_ate_precompute_G2(Q:&alt_bn128_G2);

// alt_bn128_Fq12 alt_bn128_ate_miller_loop(prec_P:&alt_bn128_ate_G1_precomp,
//                               prec_Q:&alt_bn128_ate_G2_precomp);
// alt_bn128_Fq12 alt_bn128_ate_double_miller_loop(prec_P1:&alt_bn128_ate_G1_precomp,
//                                      prec_Q1:&alt_bn128_ate_G2_precomp,
//                                      prec_P2:&alt_bn128_ate_G1_precomp,
//                                      prec_Q2:&alt_bn128_ate_G2_precomp);

// alt_bn128_Fq12 alt_bn128_ate_pairing(P:alt_bn128_G1&,
//                           Q:&alt_bn128_G2);
// alt_bn128_GT alt_bn128_ate_reduced_pairing(P:&alt_bn128_G1,
//                                  Q:&alt_bn128_G2);

/* choice of pairing */

pub type alt_bn128_G1_precomp = alt_bn128_ate_G1_precomp;
pub type alt_bn128_G2_precomp = alt_bn128_ate_G2_precomp;

// alt_bn128_G1_precomp alt_bn128_precompute_G1(P:&alt_bn128_G1);

// alt_bn128_G2_precomp alt_bn128_precompute_G2(Q:&alt_bn128_G2);

// alt_bn128_Fq12 alt_bn128_miller_loop(prec_P:&alt_bn128_G1_precomp,
//                           prec_Q:&alt_bn128_G2_precomp);

// alt_bn128_Fq12 alt_bn128_double_miller_loop(prec_P1:&alt_bn128_G1_precomp,
//                                  prec_Q1:&alt_bn128_G2_precomp,
//                                  prec_P2:&alt_bn128_G1_precomp,
//                                  prec_Q2:&alt_bn128_G2_precomp);

// alt_bn128_Fq12 alt_bn128_pairing(P:alt_bn128_G1&,
//                       Q:&alt_bn128_G2);

// alt_bn128_GT alt_bn128_reduced_pairing(P:&alt_bn128_G1,
//                              Q:&alt_bn128_G2);

// alt_bn128_GT alt_bn128_affine_reduced_pairing(P:&alt_bn128_G1,
//                                     Q:&alt_bn128_G2);

// } // namespace libff
//#endif // ALT_BN128_PAIRING_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#include <cassert>
// use crate::algebra::curves::alt_bn128::alt_bn128_g1;
// use crate::algebra::curves::alt_bn128::alt_bn128_g2;
// use crate::algebra::curves::alt_bn128::alt_bn128_init;
use crate::algebra::curves::alt_bn128::alt_bn128_pairing;
use ffec::common::profiling;

// namespace libff {

// using std::usize;

// bool alt_bn128_ate_G1_precomp::operator==(other:&alt_bn128_ate_G1_precomp) const
// {
//     return (this->PX == other.PX &&
//             this->PY == other.PY);
// }

// std::ostream& operator<<(std::ostream &out, prec_P:&alt_bn128_ate_G1_precomp)
// {
//     out << prec_P.PX << OUTPUT_SEPARATOR << prec_P.PY;

//     return out;
// }

// std::istream& operator>>(std::istream &in, alt_bn128_ate_G1_precomp &prec_P)
// {
//     in >> prec_P.PX;
//     consume_OUTPUT_SEPARATOR(in);
//     in >> prec_P.PY;

//     return in;
// }

// bool  alt_bn128_ate_ell_coeffs::operator==(other:&alt_bn128_ate_ell_coeffs) const
// {
//     return (this->ell_0 == other.ell_0 &&
//             this->ell_VW == other.ell_VW &&
//             this->ell_VV == other.ell_VV);
// }

// std::ostream& operator<<(std::ostream &out, c:&alt_bn128_ate_ell_coeffs)
// {
//     out << c.ell_0 << OUTPUT_SEPARATOR << c.ell_VW << OUTPUT_SEPARATOR << c.ell_VV;
//     return out;
// }

// std::istream& operator>>(std::istream &in, c:&alt_bn128_ate_ell_coeffs)
// {
//     in >> c.ell_0;
//     consume_OUTPUT_SEPARATOR(in);
//     in >> c.ell_VW;
//     consume_OUTPUT_SEPARATOR(in);
//     in >> c.ell_VV;

//     return in;
// }

// bool alt_bn128_ate_G2_precomp::operator==(other:&alt_bn128_ate_G2_precomp) const
// {
//     return (this->QX == other.QX &&
//             this->QY == other.QY &&
//             this->coeffs == other.coeffs);
// }

// std::ostream& operator<<(std::ostream& out, prec_Q:&alt_bn128_ate_G2_precomp)
// {
//     out << prec_Q.QX << OUTPUT_SEPARATOR << prec_Q.QY << "\n";
//     out << prec_Q.coeffs.len() << "\n";
//     for c in &prec_Q.coeffs
//     {
//         out << c << OUTPUT_NEWLINE;
//     }
//     return out;
// }

// std::istream& operator>>(std::istream& in, alt_bn128_ate_G2_precomp &prec_Q)
// {
//     in >> prec_Q.QX;
//     consume_OUTPUT_SEPARATOR(in);
//     in >> prec_Q.QY;
//     consume_newline(in);

//     prec_Q.coeffs.clear();
//     usize s;
//     in >> s;

//     consume_newline(in);

//     prec_Q.coeffs.reserve(s);

//     for i in 0..s
//     {
//         alt_bn128_ate_ell_coeffs c;
//         in >> c;
//         consume_OUTPUT_NEWLINE(in);
//         prec_Q.coeffs.emplace_back(c);
//     }

//     return in;
// }

/* final exponentiations */

pub fn alt_bn128_final_exponentiation_first_chunk(elt: &alt_bn128_Fq12) -> alt_bn128_Fq12 {
    // enter_block("Call to alt_bn128_final_exponentiation_first_chunk");

    /*
      Computes result = elt^((q^6-1)*(q^2+1)).
      Follows, e.g., Beuchat et al page 9, by computing result as follows:
         elt^((q^6-1)*(q^2+1)) = (conj(elt) * elt^(-1))^(q^2+1)
      More precisely:
      A = conj(elt)
      B = elt.inverse()
      C = A * B
      D = C.Frobenius_map(2)
      result = D * C
    */

    // let A= alt_bn128_Fq12(elt.c0,-elt.c1);
    // let B= elt.inverse();
    // let C= A * B;
    // let D= C.Frobenius_map(2);
    // let result= D * C;

    // leave_block("Call to alt_bn128_final_exponentiation_first_chunk");

    // return result;
    elt.clone()
}

pub fn alt_bn128_exp_by_neg_z(elt: &alt_bn128_Fq12) -> alt_bn128_Fq12 {
    // enter_block("Call to alt_bn128_exp_by_neg_z");

    // alt_bn128_Fq12 result = elt.cyclotomic_exp(alt_bn128_final_exponent_z);
    // if !alt_bn128_final_exponent_is_z_neg
    // {
    //     result = result.unitary_inverse();
    // }

    // leave_block("Call to alt_bn128_exp_by_neg_z");

    // return result;
    elt.clone()
}

pub fn lt_bn128_final_exponentiation_last_chunk(elt: &alt_bn128_Fq12) -> alt_bn128_Fq12 {
    // enter_block("Call to alt_bn128_final_exponentiation_last_chunk");

    /*
      Follows Laura Fuentes-Castaneda et al. "Faster hashing to G2"
      by computing:

      result = elt^(q^3 * (12*z^3 + 6z^2 + 4z - 1) +
                    q^2 * (12*z^3 + 6z^2 + 6z) +
                    q   * (12*z^3 + 6z^2 + 4z) +
                    1   * (12*z^3 + 12z^2 + 6z + 1))
      which equals

      result = elt^( 2z * ( 6z^2 + 3z + 1 ) * (q^4 - q^2 + 1)/r ).

      Using the following addition chain:

      A = exp_by_neg_z(elt)  // = elt^(-z)
      B = A^2                // = elt^(-2*z)
      C = B^2                // = elt^(-4*z)
      D = C * B              // = elt^(-6*z)
      E = exp_by_neg_z(D)    // = elt^(6*z^2)
      F = E^2                // = elt^(12*z^2)
      G = epx_by_neg_z(F)    // = elt^(-12*z^3)
      H = conj(D)            // = elt^(6*z)
      I = conj(G)            // = elt^(12*z^3)
      J = I * E              // = elt^(12*z^3 + 6*z^2)
      K = J * H              // = elt^(12*z^3 + 6*z^2 + 6*z)
      L = K * B              // = elt^(12*z^3 + 6*z^2 + 4*z)
      M = K * E              // = elt^(12*z^3 + 12*z^2 + 6*z)
      N = M * elt            // = elt^(12*z^3 + 12*z^2 + 6*z + 1)
      O = L.Frobenius_map(1) // = elt^(q*(12*z^3 + 6*z^2 + 4*z))
      P = O * N              // = elt^(q*(12*z^3 + 6*z^2 + 4*z) * (12*z^3 + 12*z^2 + 6*z + 1))
      Q = K.Frobenius_map(2) // = elt^(q^2 * (12*z^3 + 6*z^2 + 6*z))
      R = Q * P              // = elt^(q^2 * (12*z^3 + 6*z^2 + 6*z) + q*(12*z^3 + 6*z^2 + 4*z) * (12*z^3 + 12*z^2 + 6*z + 1))
      S = conj(elt)          // = elt^(-1)
      T = S * L              // = elt^(12*z^3 + 6*z^2 + 4*z - 1)
      U = T.Frobenius_map(3) // = elt^(q^3(12*z^3 + 6*z^2 + 4*z - 1))
      V = U * R              // = elt^(q^3(12*z^3 + 6*z^2 + 4*z - 1) + q^2 * (12*z^3 + 6*z^2 + 6*z) + q*(12*z^3 + 6*z^2 + 4*z) * (12*z^3 + 12*z^2 + 6*z + 1))
      result = V

    */

    // let A= alt_bn128_exp_by_neg_z(elt);
    // let B= A.cyclotomic_squared();
    // let C= B.cyclotomic_squared();
    // let D= C * B;
    // let E= alt_bn128_exp_by_neg_z(D);
    // let F= E.cyclotomic_squared();
    // let G= alt_bn128_exp_by_neg_z(F);
    // let H= D.unitary_inverse();
    // let I= G.unitary_inverse();
    // let J= I * E;
    // let K= J * H;
    // let L= K * B;
    // let M= K * E;
    // let N= M * elt;
    // let O= L.Frobenius_map(1);
    // let P= O * N;
    // let Q= K.Frobenius_map(2);
    // let R= Q * P;
    // let S= elt.unitary_inverse();
    // let T= S * L;
    // let U= T.Frobenius_map(3);
    // let V= U * R;

    // let result= V;

    // leave_block("Call to alt_bn128_final_exponentiation_last_chunk");

    // return result;
    elt.clone()
}

pub fn alt_bn128_final_exponentiation(elt: &alt_bn128_Fq12) -> alt_bn128_GT {
    // enter_block("Call to alt_bn128_final_exponentiation");
    // alt_bn128_Fq12 A = alt_bn128_final_exponentiation_first_chunk(elt);
    // alt_bn128_GT result = alt_bn128_final_exponentiation_last_chunk(A);

    // leave_block("Call to alt_bn128_final_exponentiation");
    // return result;
    elt.clone()
}

/* ate pairing */

pub fn doubling_step_for_flipped_miller_loop(
    two_inv: alt_bn128_Fq,
    current: &alt_bn128_G2,
    c: &alt_bn128_ate_ell_coeffs,
) {
    // let X= current.X, Y = current.Y, Z = current.Z;

    // let A= two_inv * (X * Y);                     // A = X1 * Y1 / 2
    // let B= Y.squared();                           // B = Y1^2
    // let C= Z.squared();                           // C = Z1^2
    // let D= C+C+C;                                 // D = 3 * C
    // let E= alt_bn128_twist_coeff_b * D;             // E = twist_b * D
    // let F= E+E+E;                                 // F = 3 * E
    // let G= two_inv * (B+F);                       // G = (B+F)/2
    // let H= (Y+Z).squared() - (B+C);               // H = (Y1+Z1)^2-(B+C)
    // let I= E-B;                                   // I = E-B
    // let J= X.squared();                           // J = X1^2
    // let E_squared= E.squared();                   // E_squared = E^2

    // current.X = A * (B-F);                                       // X3 = A * (B-F)
    // current.Y = G.squared() - (E_squared+E_squared+E_squared);   // Y3 = G^2 - 3*E^2
    // current.Z = B * H;                                           // Z3 = B * H
    // c.ell_0 = alt_bn128_twist * I;                                 // ell_0 = xi * I
    // c.ell_VW = -H;                                               // ell_VW = - H (later: * yP)
    // c.ell_VV = J+J+J;                                            // ell_VV = 3*J (later: * xP)
}

pub fn mixed_addition_step_for_flipped_miller_loop(
    base: alt_bn128_G2,
    current: &alt_bn128_G2,
    c: &alt_bn128_ate_ell_coeffs,
) {
    // let X1= current.X;let  Y1 = current.Y;let  Z1 = current.Z;
    // x2:&alt_bn128_Fq2 = base.X, &y2 = base.Y;

    // let D= X1 - x2 * Z1;          // D = X1 - X2*Z1
    // let E= Y1 - y2 * Z1;          // E = Y1 - Y2*Z1
    // let F= D.squared();           // F = D^2
    // let G= E.squared();           // G = E^2
    // let H= D*F;                   // H = D*F
    // let I= X1 * F;                // I = X1 * F
    // let J= H + Z1*G - (I+I);      // J = H + Z1*G - (I+I)

    // current.X = D * J;                           // X3 = D*J
    // current.Y = E * (I-J)-(H * Y1);              // Y3 = E*(I-J)-(H*Y1)
    // current.Z = Z1 * H;                          // Z3 = Z1*H
    // c.ell_0 = alt_bn128_twist * (E * x2 - D * y2); // ell_0 = xi * (E * X2 - D * Y2)
    // c.ell_VV = - E;                              // ell_VV = - E (later: * xP)
    // c.ell_VW = D;                                // ell_VW = D (later: * yP    )
}

pub fn alt_bn128_ate_precompute_G1(P: &alt_bn128_G1) -> alt_bn128_ate_G1_precomp {
    // enter_block("Call to alt_bn128_ate_precompute_G1");

    // alt_bn128_G1 Pcopy = P;
    // Pcopy.to_affine_coordinates();

    // alt_bn128_ate_G1_precomp result;
    // result.PX = Pcopy.X;
    // result.PY = Pcopy.Y;

    // leave_block("Call to alt_bn128_ate_precompute_G1");
    // return result;
    let g: G1Affine = (*P).into();
    alt_bn128_ate_G1_precomp::from(g)
}

pub fn alt_bn128_ate_precompute_G2(Q: &alt_bn128_G2) -> alt_bn128_ate_G2_precomp {
    // enter_block("Call to alt_bn128_ate_precompute_G2");

    // alt_bn128_G2 Qcopy(Q);
    // Qcopy.to_affine_coordinates();

    // alt_bn128_Fq two_inv = (alt_bn128_Fq("2").inverse()); // could add to global params if needed

    // alt_bn128_ate_G2_precomp result;
    // result.QX = Qcopy.X;
    // result.QY = Qcopy.Y;

    // alt_bn128_G2 R;
    // R.X = Qcopy.X;
    // R.Y = Qcopy.Y;
    // R.Z = alt_bn128_Fq2::one();

    // loop_count:&bigint<alt_bn128_Fr::num_limbs> = alt_bn128_ate_loop_count;
    // bool found_one = false;
    // alt_bn128_ate_ell_coeffs c;

    // for i in ( 0..=loop_count.max_bits()).rev()
    // {
    //     let mut bit = loop_count.test_bit(i);
    //     if !found_one
    //     {
    //         /* this skips the MSB itself */
    //         found_one |= bit;
    //         continue;
    //     }

    //     doubling_step_for_flipped_miller_loop(two_inv, R, c);
    //     result.coeffs.push_back(c);

    //     if bit
    //     {
    //         mixed_addition_step_for_flipped_miller_loop(Qcopy, R, c);
    //         result.coeffs.push_back(c);
    //     }
    // }

    // alt_bn128_G2 Q1 = Qcopy.mul_by_q();
    // assert!(Q1.Z == alt_bn128_Fq2::one());
    // alt_bn128_G2 Q2 = Q1.mul_by_q();
    // assert!(Q2.Z == alt_bn128_Fq2::one());

    // if alt_bn128_ate_is_loop_count_neg
    // {
    //     R.Y = - R.Y;
    // }
    // Q2.Y = - Q2.Y;

    // mixed_addition_step_for_flipped_miller_loop(Q1, R, c);
    // result.coeffs.push_back(c);

    // mixed_addition_step_for_flipped_miller_loop(Q2, R, c);
    // result.coeffs.push_back(c);

    // leave_block("Call to alt_bn128_ate_precompute_G2");
    // return result;
    let g: G2Affine = (*Q).into();
    alt_bn128_ate_G2_precomp::from(g)
}

pub fn alt_bn128_ate_miller_loop(
    prec_P: &alt_bn128_ate_G1_precomp,
    prec_Q: &alt_bn128_ate_G2_precomp,
) -> alt_bn128_Fq12 {
    // enter_block("Call to alt_bn128_ate_miller_loop");

    // alt_bn128_Fq12 f = alt_bn128_Fq12::one();

    // bool found_one = false;
    // usize idx = 0;

    // loop_count:&bigint<alt_bn128_Fr::num_limbs> = alt_bn128_ate_loop_count;
    // alt_bn128_ate_ell_coeffs c;

    // for i in ( 0..=loop_count.max_bits()).rev()
    // {
    //     let mut bit = loop_count.test_bit(i);
    //     if !found_one
    //     {
    //         /* this skips the MSB itself */
    //         found_one |= bit;
    //         continue;
    //     }

    //     /* code below gets executed for all bits (EXCEPT the MSB itself) of
    //        alt_bn128_param_p (skipping leading zeros) in MSB to LSB
    //        order */
    //     c = prec_Q.coeffs[idx++];
    //     f = f.squared();
    //     f = f.mul_by_024(c.ell_0, prec_P.PY * c.ell_VW, prec_P.PX * c.ell_VV);

    //     if bit
    //     {
    //         c = prec_Q.coeffs[idx++];
    //         f = f.mul_by_024(c.ell_0, prec_P.PY * c.ell_VW, prec_P.PX * c.ell_VV);
    //     }

    // }

    // if alt_bn128_ate_is_loop_count_neg
    // {
    // 	f = f.inverse();
    // }

    // c = prec_Q.coeffs[idx++];
    // f = f.mul_by_024(c.ell_0,prec_P.PY * c.ell_VW,prec_P.PX * c.ell_VV);

    // c = prec_Q.coeffs[idx++];
    // f = f.mul_by_024(c.ell_0,prec_P.PY * c.ell_VW,prec_P.PX * c.ell_VV);

    // leave_block("Call to alt_bn128_ate_miller_loop");
    // return f;
    alt_bn128_Fq12::one()
}

pub fn alt_bn128_ate_double_miller_loop(
    prec_P1: &alt_bn128_ate_G1_precomp,
    prec_Q1: &alt_bn128_ate_G2_precomp,
    prec_P2: &alt_bn128_ate_G1_precomp,
    prec_Q2: &alt_bn128_ate_G2_precomp,
) -> alt_bn128_Fq12 {
    // enter_block("Call to alt_bn128_ate_double_miller_loop");

    // alt_bn128_Fq12 f = alt_bn128_Fq12::one();

    // bool found_one = false;
    // usize idx = 0;

    // loop_count:&bigint<alt_bn128_Fr::num_limbs> = alt_bn128_ate_loop_count;
    // for i in ( 0..=loop_count.max_bits()).rev()
    // {
    //     let mut bit = loop_count.test_bit(i);
    //     if !found_one
    //     {
    //         /* this skips the MSB itself */
    //         found_one |= bit;
    //         continue;
    //     }

    //     /* code below gets executed for all bits (EXCEPT the MSB itself) of
    //        alt_bn128_param_p (skipping leading zeros) in MSB to LSB
    //        order */
    //     alt_bn128_ate_ell_coeffs c1 = prec_Q1.coeffs[idx];
    //     alt_bn128_ate_ell_coeffs c2 = prec_Q2.coeffs[idx];
    //     ++idx;

    //     f = f.squared();

    //     f = f.mul_by_024(c1.ell_0, prec_P1.PY * c1.ell_VW, prec_P1.PX * c1.ell_VV);
    //     f = f.mul_by_024(c2.ell_0, prec_P2.PY * c2.ell_VW, prec_P2.PX * c2.ell_VV);

    //     if bit
    //     {
    //         alt_bn128_ate_ell_coeffs c1 = prec_Q1.coeffs[idx];
    //         alt_bn128_ate_ell_coeffs c2 = prec_Q2.coeffs[idx];
    //         ++idx;

    //         f = f.mul_by_024(c1.ell_0, prec_P1.PY * c1.ell_VW, prec_P1.PX * c1.ell_VV);
    //         f = f.mul_by_024(c2.ell_0, prec_P2.PY * c2.ell_VW, prec_P2.PX * c2.ell_VV);
    //     }
    // }

    // if alt_bn128_ate_is_loop_count_neg
    // {
    // 	f = f.inverse();
    // }

    // alt_bn128_ate_ell_coeffs c1 = prec_Q1.coeffs[idx];
    // alt_bn128_ate_ell_coeffs c2 = prec_Q2.coeffs[idx];
    // ++idx;
    // f = f.mul_by_024(c1.ell_0, prec_P1.PY * c1.ell_VW, prec_P1.PX * c1.ell_VV);
    // f = f.mul_by_024(c2.ell_0, prec_P2.PY * c2.ell_VW, prec_P2.PX * c2.ell_VV);

    // c1 = prec_Q1.coeffs[idx];
    // c2 = prec_Q2.coeffs[idx];
    // ++idx;
    // f = f.mul_by_024(c1.ell_0, prec_P1.PY * c1.ell_VW, prec_P1.PX * c1.ell_VV);
    // f = f.mul_by_024(c2.ell_0, prec_P2.PY * c2.ell_VW, prec_P2.PX * c2.ell_VV);

    // leave_block("Call to alt_bn128_ate_double_miller_loop");

    // return f;
    alt_bn128_Fq12::one()
}

pub fn alt_bn128_ate_pairing(P: &alt_bn128_G1, Q: &alt_bn128_G2) -> alt_bn128_Fq12 {
    // enter_block("Call to alt_bn128_ate_pairing");
    // alt_bn128_ate_G1_precomp prec_P = alt_bn128_ate_precompute_G1(P);
    // alt_bn128_ate_G2_precomp prec_Q = alt_bn128_ate_precompute_G2(Q);
    // alt_bn128_Fq12 result = alt_bn128_ate_miller_loop(prec_P, prec_Q);
    // leave_block("Call to alt_bn128_ate_pairing");
    // return result;
    alt_bn128_Fq12::one()
}

pub fn alt_bn128_ate_reduced_pairing(P: &alt_bn128_G1, Q: &alt_bn128_G2) -> alt_bn128_GT {
    // enter_block("Call to alt_bn128_ate_reduced_pairing");
    // let f= alt_bn128_ate_pairing(P, Q);
    // let result= alt_bn128_final_exponentiation(f);
    // leave_block("Call to alt_bn128_ate_reduced_pairing");
    // return result;
    alt_bn128_GT::one()
}

/* choice of pairing */

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
// } // namespace libff
