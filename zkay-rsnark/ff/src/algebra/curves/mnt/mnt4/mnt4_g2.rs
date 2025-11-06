// /** @file
//  *****************************************************************************

//  Declaration of interfaces for the MNT4 G2 group.

//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// //#ifndef MNT4_G2_HPP_
// // #define MNT4_G2_HPP_

// //#include <vector>

// use crate::algebra::curves::curve_utils;
// use crate::algebra::curves::mnt::mnt4::mnt4_init;

// // namespace libff {

// // pub struct mnt4_G2;
// // std::ostream& operator<<(std::ostream &, const mnt4_G2&);
// // std::istream& operator>>(std::istream &, mnt4_G2&);

// pub struct mnt4_G2 {
// // 
// // // #ifdef PROFILE_OP_COUNTS
// //     static i64 add_cnt;
// //     static i64 dbl_cnt;
// // //#endif
// //     static Vec<std::usize> wnaf_window_table;
// //     static Vec<std::usize> fixed_base_exp_window_table;
// //     static mnt4_G2 G2_zero;
// //     static mnt4_G2 G2_one;
// //     static bool initialized;
// //     static mnt4_Fq2 twist;
// //     static mnt4_Fq2 coeff_a;
// //     static mnt4_Fq2 coeff_b;

// //     type base_field=mnt4_Fq;
// //     type twist_field=mnt4_Fq2;
// //     type scalar_field=mnt4_Fr;

// //     // Cofactor
// //     static let h_bitcount= 298;
// //     static let h_limbs= (h_bitcount+GMP_NUMB_BITS-1)/GMP_NUMB_BITS;
// //     static bigint<h_limbs> h;

//      X:mnt4_Fq2, Y:mnt4_Fq2, Z:mnt4_Fq2,
// }

// //     // using projective coordinates
// //     mnt4_G2();
// //     mnt4_G2(X:mnt4_Fq2&, Y:mnt4_Fq2&, Z:&mnt4_Fq2)->SelfX,Y,Z {};

// //     static mnt4_Fq2 mul_by_a(elt:&mnt4_Fq2);
// //     static mnt4_Fq2 mul_by_b(elt:&mnt4_Fq2);

// //     pub fn  print() const;
// //     pub fn  print_coordinates() const;

// //     pub fn  to_affine_coordinates();
// //     pub fn  to_special();
// //     bool is_special() const;

// //     bool is_zero() const;

// //     bool operator==(other:&mnt4_G2) const;
// //     bool operator!=(other:&mnt4_G2) const;

// //     mnt4_G2 operator+(other:&mnt4_G2) const;
// //     mnt4_G2 operator-() const;
// //     mnt4_G2 operator-(other:&mnt4_G2) const;

// //     mnt4_G2 add(other:&mnt4_G2) const;
// //     mnt4_G2 mixed_add(other:&mnt4_G2) const;
// //     mnt4_G2 dbl() const;
// //     mnt4_G2 mul_by_q() const;
// //     mnt4_G2 mul_by_cofactor() const;

// //     bool is_well_formed() const;

// //     static mnt4_G2 zero();
// //     static mnt4_G2 one();
// //     static mnt4_G2 random_element();

// //     static std::usize size_in_bits() { return mnt4_Fq2::ceil_size_in_bits() + 1; }
// //     static bigint<mnt4_Fq::num_limbs> field_char() { return mnt4_Fq::field_char(); }
// //     static bigint<mnt4_Fr::num_limbs> order() { return mnt4_Fr::field_char(); }

// //     friend std::ostream& operator<<(std::ostream &out, g:&mnt4_G2);
// //     friend std::istream& operator>>(std::istream &in, mnt4_G2 &g);

// //     static pub fn  batch_to_special_all_non_zeros(Vec<mnt4_G2> &vec);
// // };


// // } // namespace libff

// //#endif // MNT4_G2_HPP_
// /** @file
//  *****************************************************************************

//  Implementation of interfaces for the MNT4 G2 group.

//  See mnt4_g2.hpp .

//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// use crate::algebra::curves::mnt::mnt4::mnt4_g2;

// // namespace libff {

// // using std::usize;

// // // #ifdef PROFILE_OP_COUNTS
// // i64 mnt4_G2::add_cnt = 0;
// // i64 mnt4_G2::dbl_cnt = 0;
// // //#endif

// // Vec<usize> mnt4_G2::wnaf_window_table;
// // Vec<usize> mnt4_G2::fixed_base_exp_window_table;
// // mnt4_Fq2 mnt4_G2::twist;
// // mnt4_Fq2 mnt4_G2::coeff_a;
// // mnt4_Fq2 mnt4_G2::coeff_b;
// // mnt4_G2 mnt4_G2::G2_zero = {};
// // mnt4_G2 mnt4_G2::G2_one = {};
// // bool mnt4_G2::initialized;
// // bigint<mnt4_G2::h_limbs> mnt4_G2::h;
// {
// mnt4_Fq2 mnt4_G2::mul_by_a(elt:&mnt4_Fq2)
// {
//     return mnt4_Fq2(mnt4_twist_mul_by_a_c0 * elt.c0, mnt4_twist_mul_by_a_c1 * elt.c1);
// }

// mnt4_Fq2 mnt4_G2::mul_by_b(elt:&mnt4_Fq2)
// {
//     return mnt4_Fq2(mnt4_twist_mul_by_b_c0 * elt.c1, mnt4_twist_mul_by_b_c1 * elt.c0);
// }

// pub fn new()
// {
//     if mnt4_G2::initialized
//     {
//         this->X = G2_zero.X;
//         this->Y = G2_zero.Y;
//         this->Z = G2_zero.Z;
//     }
// }

// pub fn print() const
// {
//     if this->is_zero()
//     {
//         print!("O\n");
//     }
//     else
//     {
//         mnt4_G2 copy(*this);
//         copy.to_affine_coordinates();
//         print!("(%Nd*z + %Nd , %Nd*z + %Nd)\n",
//                    copy.X.c1.as_bigint().0.0, mnt4_Fq::num_limbs,
//                    copy.X.c0.as_bigint().0.0, mnt4_Fq::num_limbs,
//                    copy.Y.c1.as_bigint().0.0, mnt4_Fq::num_limbs,
//                    copy.Y.c0.as_bigint().0.0, mnt4_Fq::num_limbs);
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
//                    this->X.c1.as_bigint().0.0, mnt4_Fq::num_limbs,
//                    this->X.c0.as_bigint().0.0, mnt4_Fq::num_limbs,
//                    this->Y.c1.as_bigint().0.0, mnt4_Fq::num_limbs,
//                    this->Y.c0.as_bigint().0.0, mnt4_Fq::num_limbs,
//                    this->Z.c1.as_bigint().0.0, mnt4_Fq::num_limbs,
//                    this->Z.c0.as_bigint().0.0, mnt4_Fq::num_limbs);
//     }
// }

// pub fn to_affine_coordinates()
// {
//     if this->is_zero()
//     {
//         this->X = mnt4_Fq2::zero();
//         this->Y = mnt4_Fq2::one();
//         this->Z = mnt4_Fq2::zero();
//     }
//     else
//     {
//         let Z_inv= Z.inverse();
//         X = X * Z_inv;
//         Y = Y * Z_inv;
//         Z = mnt4_Fq2::one();
//     }
// }

// pub fn to_special()
// {
//     this->to_affine_coordinates();
// }

// pub fn is_special()->bool
// {
//     return (this->is_zero() || this->Z == mnt4_Fq2::one());
// }

// pub fn is_zero()->bool
// {
//     return (this->X.is_zero() && this->Z.is_zero());
// }


// pub fn add(other:&mnt4_G2)->mnt4_G2
// {
//     // handle special cases having to do with O
//     if this->is_zero()
//     {
//         return other;
//     }

//     if other.is_zero()
//     {
//         return (*this);
//     }

//     // no need to handle points of order 2,4
//     // (they cannot exist in a prime-order subgroup)

//     // handle double case
//     if this->operator==(other)
//     {
//         return this->dbl();
//     }

// // #ifdef PROFILE_OP_COUNTS
//     this->add_cnt++;
// //#endif
//     // NOTE: does not handle O and pts of order 2,4
//     // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html#addition-add-1998-cmo-2

//     let Y1Z2= (this->Y) * (other.Z);        // Y1Z2 = Y1*Z2
//     let X1Z2= (this->X) * (other.Z);        // X1Z2 = X1*Z2
//     let Z1Z2= (this->Z) * (other.Z);        // Z1Z2 = Z1*Z2
//     let u= (other.Y) * (this->Z) - Y1Z2; // u    = Y2*Z1-Y1Z2
//     let uu= u.squared();                    // uu   = u^2
//     let v= (other.X) * (this->Z) - X1Z2; // v    = X2*Z1-X1Z2
//     let vv= v.squared();                    // vv   = v^2
//     let vvv= v * vv;                         // vvv  = v*vv
//     let R= vv * X1Z2;                      // R    = vv*X1Z2
//     let A= uu * Z1Z2 - (vvv + R + R);      // A    = uu*Z1Z2 - vvv - 2*R
//     let X3= v * A;                          // X3   = v*A
//     let Y3= u * (R-A) - vvv * Y1Z2;         // Y3   = u*(R-A) - vvv*Y1Z2
//     let Z3= vvv * Z1Z2;                     // Z3   = vvv*Z1Z2

//     return mnt4_G2(X3, Y3, Z3);
// }

// pub fn mixed_add(other:&mnt4_G2)->mnt4_G2
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->add_cnt++;
// //#endif
//     // NOTE: does not handle O and pts of order 2,4
//     // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html#addition-add-1998-cmo-2
//     //assert!(other.Z == mnt4_Fq2::one());

//     if this->is_zero()
//     {
//         return other;
//     }

//     if other.is_zero()
//     {
//         return (*this);
//     }

// // #ifdef DEBUG
//     assert!(other.is_special());
// //#endif

//     X1Z2:&mnt4_Fq2 = (this->X);                   // X1Z2 = X1*Z2 (but other is special and not zero)
//     let X2Z1= (this->Z) * (other.X);       // X2Z1 = X2*Z1

//     // (used both in add and double checks)

//     Y1Z2:&mnt4_Fq2 = (this->Y);                   // Y1Z2 = Y1*Z2 (but other is special and not zero)
//     let Y2Z1= (this->Z) * (other.Y);       // Y2Z1 = Y2*Z1

//     if X1Z2 == X2Z1 && Y1Z2 == Y2Z1
//     {
//         return this->dbl();
//     }

//     let u= Y2Z1 - this->Y;              // u = Y2*Z1-Y1
//     let uu= u.squared();                 // uu = u2
//     let v= X2Z1 - this->X;              // v = X2*Z1-X1
//     let vv= v.squared();                 // vv = v2
//     let vvv= v*vv;                       // vvv = v*vv
//     let R= vv * this->X;                // R = vv*X1
//     let A= uu * this->Z - vvv - R - R;  // A = uu*Z1-vvv-2*R
//     let X3= v * A;                       // X3 = v*A
//     let Y3= u*(R-A) - vvv * this->Y;    // Y3 = u*(R-A)-vvv*Y1
//     let Z3= vvv * this->Z;              // Z3 = vvv*Z1

//     return mnt4_G2(X3, Y3, Z3);
// }

// pub fn dbl()->mnt4_G2
// {
// // #ifdef PROFILE_OP_COUNTS
//     this->dbl_cnt++;
// //#endif
//     if this->is_zero()
//     {
//         return (*this);
//     }
//     // NOTE: does not handle O and pts of order 2,4
//     // http://www.hyperelliptic.org/EFD/g1p/auto-shortw-projective.html#doubling-dbl-2007-bl

//     let XX= (this->X).squared();                     // XX  = X1^2
//     let ZZ= (this->Z).squared();                     // ZZ  = Z1^2
//     let w= mnt4_G2::mul_by_a(ZZ) + (XX + XX + XX);   // w   = a*ZZ + 3*XX
//     let Y1Z1= (this->Y) * (this->Z);
//     let s= Y1Z1 + Y1Z1;                              // s   = 2*Y1*Z1
//     let ss= s.squared();                              // ss  = s^2
//     let sss= s * ss;                                   // sss = s*ss
//     let R= (this->Y) * s;                           // R   = Y1*s
//     let RR= R.squared();                              // RR  = R^2
//     let B= ((this->X)+R).squared()-XX-RR;           // B   = (X1+R)^2 - XX - RR
//     let h= w.squared() - (B+B);                      // h   = w^2-2*B
//     let X3= h * s;                                    // X3  = h*s
//     let Y3= w * (B-h)-(RR+RR);                        // Y3  = w*(B-h) - 2*RR
//     let Z3= sss;                                      // Z3  = sss

//     return mnt4_G2(X3, Y3, Z3);
// }

// pub fn mul_by_q()->mnt4_G2
// {
//     return mnt4_G2(mnt4_twist_mul_by_q_X * (this->X).Frobenius_map(1),
//                    mnt4_twist_mul_by_q_Y * (this->Y).Frobenius_map(1),
//                    (this->Z).Frobenius_map(1));
// }

// pub fn mul_by_cofactor()->mnt4_G2
// {
//     return mnt4_G2::h * (*this);
// }

// pub fn is_well_formed()->bool
// {
//     if this->is_zero()
//     {
//         return true;
//     }
//     /*
//         y^2 = x^3 + ax + b

//         We are using projective, so equation we need to check is actually

//         (y/z)^2 = (x/z)^3 + a (x/z) + b
//         z y^2 = x^3  + a z^2 x + b z^3

//         z (y^2 - b z^2) = x ( x^2 + a z^2)
//     */
//     let X2= this->X.squared();
//     let Y2= this->Y.squared();
//     let Z2= this->Z.squared();
//     let aZ2=  mnt4_twist_coeff_a * Z2;

//     return (this->Z * (Y2 - mnt4_twist_coeff_b * Z2) == this->X * (X2 + aZ2));
// }

// mnt4_G2 mnt4_G2::zero()
// {
//     return G2_zero;
// }

// mnt4_G2 mnt4_G2::one()
// {
//     return G2_one;
// }

// mnt4_G2 mnt4_G2::random_element()
// {
//     return (mnt4_Fr::random_element().as_bigint()) * G2_one;
// }

// pub fn batch_to_special_all_non_zeros(Vec<mnt4_G2> &vec)
// {
//     Vec<mnt4_Fq2> Z_vec;
//     Z_vec.reserve(vec.len());

//     for el in &vec
//     {
//         Z_vec.emplace_back(el.Z);
//     }
//     batch_invert<mnt4_Fq2>(Z_vec);

//     let one= mnt4_Fq2::one();

//     for i in 0..vec.len()
//     {
//         vec[i] = mnt4_G2(vec[i].X * Z_vec[i], vec[i].Y * Z_vec[i], one);
//     }
// }
// }

// // } // namespace libff

// // 
// // mnt4_G2 operator*(lhs:&bigint<m>, rhs:&mnt4_G2)
// // {
// //     return scalar_mul<mnt4_G2, m>(rhs, lhs);
// // }

// // 
// // mnt4_G2 operator*(lhs:&Fp_model<m,modulus_p>, rhs:&mnt4_G2)
// // {
// //     return scalar_mul<mnt4_G2, m>(rhs, lhs.as_bigint());
// // }


// // std::ostream& operator<<(std::ostream &out, g:&mnt4_G2)
// // {
// //     mnt4_G2 copy(g);
// //     copy.to_affine_coordinates();

// //     out << if copy.is_zero() {1} else{0} << OUTPUT_SEPARATOR;
// // // #ifdef NO_PT_COMPRESSION
// //     out << copy.X << OUTPUT_SEPARATOR << copy.Y;
// // #else
// //     /* storing LSB of Y */
// //     out << copy.X << OUTPUT_SEPARATOR << (copy.Y.c0.as_bigint().0.0[0] & 1);
// // //#endif

// //     return out;
// // }

// // std::istream& operator>>(std::istream &in, mnt4_G2 &g)
// // {
// //     char is_zero;
// //     mnt4_Fq2 tX, tY;

// // // #ifdef NO_PT_COMPRESSION
// //     in >> is_zero >> tX >> tY;
// //     is_zero -= '0';
// // #else
// //     in.read((char*)&is_zero, 1); // this reads is_zero;
// //     is_zero -= '0';
// //     consume_OUTPUT_SEPARATOR(in);

// //     unsigned char Y_lsb;
// //     in >> tX;
// //     consume_OUTPUT_SEPARATOR(in);
// //     in.read((char*)&Y_lsb, 1);
// //     Y_lsb -= '0';

// //     // y = +/- sqrt(x^3 + a*x + b)
// //     if is_zero == 0
// //     {
// //         mnt4_Fq2 tX2 = tX.squared();
// //         mnt4_Fq2 tY2 = (tX2 + mnt4_twist_coeff_a ) * tX + mnt4_twist_coeff_b;
// //         tY = tY2.sqrt();

// //         if (tY.c0.as_bigint().0.0[0] & 1) != Y_lsb
// //         {
// //             tY = -tY;
// //         }
// //     }
// // //#endif
// //     // using projective coordinates
// //     if is_zero == 0
// //     {
// //         g.X = tX;
// //         g.Y = tY;
// //         g.Z = mnt4_Fq2::one();
// //     }
// //     else
// //     {
// //         g = mnt4_G2::zero();
// //     }

// //     return in;
// // }

// // bool mnt4_G2::operator==(other:&mnt4_G2) const
// // {
// //     if this->is_zero()
// //     {
// //         return other.is_zero();
// //     }

// //     if other.is_zero()
// //     {
// //         return false;
// //     }

// //     /* now neither is O */

// //     // X1/Z1 = X2/Z2 <=> X1*Z2 = X2*Z1
// //     if (this->X * other.Z) != (other.X * this->Z)
// //     {
// //         return false;
// //     }

// //     // Y1/Z1 = Y2/Z2 <=> Y1*Z2 = Y2*Z1
// //     if (this->Y * other.Z) != (other.Y * this->Z)
// //     {
// //         return false;
// //     }

// //     return true;
// // }

// // bool mnt4_G2::operator!=(other:&mnt4_G2) const
// // {
// //     return !(operator==(other));
// // }

// // mnt4_G2 mnt4_G2::operator+(other:&mnt4_G2) const
// // {
// //     // handle special cases having to do with O
// //     if this->is_zero()
// //     {
// //         return other;
// //     }

// //     if other.is_zero()
// //     {
// //         return *this;
// //     }

// //     // no need to handle points of order 2,4
// //     // (they cannot exist in a prime-order subgroup)

// //     // handle double case, and then all the rest
// //     /*
// //       The code below is equivalent to (but faster than) the snippet below:

// //       if this->operator==(other)
// //       {
// //       return this->dbl();
// //       }
// //       else
// //       {
// //       return this->add(other);
// //       }
// //     */

// //     let X1Z2= (this->X) * (other.Z);        // X1Z2 = X1*Z2
// //     let X2Z1= (this->Z) * (other.X);        // X2Z1 = X2*Z1

// //     // (used both in add and double checks)

// //     let Y1Z2= (this->Y) * (other.Z);        // Y1Z2 = Y1*Z2
// //     let Y2Z1= (this->Z) * (other.Y);        // Y2Z1 = Y2*Z1

// //     if X1Z2 == X2Z1 && Y1Z2 == Y2Z1
// //     {
// //         // perform dbl case
// //         let XX= (this->X).squared();                   // XX  = X1^2
// //         let ZZ= (this->Z).squared();                   // ZZ  = Z1^2
// //         let w= mnt4_G2::mul_by_a(ZZ) + (XX + XX + XX); // w   = a*ZZ + 3*XX
// //         let Y1Z1= (this->Y) * (this->Z);
// //         let s= Y1Z1 + Y1Z1;                            // s   = 2*Y1*Z1
// //         let ss= s.squared();                            // ss  = s^2
// //         let sss= s * ss;                                 // sss = s*ss
// //         let R= (this->Y) * s;                         // R   = Y1*s
// //         let RR= R.squared();                            // RR  = R^2
// //         let B= ((this->X)+R).squared()-XX-RR;         // B   = (X1+R)^2 - XX - RR
// //         let h= w.squared() - (B+B);                    // h   = w^2 - 2*B
// //         let X3= h * s;                                  // X3  = h*s
// //         let Y3= w * (B-h)-(RR+RR);                      // Y3  = w*(B-h) - 2*RR
// //         let Z3= sss;                                    // Z3  = sss

// //         return mnt4_G2(X3, Y3, Z3);
// //     }

// //     // if we have arrived here we are in the add case
// //     let Z1Z2= (this->Z) * (other.Z);      // Z1Z2 = Z1*Z2
// //     let u= Y2Z1 - Y1Z2;                  // u    = Y2*Z1-Y1Z2
// //     let uu= u.squared();                  // uu   = u^2
// //     let v= X2Z1 - X1Z2;                  // v    = X2*Z1-X1Z2
// //     let vv= v.squared();                  // vv   = v^2
// //     let vvv= v * vv;                       // vvv  = v*vv
// //     let R= vv * X1Z2;                    // R    = vv*X1Z2
// //     let A= uu * Z1Z2 - (vvv + R + R);    // A    = uu*Z1Z2 - vvv - 2*R
// //     let X3= v * A;                        // X3   = v*A
// //     let Y3= u * (R-A) - vvv * Y1Z2;       // Y3   = u*(R-A) - vvv*Y1Z2
// //     let Z3= vvv * Z1Z2;                   // Z3   = vvv*Z1Z2

// //     return mnt4_G2(X3, Y3, Z3);
// // }

// // mnt4_G2 mnt4_G2::operator-() const
// // {
// //     return mnt4_G2(this->X, -(this->Y), this->Z);
// // }


// // mnt4_G2 mnt4_G2::operator-(other:&mnt4_G2) const
// // {
// //     return (*this) + (-other);
// // }