// /** @file
//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// //#ifndef ALT_BN128_G2_HPP_
// // #define ALT_BN128_G2_HPP_
// //#include <vector>

// use crate::algebra::curves::alt_bn128::alt_bn128_init;
// use crate::algebra::curves::curve_utils;
use crate::algebra::curves::pairing::Pairing;
use crate::algebra::curves::alt_bn128::curves::Bn254; 
pub type alt_bn128_G2=<Bn254 as Pairing>::G2;
// // namespace libff {

// pub struct alt_bn128_G2;
// std::ostream& operator<<(std::ostream &, const alt_bn128_G2&);
// std::istream& operator>>(std::istream &, alt_bn128_G2&);

// pub struct alt_bn128_G2 {

// // #ifdef PROFILE_OP_COUNTS
//     static i64 add_cnt;
//     static i64 dbl_cnt;
// //#endif
//     static Vec<std::usize> wnaf_window_table;
//     static Vec<std::usize> fixed_base_exp_window_table;
//     static alt_bn128_G2 G2_zero;
//     static alt_bn128_G2 G2_one;
//     static bool initialized;

//     type base_field=alt_bn128_Fq;
//     type twist_field=alt_bn128_Fq2;
//     type scalar_field=alt_bn128_Fr;

//     // Cofactor
//     static let h_bitcount= 256;
//     static let h_limbs= (h_bitcount+GMP_NUMB_BITS-1)/GMP_NUMB_BITS;
//     static bigint<h_limbs> h;

//     alt_bn128_Fq2 X, Y, Z;

//     // using Jacobian coordinates
//     alt_bn128_G2();
//     alt_bn128_G2(X:alt_bn128_Fq2&, Y:alt_bn128_Fq2&, Z:&alt_bn128_Fq2)->SelfX,Y,Z {};

//     static alt_bn128_Fq2 mul_by_b(elt:&alt_bn128_Fq2);

//     pub fn  print() const;
//     pub fn  print_coordinates() const;

//     pub fn  to_affine_coordinates();
//     pub fn  to_special();
//     bool is_special() const;

//     bool is_zero() const;

//     bool operator==(other:&alt_bn128_G2) const;
//     bool operator!=(other:&alt_bn128_G2) const;

//     alt_bn128_G2 operator+(other:&alt_bn128_G2) const;
//     alt_bn128_G2 operator-() const;
//     alt_bn128_G2 operator-(other:&alt_bn128_G2) const;

//     alt_bn128_G2 add(other:&alt_bn128_G2) const;
//     alt_bn128_G2 mixed_add(other:&alt_bn128_G2) const;
//     alt_bn128_G2 dbl() const;
//     alt_bn128_G2 mul_by_q() const;
//     alt_bn128_G2 mul_by_cofactor() const;

//     bool is_well_formed() const;

//     static alt_bn128_G2 zero();
//     static alt_bn128_G2 one();
//     static alt_bn128_G2 random_element();

//     static std::usize size_in_bits() { return twist_field::ceil_size_in_bits() + 1; }
//     static bigint<base_field::num_limbs> field_char() { return base_field::field_char(); }
//     static bigint<scalar_field::num_limbs> order() { return scalar_field::field_char(); }

//     friend std::ostream& operator<<(std::ostream &out, g:&alt_bn128_G2);
//     friend std::istream& operator>>(std::istream &in, alt_bn128_G2 &g);

//     static pub fn  batch_to_special_all_non_zeros(Vec<alt_bn128_G2> &vec);
// };


// alt_bn128_G2 operator*(lhs:&bigint<m>, rhs:&alt_bn128_G2)
// {
//     return scalar_mul<alt_bn128_G2, m>(rhs, lhs);
// }


// alt_bn128_G2 operator*(lhs:&Fp_model<m,modulus_p>, rhs:&alt_bn128_G2)
// {
//     return scalar_mul<alt_bn128_G2, m>(rhs, lhs.as_bigint());
// }


// // } // namespace libff
// //#endif // ALT_BN128_G2_HPP_
// /** @file
//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// use crate::algebra::curves::alt_bn128::alt_bn128_g2;

// // namespace libff {

// using std::usize;

// // #ifdef PROFILE_OP_COUNTS
// i64 alt_bn128_G2::add_cnt = 0;
// i64 alt_bn128_G2::dbl_cnt = 0;
// //#endif

// Vec<usize> alt_bn128_G2::wnaf_window_table;
// Vec<usize> alt_bn128_G2::fixed_base_exp_window_table;
// alt_bn128_G2 alt_bn128_G2::G2_zero = {};
// alt_bn128_G2 alt_bn128_G2::G2_one = {};
// bool alt_bn128_G2::initialized = false;
// bigint<alt_bn128_G2::h_limbs> alt_bn128_G2::h;

// pub fn new()
// {
//     if initialized
//     {
//         this->X = G2_zero.X;
//         this->Y = G2_zero.Y;
//         this->Z = G2_zero.Z;
//     }
// }

// alt_bn128_Fq2 alt_bn128_G2::mul_by_b(elt:&alt_bn128_Fq2)
// {
//     return alt_bn128_Fq2(alt_bn128_twist_mul_by_b_c0 * elt.c0, alt_bn128_twist_mul_by_b_c1 * elt.c1);
// }

// pub fn print() const
// {
//     if this->is_zero()
//     {
//         print!("O\n");
//     }
//     else
//     {
//         alt_bn128_G2 copy(*this);
//         copy.to_affine_coordinates();
//         print!("(%Nd*z + %Nd , %Nd*z + %Nd)\n",
//                    copy.X.c1.as_bigint().0.0, alt_bn128_Fq::num_limbs,
//                    copy.X.c0.as_bigint().0.0, alt_bn128_Fq::num_limbs,
//                    copy.Y.c1.as_bigint().0.0, alt_bn128_Fq::num_limbs,
//                    copy.Y.c0.as_bigint().0.0, alt_bn128_Fq::num_limbs);
//     }
// }

// pub fn print_coordinates() const
// {
//     if this->is_zero()
//     {
//         print!("O\n");
//     }
//     else
//     {
//         print!("(%Nd*z + %Nd : %Nd*z + %Nd : %Nd*z + %Nd)\n",
//                    this->X.c1.as_bigint().0.0, alt_bn128_Fq::num_limbs,
//                    this->X.c0.as_bigint().0.0, alt_bn128_Fq::num_limbs,
//                    this->Y.c1.as_bigint().0.0, alt_bn128_Fq::num_limbs,
//                    this->Y.c0.as_bigint().0.0, alt_bn128_Fq::num_limbs,
//                    this->Z.c1.as_bigint().0.0, alt_bn128_Fq::num_limbs,
//                    this->Z.c0.as_bigint().0.0, alt_bn128_Fq::num_limbs);
//     }
// }

// pub fn to_affine_coordinates()
// {
//     if this->is_zero()
//     {
//         this->X = alt_bn128_Fq2::zero();
//         this->Y = alt_bn128_Fq2::one();
//         this->Z = alt_bn128_Fq2::zero();
//     }
//     else
//     {
//         alt_bn128_Fq2 Z_inv = Z.inverse();
//         alt_bn128_Fq2 Z2_inv = Z_inv.squared();
//         alt_bn128_Fq2 Z3_inv = Z2_inv * Z_inv;
//         this->X = this->X * Z2_inv;
//         this->Y = this->Y * Z3_inv;
//         this->Z = alt_bn128_Fq2::one();
//     }
// }

// pub fn to_special()
// {
//     this->to_affine_coordinates();
// }

// pub fn is_special()->bool
// {
//     return (this->is_zero() || this->Z == alt_bn128_Fq2::one());
// }

// pub fn is_zero()->bool
// {
//     return (this->Z.is_zero());
// }

// bool alt_bn128_G2::operator==(other:&alt_bn128_G2) const
// {
//     if this->is_zero()
//     {
//         return other.is_zero();
//     }

//     if other.is_zero()
//     {
//         return false;
//     }

//     /* now neither is O */

//     // using Jacobian coordinates so:
//     // (X1:Y1:Z1) = (X2:Y2:Z2)
//     // iff
//     // X1/Z1^2 == X2/Z2^2 and Y1/Z1^3 == Y2/Z2^3
//     // iff
//     // X1 * Z2^2 == X2 * Z1^2 and Y1 * Z2^3 == Y2 * Z1^3

//     alt_bn128_Fq2 Z1_squared = (this->Z).squared();
//     alt_bn128_Fq2 Z2_squared = (other.Z).squared();

//     if (this->X * Z2_squared) != (other.X * Z1_squared)
//     {
//         return false;
//     }

//     alt_bn128_Fq2 Z1_cubed = (this->Z) * Z1_squared;
//     alt_bn128_Fq2 Z2_cubed = (other.Z) * Z2_squared;

//     return !((this->Y * Z2_cubed) != (other.Y * Z1_cubed));
// }

// bool alt_bn128_G2::operator!=(other:&alt_bn128_G2) const
// {
//     return !(operator==(other));
// }

// alt_bn128_G2 alt_bn128_G2::operator+(other:&alt_bn128_G2) const
// {
//     // handle special cases having to do with O
//     if this->is_zero()
//     {
//         return other;
//     }

//     if other.is_zero()
//     {
//         return *this;
//     }

//     // no need to handle points of order 2,4
//     // (they cannot exist in a prime-order subgroup)

//     // using Jacobian coordinates according to
//     // https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#addition-add-2007-bl
//     // Note: (X1:Y1:Z1) = (X2:Y2:Z2)
//     // iff
//     // X1/Z1^2 == X2/Z2^2 and Y1/Z1^3 == Y2/Z2^3
//     // iff
//     // X1 * Z2^2 == X2 * Z1^2 and Y1 * Z2^3 == Y2 * Z1^3

//     alt_bn128_Fq2 Z1Z1 = (this->Z).squared();
//     alt_bn128_Fq2 Z2Z2 = (other.Z).squared();

//     alt_bn128_Fq2 U1 = this->X * Z2Z2;
//     alt_bn128_Fq2 U2 = other.X * Z1Z1;

//     alt_bn128_Fq2 Z1_cubed = (this->Z) * Z1Z1;
//     alt_bn128_Fq2 Z2_cubed = (other.Z) * Z2Z2;

//     alt_bn128_Fq2 S1 = (this->Y) * Z2_cubed;      // S1 = Y1 * Z2 * Z2Z2
//     alt_bn128_Fq2 S2 = (other.Y) * Z1_cubed;      // S2 = Y2 * Z1 * Z1Z1

//     // check for doubling case
//     if U1 == U2 && S1 == S2
//     {
//         // dbl case; nothing of above can be reused
//         return this->dbl();
//     }

// // #ifdef PROFILE_OP_COUNTS
//     this->add_cnt++;
// //#endif

//     // rest of add case
//     alt_bn128_Fq2 H = U2 - U1;                            // H = U2-U1
//     alt_bn128_Fq2 S2_minus_S1 = S2-S1;
//     alt_bn128_Fq2 I = (H+H).squared();                    // I = (2 * H)^2
//     alt_bn128_Fq2 J = H * I;                              // J = H * I
//     alt_bn128_Fq2 r = S2_minus_S1 + S2_minus_S1;          // r = 2 * (S2-S1)
//     alt_bn128_Fq2 V = U1 * I;                             // V = U1 * I
//     alt_bn128_Fq2 X3 = r.squared() - J - (V+V);           // X3 = r^2 - J - 2 * V
//     alt_bn128_Fq2 S1_J = S1 * J;
//     alt_bn128_Fq2 Y3 = r * (V-X3) - (S1_J+S1_J);          // Y3 = r * (V-X3)-2 S1 J
//     alt_bn128_Fq2 Z3 = ((this->Z+other.Z).squared()-Z1Z1-Z2Z2) * H; // Z3 = ((Z1+Z2)^2-Z1Z1-Z2Z2) * H

//     return alt_bn128_G2(X3, Y3, Z3);
// }

// alt_bn128_G2 alt_bn128_G2::operator-() const
// {
//     return alt_bn128_G2(this->X, -(this->Y), this->Z);
// }


// alt_bn128_G2 alt_bn128_G2::operator-(other:&alt_bn128_G2) const
// {
//     return (*this) + (-other);
// }

// pub fn add(other:&alt_bn128_G2)->alt_bn128_G2
// {
//     return (*this) + other;
// }

// pub fn mixed_add(other:&alt_bn128_G2)->alt_bn128_G2
// {
// // #ifdef DEBUG
//     assert!(other.is_special());
// //#endif

//     // handle special cases having to do with O
//     if this->is_zero()
//     {
//         return other;
//     }

//     if other.is_zero()
//     {
//         return *this;
//     }

//     // no need to handle points of order 2,4
//     // (they cannot exist in a prime-order subgroup)

//     // using Jacobian coordinates according to
//     // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#addition-madd-2007-bl
//     // Note: (X1:Y1:Z1) = (X2:Y2:Z2)
//     // iff
//     // X1/Z1^2 == X2/Z2^2 and Y1/Z1^3 == Y2/Z2^3
//     // iff
//     // X1 * Z2^2 == X2 * Z1^2 and Y1 * Z2^3 == Y2 * Z1^3
//     // we know that Z2 = 1

//     let Z1Z1= (this->Z).squared();

//     U1:&alt_bn128_Fq2 = this->X;
//     let U2= other.X * Z1Z1;

//     let Z1_cubed= (this->Z) * Z1Z1;

//     S1:&alt_bn128_Fq2 = (this->Y);                // S1 = Y1 * Z2 * Z2Z2
//     let S2= (other.Y) * Z1_cubed;      // S2 = Y2 * Z1 * Z1Z1

//     // check for doubling case
//     if U1 == U2 && S1 == S2
//     {
//         // dbl case; nothing of above can be reused
//         return this->dbl();
//     }

// // #ifdef PROFILE_OP_COUNTS
//     this->add_cnt++;
// //#endif

//     alt_bn128_Fq2 H = U2-(this->X);                         // H = U2-X1
//     alt_bn128_Fq2 HH = H.squared() ;                        // HH = H^2
//     alt_bn128_Fq2 I = HH+HH;                                // I = 4*HH
//     I = I + I;
//     alt_bn128_Fq2 J = H*I;                                  // J = H*I
//     alt_bn128_Fq2 r = S2-(this->Y);                         // r = 2*(S2-Y1)
//     r = r + r;
//     alt_bn128_Fq2 V = (this->X) * I ;                       // V = X1*I
//     alt_bn128_Fq2 X3 = r.squared()-J-V-V;                   // X3 = r^2-J-2*V
//     alt_bn128_Fq2 Y3 = (this->Y)*J;                         // Y3 = r*(V-X3)-2*Y1*J
//     Y3 = r*(V-X3) - Y3 - Y3;
//     alt_bn128_Fq2 Z3 = ((this->Z)+H).squared() - Z1Z1 - HH; // Z3 = (Z1+H)^2-Z1Z1-HH

//     return alt_bn128_G2(X3, Y3, Z3);
// }

// pub fn dbl()->alt_bn128_G2
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->dbl_cnt++;
// //#endif
//     // handle point at infinity
//     if this->is_zero()
//     {
//         return (*this);
//     }

//     // NOTE: does not handle O and pts of order 2,4
//     // (they cannot exist in a prime-order subgroup)

//     // using Jacobian coordinates according to
//     // https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#doubling-dbl-2009-l
    
//     alt_bn128_Fq2 A = (this->X).squared();         // A = X1^2
//     alt_bn128_Fq2 B = (this->Y).squared();        // B = Y1^2
//     alt_bn128_Fq2 C = B.squared();                // C = B^2
//     alt_bn128_Fq2 D = (this->X + B).squared() - A - C;
//     D = D+D;                        // D = 2 * ((X1 + B)^2 - A - C)
//     alt_bn128_Fq2 E = A + A + A;                  // E = 3 * A
//     alt_bn128_Fq2 F = E.squared();                // F = E^2
//     alt_bn128_Fq2 X3 = F - (D+D);                 // X3 = F - 2 D
//     alt_bn128_Fq2 eightC = C+C;
//     eightC = eightC + eightC;
//     eightC = eightC + eightC;
//     alt_bn128_Fq2 Y3 = E * (D - X3) - eightC;     // Y3 = E * (D - X3) - 8 * C
//     alt_bn128_Fq2 Y1Z1 = (this->Y)*(this->Z);
//     alt_bn128_Fq2 Z3 = Y1Z1 + Y1Z1;               // Z3 = 2 * Y1 * Z1

//     return alt_bn128_G2(X3, Y3, Z3);
// }

// pub fn mul_by_q()->alt_bn128_G2
// {
//     return alt_bn128_G2(alt_bn128_twist_mul_by_q_X * (this->X).Frobenius_map(1),
//                       alt_bn128_twist_mul_by_q_Y * (this->Y).Frobenius_map(1),
//                       (this->Z).Frobenius_map(1));
// }

// pub fn mul_by_cofactor()->alt_bn128_G2
// {
//     return alt_bn128_G2::h * (*this);
// }

// pub fn is_well_formed()->bool
// {
//     if this->is_zero()
//     {
//         return true;
//     }
//     /*
//         y^2 = x^3 + b

//         We are using Jacobian coordinates, so equation we need to check is actually

//         (y/z^3)^2 = (x/z^2)^3 + b
//         y^2 / z^6 = x^3 / z^6 + b
//         y^2 = x^3 + b z^6
//     */
//     alt_bn128_Fq2 X2 = this->X.squared();
//     alt_bn128_Fq2 Y2 = this->Y.squared();
//     alt_bn128_Fq2 Z2 = this->Z.squared();

//     alt_bn128_Fq2 X3 = this->X * X2;
//     alt_bn128_Fq2 Z3 = this->Z * Z2;
//     alt_bn128_Fq2 Z6 = Z3.squared();

//     return (Y2 == X3 + alt_bn128_twist_coeff_b * Z6);
// }

// alt_bn128_G2 alt_bn128_G2::zero()
// {
//     return G2_zero;
// }

// alt_bn128_G2 alt_bn128_G2::one()
// {
//     return G2_one;
// }

// alt_bn128_G2 alt_bn128_G2::random_element()
// {
//     return (alt_bn128_Fr::random_element().as_bigint()) * G2_one;
// }

// std::ostream& operator<<(std::ostream &out, g:&alt_bn128_G2)
// {
//     alt_bn128_G2 copy(g);
//     copy.to_affine_coordinates();
//     out << if copy.is_zero() {1} else{0} << OUTPUT_SEPARATOR;
// // #ifdef NO_PT_COMPRESSION
//     out << copy.X << OUTPUT_SEPARATOR << copy.Y;
// #else
//     /* storing LSB of Y */
//     out << copy.X << OUTPUT_SEPARATOR << (copy.Y.c0.as_bigint().0.0[0] & 1);
// //#endif

//     return out;
// }

// std::istream& operator>>(std::istream &in, alt_bn128_G2 &g)
// {
//     char is_zero;
//     alt_bn128_Fq2 tX, tY;

// // #ifdef NO_PT_COMPRESSION
//     in >> is_zero >> tX >> tY;
//     is_zero -= '0';
// #else
//     in.read((char*)&is_zero, 1); // this reads is_zero;
//     is_zero -= '0';
//     consume_OUTPUT_SEPARATOR(in);

//     unsigned char Y_lsb;
//     in >> tX;
//     consume_OUTPUT_SEPARATOR(in);
//     in.read((char*)&Y_lsb, 1);
//     Y_lsb -= '0';

//     // y = +/- sqrt(x^3 + b)
//     if is_zero == 0
//     {
//         alt_bn128_Fq2 tX2 = tX.squared();
//         alt_bn128_Fq2 tY2 = tX2 * tX + alt_bn128_twist_coeff_b;
//         tY = tY2.sqrt();

//         if (tY.c0.as_bigint().0.0[0] & 1) != Y_lsb
//         {
//             tY = -tY;
//         }
//     }
// //#endif
//     // using projective coordinates
//     if is_zero == 0
//     {
//         g.X = tX;
//         g.Y = tY;
//         g.Z = alt_bn128_Fq2::one();
//     }
//     else
//     {
//         g = alt_bn128_G2::zero();
//     }

//     return in;
// }

// pub fn batch_to_special_all_non_zeros(Vec<alt_bn128_G2> &vec)
// {
//     Vec<alt_bn128_Fq2> Z_vec;
//     Z_vec.reserve(vec.len());

//     for el in &vec
//     {
//         Z_vec.emplace_back(el.Z);
//     }
//     batch_invert<alt_bn128_Fq2>(Z_vec);

//     let one= alt_bn128_Fq2::one();

//     for i in 0..vec.len()
//     {
//         alt_bn128_Fq2 Z2 = Z_vec[i].squared();
//         alt_bn128_Fq2 Z3 = Z_vec[i] * Z2;

//         vec[i].X = vec[i].X * Z2;
//         vec[i].Y = vec[i].Y * Z3;
//         vec[i].Z = one;
//     }
// }

// // } // namespace libff
